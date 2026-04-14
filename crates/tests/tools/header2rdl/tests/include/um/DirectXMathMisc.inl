//-------------------------------------------------------------------------------------
// DirectXMathMisc.inl -- SIMD C++ Math library
//
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
//
// http://go.microsoft.com/fwlink/?LinkID=615560
//-------------------------------------------------------------------------------------

#pragma once

/****************************************************************************
 *
 * Quaternion
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Comparison operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------

inline bool XM_CALLCONV XMQuaternionEqual
(
    FXMVECTOR Q1,
    FXMVECTOR Q2
) noexcept
{
    return XMVector4Equal(Q1, Q2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMQuaternionNotEqual
(
    FXMVECTOR Q1,
    FXMVECTOR Q2
) noexcept
{
    return XMVector4NotEqual(Q1, Q2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMQuaternionIsNaN(FXMVECTOR Q) noexcept
{
    return XMVector4IsNaN(Q);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMQuaternionIsInfinite(FXMVECTOR Q) noexcept
{
    return XMVector4IsInfinite(Q);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMQuaternionIsIdentity(FXMVECTOR Q) noexcept
{
    return XMVector4Equal(Q, g_XMIdentityR3.v);
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionDot
(
    FXMVECTOR Q1,
    FXMVECTOR Q2
) noexcept
{
    return XMVector4Dot(Q1, Q2);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionMultiply
(
    FXMVECTOR Q1,
    FXMVECTOR Q2
) noexcept
{
    // Returns the product Q2*Q1 (which is the concatenation of a rotation Q1 followed by the rotation Q2)

    // [ (Q2.w * Q1.x) + (Q2.x * Q1.w) + (Q2.y * Q1.z) - (Q2.z * Q1.y),
    //   (Q2.w * Q1.y) - (Q2.x * Q1.z) + (Q2.y * Q1.w) + (Q2.z * Q1.x),
    //   (Q2.w * Q1.z) + (Q2.x * Q1.y) - (Q2.y * Q1.x) + (Q2.z * Q1.w),
    //   (Q2.w * Q1.w) - (Q2.x * Q1.x) - (Q2.y * Q1.y) - (Q2.z * Q1.z) ]

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            (Q2.vector4_f32[3] * Q1.vector4_f32[0]) + (Q2.vector4_f32[0] * Q1.vector4_f32[3]) + (Q2.vector4_f32[1] * Q1.vector4_f32[2]) - (Q2.vector4_f32[2] * Q1.vector4_f32[1]),
            (Q2.vector4_f32[3] * Q1.vector4_f32[1]) - (Q2.vector4_f32[0] * Q1.vector4_f32[2]) + (Q2.vector4_f32[1] * Q1.vector4_f32[3]) + (Q2.vector4_f32[2] * Q1.vector4_f32[0]),
            (Q2.vector4_f32[3] * Q1.vector4_f32[2]) + (Q2.vector4_f32[0] * Q1.vector4_f32[1]) - (Q2.vector4_f32[1] * Q1.vector4_f32[0]) + (Q2.vector4_f32[2] * Q1.vector4_f32[3]),
            (Q2.vector4_f32[3] * Q1.vector4_f32[3]) - (Q2.vector4_f32[0] * Q1.vector4_f32[0]) - (Q2.vector4_f32[1] * Q1.vector4_f32[1]) - (Q2.vector4_f32[2] * Q1.vector4_f32[2])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 ControlWZYX = { { { 1.0f, -1.0f, 1.0f, -1.0f } } };
    static const XMVECTORF32 ControlZWXY = { { { 1.0f, 1.0f, -1.0f, -1.0f } } };
    static const XMVECTORF32 ControlYXWZ = { { { -1.0f, 1.0f, 1.0f, -1.0f } } };

    float32x2_t Q2L = vget_low_f32(Q2);
    float32x2_t Q2H = vget_high_f32(Q2);

    float32x4_t Q2X = vdupq_lane_f32(Q2L, 0);
    float32x4_t Q2Y = vdupq_lane_f32(Q2L, 1);
    float32x4_t Q2Z = vdupq_lane_f32(Q2H, 0);
    XMVECTOR vResult = vmulq_lane_f32(Q1, Q2H, 1);

    // Mul by Q1WZYX
    float32x4_t vTemp = vrev64q_f32(Q1);
    vTemp = vcombine_f32(vget_high_f32(vTemp), vget_low_f32(vTemp));
    Q2X = vmulq_f32(Q2X, vTemp);
    vResult = vmlaq_f32(vResult, Q2X, ControlWZYX);

    // Mul by Q1ZWXY
    vTemp = vreinterpretq_f32_u32(vrev64q_u32(vreinterpretq_u32_f32(vTemp)));
    Q2Y = vmulq_f32(Q2Y, vTemp);
    vResult = vmlaq_f32(vResult, Q2Y, ControlZWXY);

    // Mul by Q1YXWZ
    vTemp = vreinterpretq_f32_u32(vrev64q_u32(vreinterpretq_u32_f32(vTemp)));
    vTemp = vcombine_f32(vget_high_f32(vTemp), vget_low_f32(vTemp));
    Q2Z = vmulq_f32(Q2Z, vTemp);
    vResult = vmlaq_f32(vResult, Q2Z, ControlYXWZ);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ControlWZYX = { { { 1.0f, -1.0f, 1.0f, -1.0f } } };
    static const XMVECTORF32 ControlZWXY = { { { 1.0f, 1.0f, -1.0f, -1.0f } } };
    static const XMVECTORF32 ControlYXWZ = { { { -1.0f, 1.0f, 1.0f, -1.0f } } };
    // Copy to SSE registers and use as few as possible for x86
    XMVECTOR Q2X = Q2;
    XMVECTOR Q2Y = Q2;
    XMVECTOR Q2Z = Q2;
    XMVECTOR vResult = Q2;
    // Splat with one instruction
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 3, 3, 3));
    Q2X = XM_PERMUTE_PS(Q2X, _MM_SHUFFLE(0, 0, 0, 0));
    Q2Y = XM_PERMUTE_PS(Q2Y, _MM_SHUFFLE(1, 1, 1, 1));
    Q2Z = XM_PERMUTE_PS(Q2Z, _MM_SHUFFLE(2, 2, 2, 2));
    // Retire Q1 and perform Q1*Q2W
    vResult = _mm_mul_ps(vResult, Q1);
    XMVECTOR Q1Shuffle = Q1;
    // Shuffle the copies of Q1
    Q1Shuffle = XM_PERMUTE_PS(Q1Shuffle, _MM_SHUFFLE(0, 1, 2, 3));
    // Mul by Q1WZYX
    Q2X = _mm_mul_ps(Q2X, Q1Shuffle);
    Q1Shuffle = XM_PERMUTE_PS(Q1Shuffle, _MM_SHUFFLE(2, 3, 0, 1));
    // Flip the signs on y and z
    vResult = XM_FMADD_PS(Q2X, ControlWZYX, vResult);
    // Mul by Q1ZWXY
    Q2Y = _mm_mul_ps(Q2Y, Q1Shuffle);
    Q1Shuffle = XM_PERMUTE_PS(Q1Shuffle, _MM_SHUFFLE(0, 1, 2, 3));
    // Flip the signs on z and w
    Q2Y = _mm_mul_ps(Q2Y, ControlZWXY);
    // Mul by Q1YXWZ
    Q2Z = _mm_mul_ps(Q2Z, Q1Shuffle);
    // Flip the signs on x and w
    Q2Y = XM_FMADD_PS(Q2Z, ControlYXWZ, Q2Y);
    vResult = _mm_add_ps(vResult, Q2Y);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionLengthSq(FXMVECTOR Q) noexcept
{
    return XMVector4LengthSq(Q);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionReciprocalLength(FXMVECTOR Q) noexcept
{
    return XMVector4ReciprocalLength(Q);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionLength(FXMVECTOR Q) noexcept
{
    return XMVector4Length(Q);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionNormalizeEst(FXMVECTOR Q) noexcept
{
    return XMVector4NormalizeEst(Q);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionNormalize(FXMVECTOR Q) noexcept
{
    return XMVector4Normalize(Q);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionConjugate(FXMVECTOR Q) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            -Q.vector4_f32[0],
            -Q.vector4_f32[1],
            -Q.vector4_f32[2],
            Q.vector4_f32[3]
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 NegativeOne3 = { { { -1.0f, -1.0f, -1.0f, 1.0f } } };
    return vmulq_f32(Q, NegativeOne3.v);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 NegativeOne3 = { { { -1.0f, -1.0f, -1.0f, 1.0f } } };
    return _mm_mul_ps(Q, NegativeOne3);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionInverse(FXMVECTOR Q) noexcept
{
    XMVECTOR L = XMVector4LengthSq(Q);
    XMVECTOR Conjugate = XMQuaternionConjugate(Q);

    XMVECTOR Control = XMVectorLessOrEqual(L, g_XMEpsilon.v);

    XMVECTOR Result = XMVectorDivide(Conjugate, L);

    Result = XMVectorSelect(Result, g_XMZero, Control);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionLn(FXMVECTOR Q) noexcept
{
    static const XMVECTORF32 OneMinusEpsilon = { { { 1.0f - 0.00001f, 1.0f - 0.00001f, 1.0f - 0.00001f, 1.0f - 0.00001f } } };

    XMVECTOR QW = XMVectorSplatW(Q);
    XMVECTOR Q0 = XMVectorSelect(g_XMSelect1110.v, Q, g_XMSelect1110.v);

    XMVECTOR ControlW = XMVectorInBounds(QW, OneMinusEpsilon.v);

    XMVECTOR Theta = XMVectorACos(QW);
    XMVECTOR SinTheta = XMVectorSin(Theta);

    XMVECTOR S = XMVectorDivide(Theta, SinTheta);

    XMVECTOR Result = XMVectorMultiply(Q0, S);
    Result = XMVectorSelect(Q0, Result, ControlW);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionExp(FXMVECTOR Q) noexcept
{
    XMVECTOR Theta = XMVector3Length(Q);

    XMVECTOR SinTheta, CosTheta;
    XMVectorSinCos(&SinTheta, &CosTheta, Theta);

    XMVECTOR S = XMVectorDivide(SinTheta, Theta);

    XMVECTOR Result = XMVectorMultiply(Q, S);

    const XMVECTOR Zero = XMVectorZero();
    XMVECTOR Control = XMVectorNearEqual(Theta, Zero, g_XMEpsilon.v);
    Result = XMVectorSelect(Result, Q, Control);

    Result = XMVectorSelect(CosTheta, Result, g_XMSelect1110.v);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionSlerp
(
    FXMVECTOR Q0,
    FXMVECTOR Q1,
    float    t
) noexcept
{
    XMVECTOR T = XMVectorReplicate(t);
    return XMQuaternionSlerpV(Q0, Q1, T);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionSlerpV
(
    FXMVECTOR Q0,
    FXMVECTOR Q1,
    FXMVECTOR T
) noexcept
{
    assert((XMVectorGetY(T) == XMVectorGetX(T)) && (XMVectorGetZ(T) == XMVectorGetX(T)) && (XMVectorGetW(T) == XMVectorGetX(T)));

    // Result = Q0 * sin((1.0 - t) * Omega) / sin(Omega) + Q1 * sin(t * Omega) / sin(Omega)

#if defined(_XM_NO_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    const XMVECTORF32 OneMinusEpsilon = { { { 1.0f - 0.00001f, 1.0f - 0.00001f, 1.0f - 0.00001f, 1.0f - 0.00001f } } };

    XMVECTOR CosOmega = XMQuaternionDot(Q0, Q1);

    const XMVECTOR Zero = XMVectorZero();
    XMVECTOR Control = XMVectorLess(CosOmega, Zero);
    XMVECTOR Sign = XMVectorSelect(g_XMOne.v, g_XMNegativeOne.v, Control);

    CosOmega = XMVectorMultiply(CosOmega, Sign);

    Control = XMVectorLess(CosOmega, OneMinusEpsilon);

    XMVECTOR SinOmega = XMVectorNegativeMultiplySubtract(CosOmega, CosOmega, g_XMOne.v);
    SinOmega = XMVectorSqrt(SinOmega);

    XMVECTOR Omega = XMVectorATan2(SinOmega, CosOmega);

    XMVECTOR SignMask = XMVectorSplatSignMask();
    XMVECTOR V01 = XMVectorShiftLeft(T, Zero, 2);
    SignMask = XMVectorShiftLeft(SignMask, Zero, 3);
    V01 = XMVectorXorInt(V01, SignMask);
    V01 = XMVectorAdd(g_XMIdentityR0.v, V01);

    XMVECTOR InvSinOmega = XMVectorReciprocal(SinOmega);

    XMVECTOR S0 = XMVectorMultiply(V01, Omega);
    S0 = XMVectorSin(S0);
    S0 = XMVectorMultiply(S0, InvSinOmega);

    S0 = XMVectorSelect(V01, S0, Control);

    XMVECTOR S1 = XMVectorSplatY(S0);
    S0 = XMVectorSplatX(S0);

    S1 = XMVectorMultiply(S1, Sign);

    XMVECTOR Result = XMVectorMultiply(Q0, S0);
    Result = XMVectorMultiplyAdd(Q1, S1, Result);

    return Result;

#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 OneMinusEpsilon = { { { 1.0f - 0.00001f, 1.0f - 0.00001f, 1.0f - 0.00001f, 1.0f - 0.00001f } } };
    static const XMVECTORU32 SignMask2 = { { { 0x80000000, 0x00000000, 0x00000000, 0x00000000 } } };

    XMVECTOR CosOmega = XMQuaternionDot(Q0, Q1);

    const XMVECTOR Zero = XMVectorZero();
    XMVECTOR Control = XMVectorLess(CosOmega, Zero);
    XMVECTOR Sign = XMVectorSelect(g_XMOne, g_XMNegativeOne, Control);

    CosOmega = _mm_mul_ps(CosOmega, Sign);

    Control = XMVectorLess(CosOmega, OneMinusEpsilon);

    XMVECTOR SinOmega = _mm_mul_ps(CosOmega, CosOmega);
    SinOmega = _mm_sub_ps(g_XMOne, SinOmega);
    SinOmega = _mm_sqrt_ps(SinOmega);

    XMVECTOR Omega = XMVectorATan2(SinOmega, CosOmega);

    XMVECTOR V01 = XM_PERMUTE_PS(T, _MM_SHUFFLE(2, 3, 0, 1));
    V01 = _mm_and_ps(V01, g_XMMaskXY);
    V01 = _mm_xor_ps(V01, SignMask2);
    V01 = _mm_add_ps(g_XMIdentityR0, V01);

    XMVECTOR S0 = _mm_mul_ps(V01, Omega);
    S0 = XMVectorSin(S0);
    S0 = _mm_div_ps(S0, SinOmega);

    S0 = XMVectorSelect(V01, S0, Control);

    XMVECTOR S1 = XMVectorSplatY(S0);
    S0 = XMVectorSplatX(S0);

    S1 = _mm_mul_ps(S1, Sign);
    XMVECTOR Result = _mm_mul_ps(Q0, S0);
    S1 = _mm_mul_ps(S1, Q1);
    Result = _mm_add_ps(Result, S1);
    return Result;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionSquad
(
    FXMVECTOR Q0,
    FXMVECTOR Q1,
    FXMVECTOR Q2,
    GXMVECTOR Q3,
    float    t
) noexcept
{
    XMVECTOR T = XMVectorReplicate(t);
    return XMQuaternionSquadV(Q0, Q1, Q2, Q3, T);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionSquadV
(
    FXMVECTOR Q0,
    FXMVECTOR Q1,
    FXMVECTOR Q2,
    GXMVECTOR Q3,
    HXMVECTOR T
) noexcept
{
    assert((XMVectorGetY(T) == XMVectorGetX(T)) && (XMVectorGetZ(T) == XMVectorGetX(T)) && (XMVectorGetW(T) == XMVectorGetX(T)));

    XMVECTOR TP = T;
    const XMVECTOR Two = XMVectorSplatConstant(2, 0);

    XMVECTOR Q03 = XMQuaternionSlerpV(Q0, Q3, T);
    XMVECTOR Q12 = XMQuaternionSlerpV(Q1, Q2, T);

    TP = XMVectorNegativeMultiplySubtract(TP, TP, TP);
    TP = XMVectorMultiply(TP, Two);

    XMVECTOR Result = XMQuaternionSlerpV(Q03, Q12, TP);

    return Result;
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMQuaternionSquadSetup
(
    XMVECTOR* pA,
    XMVECTOR* pB,
    XMVECTOR* pC,
    FXMVECTOR  Q0,
    FXMVECTOR  Q1,
    FXMVECTOR  Q2,
    GXMVECTOR  Q3
) noexcept
{
    assert(pA);
    assert(pB);
    assert(pC);

    XMVECTOR LS12 = XMQuaternionLengthSq(XMVectorAdd(Q1, Q2));
    XMVECTOR LD12 = XMQuaternionLengthSq(XMVectorSubtract(Q1, Q2));
    XMVECTOR SQ2 = XMVectorNegate(Q2);

    XMVECTOR Control1 = XMVectorLess(LS12, LD12);
    SQ2 = XMVectorSelect(Q2, SQ2, Control1);

    XMVECTOR LS01 = XMQuaternionLengthSq(XMVectorAdd(Q0, Q1));
    XMVECTOR LD01 = XMQuaternionLengthSq(XMVectorSubtract(Q0, Q1));
    XMVECTOR SQ0 = XMVectorNegate(Q0);

    XMVECTOR LS23 = XMQuaternionLengthSq(XMVectorAdd(SQ2, Q3));
    XMVECTOR LD23 = XMQuaternionLengthSq(XMVectorSubtract(SQ2, Q3));
    XMVECTOR SQ3 = XMVectorNegate(Q3);

    XMVECTOR Control0 = XMVectorLess(LS01, LD01);
    XMVECTOR Control2 = XMVectorLess(LS23, LD23);

    SQ0 = XMVectorSelect(Q0, SQ0, Control0);
    SQ3 = XMVectorSelect(Q3, SQ3, Control2);

    XMVECTOR InvQ1 = XMQuaternionInverse(Q1);
    XMVECTOR InvQ2 = XMQuaternionInverse(SQ2);

    XMVECTOR LnQ0 = XMQuaternionLn(XMQuaternionMultiply(InvQ1, SQ0));
    XMVECTOR LnQ2 = XMQuaternionLn(XMQuaternionMultiply(InvQ1, SQ2));
    XMVECTOR LnQ1 = XMQuaternionLn(XMQuaternionMultiply(InvQ2, Q1));
    XMVECTOR LnQ3 = XMQuaternionLn(XMQuaternionMultiply(InvQ2, SQ3));

    const XMVECTOR NegativeOneQuarter = XMVectorSplatConstant(-1, 2);

    XMVECTOR ExpQ02 = XMVectorMultiply(XMVectorAdd(LnQ0, LnQ2), NegativeOneQuarter);
    XMVECTOR ExpQ13 = XMVectorMultiply(XMVectorAdd(LnQ1, LnQ3), NegativeOneQuarter);
    ExpQ02 = XMQuaternionExp(ExpQ02);
    ExpQ13 = XMQuaternionExp(ExpQ13);

    *pA = XMQuaternionMultiply(Q1, ExpQ02);
    *pB = XMQuaternionMultiply(SQ2, ExpQ13);
    *pC = SQ2;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionBaryCentric
(
    FXMVECTOR Q0,
    FXMVECTOR Q1,
    FXMVECTOR Q2,
    float    f,
    float    g
) noexcept
{
    float s = f + g;

    XMVECTOR Result;
    if ((s < 0.00001f) && (s > -0.00001f))
    {
        Result = Q0;
    }
    else
    {
        XMVECTOR Q01 = XMQuaternionSlerp(Q0, Q1, s);
        XMVECTOR Q02 = XMQuaternionSlerp(Q0, Q2, s);

        Result = XMQuaternionSlerp(Q01, Q02, g / s);
    }

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionBaryCentricV
(
    FXMVECTOR Q0,
    FXMVECTOR Q1,
    FXMVECTOR Q2,
    GXMVECTOR F,
    HXMVECTOR G
) noexcept
{
    assert((XMVectorGetY(F) == XMVectorGetX(F)) && (XMVectorGetZ(F) == XMVectorGetX(F)) && (XMVectorGetW(F) == XMVectorGetX(F)));
    assert((XMVectorGetY(G) == XMVectorGetX(G)) && (XMVectorGetZ(G) == XMVectorGetX(G)) && (XMVectorGetW(G) == XMVectorGetX(G)));

    const XMVECTOR Epsilon = XMVectorSplatConstant(1, 16);

    XMVECTOR S = XMVectorAdd(F, G);

    XMVECTOR Result;
    if (XMVector4InBounds(S, Epsilon))
    {
        Result = Q0;
    }
    else
    {
        XMVECTOR Q01 = XMQuaternionSlerpV(Q0, Q1, S);
        XMVECTOR Q02 = XMQuaternionSlerpV(Q0, Q2, S);
        XMVECTOR GS = XMVectorReciprocal(S);
        GS = XMVectorMultiply(G, GS);

        Result = XMQuaternionSlerpV(Q01, Q02, GS);
    }

    return Result;
}

//------------------------------------------------------------------------------
// Transformation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionIdentity() noexcept
{
    return g_XMIdentityR3.v;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionRotationRollPitchYaw
(
    float Pitch,
    float Yaw,
    float Roll
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    const float halfpitch = Pitch * 0.5f;
    float cp = cosf(halfpitch);
    float sp = sinf(halfpitch);

    const float halfyaw = Yaw * 0.5f;
    float cy = cosf(halfyaw);
    float sy = sinf(halfyaw);

    const float halfroll = Roll * 0.5f;
    float cr = cosf(halfroll);
    float sr = sinf(halfroll);

    XMVECTORF32 vResult = { { {
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
            sr * cp * cy - cr * sp * sy,
            cr * cp * cy + sr * sp * sy
        } } };
    return vResult;
#else
    XMVECTOR Angles = XMVectorSet(Pitch, Yaw, Roll, 0.0f);
    return XMQuaternionRotationRollPitchYawFromVector(Angles);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionRotationRollPitchYawFromVector
(
    FXMVECTOR Angles // <Pitch, Yaw, Roll, 0>
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    const float halfpitch = Angles.vector4_f32[0] * 0.5f;
    float cp = cosf(halfpitch);
    float sp = sinf(halfpitch);

    const float halfyaw = Angles.vector4_f32[1] * 0.5f;
    float cy = cosf(halfyaw);
    float sy = sinf(halfyaw);

    const float halfroll = Angles.vector4_f32[2] * 0.5f;
    float cr = cosf(halfroll);
    float sr = sinf(halfroll);

    XMVECTORF32 vResult = { { {
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
            sr * cp * cy - cr * sp * sy,
            cr * cp * cy + sr * sp * sy
        } } };
    return vResult;
#else
    static const XMVECTORF32  Sign = { { { 1.0f, -1.0f, -1.0f, 1.0f } } };

    XMVECTOR HalfAngles = XMVectorMultiply(Angles, g_XMOneHalf.v);

    XMVECTOR SinAngles, CosAngles;
    XMVectorSinCos(&SinAngles, &CosAngles, HalfAngles);

    XMVECTOR P0 = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_1X, XM_PERMUTE_1X, XM_PERMUTE_1X>(SinAngles, CosAngles);
    XMVECTOR Y0 = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0Y, XM_PERMUTE_1Y, XM_PERMUTE_1Y>(SinAngles, CosAngles);
    XMVECTOR R0 = XMVectorPermute<XM_PERMUTE_1Z, XM_PERMUTE_1Z, XM_PERMUTE_0Z, XM_PERMUTE_1Z>(SinAngles, CosAngles);
    XMVECTOR P1 = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_1X, XM_PERMUTE_1X, XM_PERMUTE_1X>(CosAngles, SinAngles);
    XMVECTOR Y1 = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0Y, XM_PERMUTE_1Y, XM_PERMUTE_1Y>(CosAngles, SinAngles);
    XMVECTOR R1 = XMVectorPermute<XM_PERMUTE_1Z, XM_PERMUTE_1Z, XM_PERMUTE_0Z, XM_PERMUTE_1Z>(CosAngles, SinAngles);

    XMVECTOR Q1 = XMVectorMultiply(P1, Sign.v);
    XMVECTOR Q0 = XMVectorMultiply(P0, Y0);
    Q1 = XMVectorMultiply(Q1, Y1);
    Q0 = XMVectorMultiply(Q0, R0);
    XMVECTOR Q = XMVectorMultiplyAdd(Q1, R1, Q0);

    return Q;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionRotationNormal
(
    FXMVECTOR NormalAxis,
    float    Angle
) noexcept
{
#if defined(_XM_NO_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    XMVECTOR N = XMVectorSelect(g_XMOne.v, NormalAxis, g_XMSelect1110.v);

    float SinV, CosV;
    XMScalarSinCos(&SinV, &CosV, 0.5f * Angle);

    XMVECTOR Scale = XMVectorSet(SinV, SinV, SinV, CosV);
    return XMVectorMultiply(N, Scale);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR N = _mm_and_ps(NormalAxis, g_XMMask3);
    N = _mm_or_ps(N, g_XMIdentityR3);
    XMVECTOR Scale = _mm_set_ps1(0.5f * Angle);
    XMVECTOR vSine;
    XMVECTOR vCosine;
    XMVectorSinCos(&vSine, &vCosine, Scale);
    Scale = _mm_and_ps(vSine, g_XMMask3);
    vCosine = _mm_and_ps(vCosine, g_XMMaskW);
    Scale = _mm_or_ps(Scale, vCosine);
    N = _mm_mul_ps(N, Scale);
    return N;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionRotationAxis
(
    FXMVECTOR Axis,
    float    Angle
) noexcept
{
    assert(!XMVector3Equal(Axis, XMVectorZero()));
    assert(!XMVector3IsInfinite(Axis));

    XMVECTOR Normal = XMVector3Normalize(Axis);
    XMVECTOR Q = XMQuaternionRotationNormal(Normal, Angle);
    return Q;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMQuaternionRotationMatrix(FXMMATRIX M) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 q;
    float r22 = M.m[2][2];
    if (r22 <= 0.f)  // x^2 + y^2 >= z^2 + w^2
    {
        float dif10 = M.m[1][1] - M.m[0][0];
        float omr22 = 1.f - r22;
        if (dif10 <= 0.f)  // x^2 >= y^2
        {
            float fourXSqr = omr22 - dif10;
            float inv4x = 0.5f / sqrtf(fourXSqr);
            q.f[0] = fourXSqr * inv4x;
            q.f[1] = (M.m[0][1] + M.m[1][0]) * inv4x;
            q.f[2] = (M.m[0][2] + M.m[2][0]) * inv4x;
            q.f[3] = (M.m[1][2] - M.m[2][1]) * inv4x;
        }
        else  // y^2 >= x^2
        {
            float fourYSqr = omr22 + dif10;
            float inv4y = 0.5f / sqrtf(fourYSqr);
            q.f[0] = (M.m[0][1] + M.m[1][0]) * inv4y;
            q.f[1] = fourYSqr * inv4y;
            q.f[2] = (M.m[1][2] + M.m[2][1]) * inv4y;
            q.f[3] = (M.m[2][0] - M.m[0][2]) * inv4y;
        }
    }
    else  // z^2 + w^2 >= x^2 + y^2
    {
        float sum10 = M.m[1][1] + M.m[0][0];
        float opr22 = 1.f + r22;
        if (sum10 <= 0.f)  // z^2 >= w^2
        {
            float fourZSqr = opr22 - sum10;
            float inv4z = 0.5f / sqrtf(fourZSqr);
            q.f[0] = (M.m[0][2] + M.m[2][0]) * inv4z;
            q.f[1] = (M.m[1][2] + M.m[2][1]) * inv4z;
            q.f[2] = fourZSqr * inv4z;
            q.f[3] = (M.m[0][1] - M.m[1][0]) * inv4z;
        }
        else  // w^2 >= z^2
        {
            float fourWSqr = opr22 + sum10;
            float inv4w = 0.5f / sqrtf(fourWSqr);
            q.f[0] = (M.m[1][2] - M.m[2][1]) * inv4w;
            q.f[1] = (M.m[2][0] - M.m[0][2]) * inv4w;
            q.f[2] = (M.m[0][1] - M.m[1][0]) * inv4w;
            q.f[3] = fourWSqr * inv4w;
        }
    }
    return q.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 XMPMMP = { { { +1.0f, -1.0f, -1.0f, +1.0f } } };
    static const XMVECTORF32 XMMPMP = { { { -1.0f, +1.0f, -1.0f, +1.0f } } };
    static const XMVECTORF32 XMMMPP = { { { -1.0f, -1.0f, +1.0f, +1.0f } } };
    static const XMVECTORU32 Select0110 = { { { XM_SELECT_0, XM_SELECT_1, XM_SELECT_1, XM_SELECT_0 } } };
    static const XMVECTORU32 Select0010 = { { { XM_SELECT_0, XM_SELECT_0, XM_SELECT_1, XM_SELECT_0 } } };

    float32x4_t r0 = M.r[0];
    float32x4_t r1 = M.r[1];
    float32x4_t r2 = M.r[2];

    float32x4_t r00 = vdupq_lane_f32(vget_low_f32(r0), 0);
    float32x4_t r11 = vdupq_lane_f32(vget_low_f32(r1), 1);
    float32x4_t r22 = vdupq_lane_f32(vget_high_f32(r2), 0);

    // x^2 >= y^2 equivalent to r11 - r00 <= 0
    float32x4_t r11mr00 = vsubq_f32(r11, r00);
    uint32x4_t x2gey2 = vcleq_f32(r11mr00, g_XMZero);

    // z^2 >= w^2 equivalent to r11 + r00 <= 0
    float32x4_t r11pr00 = vaddq_f32(r11, r00);
    uint32x4_t z2gew2 = vcleq_f32(r11pr00, g_XMZero);

    // x^2 + y^2 >= z^2 + w^2 equivalent to r22 <= 0
    uint32x4_t x2py2gez2pw2 = vcleq_f32(r22, g_XMZero);

    // (4*x^2, 4*y^2, 4*z^2, 4*w^2)
    float32x4_t t0 = vmulq_f32(XMPMMP, r00);
    float32x4_t x2y2z2w2 = vmlaq_f32(t0, XMMPMP, r11);
    x2y2z2w2 = vmlaq_f32(x2y2z2w2, XMMMPP, r22);
    x2y2z2w2 = vaddq_f32(x2y2z2w2, g_XMOne);

    // (r01, r02, r12, r11)
    t0 = vextq_f32(r0, r0, 1);
    float32x4_t t1 = vextq_f32(r1, r1, 1);
    t0 = vcombine_f32(vget_low_f32(t0), vrev64_f32(vget_low_f32(t1)));

    // (r10, r20, r21, r10)
    t1 = vextq_f32(r2, r2, 3);
    float32x4_t r10 = vdupq_lane_f32(vget_low_f32(r1), 0);
    t1 = vbslq_f32(Select0110, t1, r10);

    // (4*x*y, 4*x*z, 4*y*z, unused)
    float32x4_t xyxzyz = vaddq_f32(t0, t1);

    // (r21, r20, r10, r10)
    t0 = vcombine_f32(vrev64_f32(vget_low_f32(r2)), vget_low_f32(r10));

    // (r12, r02, r01, r12)
    float32x4_t t2 = vcombine_f32(vrev64_f32(vget_high_f32(r0)), vrev64_f32(vget_low_f32(r0)));
    float32x4_t t3 = vdupq_lane_f32(vget_high_f32(r1), 0);
    t1 = vbslq_f32(Select0110, t2, t3);

    // (4*x*w, 4*y*w, 4*z*w, unused)
    float32x4_t xwywzw = vsubq_f32(t0, t1);
    xwywzw = vmulq_f32(XMMPMP, xwywzw);

    // (4*x*x, 4*x*y, 4*x*z, 4*x*w)
    t0 = vextq_f32(xyxzyz, xyxzyz, 3);
    t1 = vbslq_f32(Select0110, t0, x2y2z2w2);
    t2 = vdupq_lane_f32(vget_low_f32(xwywzw), 0);
    float32x4_t tensor0 = vbslq_f32(g_XMSelect1110, t1, t2);

    // (4*y*x, 4*y*y, 4*y*z, 4*y*w)
    t0 = vbslq_f32(g_XMSelect1011, xyxzyz, x2y2z2w2);
    t1 = vdupq_lane_f32(vget_low_f32(xwywzw), 1);
    float32x4_t tensor1 = vbslq_f32(g_XMSelect1110, t0, t1);

    // (4*z*x, 4*z*y, 4*z*z, 4*z*w)
    t0 = vextq_f32(xyxzyz, xyxzyz, 1);
    t1 = vcombine_f32(vget_low_f32(t0), vrev64_f32(vget_high_f32(xwywzw)));
    float32x4_t tensor2 = vbslq_f32(Select0010, x2y2z2w2, t1);

    // (4*w*x, 4*w*y, 4*w*z, 4*w*w)
    float32x4_t tensor3 = vbslq_f32(g_XMSelect1110, xwywzw, x2y2z2w2);

    // Select the row of the tensor-product matrix that has the largest
    // magnitude.
    t0 = vbslq_f32(x2gey2, tensor0, tensor1);
    t1 = vbslq_f32(z2gew2, tensor2, tensor3);
    t2 = vbslq_f32(x2py2gez2pw2, t0, t1);

    // Normalize the row.  No division by zero is possible because the
    // quaternion is unit-length (and the row is a nonzero multiple of
    // the quaternion).
    t0 = XMVector4Length(t2);
    return XMVectorDivide(t2, t0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 XMPMMP = { { { +1.0f, -1.0f, -1.0f, +1.0f } } };
    static const XMVECTORF32 XMMPMP = { { { -1.0f, +1.0f, -1.0f, +1.0f } } };
    static const XMVECTORF32 XMMMPP = { { { -1.0f, -1.0f, +1.0f, +1.0f } } };

    XMVECTOR r0 = M.r[0];  // (r00, r01, r02, 0)
    XMVECTOR r1 = M.r[1];  // (r10, r11, r12, 0)
    XMVECTOR r2 = M.r[2];  // (r20, r21, r22, 0)

    // (r00, r00, r00, r00)
    XMVECTOR r00 = XM_PERMUTE_PS(r0, _MM_SHUFFLE(0, 0, 0, 0));
    // (r11, r11, r11, r11)
    XMVECTOR r11 = XM_PERMUTE_PS(r1, _MM_SHUFFLE(1, 1, 1, 1));
    // (r22, r22, r22, r22)
    XMVECTOR r22 = XM_PERMUTE_PS(r2, _MM_SHUFFLE(2, 2, 2, 2));

    // x^2 >= y^2 equivalent to r11 - r00 <= 0
    // (r11 - r00, r11 - r00, r11 - r00, r11 - r00)
    XMVECTOR r11mr00 = _mm_sub_ps(r11, r00);
    XMVECTOR x2gey2 = _mm_cmple_ps(r11mr00, g_XMZero);

    // z^2 >= w^2 equivalent to r11 + r00 <= 0
    // (r11 + r00, r11 + r00, r11 + r00, r11 + r00)
    XMVECTOR r11pr00 = _mm_add_ps(r11, r00);
    XMVECTOR z2gew2 = _mm_cmple_ps(r11pr00, g_XMZero);

    // x^2 + y^2 >= z^2 + w^2 equivalent to r22 <= 0
    XMVECTOR x2py2gez2pw2 = _mm_cmple_ps(r22, g_XMZero);

    // (4*x^2, 4*y^2, 4*z^2, 4*w^2)
    XMVECTOR t0 = XM_FMADD_PS(XMPMMP, r00, g_XMOne);
    XMVECTOR t1 = _mm_mul_ps(XMMPMP, r11);
    XMVECTOR t2 = XM_FMADD_PS(XMMMPP, r22, t0);
    XMVECTOR x2y2z2w2 = _mm_add_ps(t1, t2);

    // (r01, r02, r12, r11)
    t0 = _mm_shuffle_ps(r0, r1, _MM_SHUFFLE(1, 2, 2, 1));
    // (r10, r10, r20, r21)
    t1 = _mm_shuffle_ps(r1, r2, _MM_SHUFFLE(1, 0, 0, 0));
    // (r10, r20, r21, r10)
    t1 = XM_PERMUTE_PS(t1, _MM_SHUFFLE(1, 3, 2, 0));
    // (4*x*y, 4*x*z, 4*y*z, unused)
    XMVECTOR xyxzyz = _mm_add_ps(t0, t1);

    // (r21, r20, r10, r10)
    t0 = _mm_shuffle_ps(r2, r1, _MM_SHUFFLE(0, 0, 0, 1));
    // (r12, r12, r02, r01)
    t1 = _mm_shuffle_ps(r1, r0, _MM_SHUFFLE(1, 2, 2, 2));
    // (r12, r02, r01, r12)
    t1 = XM_PERMUTE_PS(t1, _MM_SHUFFLE(1, 3, 2, 0));
    // (4*x*w, 4*y*w, 4*z*w, unused)
    XMVECTOR xwywzw = _mm_sub_ps(t0, t1);
    xwywzw = _mm_mul_ps(XMMPMP, xwywzw);

    // (4*x^2, 4*y^2, 4*x*y, unused)
    t0 = _mm_shuffle_ps(x2y2z2w2, xyxzyz, _MM_SHUFFLE(0, 0, 1, 0));
    // (4*z^2, 4*w^2, 4*z*w, unused)
    t1 = _mm_shuffle_ps(x2y2z2w2, xwywzw, _MM_SHUFFLE(0, 2, 3, 2));
    // (4*x*z, 4*y*z, 4*x*w, 4*y*w)
    t2 = _mm_shuffle_ps(xyxzyz, xwywzw, _MM_SHUFFLE(1, 0, 2, 1));

    // (4*x*x, 4*x*y, 4*x*z, 4*x*w)
    XMVECTOR tensor0 = _mm_shuffle_ps(t0, t2, _MM_SHUFFLE(2, 0, 2, 0));
    // (4*y*x, 4*y*y, 4*y*z, 4*y*w)
    XMVECTOR tensor1 = _mm_shuffle_ps(t0, t2, _MM_SHUFFLE(3, 1, 1, 2));
    // (4*z*x, 4*z*y, 4*z*z, 4*z*w)
    XMVECTOR tensor2 = _mm_shuffle_ps(t2, t1, _MM_SHUFFLE(2, 0, 1, 0));
    // (4*w*x, 4*w*y, 4*w*z, 4*w*w)
    XMVECTOR tensor3 = _mm_shuffle_ps(t2, t1, _MM_SHUFFLE(1, 2, 3, 2));

    // Select the row of the tensor-product matrix that has the largest
    // magnitude.
    t0 = _mm_and_ps(x2gey2, tensor0);
    t1 = _mm_andnot_ps(x2gey2, tensor1);
    t0 = _mm_or_ps(t0, t1);
    t1 = _mm_and_ps(z2gew2, tensor2);
    t2 = _mm_andnot_ps(z2gew2, tensor3);
    t1 = _mm_or_ps(t1, t2);
    t0 = _mm_and_ps(x2py2gez2pw2, t0);
    t1 = _mm_andnot_ps(x2py2gez2pw2, t1);
    t2 = _mm_or_ps(t0, t1);

    // Normalize the row.  No division by zero is possible because the
    // quaternion is unit-length (and the row is a nonzero multiple of
    // the quaternion).
    t0 = XMVector4Length(t2);
    return _mm_div_ps(t2, t0);
#endif
}

//------------------------------------------------------------------------------
// Conversion operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMQuaternionToAxisAngle
(
    XMVECTOR* pAxis,
    float* pAngle,
    FXMVECTOR  Q
) noexcept
{
    assert(pAxis);
    assert(pAngle);

    *pAxis = Q;

    *pAngle = 2.0f * XMScalarACos(XMVectorGetW(Q));
}

/****************************************************************************
 *
 * Plane
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Comparison operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------

inline bool XM_CALLCONV XMPlaneEqual
(
    FXMVECTOR P1,
    FXMVECTOR P2
) noexcept
{
    return XMVector4Equal(P1, P2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMPlaneNearEqual
(
    FXMVECTOR P1,
    FXMVECTOR P2,
    FXMVECTOR Epsilon
) noexcept
{
    XMVECTOR NP1 = XMPlaneNormalize(P1);
    XMVECTOR NP2 = XMPlaneNormalize(P2);
    return XMVector4NearEqual(NP1, NP2, Epsilon);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMPlaneNotEqual
(
    FXMVECTOR P1,
    FXMVECTOR P2
) noexcept
{
    return XMVector4NotEqual(P1, P2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMPlaneIsNaN(FXMVECTOR P) noexcept
{
    return XMVector4IsNaN(P);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMPlaneIsInfinite(FXMVECTOR P) noexcept
{
    return XMVector4IsInfinite(P);
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneDot
(
    FXMVECTOR P,
    FXMVECTOR V
) noexcept
{
    return XMVector4Dot(P, V);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneDotCoord
(
    FXMVECTOR P,
    FXMVECTOR V
) noexcept
{
    // Result = P[0] * V[0] + P[1] * V[1] + P[2] * V[2] + P[3]

    XMVECTOR V3 = XMVectorSelect(g_XMOne.v, V, g_XMSelect1110.v);
    XMVECTOR Result = XMVector4Dot(P, V3);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneDotNormal
(
    FXMVECTOR P,
    FXMVECTOR V
) noexcept
{
    return XMVector3Dot(P, V);
}

//------------------------------------------------------------------------------
// XMPlaneNormalizeEst uses a reciprocal estimate and
// returns QNaN on zero and infinite vectors.

inline XMVECTOR XM_CALLCONV XMPlaneNormalizeEst(FXMVECTOR P) noexcept
{
#if defined(_XM_NO_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    XMVECTOR Result = XMVector3ReciprocalLengthEst(P);
    return XMVectorMultiply(P, Result);

#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(P, P, 0x7f);
    XMVECTOR vResult = _mm_rsqrt_ps(vTemp);
    return _mm_mul_ps(vResult, P);
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product
    XMVECTOR vDot = _mm_mul_ps(P, P);
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
    // Get the reciprocal
    vDot = _mm_mul_ps(vDot, P);
    return vDot;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneNormalize(FXMVECTOR P) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float fLengthSq = sqrtf((P.vector4_f32[0] * P.vector4_f32[0]) + (P.vector4_f32[1] * P.vector4_f32[1]) + (P.vector4_f32[2] * P.vector4_f32[2]));
    // Prevent divide by zero
    if (fLengthSq > 0)
    {
        fLengthSq = 1.0f / fLengthSq;
    }
    XMVECTORF32 vResult = { { {
            P.vector4_f32[0] * fLengthSq,
            P.vector4_f32[1] * fLengthSq,
            P.vector4_f32[2] * fLengthSq,
            P.vector4_f32[3] * fLengthSq
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR vLength = XMVector3ReciprocalLength(P);
    return XMVectorMultiply(P, vLength);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_dp_ps(P, P, 0x7f);
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Reciprocal mul to perform the normalization
    vResult = _mm_div_ps(P, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vLengthSq);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y and z only
    XMVECTOR vLengthSq = _mm_mul_ps(P, P);
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 1, 2, 1));
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Reciprocal mul to perform the normalization
    vResult = _mm_div_ps(P, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vLengthSq);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneIntersectLine
(
    FXMVECTOR P,
    FXMVECTOR LinePoint1,
    FXMVECTOR LinePoint2
) noexcept
{
    XMVECTOR V1 = XMVector3Dot(P, LinePoint1);
    XMVECTOR V2 = XMVector3Dot(P, LinePoint2);
    XMVECTOR D = XMVectorSubtract(V1, V2);

    XMVECTOR VT = XMPlaneDotCoord(P, LinePoint1);
    VT = XMVectorDivide(VT, D);

    XMVECTOR Point = XMVectorSubtract(LinePoint2, LinePoint1);
    Point = XMVectorMultiplyAdd(Point, VT, LinePoint1);

    const XMVECTOR Zero = XMVectorZero();
    XMVECTOR Control = XMVectorNearEqual(D, Zero, g_XMEpsilon.v);

    return XMVectorSelect(Point, g_XMQNaN.v, Control);
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMPlaneIntersectPlane
(
    XMVECTOR* pLinePoint1,
    XMVECTOR* pLinePoint2,
    FXMVECTOR  P1,
    FXMVECTOR  P2
) noexcept
{
    assert(pLinePoint1);
    assert(pLinePoint2);

    XMVECTOR V1 = XMVector3Cross(P2, P1);

    XMVECTOR LengthSq = XMVector3LengthSq(V1);

    XMVECTOR V2 = XMVector3Cross(P2, V1);

    XMVECTOR P1W = XMVectorSplatW(P1);
    XMVECTOR Point = XMVectorMultiply(V2, P1W);

    XMVECTOR V3 = XMVector3Cross(V1, P1);

    XMVECTOR P2W = XMVectorSplatW(P2);
    Point = XMVectorMultiplyAdd(V3, P2W, Point);

    XMVECTOR LinePoint1 = XMVectorDivide(Point, LengthSq);

    XMVECTOR LinePoint2 = XMVectorAdd(LinePoint1, V1);

    XMVECTOR Control = XMVectorLessOrEqual(LengthSq, g_XMEpsilon.v);
    *pLinePoint1 = XMVectorSelect(LinePoint1, g_XMQNaN.v, Control);
    *pLinePoint2 = XMVectorSelect(LinePoint2, g_XMQNaN.v, Control);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneTransform
(
    FXMVECTOR P,
    FXMMATRIX ITM
) noexcept
{
    XMVECTOR W = XMVectorSplatW(P);
    XMVECTOR Z = XMVectorSplatZ(P);
    XMVECTOR Y = XMVectorSplatY(P);
    XMVECTOR X = XMVectorSplatX(P);

    XMVECTOR Result = XMVectorMultiply(W, ITM.r[3]);
    Result = XMVectorMultiplyAdd(Z, ITM.r[2], Result);
    Result = XMVectorMultiplyAdd(Y, ITM.r[1], Result);
    Result = XMVectorMultiplyAdd(X, ITM.r[0], Result);
    return Result;
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMFLOAT4* XM_CALLCONV XMPlaneTransformStream
(
    XMFLOAT4*       pOutputStream,
    size_t          OutputStride,
    const XMFLOAT4* pInputStream,
    size_t          InputStride,
    size_t          PlaneCount,
    FXMMATRIX       ITM
) noexcept
{
    return XMVector4TransformStream(pOutputStream,
        OutputStride,
        pInputStream,
        InputStride,
        PlaneCount,
        ITM);
}

//------------------------------------------------------------------------------
// Conversion operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneFromPointNormal
(
    FXMVECTOR Point,
    FXMVECTOR Normal
) noexcept
{
    XMVECTOR W = XMVector3Dot(Point, Normal);
    W = XMVectorNegate(W);
    return XMVectorSelect(W, Normal, g_XMSelect1110.v);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMPlaneFromPoints
(
    FXMVECTOR Point1,
    FXMVECTOR Point2,
    FXMVECTOR Point3
) noexcept
{
    XMVECTOR V21 = XMVectorSubtract(Point1, Point2);
    XMVECTOR V31 = XMVectorSubtract(Point1, Point3);

    XMVECTOR N = XMVector3Cross(V21, V31);
    N = XMVector3Normalize(N);

    XMVECTOR D = XMPlaneDotNormal(N, Point1);
    D = XMVectorNegate(D);

    XMVECTOR Result = XMVectorSelect(D, N, g_XMSelect1110.v);

    return Result;
}

/****************************************************************************
 *
 * Color
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Comparison operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorEqual
(
    FXMVECTOR C1,
    FXMVECTOR C2
) noexcept
{
    return XMVector4Equal(C1, C2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorNotEqual
(
    FXMVECTOR C1,
    FXMVECTOR C2
) noexcept
{
    return XMVector4NotEqual(C1, C2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorGreater
(
    FXMVECTOR C1,
    FXMVECTOR C2
) noexcept
{
    return XMVector4Greater(C1, C2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorGreaterOrEqual
(
    FXMVECTOR C1,
    FXMVECTOR C2
) noexcept
{
    return XMVector4GreaterOrEqual(C1, C2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorLess
(
    FXMVECTOR C1,
    FXMVECTOR C2
) noexcept
{
    return XMVector4Less(C1, C2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorLessOrEqual
(
    FXMVECTOR C1,
    FXMVECTOR C2
) noexcept
{
    return XMVector4LessOrEqual(C1, C2);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorIsNaN(FXMVECTOR C) noexcept
{
    return XMVector4IsNaN(C);
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMColorIsInfinite(FXMVECTOR C) noexcept
{
    return XMVector4IsInfinite(C);
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorNegative(FXMVECTOR vColor) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            1.0f - vColor.vector4_f32[0],
            1.0f - vColor.vector4_f32[1],
            1.0f - vColor.vector4_f32[2],
            vColor.vector4_f32[3]
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vTemp = veorq_u32(vreinterpretq_u32_f32(vColor), g_XMNegate3);
    return vaddq_f32(vreinterpretq_f32_u32(vTemp), g_XMOne3);
#elif defined(_XM_SSE_INTRINSICS_)
    // Negate only x,y and z.
    XMVECTOR vTemp = _mm_xor_ps(vColor, g_XMNegate3);
    // Add 1,1,1,0 to -x,-y,-z,w
    return _mm_add_ps(vTemp, g_XMOne3);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorModulate
(
    FXMVECTOR C1,
    FXMVECTOR C2
) noexcept
{
    return XMVectorMultiply(C1, C2);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorAdjustSaturation
(
    FXMVECTOR vColor,
    float    fSaturation
) noexcept
{
    // Luminance = 0.2125f * C[0] + 0.7154f * C[1] + 0.0721f * C[2];
    // Result = (C - Luminance) * Saturation + Luminance;

    const XMVECTORF32 gvLuminance = { { { 0.2125f, 0.7154f, 0.0721f, 0.0f } } };
#if defined(_XM_NO_INTRINSICS_)
    float fLuminance = (vColor.vector4_f32[0] * gvLuminance.f[0]) + (vColor.vector4_f32[1] * gvLuminance.f[1]) + (vColor.vector4_f32[2] * gvLuminance.f[2]);
    XMVECTOR vResult;
    vResult.vector4_f32[0] = ((vColor.vector4_f32[0] - fLuminance) * fSaturation) + fLuminance;
    vResult.vector4_f32[1] = ((vColor.vector4_f32[1] - fLuminance) * fSaturation) + fLuminance;
    vResult.vector4_f32[2] = ((vColor.vector4_f32[2] - fLuminance) * fSaturation) + fLuminance;
    vResult.vector4_f32[3] = vColor.vector4_f32[3];
    return vResult;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR vLuminance = XMVector3Dot(vColor, gvLuminance);
    XMVECTOR vResult = vsubq_f32(vColor, vLuminance);
    vResult = vmlaq_n_f32(vLuminance, vResult, fSaturation);
    return vbslq_f32(g_XMSelect1110, vResult, vColor);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vLuminance = XMVector3Dot(vColor, gvLuminance);
    // Splat fSaturation
    XMVECTOR vSaturation = _mm_set_ps1(fSaturation);
    // vResult = ((vColor-vLuminance)*vSaturation)+vLuminance;
    XMVECTOR vResult = _mm_sub_ps(vColor, vLuminance);
    vResult = XM_FMADD_PS(vResult, vSaturation, vLuminance);
    // Retain w from the source color
    vLuminance = _mm_shuffle_ps(vResult, vColor, _MM_SHUFFLE(3, 2, 2, 2));   // x = vResult.z,y = vResult.z,z = vColor.z,w=vColor.w
    vResult = _mm_shuffle_ps(vResult, vLuminance, _MM_SHUFFLE(3, 0, 1, 0));  // x = vResult.x,y = vResult.y,z = vResult.z,w=vColor.w
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorAdjustContrast
(
    FXMVECTOR vColor,
    float    fContrast
) noexcept
{
    // Result = (vColor - 0.5f) * fContrast + 0.5f;

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            ((vColor.vector4_f32[0] - 0.5f) * fContrast) + 0.5f,
            ((vColor.vector4_f32[1] - 0.5f) * fContrast) + 0.5f,
            ((vColor.vector4_f32[2] - 0.5f) * fContrast) + 0.5f,
            vColor.vector4_f32[3]        // Leave W untouched
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR vResult = vsubq_f32(vColor, g_XMOneHalf.v);
    vResult = vmlaq_n_f32(g_XMOneHalf.v, vResult, fContrast);
    return vbslq_f32(g_XMSelect1110, vResult, vColor);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vScale = _mm_set_ps1(fContrast);           // Splat the scale
    XMVECTOR vResult = _mm_sub_ps(vColor, g_XMOneHalf);  // Subtract 0.5f from the source (Saving source)
    vResult = XM_FMADD_PS(vResult, vScale, g_XMOneHalf);
// Retain w from the source color
    vScale = _mm_shuffle_ps(vResult, vColor, _MM_SHUFFLE(3, 2, 2, 2));   // x = vResult.z,y = vResult.z,z = vColor.z,w=vColor.w
    vResult = _mm_shuffle_ps(vResult, vScale, _MM_SHUFFLE(3, 0, 1, 0));  // x = vResult.x,y = vResult.y,z = vResult.z,w=vColor.w
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorRGBToHSL(FXMVECTOR rgb) noexcept
{
    XMVECTOR r = XMVectorSplatX(rgb);
    XMVECTOR g = XMVectorSplatY(rgb);
    XMVECTOR b = XMVectorSplatZ(rgb);

    XMVECTOR min = XMVectorMin(r, XMVectorMin(g, b));
    XMVECTOR max = XMVectorMax(r, XMVectorMax(g, b));

    XMVECTOR l = XMVectorMultiply(XMVectorAdd(min, max), g_XMOneHalf);

    XMVECTOR d = XMVectorSubtract(max, min);

    XMVECTOR la = XMVectorSelect(rgb, l, g_XMSelect1110);

    if (XMVector3Less(d, g_XMEpsilon))
    {
        // Achromatic, assume H and S of 0
        return XMVectorSelect(la, g_XMZero, g_XMSelect1100);
    }
    else
    {
        XMVECTOR s, h;

        XMVECTOR d2 = XMVectorAdd(min, max);

        if (XMVector3Greater(l, g_XMOneHalf))
        {
            // d / (2-max-min)
            s = XMVectorDivide(d, XMVectorSubtract(g_XMTwo, d2));
        }
        else
        {
            // d / (max+min)
            s = XMVectorDivide(d, d2);
        }

        if (XMVector3Equal(r, max))
        {
            // Red is max
            h = XMVectorDivide(XMVectorSubtract(g, b), d);
        }
        else if (XMVector3Equal(g, max))
        {
            // Green is max
            h = XMVectorDivide(XMVectorSubtract(b, r), d);
            h = XMVectorAdd(h, g_XMTwo);
        }
        else
        {
            // Blue is max
            h = XMVectorDivide(XMVectorSubtract(r, g), d);
            h = XMVectorAdd(h, g_XMFour);
        }

        h = XMVectorDivide(h, g_XMSix);

        if (XMVector3Less(h, g_XMZero))
            h = XMVectorAdd(h, g_XMOne);

        XMVECTOR lha = XMVectorSelect(la, h, g_XMSelect1100);
        return XMVectorSelect(s, lha, g_XMSelect1011);
    }
}

//------------------------------------------------------------------------------

namespace Internal
{

    inline XMVECTOR XM_CALLCONV XMColorHue2Clr(FXMVECTOR p, FXMVECTOR q, FXMVECTOR h) noexcept
    {
        static const XMVECTORF32 oneSixth = { { { 1.0f / 6.0f, 1.0f / 6.0f, 1.0f / 6.0f, 1.0f / 6.0f } } };
        static const XMVECTORF32 twoThirds = { { { 2.0f / 3.0f, 2.0f / 3.0f, 2.0f / 3.0f, 2.0f / 3.0f } } };

        XMVECTOR t = h;

        if (XMVector3Less(t, g_XMZero))
            t = XMVectorAdd(t, g_XMOne);

        if (XMVector3Greater(t, g_XMOne))
            t = XMVectorSubtract(t, g_XMOne);

        if (XMVector3Less(t, oneSixth))
        {
            // p + (q - p) * 6 * t
            XMVECTOR t1 = XMVectorSubtract(q, p);
            XMVECTOR t2 = XMVectorMultiply(g_XMSix, t);
            return XMVectorMultiplyAdd(t1, t2, p);
        }

        if (XMVector3Less(t, g_XMOneHalf))
            return q;

        if (XMVector3Less(t, twoThirds))
        {
            // p + (q - p) * 6 * (2/3 - t)
            XMVECTOR t1 = XMVectorSubtract(q, p);
            XMVECTOR t2 = XMVectorMultiply(g_XMSix, XMVectorSubtract(twoThirds, t));
            return XMVectorMultiplyAdd(t1, t2, p);
        }

        return p;
    }

} // namespace Internal

inline XMVECTOR XM_CALLCONV XMColorHSLToRGB(FXMVECTOR hsl) noexcept
{
    static const XMVECTORF32 oneThird = { { { 1.0f / 3.0f, 1.0f / 3.0f, 1.0f / 3.0f, 1.0f / 3.0f } } };

    XMVECTOR s = XMVectorSplatY(hsl);
    XMVECTOR l = XMVectorSplatZ(hsl);

    if (XMVector3NearEqual(s, g_XMZero, g_XMEpsilon))
    {
        // Achromatic
        return XMVectorSelect(hsl, l, g_XMSelect1110);
    }
    else
    {
        XMVECTOR h = XMVectorSplatX(hsl);

        XMVECTOR q;
        if (XMVector3Less(l, g_XMOneHalf))
        {
            q = XMVectorMultiply(l, XMVectorAdd(g_XMOne, s));
        }
        else
        {
            q = XMVectorSubtract(XMVectorAdd(l, s), XMVectorMultiply(l, s));
        }

        XMVECTOR p = XMVectorSubtract(XMVectorMultiply(g_XMTwo, l), q);

        XMVECTOR r = DirectX::Internal::XMColorHue2Clr(p, q, XMVectorAdd(h, oneThird));
        XMVECTOR g = DirectX::Internal::XMColorHue2Clr(p, q, h);
        XMVECTOR b = DirectX::Internal::XMColorHue2Clr(p, q, XMVectorSubtract(h, oneThird));

        XMVECTOR rg = XMVectorSelect(g, r, g_XMSelect1000);
        XMVECTOR ba = XMVectorSelect(hsl, b, g_XMSelect1110);

        return XMVectorSelect(ba, rg, g_XMSelect1100);
    }
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorRGBToHSV(FXMVECTOR rgb) noexcept
{
    XMVECTOR r = XMVectorSplatX(rgb);
    XMVECTOR g = XMVectorSplatY(rgb);
    XMVECTOR b = XMVectorSplatZ(rgb);

    XMVECTOR min = XMVectorMin(r, XMVectorMin(g, b));
    XMVECTOR v = XMVectorMax(r, XMVectorMax(g, b));

    XMVECTOR d = XMVectorSubtract(v, min);

    XMVECTOR s = (XMVector3NearEqual(v, g_XMZero, g_XMEpsilon)) ? g_XMZero : XMVectorDivide(d, v);

    if (XMVector3Less(d, g_XMEpsilon))
    {
        // Achromatic, assume H of 0
        XMVECTOR hv = XMVectorSelect(v, g_XMZero, g_XMSelect1000);
        XMVECTOR hva = XMVectorSelect(rgb, hv, g_XMSelect1110);
        return XMVectorSelect(s, hva, g_XMSelect1011);
    }
    else
    {
        XMVECTOR h;

        if (XMVector3Equal(r, v))
        {
            // Red is max
            h = XMVectorDivide(XMVectorSubtract(g, b), d);

            if (XMVector3Less(g, b))
                h = XMVectorAdd(h, g_XMSix);
        }
        else if (XMVector3Equal(g, v))
        {
            // Green is max
            h = XMVectorDivide(XMVectorSubtract(b, r), d);
            h = XMVectorAdd(h, g_XMTwo);
        }
        else
        {
            // Blue is max
            h = XMVectorDivide(XMVectorSubtract(r, g), d);
            h = XMVectorAdd(h, g_XMFour);
        }

        h = XMVectorDivide(h, g_XMSix);

        XMVECTOR hv = XMVectorSelect(v, h, g_XMSelect1000);
        XMVECTOR hva = XMVectorSelect(rgb, hv, g_XMSelect1110);
        return XMVectorSelect(s, hva, g_XMSelect1011);
    }
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorHSVToRGB(FXMVECTOR hsv) noexcept
{
    XMVECTOR h = XMVectorSplatX(hsv);
    XMVECTOR s = XMVectorSplatY(hsv);
    XMVECTOR v = XMVectorSplatZ(hsv);

    XMVECTOR h6 = XMVectorMultiply(h, g_XMSix);

    XMVECTOR i = XMVectorFloor(h6);
    XMVECTOR f = XMVectorSubtract(h6, i);

    // p = v* (1-s)
    XMVECTOR p = XMVectorMultiply(v, XMVectorSubtract(g_XMOne, s));

    // q = v*(1-f*s)
    XMVECTOR q = XMVectorMultiply(v, XMVectorSubtract(g_XMOne, XMVectorMultiply(f, s)));

    // t = v*(1 - (1-f)*s)
    XMVECTOR t = XMVectorMultiply(v, XMVectorSubtract(g_XMOne, XMVectorMultiply(XMVectorSubtract(g_XMOne, f), s)));

    auto ii = static_cast<int>(XMVectorGetX(XMVectorMod(i, g_XMSix)));

    XMVECTOR _rgb;

    switch (ii)
    {
    case 0: // rgb = vtp
    {
        XMVECTOR vt = XMVectorSelect(t, v, g_XMSelect1000);
        _rgb = XMVectorSelect(p, vt, g_XMSelect1100);
    }
    break;
    case 1: // rgb = qvp
    {
        XMVECTOR qv = XMVectorSelect(v, q, g_XMSelect1000);
        _rgb = XMVectorSelect(p, qv, g_XMSelect1100);
    }
    break;
    case 2: // rgb = pvt
    {
        XMVECTOR pv = XMVectorSelect(v, p, g_XMSelect1000);
        _rgb = XMVectorSelect(t, pv, g_XMSelect1100);
    }
    break;
    case 3: // rgb = pqv
    {
        XMVECTOR pq = XMVectorSelect(q, p, g_XMSelect1000);
        _rgb = XMVectorSelect(v, pq, g_XMSelect1100);
    }
    break;
    case 4: // rgb = tpv
    {
        XMVECTOR tp = XMVectorSelect(p, t, g_XMSelect1000);
        _rgb = XMVectorSelect(v, tp, g_XMSelect1100);
    }
    break;
    default: // rgb = vpq
    {
        XMVECTOR vp = XMVectorSelect(p, v, g_XMSelect1000);
        _rgb = XMVectorSelect(q, vp, g_XMSelect1100);
    }
    break;
    }

    return XMVectorSelect(hsv, _rgb, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorRGBToYUV(FXMVECTOR rgb) noexcept
{
    static const XMVECTORF32 Scale0 = { { { 0.299f, -0.147f, 0.615f, 0.0f } } };
    static const XMVECTORF32 Scale1 = { { { 0.587f, -0.289f, -0.515f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 0.114f, 0.436f, -0.100f, 0.0f } } };

    XMMATRIX M(Scale0, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(rgb, M);

    return XMVectorSelect(rgb, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorYUVToRGB(FXMVECTOR yuv) noexcept
{
    static const XMVECTORF32 Scale1 = { { { 0.0f, -0.395f, 2.032f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 1.140f, -0.581f, 0.0f, 0.0f } } };

    XMMATRIX M(g_XMOne, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(yuv, M);

    return XMVectorSelect(yuv, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorRGBToYUV_HD(FXMVECTOR rgb) noexcept
{
    static const XMVECTORF32 Scale0 = { { { 0.2126f, -0.0997f, 0.6150f, 0.0f } } };
    static const XMVECTORF32 Scale1 = { { { 0.7152f, -0.3354f, -0.5586f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 0.0722f, 0.4351f, -0.0564f, 0.0f } } };

    XMMATRIX M(Scale0, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(rgb, M);

    return XMVectorSelect(rgb, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorYUVToRGB_HD(FXMVECTOR yuv) noexcept
{
    static const XMVECTORF32 Scale1 = { { { 0.0f, -0.2153f, 2.1324f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 1.2803f, -0.3806f, 0.0f, 0.0f } } };

    XMMATRIX M(g_XMOne, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(yuv, M);

    return XMVectorSelect(yuv, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorRGBToYUV_UHD(FXMVECTOR rgb) noexcept
{
    static const XMVECTORF32 Scale0 = { { { 0.2627f, -0.1215f,  0.6150f, 0.0f } } };
    static const XMVECTORF32 Scale1 = { { { 0.6780f, -0.3136f, -0.5655f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 0.0593f,  0.4351f, -0.0495f, 0.0f } } };

    XMMATRIX M(Scale0, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(rgb, M);

    return XMVectorSelect(rgb, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorYUVToRGB_UHD(FXMVECTOR yuv) noexcept
{
    static const XMVECTORF32 Scale1 = { { {    0.0f, -0.1891f, 2.1620f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 1.1989f, -0.4645f,    0.0f, 0.0f } } };

    XMMATRIX M(g_XMOne, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(yuv, M);

    return XMVectorSelect(yuv, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorRGBToXYZ(FXMVECTOR rgb) noexcept
{
    static const XMVECTORF32 Scale0 = { { { 0.4887180f, 0.1762044f, 0.0000000f, 0.0f } } };
    static const XMVECTORF32 Scale1 = { { { 0.3106803f, 0.8129847f, 0.0102048f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 0.2006017f, 0.0108109f, 0.9897952f, 0.0f } } };
    static const XMVECTORF32 Scale = { { { 1.f / 0.17697f, 1.f / 0.17697f, 1.f / 0.17697f, 0.0f } } };

    XMMATRIX M(Scale0, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVectorMultiply(XMVector3Transform(rgb, M), Scale);

    return XMVectorSelect(rgb, clr, g_XMSelect1110);
}

inline XMVECTOR XM_CALLCONV XMColorXYZToRGB(FXMVECTOR xyz) noexcept
{
    static const XMVECTORF32 Scale0 = { { { 2.3706743f, -0.5138850f, 0.0052982f, 0.0f } } };
    static const XMVECTORF32 Scale1 = { { { -0.9000405f, 1.4253036f, -0.0146949f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { -0.4706338f, 0.0885814f, 1.0093968f, 0.0f } } };
    static const XMVECTORF32 Scale = { { { 0.17697f, 0.17697f, 0.17697f, 0.0f } } };

    XMMATRIX M(Scale0, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(XMVectorMultiply(xyz, Scale), M);

    return XMVectorSelect(xyz, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorXYZToSRGB(FXMVECTOR xyz) noexcept
{
    static const XMVECTORF32 Scale0 = { { { 3.2406f, -0.9689f, 0.0557f, 0.0f } } };
    static const XMVECTORF32 Scale1 = { { { -1.5372f, 1.8758f, -0.2040f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { -0.4986f, 0.0415f, 1.0570f, 0.0f } } };
    static const XMVECTORF32 Cutoff = { { { 0.0031308f, 0.0031308f, 0.0031308f, 0.0f } } };
    static const XMVECTORF32 Exp = { { { 1.0f / 2.4f, 1.0f / 2.4f, 1.0f / 2.4f, 1.0f } } };

    XMMATRIX M(Scale0, Scale1, Scale2, g_XMZero);
    XMVECTOR lclr = XMVector3Transform(xyz, M);

    XMVECTOR sel = XMVectorGreater(lclr, Cutoff);

    // clr = 12.92 * lclr for lclr <= 0.0031308f
    XMVECTOR smallC = XMVectorMultiply(lclr, g_XMsrgbScale);

    // clr = (1+a)*pow(lclr, 1/2.4) - a for lclr > 0.0031308 (where a = 0.055)
    XMVECTOR largeC = XMVectorSubtract(XMVectorMultiply(g_XMsrgbA1, XMVectorPow(lclr, Exp)), g_XMsrgbA);

    XMVECTOR clr = XMVectorSelect(smallC, largeC, sel);

    return XMVectorSelect(xyz, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorSRGBToXYZ(FXMVECTOR srgb) noexcept
{
    static const XMVECTORF32 Scale0 = { { { 0.4124f, 0.2126f, 0.0193f, 0.0f } } };
    static const XMVECTORF32 Scale1 = { { { 0.3576f, 0.7152f, 0.1192f, 0.0f } } };
    static const XMVECTORF32 Scale2 = { { { 0.1805f, 0.0722f, 0.9505f, 0.0f } } };
    static const XMVECTORF32 Cutoff = { { { 0.04045f, 0.04045f, 0.04045f, 0.0f } } };
    static const XMVECTORF32 Exp = { { { 2.4f, 2.4f, 2.4f, 1.0f } } };

    XMVECTOR sel = XMVectorGreater(srgb, Cutoff);

    // lclr = clr / 12.92
    XMVECTOR smallC = XMVectorDivide(srgb, g_XMsrgbScale);

    // lclr = pow( (clr + a) / (1+a), 2.4 )
    XMVECTOR largeC = XMVectorPow(XMVectorDivide(XMVectorAdd(srgb, g_XMsrgbA), g_XMsrgbA1), Exp);

    XMVECTOR lclr = XMVectorSelect(smallC, largeC, sel);

    XMMATRIX M(Scale0, Scale1, Scale2, g_XMZero);
    XMVECTOR clr = XMVector3Transform(lclr, M);

    return XMVectorSelect(srgb, clr, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorRGBToSRGB(FXMVECTOR rgb) noexcept
{
    static const XMVECTORF32 Cutoff = { { { 0.0031308f, 0.0031308f, 0.0031308f, 1.f } } };
    static const XMVECTORF32 Linear = { { { 12.92f, 12.92f, 12.92f, 1.f } } };
    static const XMVECTORF32 Scale = { { { 1.055f, 1.055f, 1.055f, 1.f } } };
    static const XMVECTORF32 Bias = { { { 0.055f, 0.055f, 0.055f, 0.f } } };
    static const XMVECTORF32 InvGamma = { { { 1.0f / 2.4f, 1.0f / 2.4f, 1.0f / 2.4f, 1.f } } };

    XMVECTOR V = XMVectorSaturate(rgb);
    XMVECTOR V0 = XMVectorMultiply(V, Linear);
    XMVECTOR V1 = XMVectorSubtract(XMVectorMultiply(Scale, XMVectorPow(V, InvGamma)), Bias);
    XMVECTOR select = XMVectorLess(V, Cutoff);
    V = XMVectorSelect(V1, V0, select);
    return XMVectorSelect(rgb, V, g_XMSelect1110);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMColorSRGBToRGB(FXMVECTOR srgb) noexcept
{
    static const XMVECTORF32 Cutoff = { { { 0.04045f, 0.04045f, 0.04045f, 1.f } } };
    static const XMVECTORF32 ILinear = { { { 1.f / 12.92f, 1.f / 12.92f, 1.f / 12.92f, 1.f } } };
    static const XMVECTORF32 Scale = { { { 1.f / 1.055f, 1.f / 1.055f, 1.f / 1.055f, 1.f } } };
    static const XMVECTORF32 Bias = { { { 0.055f, 0.055f, 0.055f, 0.f } } };
    static const XMVECTORF32 Gamma = { { { 2.4f, 2.4f, 2.4f, 1.f } } };

    XMVECTOR V = XMVectorSaturate(srgb);
    XMVECTOR V0 = XMVectorMultiply(V, ILinear);
    XMVECTOR V1 = XMVectorPow(XMVectorMultiply(XMVectorAdd(V, Bias), Scale), Gamma);
    XMVECTOR select = XMVectorGreater(V, Cutoff);
    V = XMVectorSelect(V0, V1, select);
    return XMVectorSelect(srgb, V, g_XMSelect1110);
}

/****************************************************************************
 *
 * Miscellaneous
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline bool XMVerifyCPUSupport() noexcept
{
#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    int CPUInfo[4] = { -1 };
#if (defined(__clang__) || defined(__GNUC__)) && defined(__cpuid)
    __cpuid(0, CPUInfo[0], CPUInfo[1], CPUInfo[2], CPUInfo[3]);
#else
    __cpuid(CPUInfo, 0);
#endif

#ifdef __AVX2__
    if (CPUInfo[0] < 7)
        return false;
#else
    if (CPUInfo[0] < 1)
        return false;
#endif

#if (defined(__clang__) || defined(__GNUC__)) && defined(__cpuid)
    __cpuid(1, CPUInfo[0], CPUInfo[1], CPUInfo[2], CPUInfo[3]);
#else
    __cpuid(CPUInfo, 1);
#endif

#if defined(__AVX2__) || defined(_XM_AVX2_INTRINSICS_)
    // The compiler can emit FMA3 instructions even without explicit intrinsics use
    if ((CPUInfo[2] & 0x38081001) != 0x38081001)
        return false; // No F16C/AVX/OSXSAVE/SSE4.1/FMA3/SSE3 support
#elif defined(_XM_FMA3_INTRINSICS_) && defined(_XM_F16C_INTRINSICS_)
    if ((CPUInfo[2] & 0x38081001) != 0x38081001)
        return false; // No F16C/AVX/OSXSAVE/SSE4.1/FMA3/SSE3 support
#elif defined(_XM_FMA3_INTRINSICS_)
    if ((CPUInfo[2] & 0x18081001) != 0x18081001)
        return false; // No AVX/OSXSAVE/SSE4.1/FMA3/SSE3 support
#elif defined(_XM_F16C_INTRINSICS_)
    if ((CPUInfo[2] & 0x38080001) != 0x38080001)
        return false; // No F16C/AVX/OSXSAVE/SSE4.1/SSE3 support
#elif defined(__AVX__) || defined(_XM_AVX_INTRINSICS_)
    if ((CPUInfo[2] & 0x18080001) != 0x18080001)
        return false; // No AVX/OSXSAVE/SSE4.1/SSE3 support
#elif defined(_XM_SSE4_INTRINSICS_)
    if ((CPUInfo[2] & 0x80001) != 0x80001)
        return false; // No SSE3/SSE4.1 support
#elif defined(_XM_SSE3_INTRINSICS_)
    if (!(CPUInfo[2] & 0x1))
        return false; // No SSE3 support
#endif

    // The x64 processor model requires SSE2 support, but no harm in checking
    if ((CPUInfo[3] & 0x6000000) != 0x6000000)
        return false; // No SSE2/SSE support

#if defined(__AVX2__) || defined(_XM_AVX2_INTRINSICS_)
#if defined(__clang__) || defined(__GNUC__)
    __cpuid_count(7, 0, CPUInfo[0], CPUInfo[1], CPUInfo[2], CPUInfo[3]);
#else
    __cpuidex(CPUInfo, 7, 0);
#endif
    if (!(CPUInfo[1] & 0x20))
        return false; // No AVX2 support
#endif

    return true;
#elif defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    // ARM-NEON support is required for the Windows on ARM platform
    return true;
#else
    // No intrinsics path always supported
    return true;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMFresnelTerm
(
    FXMVECTOR CosIncidentAngle,
    FXMVECTOR RefractionIndex
) noexcept
{
    assert(!XMVector4IsInfinite(CosIncidentAngle));

    // Result = 0.5f * (g - c)^2 / (g + c)^2 * ((c * (g + c) - 1)^2 / (c * (g - c) + 1)^2 + 1) where
    // c = CosIncidentAngle
    // g = sqrt(c^2 + RefractionIndex^2 - 1)

#if defined(_XM_NO_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    XMVECTOR G = XMVectorMultiplyAdd(RefractionIndex, RefractionIndex, g_XMNegativeOne.v);
    G = XMVectorMultiplyAdd(CosIncidentAngle, CosIncidentAngle, G);
    G = XMVectorAbs(G);
    G = XMVectorSqrt(G);

    XMVECTOR S = XMVectorAdd(G, CosIncidentAngle);
    XMVECTOR D = XMVectorSubtract(G, CosIncidentAngle);

    XMVECTOR V0 = XMVectorMultiply(D, D);
    XMVECTOR V1 = XMVectorMultiply(S, S);
    V1 = XMVectorReciprocal(V1);
    V0 = XMVectorMultiply(g_XMOneHalf.v, V0);
    V0 = XMVectorMultiply(V0, V1);

    XMVECTOR V2 = XMVectorMultiplyAdd(CosIncidentAngle, S, g_XMNegativeOne.v);
    XMVECTOR V3 = XMVectorMultiplyAdd(CosIncidentAngle, D, g_XMOne.v);
    V2 = XMVectorMultiply(V2, V2);
    V3 = XMVectorMultiply(V3, V3);
    V3 = XMVectorReciprocal(V3);
    V2 = XMVectorMultiplyAdd(V2, V3, g_XMOne.v);

    XMVECTOR Result = XMVectorMultiply(V0, V2);

    Result = XMVectorSaturate(Result);

    return Result;

#elif defined(_XM_SSE_INTRINSICS_)
    // G = sqrt(abs((RefractionIndex^2-1) + CosIncidentAngle^2))
    XMVECTOR G = _mm_mul_ps(RefractionIndex, RefractionIndex);
    XMVECTOR vTemp = _mm_mul_ps(CosIncidentAngle, CosIncidentAngle);
    G = _mm_sub_ps(G, g_XMOne);
    vTemp = _mm_add_ps(vTemp, G);
    // max((0-vTemp),vTemp) == abs(vTemp)
    // The abs is needed to deal with refraction and cosine being zero
    G = _mm_setzero_ps();
    G = _mm_sub_ps(G, vTemp);
    G = _mm_max_ps(G, vTemp);
    // Last operation, the sqrt()
    G = _mm_sqrt_ps(G);

    // Calc G-C and G+C
    XMVECTOR GAddC = _mm_add_ps(G, CosIncidentAngle);
    XMVECTOR GSubC = _mm_sub_ps(G, CosIncidentAngle);
    // Perform the term (0.5f *(g - c)^2) / (g + c)^2
    XMVECTOR vResult = _mm_mul_ps(GSubC, GSubC);
    vTemp = _mm_mul_ps(GAddC, GAddC);
    vResult = _mm_mul_ps(vResult, g_XMOneHalf);
    vResult = _mm_div_ps(vResult, vTemp);
    // Perform the term ((c * (g + c) - 1)^2 / (c * (g - c) + 1)^2 + 1)
    GAddC = _mm_mul_ps(GAddC, CosIncidentAngle);
    GSubC = _mm_mul_ps(GSubC, CosIncidentAngle);
    GAddC = _mm_sub_ps(GAddC, g_XMOne);
    GSubC = _mm_add_ps(GSubC, g_XMOne);
    GAddC = _mm_mul_ps(GAddC, GAddC);
    GSubC = _mm_mul_ps(GSubC, GSubC);
    GAddC = _mm_div_ps(GAddC, GSubC);
    GAddC = _mm_add_ps(GAddC, g_XMOne);
    // Multiply the two term parts
    vResult = _mm_mul_ps(vResult, GAddC);
    // Clamp to 0.0 - 1.0f
    vResult = _mm_max_ps(vResult, g_XMZero);
    vResult = _mm_min_ps(vResult, g_XMOne);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline bool XMScalarNearEqual
(
    float S1,
    float S2,
    float Epsilon
) noexcept
{
    float Delta = S1 - S2;
    return (fabsf(Delta) <= Epsilon);
}

//------------------------------------------------------------------------------
// Modulo the range of the given angle such that -XM_PI <= Angle < XM_PI
inline float XMScalarModAngle(float Angle) noexcept
{
    // Note: The modulo is performed with unsigned math only to work
    // around a precision error on numbers that are close to PI

    // Normalize the range from 0.0f to XM_2PI
    Angle = Angle + XM_PI;
    // Perform the modulo, unsigned
    float fTemp = fabsf(Angle);
    fTemp = fTemp - (XM_2PI * static_cast<float>(static_cast<int32_t>(fTemp / XM_2PI)));
    // Restore the number to the range of -XM_PI to XM_PI-epsilon
    fTemp = fTemp - XM_PI;
    // If the modulo'd value was negative, restore negation
    if (Angle < 0.0f)
    {
        fTemp = -fTemp;
    }
    return fTemp;
}

//------------------------------------------------------------------------------

inline float XMScalarSin(float Value) noexcept
{
    // Map Value to y in [-pi,pi], x = 2*pi*quotient + remainder.
    float quotient = XM_1DIV2PI * Value;
    if (Value >= 0.0f)
    {
        quotient = static_cast<float>(static_cast<int>(quotient + 0.5f));
    }
    else
    {
        quotient = static_cast<float>(static_cast<int>(quotient - 0.5f));
    }
    float y = Value - XM_2PI * quotient;

    // Map y to [-pi/2,pi/2] with sin(y) = sin(Value).
    if (y > XM_PIDIV2)
    {
        y = XM_PI - y;
    }
    else if (y < -XM_PIDIV2)
    {
        y = -XM_PI - y;
    }

    // 11-degree minimax approximation
    float y2 = y * y;
    return (((((-2.3889859e-08f * y2 + 2.7525562e-06f) * y2 - 0.00019840874f) * y2 + 0.0083333310f) * y2 - 0.16666667f) * y2 + 1.0f) * y;
}

//------------------------------------------------------------------------------

inline float XMScalarSinEst(float Value) noexcept
{
    // Map Value to y in [-pi,pi], x = 2*pi*quotient + remainder.
    float quotient = XM_1DIV2PI * Value;
    if (Value >= 0.0f)
    {
        quotient = static_cast<float>(static_cast<int>(quotient + 0.5f));
    }
    else
    {
        quotient = static_cast<float>(static_cast<int>(quotient - 0.5f));
    }
    float y = Value - XM_2PI * quotient;

    // Map y to [-pi/2,pi/2] with sin(y) = sin(Value).
    if (y > XM_PIDIV2)
    {
        y = XM_PI - y;
    }
    else if (y < -XM_PIDIV2)
    {
        y = -XM_PI - y;
    }

    // 7-degree minimax approximation
    float y2 = y * y;
    return (((-0.00018524670f * y2 + 0.0083139502f) * y2 - 0.16665852f) * y2 + 1.0f) * y;
}

//------------------------------------------------------------------------------

inline float XMScalarCos(float Value) noexcept
{
    // Map Value to y in [-pi,pi], x = 2*pi*quotient + remainder.
    float quotient = XM_1DIV2PI * Value;
    if (Value >= 0.0f)
    {
        quotient = static_cast<float>(static_cast<int>(quotient + 0.5f));
    }
    else
    {
        quotient = static_cast<float>(static_cast<int>(quotient - 0.5f));
    }
    float y = Value - XM_2PI * quotient;

    // Map y to [-pi/2,pi/2] with cos(y) = sign*cos(x).
    float sign;
    if (y > XM_PIDIV2)
    {
        y = XM_PI - y;
        sign = -1.0f;
    }
    else if (y < -XM_PIDIV2)
    {
        y = -XM_PI - y;
        sign = -1.0f;
    }
    else
    {
        sign = +1.0f;
    }

    // 10-degree minimax approximation
    float y2 = y * y;
    float p = ((((-2.6051615e-07f * y2 + 2.4760495e-05f) * y2 - 0.0013888378f) * y2 + 0.041666638f) * y2 - 0.5f) * y2 + 1.0f;
    return sign * p;
}

//------------------------------------------------------------------------------

inline float XMScalarCosEst(float Value) noexcept
{
    // Map Value to y in [-pi,pi], x = 2*pi*quotient + remainder.
    float quotient = XM_1DIV2PI * Value;
    if (Value >= 0.0f)
    {
        quotient = static_cast<float>(static_cast<int>(quotient + 0.5f));
    }
    else
    {
        quotient = static_cast<float>(static_cast<int>(quotient - 0.5f));
    }
    float y = Value - XM_2PI * quotient;

    // Map y to [-pi/2,pi/2] with cos(y) = sign*cos(x).
    float sign;
    if (y > XM_PIDIV2)
    {
        y = XM_PI - y;
        sign = -1.0f;
    }
    else if (y < -XM_PIDIV2)
    {
        y = -XM_PI - y;
        sign = -1.0f;
    }
    else
    {
        sign = +1.0f;
    }

    // 6-degree minimax approximation
    float y2 = y * y;
    float p = ((-0.0012712436f * y2 + 0.041493919f) * y2 - 0.49992746f) * y2 + 1.0f;
    return sign * p;
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline void XMScalarSinCos
(
    float* pSin,
    float* pCos,
    float  Value
) noexcept
{
    assert(pSin);
    assert(pCos);

    // Map Value to y in [-pi,pi], x = 2*pi*quotient + remainder.
    float quotient = XM_1DIV2PI * Value;
    if (Value >= 0.0f)
    {
        quotient = static_cast<float>(static_cast<int>(quotient + 0.5f));
    }
    else
    {
        quotient = static_cast<float>(static_cast<int>(quotient - 0.5f));
    }
    float y = Value - XM_2PI * quotient;

    // Map y to [-pi/2,pi/2] with sin(y) = sin(Value).
    float sign;
    if (y > XM_PIDIV2)
    {
        y = XM_PI - y;
        sign = -1.0f;
    }
    else if (y < -XM_PIDIV2)
    {
        y = -XM_PI - y;
        sign = -1.0f;
    }
    else
    {
        sign = +1.0f;
    }

    float y2 = y * y;

    // 11-degree minimax approximation
    *pSin = (((((-2.3889859e-08f * y2 + 2.7525562e-06f) * y2 - 0.00019840874f) * y2 + 0.0083333310f) * y2 - 0.16666667f) * y2 + 1.0f) * y;

    // 10-degree minimax approximation
    float p = ((((-2.6051615e-07f * y2 + 2.4760495e-05f) * y2 - 0.0013888378f) * y2 + 0.041666638f) * y2 - 0.5f) * y2 + 1.0f;
    *pCos = sign * p;
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline void XMScalarSinCosEst
(
    float* pSin,
    float* pCos,
    float  Value
) noexcept
{
    assert(pSin);
    assert(pCos);

    // Map Value to y in [-pi,pi], x = 2*pi*quotient + remainder.
    float quotient = XM_1DIV2PI * Value;
    if (Value >= 0.0f)
    {
        quotient = static_cast<float>(static_cast<int>(quotient + 0.5f));
    }
    else
    {
        quotient = static_cast<float>(static_cast<int>(quotient - 0.5f));
    }
    float y = Value - XM_2PI * quotient;

    // Map y to [-pi/2,pi/2] with sin(y) = sin(Value).
    float sign;
    if (y > XM_PIDIV2)
    {
        y = XM_PI - y;
        sign = -1.0f;
    }
    else if (y < -XM_PIDIV2)
    {
        y = -XM_PI - y;
        sign = -1.0f;
    }
    else
    {
        sign = +1.0f;
    }

    float y2 = y * y;

    // 7-degree minimax approximation
    *pSin = (((-0.00018524670f * y2 + 0.0083139502f) * y2 - 0.16665852f) * y2 + 1.0f) * y;

    // 6-degree minimax approximation
    float p = ((-0.0012712436f * y2 + 0.041493919f) * y2 - 0.49992746f) * y2 + 1.0f;
    *pCos = sign * p;
}

//------------------------------------------------------------------------------

inline float XMScalarASin(float Value) noexcept
{
    // Clamp input to [-1,1].
    bool nonnegative = (Value >= 0.0f);
    float x = fabsf(Value);
    float omx = 1.0f - x;
    if (omx < 0.0f)
    {
        omx = 0.0f;
    }
    float root = sqrtf(omx);

    // 7-degree minimax approximation
    float result = ((((((-0.0012624911f * x + 0.0066700901f) * x - 0.0170881256f) * x + 0.0308918810f) * x - 0.0501743046f) * x + 0.0889789874f) * x - 0.2145988016f) * x + 1.5707963050f;
    result *= root;  // acos(|x|)

    // acos(x) = pi - acos(-x) when x < 0, asin(x) = pi/2 - acos(x)
    return (nonnegative ? XM_PIDIV2 - result : result - XM_PIDIV2);
}

//------------------------------------------------------------------------------

inline float XMScalarASinEst(float Value) noexcept
{
    // Clamp input to [-1,1].
    bool nonnegative = (Value >= 0.0f);
    float x = fabsf(Value);
    float omx = 1.0f - x;
    if (omx < 0.0f)
    {
        omx = 0.0f;
    }
    float root = sqrtf(omx);

    // 3-degree minimax approximation
    float result = ((-0.0187293f * x + 0.0742610f) * x - 0.2121144f) * x + 1.5707288f;
    result *= root;  // acos(|x|)

    // acos(x) = pi - acos(-x) when x < 0, asin(x) = pi/2 - acos(x)
    return (nonnegative ? XM_PIDIV2 - result : result - XM_PIDIV2);
}

//------------------------------------------------------------------------------

inline float XMScalarACos(float Value) noexcept
{
    // Clamp input to [-1,1].
    bool nonnegative = (Value >= 0.0f);
    float x = fabsf(Value);
    float omx = 1.0f - x;
    if (omx < 0.0f)
    {
        omx = 0.0f;
    }
    float root = sqrtf(omx);

    // 7-degree minimax approximation
    float result = ((((((-0.0012624911f * x + 0.0066700901f) * x - 0.0170881256f) * x + 0.0308918810f) * x - 0.0501743046f) * x + 0.0889789874f) * x - 0.2145988016f) * x + 1.5707963050f;
    result *= root;

    // acos(x) = pi - acos(-x) when x < 0
    return (nonnegative ? result : XM_PI - result);
}

//------------------------------------------------------------------------------

inline float XMScalarACosEst(float Value) noexcept
{
    // Clamp input to [-1,1].
    bool nonnegative = (Value >= 0.0f);
    float x = fabsf(Value);
    float omx = 1.0f - x;
    if (omx < 0.0f)
    {
        omx = 0.0f;
    }
    float root = sqrtf(omx);

    // 3-degree minimax approximation
    float result = ((-0.0187293f * x + 0.0742610f) * x - 0.2121144f) * x + 1.5707288f;
    result *= root;

    // acos(x) = pi - acos(-x) when x < 0
    return (nonnegative ? result : XM_PI - result);
}

