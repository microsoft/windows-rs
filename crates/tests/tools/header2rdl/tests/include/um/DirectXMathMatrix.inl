//-------------------------------------------------------------------------------------
// DirectXMathMatrix.inl -- SIMD C++ Math library
//
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
//
// http://go.microsoft.com/fwlink/?LinkID=615560
//-------------------------------------------------------------------------------------

#pragma once

/****************************************************************************
 *
 * Matrix
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Comparison operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(push)
#pragma float_control(precise, on)
#endif

// Return true if any entry in the matrix is NaN
inline bool XM_CALLCONV XMMatrixIsNaN(FXMMATRIX M) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    size_t i = 16;
    auto pWork = reinterpret_cast<const uint32_t*>(&M.m[0][0]);
    do {
        // Fetch value into integer unit
        uint32_t uTest = pWork[0];
        // Remove sign
        uTest &= 0x7FFFFFFFU;
        // NaN is 0x7F800001 through 0x7FFFFFFF inclusive
        uTest -= 0x7F800001U;
        if (uTest < 0x007FFFFFU)
        {
            break;      // NaN found
        }
        ++pWork;        // Next entry
    } while (--i);
    return (i != 0);      // i == 0 if nothing matched
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Load in registers
    float32x4_t vX = M.r[0];
    float32x4_t vY = M.r[1];
    float32x4_t vZ = M.r[2];
    float32x4_t vW = M.r[3];
    // Test themselves to check for NaN
    uint32x4_t xmask = vmvnq_u32(vceqq_f32(vX, vX));
    uint32x4_t ymask = vmvnq_u32(vceqq_f32(vY, vY));
    uint32x4_t zmask = vmvnq_u32(vceqq_f32(vZ, vZ));
    uint32x4_t wmask = vmvnq_u32(vceqq_f32(vW, vW));
    // Or all the results
    xmask = vorrq_u32(xmask, zmask);
    ymask = vorrq_u32(ymask, wmask);
    xmask = vorrq_u32(xmask, ymask);
    // If any tested true, return true
    uint8x8x2_t vTemp = vzip_u8(
        vget_low_u8(vreinterpretq_u8_u32(xmask)),
        vget_high_u8(vreinterpretq_u8_u32(xmask)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);
    return (r != 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Load in registers
    XMVECTOR vX = M.r[0];
    XMVECTOR vY = M.r[1];
    XMVECTOR vZ = M.r[2];
    XMVECTOR vW = M.r[3];
    // Test themselves to check for NaN
    vX = _mm_cmpneq_ps(vX, vX);
    vY = _mm_cmpneq_ps(vY, vY);
    vZ = _mm_cmpneq_ps(vZ, vZ);
    vW = _mm_cmpneq_ps(vW, vW);
    // Or all the results
    vX = _mm_or_ps(vX, vZ);
    vY = _mm_or_ps(vY, vW);
    vX = _mm_or_ps(vX, vY);
    // If any tested true, return true
    return (_mm_movemask_ps(vX) != 0);
#else
#endif
}

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(pop)
#endif

//------------------------------------------------------------------------------

// Return true if any entry in the matrix is +/-INF
inline bool XM_CALLCONV XMMatrixIsInfinite(FXMMATRIX M) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    size_t i = 16;
    auto pWork = reinterpret_cast<const uint32_t*>(&M.m[0][0]);
    do {
        // Fetch value into integer unit
        uint32_t uTest = pWork[0];
        // Remove sign
        uTest &= 0x7FFFFFFFU;
        // INF is 0x7F800000
        if (uTest == 0x7F800000U)
        {
            break;      // INF found
        }
        ++pWork;        // Next entry
    } while (--i);
    return (i != 0);      // i == 0 if nothing matched
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Load in registers
    float32x4_t vX = M.r[0];
    float32x4_t vY = M.r[1];
    float32x4_t vZ = M.r[2];
    float32x4_t vW = M.r[3];
    // Mask off the sign bits
    vX = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(vX), g_XMAbsMask));
    vY = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(vY), g_XMAbsMask));
    vZ = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(vZ), g_XMAbsMask));
    vW = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(vW), g_XMAbsMask));
    // Compare to infinity
    uint32x4_t xmask = vceqq_f32(vX, g_XMInfinity);
    uint32x4_t ymask = vceqq_f32(vY, g_XMInfinity);
    uint32x4_t zmask = vceqq_f32(vZ, g_XMInfinity);
    uint32x4_t wmask = vceqq_f32(vW, g_XMInfinity);
    // Or the answers together
    xmask = vorrq_u32(xmask, zmask);
    ymask = vorrq_u32(ymask, wmask);
    xmask = vorrq_u32(xmask, ymask);
    // If any tested true, return true
    uint8x8x2_t vTemp = vzip_u8(
        vget_low_u8(vreinterpretq_u8_u32(xmask)),
        vget_high_u8(vreinterpretq_u8_u32(xmask)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);
    return (r != 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Mask off the sign bits
    XMVECTOR vTemp1 = _mm_and_ps(M.r[0], g_XMAbsMask);
    XMVECTOR vTemp2 = _mm_and_ps(M.r[1], g_XMAbsMask);
    XMVECTOR vTemp3 = _mm_and_ps(M.r[2], g_XMAbsMask);
    XMVECTOR vTemp4 = _mm_and_ps(M.r[3], g_XMAbsMask);
    // Compare to infinity
    vTemp1 = _mm_cmpeq_ps(vTemp1, g_XMInfinity);
    vTemp2 = _mm_cmpeq_ps(vTemp2, g_XMInfinity);
    vTemp3 = _mm_cmpeq_ps(vTemp3, g_XMInfinity);
    vTemp4 = _mm_cmpeq_ps(vTemp4, g_XMInfinity);
    // Or the answers together
    vTemp1 = _mm_or_ps(vTemp1, vTemp2);
    vTemp3 = _mm_or_ps(vTemp3, vTemp4);
    vTemp1 = _mm_or_ps(vTemp1, vTemp3);
    // If any are infinity, the signs are true.
    return (_mm_movemask_ps(vTemp1) != 0);
#endif
}

//------------------------------------------------------------------------------

// Return true if the XMMatrix is equal to identity
inline bool XM_CALLCONV XMMatrixIsIdentity(FXMMATRIX M) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    // Use the integer pipeline to reduce branching to a minimum
    auto pWork = reinterpret_cast<const uint32_t*>(&M.m[0][0]);
    // Convert 1.0f to zero and or them together
    uint32_t uOne = pWork[0] ^ 0x3F800000U;
    // Or all the 0.0f entries together
    uint32_t uZero = pWork[1];
    uZero |= pWork[2];
    uZero |= pWork[3];
    // 2nd row
    uZero |= pWork[4];
    uOne |= pWork[5] ^ 0x3F800000U;
    uZero |= pWork[6];
    uZero |= pWork[7];
    // 3rd row
    uZero |= pWork[8];
    uZero |= pWork[9];
    uOne |= pWork[10] ^ 0x3F800000U;
    uZero |= pWork[11];
    // 4th row
    uZero |= pWork[12];
    uZero |= pWork[13];
    uZero |= pWork[14];
    uOne |= pWork[15] ^ 0x3F800000U;
    // If all zero entries are zero, the uZero==0
    uZero &= 0x7FFFFFFF;    // Allow -0.0f
    // If all 1.0f entries are 1.0f, then uOne==0
    uOne |= uZero;
    return (uOne == 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t xmask = vceqq_f32(M.r[0], g_XMIdentityR0);
    uint32x4_t ymask = vceqq_f32(M.r[1], g_XMIdentityR1);
    uint32x4_t zmask = vceqq_f32(M.r[2], g_XMIdentityR2);
    uint32x4_t wmask = vceqq_f32(M.r[3], g_XMIdentityR3);
    xmask = vandq_u32(xmask, zmask);
    ymask = vandq_u32(ymask, wmask);
    xmask = vandq_u32(xmask, ymask);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(xmask)), vget_high_u8(vreinterpretq_u8_u32(xmask)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);
    return (r == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp1 = _mm_cmpeq_ps(M.r[0], g_XMIdentityR0);
    XMVECTOR vTemp2 = _mm_cmpeq_ps(M.r[1], g_XMIdentityR1);
    XMVECTOR vTemp3 = _mm_cmpeq_ps(M.r[2], g_XMIdentityR2);
    XMVECTOR vTemp4 = _mm_cmpeq_ps(M.r[3], g_XMIdentityR3);
    vTemp1 = _mm_and_ps(vTemp1, vTemp2);
    vTemp3 = _mm_and_ps(vTemp3, vTemp4);
    vTemp1 = _mm_and_ps(vTemp1, vTemp3);
    return (_mm_movemask_ps(vTemp1) == 0x0f);
#endif
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Perform a 4x4 matrix multiply by a 4x4 matrix
inline XMMATRIX XM_CALLCONV XMMatrixMultiply
(
    FXMMATRIX M1,
    CXMMATRIX M2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMMATRIX mResult;
    // Cache the invariants in registers
    float x = M1.m[0][0];
    float y = M1.m[0][1];
    float z = M1.m[0][2];
    float w = M1.m[0][3];
    // Perform the operation on the first row
    mResult.m[0][0] = (M2.m[0][0] * x) + (M2.m[1][0] * y) + (M2.m[2][0] * z) + (M2.m[3][0] * w);
    mResult.m[0][1] = (M2.m[0][1] * x) + (M2.m[1][1] * y) + (M2.m[2][1] * z) + (M2.m[3][1] * w);
    mResult.m[0][2] = (M2.m[0][2] * x) + (M2.m[1][2] * y) + (M2.m[2][2] * z) + (M2.m[3][2] * w);
    mResult.m[0][3] = (M2.m[0][3] * x) + (M2.m[1][3] * y) + (M2.m[2][3] * z) + (M2.m[3][3] * w);
    // Repeat for all the other rows
    x = M1.m[1][0];
    y = M1.m[1][1];
    z = M1.m[1][2];
    w = M1.m[1][3];
    mResult.m[1][0] = (M2.m[0][0] * x) + (M2.m[1][0] * y) + (M2.m[2][0] * z) + (M2.m[3][0] * w);
    mResult.m[1][1] = (M2.m[0][1] * x) + (M2.m[1][1] * y) + (M2.m[2][1] * z) + (M2.m[3][1] * w);
    mResult.m[1][2] = (M2.m[0][2] * x) + (M2.m[1][2] * y) + (M2.m[2][2] * z) + (M2.m[3][2] * w);
    mResult.m[1][3] = (M2.m[0][3] * x) + (M2.m[1][3] * y) + (M2.m[2][3] * z) + (M2.m[3][3] * w);
    x = M1.m[2][0];
    y = M1.m[2][1];
    z = M1.m[2][2];
    w = M1.m[2][3];
    mResult.m[2][0] = (M2.m[0][0] * x) + (M2.m[1][0] * y) + (M2.m[2][0] * z) + (M2.m[3][0] * w);
    mResult.m[2][1] = (M2.m[0][1] * x) + (M2.m[1][1] * y) + (M2.m[2][1] * z) + (M2.m[3][1] * w);
    mResult.m[2][2] = (M2.m[0][2] * x) + (M2.m[1][2] * y) + (M2.m[2][2] * z) + (M2.m[3][2] * w);
    mResult.m[2][3] = (M2.m[0][3] * x) + (M2.m[1][3] * y) + (M2.m[2][3] * z) + (M2.m[3][3] * w);
    x = M1.m[3][0];
    y = M1.m[3][1];
    z = M1.m[3][2];
    w = M1.m[3][3];
    mResult.m[3][0] = (M2.m[0][0] * x) + (M2.m[1][0] * y) + (M2.m[2][0] * z) + (M2.m[3][0] * w);
    mResult.m[3][1] = (M2.m[0][1] * x) + (M2.m[1][1] * y) + (M2.m[2][1] * z) + (M2.m[3][1] * w);
    mResult.m[3][2] = (M2.m[0][2] * x) + (M2.m[1][2] * y) + (M2.m[2][2] * z) + (M2.m[3][2] * w);
    mResult.m[3][3] = (M2.m[0][3] * x) + (M2.m[1][3] * y) + (M2.m[2][3] * z) + (M2.m[3][3] * w);
    return mResult;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMMATRIX mResult;
    float32x2_t VL = vget_low_f32(M1.r[0]);
    float32x2_t VH = vget_high_f32(M1.r[0]);
    // Perform the operation on the first row
    float32x4_t vX = vmulq_lane_f32(M2.r[0], VL, 0);
    float32x4_t vY = vmulq_lane_f32(M2.r[1], VL, 1);
    float32x4_t vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    float32x4_t vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    mResult.r[0] = vaddq_f32(vZ, vW);
    // Repeat for the other 3 rows
    VL = vget_low_f32(M1.r[1]);
    VH = vget_high_f32(M1.r[1]);
    vX = vmulq_lane_f32(M2.r[0], VL, 0);
    vY = vmulq_lane_f32(M2.r[1], VL, 1);
    vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    mResult.r[1] = vaddq_f32(vZ, vW);
    VL = vget_low_f32(M1.r[2]);
    VH = vget_high_f32(M1.r[2]);
    vX = vmulq_lane_f32(M2.r[0], VL, 0);
    vY = vmulq_lane_f32(M2.r[1], VL, 1);
    vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    mResult.r[2] = vaddq_f32(vZ, vW);
    VL = vget_low_f32(M1.r[3]);
    VH = vget_high_f32(M1.r[3]);
    vX = vmulq_lane_f32(M2.r[0], VL, 0);
    vY = vmulq_lane_f32(M2.r[1], VL, 1);
    vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    mResult.r[3] = vaddq_f32(vZ, vW);
    return mResult;
#elif defined(_XM_AVX2_INTRINSICS_)
    __m256 t0 = _mm256_castps128_ps256(M1.r[0]);
    t0 = _mm256_insertf128_ps(t0, M1.r[1], 1);
    __m256 t1 = _mm256_castps128_ps256(M1.r[2]);
    t1 = _mm256_insertf128_ps(t1, M1.r[3], 1);

    __m256 u0 = _mm256_castps128_ps256(M2.r[0]);
    u0 = _mm256_insertf128_ps(u0, M2.r[1], 1);
    __m256 u1 = _mm256_castps128_ps256(M2.r[2]);
    u1 = _mm256_insertf128_ps(u1, M2.r[3], 1);

    __m256 a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(0, 0, 0, 0));
    __m256 a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(0, 0, 0, 0));
    __m256 b0 = _mm256_permute2f128_ps(u0, u0, 0x00);
    __m256 c0 = _mm256_mul_ps(a0, b0);
    __m256 c1 = _mm256_mul_ps(a1, b0);

    a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(1, 1, 1, 1));
    a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(1, 1, 1, 1));
    b0 = _mm256_permute2f128_ps(u0, u0, 0x11);
    __m256 c2 = _mm256_fmadd_ps(a0, b0, c0);
    __m256 c3 = _mm256_fmadd_ps(a1, b0, c1);

    a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(2, 2, 2, 2));
    a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(2, 2, 2, 2));
    __m256 b1 = _mm256_permute2f128_ps(u1, u1, 0x00);
    __m256 c4 = _mm256_mul_ps(a0, b1);
    __m256 c5 = _mm256_mul_ps(a1, b1);

    a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(3, 3, 3, 3));
    a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(3, 3, 3, 3));
    b1 = _mm256_permute2f128_ps(u1, u1, 0x11);
    __m256 c6 = _mm256_fmadd_ps(a0, b1, c4);
    __m256 c7 = _mm256_fmadd_ps(a1, b1, c5);

    t0 = _mm256_add_ps(c2, c6);
    t1 = _mm256_add_ps(c3, c7);

    XMMATRIX mResult;
    mResult.r[0] = _mm256_castps256_ps128(t0);
    mResult.r[1] = _mm256_extractf128_ps(t0, 1);
    mResult.r[2] = _mm256_castps256_ps128(t1);
    mResult.r[3] = _mm256_extractf128_ps(t1, 1);
    return mResult;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX mResult;
    // Splat the component X,Y,Z then W
#if defined(_XM_AVX_INTRINSICS_)
    XMVECTOR vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 0);
    XMVECTOR vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 1);
    XMVECTOR vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 2);
    XMVECTOR vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 3);
#else
    // Use vW to hold the original row
    XMVECTOR vW = M1.r[0];
    XMVECTOR vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    XMVECTOR vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    XMVECTOR vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    // Perform the operation on the first row
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    // Perform a binary add to reduce cumulative errors
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    mResult.r[0] = vX;
    // Repeat for the other 3 rows
#if defined(_XM_AVX_INTRINSICS_)
    vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 0);
    vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 1);
    vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 2);
    vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 3);
#else
    vW = M1.r[1];
    vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    mResult.r[1] = vX;
#if defined(_XM_AVX_INTRINSICS_)
    vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 0);
    vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 1);
    vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 2);
    vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 3);
#else
    vW = M1.r[2];
    vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    mResult.r[2] = vX;
#if defined(_XM_AVX_INTRINSICS_)
    vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 0);
    vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 1);
    vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 2);
    vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 3);
#else
    vW = M1.r[3];
    vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    mResult.r[3] = vX;
    return mResult;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixMultiplyTranspose
(
    FXMMATRIX M1,
    CXMMATRIX M2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMMATRIX mResult;
    // Cache the invariants in registers
    float x = M2.m[0][0];
    float y = M2.m[1][0];
    float z = M2.m[2][0];
    float w = M2.m[3][0];
    // Perform the operation on the first row
    mResult.m[0][0] = (M1.m[0][0] * x) + (M1.m[0][1] * y) + (M1.m[0][2] * z) + (M1.m[0][3] * w);
    mResult.m[0][1] = (M1.m[1][0] * x) + (M1.m[1][1] * y) + (M1.m[1][2] * z) + (M1.m[1][3] * w);
    mResult.m[0][2] = (M1.m[2][0] * x) + (M1.m[2][1] * y) + (M1.m[2][2] * z) + (M1.m[2][3] * w);
    mResult.m[0][3] = (M1.m[3][0] * x) + (M1.m[3][1] * y) + (M1.m[3][2] * z) + (M1.m[3][3] * w);
    // Repeat for all the other rows
    x = M2.m[0][1];
    y = M2.m[1][1];
    z = M2.m[2][1];
    w = M2.m[3][1];
    mResult.m[1][0] = (M1.m[0][0] * x) + (M1.m[0][1] * y) + (M1.m[0][2] * z) + (M1.m[0][3] * w);
    mResult.m[1][1] = (M1.m[1][0] * x) + (M1.m[1][1] * y) + (M1.m[1][2] * z) + (M1.m[1][3] * w);
    mResult.m[1][2] = (M1.m[2][0] * x) + (M1.m[2][1] * y) + (M1.m[2][2] * z) + (M1.m[2][3] * w);
    mResult.m[1][3] = (M1.m[3][0] * x) + (M1.m[3][1] * y) + (M1.m[3][2] * z) + (M1.m[3][3] * w);
    x = M2.m[0][2];
    y = M2.m[1][2];
    z = M2.m[2][2];
    w = M2.m[3][2];
    mResult.m[2][0] = (M1.m[0][0] * x) + (M1.m[0][1] * y) + (M1.m[0][2] * z) + (M1.m[0][3] * w);
    mResult.m[2][1] = (M1.m[1][0] * x) + (M1.m[1][1] * y) + (M1.m[1][2] * z) + (M1.m[1][3] * w);
    mResult.m[2][2] = (M1.m[2][0] * x) + (M1.m[2][1] * y) + (M1.m[2][2] * z) + (M1.m[2][3] * w);
    mResult.m[2][3] = (M1.m[3][0] * x) + (M1.m[3][1] * y) + (M1.m[3][2] * z) + (M1.m[3][3] * w);
    x = M2.m[0][3];
    y = M2.m[1][3];
    z = M2.m[2][3];
    w = M2.m[3][3];
    mResult.m[3][0] = (M1.m[0][0] * x) + (M1.m[0][1] * y) + (M1.m[0][2] * z) + (M1.m[0][3] * w);
    mResult.m[3][1] = (M1.m[1][0] * x) + (M1.m[1][1] * y) + (M1.m[1][2] * z) + (M1.m[1][3] * w);
    mResult.m[3][2] = (M1.m[2][0] * x) + (M1.m[2][1] * y) + (M1.m[2][2] * z) + (M1.m[2][3] * w);
    mResult.m[3][3] = (M1.m[3][0] * x) + (M1.m[3][1] * y) + (M1.m[3][2] * z) + (M1.m[3][3] * w);
    return mResult;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(M1.r[0]);
    float32x2_t VH = vget_high_f32(M1.r[0]);
    // Perform the operation on the first row
    float32x4_t vX = vmulq_lane_f32(M2.r[0], VL, 0);
    float32x4_t vY = vmulq_lane_f32(M2.r[1], VL, 1);
    float32x4_t vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    float32x4_t vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    float32x4_t r0 = vaddq_f32(vZ, vW);
    // Repeat for the other 3 rows
    VL = vget_low_f32(M1.r[1]);
    VH = vget_high_f32(M1.r[1]);
    vX = vmulq_lane_f32(M2.r[0], VL, 0);
    vY = vmulq_lane_f32(M2.r[1], VL, 1);
    vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    float32x4_t r1 = vaddq_f32(vZ, vW);
    VL = vget_low_f32(M1.r[2]);
    VH = vget_high_f32(M1.r[2]);
    vX = vmulq_lane_f32(M2.r[0], VL, 0);
    vY = vmulq_lane_f32(M2.r[1], VL, 1);
    vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    float32x4_t r2 = vaddq_f32(vZ, vW);
    VL = vget_low_f32(M1.r[3]);
    VH = vget_high_f32(M1.r[3]);
    vX = vmulq_lane_f32(M2.r[0], VL, 0);
    vY = vmulq_lane_f32(M2.r[1], VL, 1);
    vZ = vmlaq_lane_f32(vX, M2.r[2], VH, 0);
    vW = vmlaq_lane_f32(vY, M2.r[3], VH, 1);
    float32x4_t r3 = vaddq_f32(vZ, vW);

    // Transpose result
    float32x4x2_t P0 = vzipq_f32(r0, r2);
    float32x4x2_t P1 = vzipq_f32(r1, r3);

    float32x4x2_t T0 = vzipq_f32(P0.val[0], P1.val[0]);
    float32x4x2_t T1 = vzipq_f32(P0.val[1], P1.val[1]);

    XMMATRIX mResult;
    mResult.r[0] = T0.val[0];
    mResult.r[1] = T0.val[1];
    mResult.r[2] = T1.val[0];
    mResult.r[3] = T1.val[1];
    return mResult;
#elif defined(_XM_AVX2_INTRINSICS_)
    __m256 t0 = _mm256_castps128_ps256(M1.r[0]);
    t0 = _mm256_insertf128_ps(t0, M1.r[1], 1);
    __m256 t1 = _mm256_castps128_ps256(M1.r[2]);
    t1 = _mm256_insertf128_ps(t1, M1.r[3], 1);

    __m256 u0 = _mm256_castps128_ps256(M2.r[0]);
    u0 = _mm256_insertf128_ps(u0, M2.r[1], 1);
    __m256 u1 = _mm256_castps128_ps256(M2.r[2]);
    u1 = _mm256_insertf128_ps(u1, M2.r[3], 1);

    __m256 a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(0, 0, 0, 0));
    __m256 a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(0, 0, 0, 0));
    __m256 b0 = _mm256_permute2f128_ps(u0, u0, 0x00);
    __m256 c0 = _mm256_mul_ps(a0, b0);
    __m256 c1 = _mm256_mul_ps(a1, b0);

    a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(1, 1, 1, 1));
    a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(1, 1, 1, 1));
    b0 = _mm256_permute2f128_ps(u0, u0, 0x11);
    __m256 c2 = _mm256_fmadd_ps(a0, b0, c0);
    __m256 c3 = _mm256_fmadd_ps(a1, b0, c1);

    a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(2, 2, 2, 2));
    a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(2, 2, 2, 2));
    __m256 b1 = _mm256_permute2f128_ps(u1, u1, 0x00);
    __m256 c4 = _mm256_mul_ps(a0, b1);
    __m256 c5 = _mm256_mul_ps(a1, b1);

    a0 = _mm256_shuffle_ps(t0, t0, _MM_SHUFFLE(3, 3, 3, 3));
    a1 = _mm256_shuffle_ps(t1, t1, _MM_SHUFFLE(3, 3, 3, 3));
    b1 = _mm256_permute2f128_ps(u1, u1, 0x11);
    __m256 c6 = _mm256_fmadd_ps(a0, b1, c4);
    __m256 c7 = _mm256_fmadd_ps(a1, b1, c5);

    t0 = _mm256_add_ps(c2, c6);
    t1 = _mm256_add_ps(c3, c7);

    // Transpose result
    __m256 vTemp = _mm256_unpacklo_ps(t0, t1);
    __m256 vTemp2 = _mm256_unpackhi_ps(t0, t1);
    __m256 vTemp3 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x20);
    __m256 vTemp4 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x31);
    vTemp = _mm256_unpacklo_ps(vTemp3, vTemp4);
    vTemp2 = _mm256_unpackhi_ps(vTemp3, vTemp4);
    t0 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x20);
    t1 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x31);

    XMMATRIX mResult;
    mResult.r[0] = _mm256_castps256_ps128(t0);
    mResult.r[1] = _mm256_extractf128_ps(t0, 1);
    mResult.r[2] = _mm256_castps256_ps128(t1);
    mResult.r[3] = _mm256_extractf128_ps(t1, 1);
    return mResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the component X,Y,Z then W
#if defined(_XM_AVX_INTRINSICS_)
    XMVECTOR vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 0);
    XMVECTOR vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 1);
    XMVECTOR vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 2);
    XMVECTOR vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[0]) + 3);
#else
    // Use vW to hold the original row
    XMVECTOR vW = M1.r[0];
    XMVECTOR vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    XMVECTOR vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    XMVECTOR vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    // Perform the operation on the first row
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    // Perform a binary add to reduce cumulative errors
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    XMVECTOR r0 = vX;
    // Repeat for the other 3 rows
#if defined(_XM_AVX_INTRINSICS_)
    vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 0);
    vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 1);
    vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 2);
    vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[1]) + 3);
#else
    vW = M1.r[1];
    vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    XMVECTOR r1 = vX;
#if defined(_XM_AVX_INTRINSICS_)
    vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 0);
    vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 1);
    vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 2);
    vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[2]) + 3);
#else
    vW = M1.r[2];
    vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    XMVECTOR r2 = vX;
#if defined(_XM_AVX_INTRINSICS_)
    vX = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 0);
    vY = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 1);
    vZ = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 2);
    vW = _mm_broadcast_ss(reinterpret_cast<const float*>(&M1.r[3]) + 3);
#else
    vW = M1.r[3];
    vX = XM_PERMUTE_PS(vW, _MM_SHUFFLE(0, 0, 0, 0));
    vY = XM_PERMUTE_PS(vW, _MM_SHUFFLE(1, 1, 1, 1));
    vZ = XM_PERMUTE_PS(vW, _MM_SHUFFLE(2, 2, 2, 2));
    vW = XM_PERMUTE_PS(vW, _MM_SHUFFLE(3, 3, 3, 3));
#endif
    vX = _mm_mul_ps(vX, M2.r[0]);
    vY = _mm_mul_ps(vY, M2.r[1]);
    vZ = _mm_mul_ps(vZ, M2.r[2]);
    vW = _mm_mul_ps(vW, M2.r[3]);
    vX = _mm_add_ps(vX, vZ);
    vY = _mm_add_ps(vY, vW);
    vX = _mm_add_ps(vX, vY);
    XMVECTOR r3 = vX;

    // Transpose result
    // x.x,x.y,y.x,y.y
    XMVECTOR vTemp1 = _mm_shuffle_ps(r0, r1, _MM_SHUFFLE(1, 0, 1, 0));
    // x.z,x.w,y.z,y.w
    XMVECTOR vTemp3 = _mm_shuffle_ps(r0, r1, _MM_SHUFFLE(3, 2, 3, 2));
    // z.x,z.y,w.x,w.y
    XMVECTOR vTemp2 = _mm_shuffle_ps(r2, r3, _MM_SHUFFLE(1, 0, 1, 0));
    // z.z,z.w,w.z,w.w
    XMVECTOR vTemp4 = _mm_shuffle_ps(r2, r3, _MM_SHUFFLE(3, 2, 3, 2));

    XMMATRIX mResult;
    // x.x,y.x,z.x,w.x
    mResult.r[0] = _mm_shuffle_ps(vTemp1, vTemp2, _MM_SHUFFLE(2, 0, 2, 0));
    // x.y,y.y,z.y,w.y
    mResult.r[1] = _mm_shuffle_ps(vTemp1, vTemp2, _MM_SHUFFLE(3, 1, 3, 1));
    // x.z,y.z,z.z,w.z
    mResult.r[2] = _mm_shuffle_ps(vTemp3, vTemp4, _MM_SHUFFLE(2, 0, 2, 0));
    // x.w,y.w,z.w,w.w
    mResult.r[3] = _mm_shuffle_ps(vTemp3, vTemp4, _MM_SHUFFLE(3, 1, 3, 1));
    return mResult;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixTranspose(FXMMATRIX M) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    // Original matrix:
    //
    //     m00m01m02m03
    //     m10m11m12m13
    //     m20m21m22m23
    //     m30m31m32m33

    XMMATRIX P;
    P.r[0] = XMVectorMergeXY(M.r[0], M.r[2]); // m00m20m01m21
    P.r[1] = XMVectorMergeXY(M.r[1], M.r[3]); // m10m30m11m31
    P.r[2] = XMVectorMergeZW(M.r[0], M.r[2]); // m02m22m03m23
    P.r[3] = XMVectorMergeZW(M.r[1], M.r[3]); // m12m32m13m33

    XMMATRIX MT;
    MT.r[0] = XMVectorMergeXY(P.r[0], P.r[1]); // m00m10m20m30
    MT.r[1] = XMVectorMergeZW(P.r[0], P.r[1]); // m01m11m21m31
    MT.r[2] = XMVectorMergeXY(P.r[2], P.r[3]); // m02m12m22m32
    MT.r[3] = XMVectorMergeZW(P.r[2], P.r[3]); // m03m13m23m33
    return MT;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4x2_t P0 = vzipq_f32(M.r[0], M.r[2]);
    float32x4x2_t P1 = vzipq_f32(M.r[1], M.r[3]);

    float32x4x2_t T0 = vzipq_f32(P0.val[0], P1.val[0]);
    float32x4x2_t T1 = vzipq_f32(P0.val[1], P1.val[1]);

    XMMATRIX mResult;
    mResult.r[0] = T0.val[0];
    mResult.r[1] = T0.val[1];
    mResult.r[2] = T1.val[0];
    mResult.r[3] = T1.val[1];
    return mResult;
#elif defined(_XM_AVX2_INTRINSICS_)
    __m256 t0 = _mm256_castps128_ps256(M.r[0]);
    t0 = _mm256_insertf128_ps(t0, M.r[1], 1);
    __m256 t1 = _mm256_castps128_ps256(M.r[2]);
    t1 = _mm256_insertf128_ps(t1, M.r[3], 1);

    __m256 vTemp = _mm256_unpacklo_ps(t0, t1);
    __m256 vTemp2 = _mm256_unpackhi_ps(t0, t1);
    __m256 vTemp3 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x20);
    __m256 vTemp4 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x31);
    vTemp = _mm256_unpacklo_ps(vTemp3, vTemp4);
    vTemp2 = _mm256_unpackhi_ps(vTemp3, vTemp4);
    t0 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x20);
    t1 = _mm256_permute2f128_ps(vTemp, vTemp2, 0x31);

    XMMATRIX mResult;
    mResult.r[0] = _mm256_castps256_ps128(t0);
    mResult.r[1] = _mm256_extractf128_ps(t0, 1);
    mResult.r[2] = _mm256_castps256_ps128(t1);
    mResult.r[3] = _mm256_extractf128_ps(t1, 1);
    return mResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // x.x,x.y,y.x,y.y
    XMVECTOR vTemp1 = _mm_shuffle_ps(M.r[0], M.r[1], _MM_SHUFFLE(1, 0, 1, 0));
    // x.z,x.w,y.z,y.w
    XMVECTOR vTemp3 = _mm_shuffle_ps(M.r[0], M.r[1], _MM_SHUFFLE(3, 2, 3, 2));
    // z.x,z.y,w.x,w.y
    XMVECTOR vTemp2 = _mm_shuffle_ps(M.r[2], M.r[3], _MM_SHUFFLE(1, 0, 1, 0));
    // z.z,z.w,w.z,w.w
    XMVECTOR vTemp4 = _mm_shuffle_ps(M.r[2], M.r[3], _MM_SHUFFLE(3, 2, 3, 2));

    XMMATRIX mResult;
    // x.x,y.x,z.x,w.x
    mResult.r[0] = _mm_shuffle_ps(vTemp1, vTemp2, _MM_SHUFFLE(2, 0, 2, 0));
    // x.y,y.y,z.y,w.y
    mResult.r[1] = _mm_shuffle_ps(vTemp1, vTemp2, _MM_SHUFFLE(3, 1, 3, 1));
    // x.z,y.z,z.z,w.z
    mResult.r[2] = _mm_shuffle_ps(vTemp3, vTemp4, _MM_SHUFFLE(2, 0, 2, 0));
    // x.w,y.w,z.w,w.w
    mResult.r[3] = _mm_shuffle_ps(vTemp3, vTemp4, _MM_SHUFFLE(3, 1, 3, 1));
    return mResult;
#endif
}

//------------------------------------------------------------------------------
// Return the inverse and the determinant of a 4x4 matrix
_Use_decl_annotations_
inline XMMATRIX XM_CALLCONV XMMatrixInverse
(
    XMVECTOR* pDeterminant,
    FXMMATRIX  M
) noexcept
{
#if defined(_XM_NO_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    XMMATRIX MT = XMMatrixTranspose(M);

    XMVECTOR V0[4], V1[4];
    V0[0] = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(MT.r[2]);
    V1[0] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_W>(MT.r[3]);
    V0[1] = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(MT.r[0]);
    V1[1] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_W>(MT.r[1]);
    V0[2] = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_0Z, XM_PERMUTE_1X, XM_PERMUTE_1Z>(MT.r[2], MT.r[0]);
    V1[2] = XMVectorPermute<XM_PERMUTE_0Y, XM_PERMUTE_0W, XM_PERMUTE_1Y, XM_PERMUTE_1W>(MT.r[3], MT.r[1]);

    XMVECTOR D0 = XMVectorMultiply(V0[0], V1[0]);
    XMVECTOR D1 = XMVectorMultiply(V0[1], V1[1]);
    XMVECTOR D2 = XMVectorMultiply(V0[2], V1[2]);

    V0[0] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_W>(MT.r[2]);
    V1[0] = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(MT.r[3]);
    V0[1] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_W>(MT.r[0]);
    V1[1] = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(MT.r[1]);
    V0[2] = XMVectorPermute<XM_PERMUTE_0Y, XM_PERMUTE_0W, XM_PERMUTE_1Y, XM_PERMUTE_1W>(MT.r[2], MT.r[0]);
    V1[2] = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_0Z, XM_PERMUTE_1X, XM_PERMUTE_1Z>(MT.r[3], MT.r[1]);

    D0 = XMVectorNegativeMultiplySubtract(V0[0], V1[0], D0);
    D1 = XMVectorNegativeMultiplySubtract(V0[1], V1[1], D1);
    D2 = XMVectorNegativeMultiplySubtract(V0[2], V1[2], D2);

    V0[0] = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_X, XM_SWIZZLE_Y>(MT.r[1]);
    V1[0] = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0Y, XM_PERMUTE_0W, XM_PERMUTE_0X>(D0, D2);
    V0[1] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_X>(MT.r[0]);
    V1[1] = XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1Y, XM_PERMUTE_0Y, XM_PERMUTE_0Z>(D0, D2);
    V0[2] = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_X, XM_SWIZZLE_Y>(MT.r[3]);
    V1[2] = XMVectorPermute<XM_PERMUTE_1W, XM_PERMUTE_0Y, XM_PERMUTE_0W, XM_PERMUTE_0X>(D1, D2);
    V0[3] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_X>(MT.r[2]);
    V1[3] = XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_1W, XM_PERMUTE_0Y, XM_PERMUTE_0Z>(D1, D2);

    XMVECTOR C0 = XMVectorMultiply(V0[0], V1[0]);
    XMVECTOR C2 = XMVectorMultiply(V0[1], V1[1]);
    XMVECTOR C4 = XMVectorMultiply(V0[2], V1[2]);
    XMVECTOR C6 = XMVectorMultiply(V0[3], V1[3]);

    V0[0] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Y, XM_SWIZZLE_Z>(MT.r[1]);
    V1[0] = XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_0X, XM_PERMUTE_0Y, XM_PERMUTE_1X>(D0, D2);
    V0[1] = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Y>(MT.r[0]);
    V1[1] = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0Y, XM_PERMUTE_1X, XM_PERMUTE_0X>(D0, D2);
    V0[2] = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Y, XM_SWIZZLE_Z>(MT.r[3]);
    V1[2] = XMVectorPermute<XM_PERMUTE_0W, XM_PERMUTE_0X, XM_PERMUTE_0Y, XM_PERMUTE_1Z>(D1, D2);
    V0[3] = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_Z, XM_SWIZZLE_W, XM_SWIZZLE_Y>(MT.r[2]);
    V1[3] = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0Y, XM_PERMUTE_1Z, XM_PERMUTE_0X>(D1, D2);

    C0 = XMVectorNegativeMultiplySubtract(V0[0], V1[0], C0);
    C2 = XMVectorNegativeMultiplySubtract(V0[1], V1[1], C2);
    C4 = XMVectorNegativeMultiplySubtract(V0[2], V1[2], C4);
    C6 = XMVectorNegativeMultiplySubtract(V0[3], V1[3], C6);

    V0[0] = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_X>(MT.r[1]);
    V1[0] = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_1Y, XM_PERMUTE_1X, XM_PERMUTE_0Z>(D0, D2);
    V0[1] = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Z>(MT.r[0]);
    V1[1] = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_1X>(D0, D2);
    V0[2] = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_W, XM_SWIZZLE_X>(MT.r[3]);
    V1[2] = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_1W, XM_PERMUTE_1Z, XM_PERMUTE_0Z>(D1, D2);
    V0[3] = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_W, XM_SWIZZLE_X, XM_SWIZZLE_Z>(MT.r[2]);
    V1[3] = XMVectorPermute<XM_PERMUTE_1W, XM_PERMUTE_0X, XM_PERMUTE_0W, XM_PERMUTE_1Z>(D1, D2);

    XMVECTOR C1 = XMVectorNegativeMultiplySubtract(V0[0], V1[0], C0);
    C0 = XMVectorMultiplyAdd(V0[0], V1[0], C0);
    XMVECTOR C3 = XMVectorMultiplyAdd(V0[1], V1[1], C2);
    C2 = XMVectorNegativeMultiplySubtract(V0[1], V1[1], C2);
    XMVECTOR C5 = XMVectorNegativeMultiplySubtract(V0[2], V1[2], C4);
    C4 = XMVectorMultiplyAdd(V0[2], V1[2], C4);
    XMVECTOR C7 = XMVectorMultiplyAdd(V0[3], V1[3], C6);
    C6 = XMVectorNegativeMultiplySubtract(V0[3], V1[3], C6);

    XMMATRIX R;
    R.r[0] = XMVectorSelect(C0, C1, g_XMSelect0101.v);
    R.r[1] = XMVectorSelect(C2, C3, g_XMSelect0101.v);
    R.r[2] = XMVectorSelect(C4, C5, g_XMSelect0101.v);
    R.r[3] = XMVectorSelect(C6, C7, g_XMSelect0101.v);

    XMVECTOR Determinant = XMVector4Dot(R.r[0], MT.r[0]);

    if (pDeterminant != nullptr)
        *pDeterminant = Determinant;

    XMVECTOR Reciprocal = XMVectorReciprocal(Determinant);

    XMMATRIX Result;
    Result.r[0] = XMVectorMultiply(R.r[0], Reciprocal);
    Result.r[1] = XMVectorMultiply(R.r[1], Reciprocal);
    Result.r[2] = XMVectorMultiply(R.r[2], Reciprocal);
    Result.r[3] = XMVectorMultiply(R.r[3], Reciprocal);
    return Result;

#elif defined(_XM_SSE_INTRINSICS_)
    // Transpose matrix
    XMVECTOR vTemp1 = _mm_shuffle_ps(M.r[0], M.r[1], _MM_SHUFFLE(1, 0, 1, 0));
    XMVECTOR vTemp3 = _mm_shuffle_ps(M.r[0], M.r[1], _MM_SHUFFLE(3, 2, 3, 2));
    XMVECTOR vTemp2 = _mm_shuffle_ps(M.r[2], M.r[3], _MM_SHUFFLE(1, 0, 1, 0));
    XMVECTOR vTemp4 = _mm_shuffle_ps(M.r[2], M.r[3], _MM_SHUFFLE(3, 2, 3, 2));

    XMMATRIX MT;
    MT.r[0] = _mm_shuffle_ps(vTemp1, vTemp2, _MM_SHUFFLE(2, 0, 2, 0));
    MT.r[1] = _mm_shuffle_ps(vTemp1, vTemp2, _MM_SHUFFLE(3, 1, 3, 1));
    MT.r[2] = _mm_shuffle_ps(vTemp3, vTemp4, _MM_SHUFFLE(2, 0, 2, 0));
    MT.r[3] = _mm_shuffle_ps(vTemp3, vTemp4, _MM_SHUFFLE(3, 1, 3, 1));

    XMVECTOR V00 = XM_PERMUTE_PS(MT.r[2], _MM_SHUFFLE(1, 1, 0, 0));
    XMVECTOR V10 = XM_PERMUTE_PS(MT.r[3], _MM_SHUFFLE(3, 2, 3, 2));
    XMVECTOR V01 = XM_PERMUTE_PS(MT.r[0], _MM_SHUFFLE(1, 1, 0, 0));
    XMVECTOR V11 = XM_PERMUTE_PS(MT.r[1], _MM_SHUFFLE(3, 2, 3, 2));
    XMVECTOR V02 = _mm_shuffle_ps(MT.r[2], MT.r[0], _MM_SHUFFLE(2, 0, 2, 0));
    XMVECTOR V12 = _mm_shuffle_ps(MT.r[3], MT.r[1], _MM_SHUFFLE(3, 1, 3, 1));

    XMVECTOR D0 = _mm_mul_ps(V00, V10);
    XMVECTOR D1 = _mm_mul_ps(V01, V11);
    XMVECTOR D2 = _mm_mul_ps(V02, V12);

    V00 = XM_PERMUTE_PS(MT.r[2], _MM_SHUFFLE(3, 2, 3, 2));
    V10 = XM_PERMUTE_PS(MT.r[3], _MM_SHUFFLE(1, 1, 0, 0));
    V01 = XM_PERMUTE_PS(MT.r[0], _MM_SHUFFLE(3, 2, 3, 2));
    V11 = XM_PERMUTE_PS(MT.r[1], _MM_SHUFFLE(1, 1, 0, 0));
    V02 = _mm_shuffle_ps(MT.r[2], MT.r[0], _MM_SHUFFLE(3, 1, 3, 1));
    V12 = _mm_shuffle_ps(MT.r[3], MT.r[1], _MM_SHUFFLE(2, 0, 2, 0));

    D0 = XM_FNMADD_PS(V00, V10, D0);
    D1 = XM_FNMADD_PS(V01, V11, D1);
    D2 = XM_FNMADD_PS(V02, V12, D2);
    // V11 = D0Y,D0W,D2Y,D2Y
    V11 = _mm_shuffle_ps(D0, D2, _MM_SHUFFLE(1, 1, 3, 1));
    V00 = XM_PERMUTE_PS(MT.r[1], _MM_SHUFFLE(1, 0, 2, 1));
    V10 = _mm_shuffle_ps(V11, D0, _MM_SHUFFLE(0, 3, 0, 2));
    V01 = XM_PERMUTE_PS(MT.r[0], _MM_SHUFFLE(0, 1, 0, 2));
    V11 = _mm_shuffle_ps(V11, D0, _MM_SHUFFLE(2, 1, 2, 1));
    // V13 = D1Y,D1W,D2W,D2W
    XMVECTOR V13 = _mm_shuffle_ps(D1, D2, _MM_SHUFFLE(3, 3, 3, 1));
    V02 = XM_PERMUTE_PS(MT.r[3], _MM_SHUFFLE(1, 0, 2, 1));
    V12 = _mm_shuffle_ps(V13, D1, _MM_SHUFFLE(0, 3, 0, 2));
    XMVECTOR V03 = XM_PERMUTE_PS(MT.r[2], _MM_SHUFFLE(0, 1, 0, 2));
    V13 = _mm_shuffle_ps(V13, D1, _MM_SHUFFLE(2, 1, 2, 1));

    XMVECTOR C0 = _mm_mul_ps(V00, V10);
    XMVECTOR C2 = _mm_mul_ps(V01, V11);
    XMVECTOR C4 = _mm_mul_ps(V02, V12);
    XMVECTOR C6 = _mm_mul_ps(V03, V13);

    // V11 = D0X,D0Y,D2X,D2X
    V11 = _mm_shuffle_ps(D0, D2, _MM_SHUFFLE(0, 0, 1, 0));
    V00 = XM_PERMUTE_PS(MT.r[1], _MM_SHUFFLE(2, 1, 3, 2));
    V10 = _mm_shuffle_ps(D0, V11, _MM_SHUFFLE(2, 1, 0, 3));
    V01 = XM_PERMUTE_PS(MT.r[0], _MM_SHUFFLE(1, 3, 2, 3));
    V11 = _mm_shuffle_ps(D0, V11, _MM_SHUFFLE(0, 2, 1, 2));
    // V13 = D1X,D1Y,D2Z,D2Z
    V13 = _mm_shuffle_ps(D1, D2, _MM_SHUFFLE(2, 2, 1, 0));
    V02 = XM_PERMUTE_PS(MT.r[3], _MM_SHUFFLE(2, 1, 3, 2));
    V12 = _mm_shuffle_ps(D1, V13, _MM_SHUFFLE(2, 1, 0, 3));
    V03 = XM_PERMUTE_PS(MT.r[2], _MM_SHUFFLE(1, 3, 2, 3));
    V13 = _mm_shuffle_ps(D1, V13, _MM_SHUFFLE(0, 2, 1, 2));

    C0 = XM_FNMADD_PS(V00, V10, C0);
    C2 = XM_FNMADD_PS(V01, V11, C2);
    C4 = XM_FNMADD_PS(V02, V12, C4);
    C6 = XM_FNMADD_PS(V03, V13, C6);

    V00 = XM_PERMUTE_PS(MT.r[1], _MM_SHUFFLE(0, 3, 0, 3));
    // V10 = D0Z,D0Z,D2X,D2Y
    V10 = _mm_shuffle_ps(D0, D2, _MM_SHUFFLE(1, 0, 2, 2));
    V10 = XM_PERMUTE_PS(V10, _MM_SHUFFLE(0, 2, 3, 0));
    V01 = XM_PERMUTE_PS(MT.r[0], _MM_SHUFFLE(2, 0, 3, 1));
    // V11 = D0X,D0W,D2X,D2Y
    V11 = _mm_shuffle_ps(D0, D2, _MM_SHUFFLE(1, 0, 3, 0));
    V11 = XM_PERMUTE_PS(V11, _MM_SHUFFLE(2, 1, 0, 3));
    V02 = XM_PERMUTE_PS(MT.r[3], _MM_SHUFFLE(0, 3, 0, 3));
    // V12 = D1Z,D1Z,D2Z,D2W
    V12 = _mm_shuffle_ps(D1, D2, _MM_SHUFFLE(3, 2, 2, 2));
    V12 = XM_PERMUTE_PS(V12, _MM_SHUFFLE(0, 2, 3, 0));
    V03 = XM_PERMUTE_PS(MT.r[2], _MM_SHUFFLE(2, 0, 3, 1));
    // V13 = D1X,D1W,D2Z,D2W
    V13 = _mm_shuffle_ps(D1, D2, _MM_SHUFFLE(3, 2, 3, 0));
    V13 = XM_PERMUTE_PS(V13, _MM_SHUFFLE(2, 1, 0, 3));

    V00 = _mm_mul_ps(V00, V10);
    V01 = _mm_mul_ps(V01, V11);
    V02 = _mm_mul_ps(V02, V12);
    V03 = _mm_mul_ps(V03, V13);
    XMVECTOR C1 = _mm_sub_ps(C0, V00);
    C0 = _mm_add_ps(C0, V00);
    XMVECTOR C3 = _mm_add_ps(C2, V01);
    C2 = _mm_sub_ps(C2, V01);
    XMVECTOR C5 = _mm_sub_ps(C4, V02);
    C4 = _mm_add_ps(C4, V02);
    XMVECTOR C7 = _mm_add_ps(C6, V03);
    C6 = _mm_sub_ps(C6, V03);

    C0 = _mm_shuffle_ps(C0, C1, _MM_SHUFFLE(3, 1, 2, 0));
    C2 = _mm_shuffle_ps(C2, C3, _MM_SHUFFLE(3, 1, 2, 0));
    C4 = _mm_shuffle_ps(C4, C5, _MM_SHUFFLE(3, 1, 2, 0));
    C6 = _mm_shuffle_ps(C6, C7, _MM_SHUFFLE(3, 1, 2, 0));
    C0 = XM_PERMUTE_PS(C0, _MM_SHUFFLE(3, 1, 2, 0));
    C2 = XM_PERMUTE_PS(C2, _MM_SHUFFLE(3, 1, 2, 0));
    C4 = XM_PERMUTE_PS(C4, _MM_SHUFFLE(3, 1, 2, 0));
    C6 = XM_PERMUTE_PS(C6, _MM_SHUFFLE(3, 1, 2, 0));
    // Get the determinant
    XMVECTOR vTemp = XMVector4Dot(C0, MT.r[0]);
    if (pDeterminant != nullptr)
        *pDeterminant = vTemp;
    vTemp = _mm_div_ps(g_XMOne, vTemp);
    XMMATRIX mResult;
    mResult.r[0] = _mm_mul_ps(C0, vTemp);
    mResult.r[1] = _mm_mul_ps(C2, vTemp);
    mResult.r[2] = _mm_mul_ps(C4, vTemp);
    mResult.r[3] = _mm_mul_ps(C6, vTemp);
    return mResult;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixVectorTensorProduct
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    XMMATRIX mResult;
    mResult.r[0] = XMVectorMultiply(XMVectorSwizzle<0, 0, 0, 0>(V1), V2);
    mResult.r[1] = XMVectorMultiply(XMVectorSwizzle<1, 1, 1, 1>(V1), V2);
    mResult.r[2] = XMVectorMultiply(XMVectorSwizzle<2, 2, 2, 2>(V1), V2);
    mResult.r[3] = XMVectorMultiply(XMVectorSwizzle<3, 3, 3, 3>(V1), V2);
    return mResult;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMMatrixDeterminant(FXMMATRIX M) noexcept
{
    static const XMVECTORF32 Sign = { { { 1.0f, -1.0f, 1.0f, -1.0f } } };

    XMVECTOR V0 = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_X>(M.r[2]);
    XMVECTOR V1 = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(M.r[3]);
    XMVECTOR V2 = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_X>(M.r[2]);
    XMVECTOR V3 = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_Z>(M.r[3]);
    XMVECTOR V4 = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(M.r[2]);
    XMVECTOR V5 = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_Z>(M.r[3]);

    XMVECTOR P0 = XMVectorMultiply(V0, V1);
    XMVECTOR P1 = XMVectorMultiply(V2, V3);
    XMVECTOR P2 = XMVectorMultiply(V4, V5);

    V0 = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(M.r[2]);
    V1 = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_X>(M.r[3]);
    V2 = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_Z>(M.r[2]);
    V3 = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_X>(M.r[3]);
    V4 = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_Z>(M.r[2]);
    V5 = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(M.r[3]);

    P0 = XMVectorNegativeMultiplySubtract(V0, V1, P0);
    P1 = XMVectorNegativeMultiplySubtract(V2, V3, P1);
    P2 = XMVectorNegativeMultiplySubtract(V4, V5, P2);

    V0 = XMVectorSwizzle<XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_W, XM_SWIZZLE_Z>(M.r[1]);
    V1 = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(M.r[1]);
    V2 = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_X>(M.r[1]);

    XMVECTOR S = XMVectorMultiply(M.r[0], Sign.v);
    XMVECTOR R = XMVectorMultiply(V0, P0);
    R = XMVectorNegativeMultiplySubtract(V1, P1, R);
    R = XMVectorMultiplyAdd(V2, P2, R);

    return XMVector4Dot(S, R);
}

#define XM3RANKDECOMPOSE(a, b, c, x, y, z)      \
    if((x) < (y))                   \
    {                               \
        if((y) < (z))               \
        {                           \
            (a) = 2;                \
            (b) = 1;                \
            (c) = 0;                \
        }                           \
        else                        \
        {                           \
            (a) = 1;                \
                                    \
            if((x) < (z))           \
            {                       \
                (b) = 2;            \
                (c) = 0;            \
            }                       \
            else                    \
            {                       \
                (b) = 0;            \
                (c) = 2;            \
            }                       \
        }                           \
    }                               \
    else                            \
    {                               \
        if((x) < (z))               \
        {                           \
            (a) = 2;                \
            (b) = 0;                \
            (c) = 1;                \
        }                           \
        else                        \
        {                           \
            (a) = 0;                \
                                    \
            if((y) < (z))           \
            {                       \
                (b) = 2;            \
                (c) = 1;            \
            }                       \
            else                    \
            {                       \
                (b) = 1;            \
                (c) = 2;            \
            }                       \
        }                           \
    }

#define XM3_DECOMP_EPSILON 0.0001f

_Use_decl_annotations_
inline bool XM_CALLCONV XMMatrixDecompose
(
    XMVECTOR* outScale,
    XMVECTOR* outRotQuat,
    XMVECTOR* outTrans,
    FXMMATRIX M
) noexcept
{
    static const XMVECTOR* pvCanonicalBasis[3] = {
        &g_XMIdentityR0.v,
        &g_XMIdentityR1.v,
        &g_XMIdentityR2.v
    };

    assert(outScale != nullptr);
    assert(outRotQuat != nullptr);
    assert(outTrans != nullptr);

    // Get the translation
    outTrans[0] = M.r[3];

    XMVECTOR* ppvBasis[3];
    XMMATRIX matTemp;
    ppvBasis[0] = &matTemp.r[0];
    ppvBasis[1] = &matTemp.r[1];
    ppvBasis[2] = &matTemp.r[2];

    matTemp.r[0] = M.r[0];
    matTemp.r[1] = M.r[1];
    matTemp.r[2] = M.r[2];
    matTemp.r[3] = g_XMIdentityR3.v;

    auto pfScales = reinterpret_cast<float*>(outScale);

    size_t a, b, c;
    XMVectorGetXPtr(&pfScales[0], XMVector3Length(ppvBasis[0][0]));
    XMVectorGetXPtr(&pfScales[1], XMVector3Length(ppvBasis[1][0]));
    XMVectorGetXPtr(&pfScales[2], XMVector3Length(ppvBasis[2][0]));
    pfScales[3] = 0.f;

    XM3RANKDECOMPOSE(a, b, c, pfScales[0], pfScales[1], pfScales[2])

        if (pfScales[a] < XM3_DECOMP_EPSILON)
        {
            ppvBasis[a][0] = pvCanonicalBasis[a][0];
        }
    ppvBasis[a][0] = XMVector3Normalize(ppvBasis[a][0]);

    if (pfScales[b] < XM3_DECOMP_EPSILON)
    {
        size_t aa, bb, cc;
        float fAbsX, fAbsY, fAbsZ;

        fAbsX = fabsf(XMVectorGetX(ppvBasis[a][0]));
        fAbsY = fabsf(XMVectorGetY(ppvBasis[a][0]));
        fAbsZ = fabsf(XMVectorGetZ(ppvBasis[a][0]));

        XM3RANKDECOMPOSE(aa, bb, cc, fAbsX, fAbsY, fAbsZ)

            ppvBasis[b][0] = XMVector3Cross(ppvBasis[a][0], pvCanonicalBasis[cc][0]);
    }

    ppvBasis[b][0] = XMVector3Normalize(ppvBasis[b][0]);

    if (pfScales[c] < XM3_DECOMP_EPSILON)
    {
        ppvBasis[c][0] = XMVector3Cross(ppvBasis[a][0], ppvBasis[b][0]);
    }

    ppvBasis[c][0] = XMVector3Normalize(ppvBasis[c][0]);

    float fDet = XMVectorGetX(XMMatrixDeterminant(matTemp));

    // use Kramer's rule to check for handedness of coordinate system
    if (fDet < 0.0f)
    {
        // switch coordinate system by negating the scale and inverting the basis vector on the x-axis
        pfScales[a] = -pfScales[a];
        ppvBasis[a][0] = XMVectorNegate(ppvBasis[a][0]);

        fDet = -fDet;
    }

    fDet -= 1.0f;
    fDet *= fDet;

    if (XM3_DECOMP_EPSILON < fDet)
    {
        // Non-SRT matrix encountered
        return false;
    }

    // generate the quaternion from the matrix
    outRotQuat[0] = XMQuaternionRotationMatrix(matTemp);
    return true;
}

#undef XM3_DECOMP_EPSILON
#undef XM3RANKDECOMPOSE

//------------------------------------------------------------------------------
// Transformation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixIdentity() noexcept
{
    XMMATRIX M;
    M.r[0] = g_XMIdentityR0.v;
    M.r[1] = g_XMIdentityR1.v;
    M.r[2] = g_XMIdentityR2.v;
    M.r[3] = g_XMIdentityR3.v;
    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixSet
(
    float m00, float m01, float m02, float m03,
    float m10, float m11, float m12, float m13,
    float m20, float m21, float m22, float m23,
    float m30, float m31, float m32, float m33
) noexcept
{
    XMMATRIX M;
#if defined(_XM_NO_INTRINSICS_)
    M.m[0][0] = m00; M.m[0][1] = m01; M.m[0][2] = m02; M.m[0][3] = m03;
    M.m[1][0] = m10; M.m[1][1] = m11; M.m[1][2] = m12; M.m[1][3] = m13;
    M.m[2][0] = m20; M.m[2][1] = m21; M.m[2][2] = m22; M.m[2][3] = m23;
    M.m[3][0] = m30; M.m[3][1] = m31; M.m[3][2] = m32; M.m[3][3] = m33;
#else
    M.r[0] = XMVectorSet(m00, m01, m02, m03);
    M.r[1] = XMVectorSet(m10, m11, m12, m13);
    M.r[2] = XMVectorSet(m20, m21, m22, m23);
    M.r[3] = XMVectorSet(m30, m31, m32, m33);
#endif
    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixTranslation
(
    float OffsetX,
    float OffsetY,
    float OffsetZ
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMMATRIX M;
    M.m[0][0] = 1.0f;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = 1.0f;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = 1.0f;
    M.m[2][3] = 0.0f;

    M.m[3][0] = OffsetX;
    M.m[3][1] = OffsetY;
    M.m[3][2] = OffsetZ;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_SSE_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)
    XMMATRIX M;
    M.r[0] = g_XMIdentityR0.v;
    M.r[1] = g_XMIdentityR1.v;
    M.r[2] = g_XMIdentityR2.v;
    M.r[3] = XMVectorSet(OffsetX, OffsetY, OffsetZ, 1.f);
    return M;
#endif
}


//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixTranslationFromVector(FXMVECTOR Offset) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMMATRIX M;
    M.m[0][0] = 1.0f;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = 1.0f;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = 1.0f;
    M.m[2][3] = 0.0f;

    M.m[3][0] = Offset.vector4_f32[0];
    M.m[3][1] = Offset.vector4_f32[1];
    M.m[3][2] = Offset.vector4_f32[2];
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_SSE_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)
    XMMATRIX M;
    M.r[0] = g_XMIdentityR0.v;
    M.r[1] = g_XMIdentityR1.v;
    M.r[2] = g_XMIdentityR2.v;
    M.r[3] = XMVectorSelect(g_XMIdentityR3.v, Offset, g_XMSelect1110.v);
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixScaling
(
    float ScaleX,
    float ScaleY,
    float ScaleZ
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMMATRIX M;
    M.m[0][0] = ScaleX;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = ScaleY;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = ScaleZ;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = 0.0f;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    const XMVECTOR Zero = vdupq_n_f32(0);
    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(ScaleX, Zero, 0);
    M.r[1] = vsetq_lane_f32(ScaleY, Zero, 1);
    M.r[2] = vsetq_lane_f32(ScaleZ, Zero, 2);
    M.r[3] = g_XMIdentityR3.v;
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    M.r[0] = _mm_set_ps(0, 0, 0, ScaleX);
    M.r[1] = _mm_set_ps(0, 0, ScaleY, 0);
    M.r[2] = _mm_set_ps(0, ScaleZ, 0, 0);
    M.r[3] = g_XMIdentityR3.v;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixScalingFromVector(FXMVECTOR Scale) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMMATRIX M;
    M.m[0][0] = Scale.vector4_f32[0];
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = Scale.vector4_f32[1];
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = Scale.vector4_f32[2];
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = 0.0f;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMMATRIX M;
    M.r[0] = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(Scale), g_XMMaskX));
    M.r[1] = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(Scale), g_XMMaskY));
    M.r[2] = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(Scale), g_XMMaskZ));
    M.r[3] = g_XMIdentityR3.v;
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    M.r[0] = _mm_and_ps(Scale, g_XMMaskX);
    M.r[1] = _mm_and_ps(Scale, g_XMMaskY);
    M.r[2] = _mm_and_ps(Scale, g_XMMaskZ);
    M.r[3] = g_XMIdentityR3.v;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationX(float Angle) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    XMMATRIX M;
    M.m[0][0] = 1.0f;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = fCosAngle;
    M.m[1][2] = fSinAngle;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = -fSinAngle;
    M.m[2][2] = fCosAngle;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = 0.0f;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    const float32x4_t Zero = vdupq_n_f32(0);

    float32x4_t T1 = vsetq_lane_f32(fCosAngle, Zero, 1);
    T1 = vsetq_lane_f32(fSinAngle, T1, 2);

    float32x4_t T2 = vsetq_lane_f32(-fSinAngle, Zero, 1);
    T2 = vsetq_lane_f32(fCosAngle, T2, 2);

    XMMATRIX M;
    M.r[0] = g_XMIdentityR0.v;
    M.r[1] = T1;
    M.r[2] = T2;
    M.r[3] = g_XMIdentityR3.v;
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    float    SinAngle;
    float    CosAngle;
    XMScalarSinCos(&SinAngle, &CosAngle, Angle);

    XMVECTOR vSin = _mm_set_ss(SinAngle);
    XMVECTOR vCos = _mm_set_ss(CosAngle);
    // x = 0,y = cos,z = sin, w = 0
    vCos = _mm_shuffle_ps(vCos, vSin, _MM_SHUFFLE(3, 0, 0, 3));
    XMMATRIX M;
    M.r[0] = g_XMIdentityR0;
    M.r[1] = vCos;
    // x = 0,y = sin,z = cos, w = 0
    vCos = XM_PERMUTE_PS(vCos, _MM_SHUFFLE(3, 1, 2, 0));
    // x = 0,y = -sin,z = cos, w = 0
    vCos = _mm_mul_ps(vCos, g_XMNegateY);
    M.r[2] = vCos;
    M.r[3] = g_XMIdentityR3;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationY(float Angle) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    XMMATRIX M;
    M.m[0][0] = fCosAngle;
    M.m[0][1] = 0.0f;
    M.m[0][2] = -fSinAngle;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = 1.0f;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = fSinAngle;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fCosAngle;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = 0.0f;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    const float32x4_t Zero = vdupq_n_f32(0);

    float32x4_t T0 = vsetq_lane_f32(fCosAngle, Zero, 0);
    T0 = vsetq_lane_f32(-fSinAngle, T0, 2);

    float32x4_t T2 = vsetq_lane_f32(fSinAngle, Zero, 0);
    T2 = vsetq_lane_f32(fCosAngle, T2, 2);

    XMMATRIX M;
    M.r[0] = T0;
    M.r[1] = g_XMIdentityR1.v;
    M.r[2] = T2;
    M.r[3] = g_XMIdentityR3.v;
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    float    SinAngle;
    float    CosAngle;
    XMScalarSinCos(&SinAngle, &CosAngle, Angle);

    XMVECTOR vSin = _mm_set_ss(SinAngle);
    XMVECTOR vCos = _mm_set_ss(CosAngle);
    // x = sin,y = 0,z = cos, w = 0
    vSin = _mm_shuffle_ps(vSin, vCos, _MM_SHUFFLE(3, 0, 3, 0));
    XMMATRIX M;
    M.r[2] = vSin;
    M.r[1] = g_XMIdentityR1;
    // x = cos,y = 0,z = sin, w = 0
    vSin = XM_PERMUTE_PS(vSin, _MM_SHUFFLE(3, 0, 1, 2));
    // x = cos,y = 0,z = -sin, w = 0
    vSin = _mm_mul_ps(vSin, g_XMNegateZ);
    M.r[0] = vSin;
    M.r[3] = g_XMIdentityR3;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationZ(float Angle) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    XMMATRIX M;
    M.m[0][0] = fCosAngle;
    M.m[0][1] = fSinAngle;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = -fSinAngle;
    M.m[1][1] = fCosAngle;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = 1.0f;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = 0.0f;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    const float32x4_t Zero = vdupq_n_f32(0);

    float32x4_t T0 = vsetq_lane_f32(fCosAngle, Zero, 0);
    T0 = vsetq_lane_f32(fSinAngle, T0, 1);

    float32x4_t T1 = vsetq_lane_f32(-fSinAngle, Zero, 0);
    T1 = vsetq_lane_f32(fCosAngle, T1, 1);

    XMMATRIX M;
    M.r[0] = T0;
    M.r[1] = T1;
    M.r[2] = g_XMIdentityR2.v;
    M.r[3] = g_XMIdentityR3.v;
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    float    SinAngle;
    float    CosAngle;
    XMScalarSinCos(&SinAngle, &CosAngle, Angle);

    XMVECTOR vSin = _mm_set_ss(SinAngle);
    XMVECTOR vCos = _mm_set_ss(CosAngle);
    // x = cos,y = sin,z = 0, w = 0
    vCos = _mm_unpacklo_ps(vCos, vSin);
    XMMATRIX M;
    M.r[0] = vCos;
    // x = sin,y = cos,z = 0, w = 0
    vCos = XM_PERMUTE_PS(vCos, _MM_SHUFFLE(3, 2, 0, 1));
    // x = cos,y = -sin,z = 0, w = 0
    vCos = _mm_mul_ps(vCos, g_XMNegateX);
    M.r[1] = vCos;
    M.r[2] = g_XMIdentityR2;
    M.r[3] = g_XMIdentityR3;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationRollPitchYaw
(
    float Pitch,
    float Yaw,
    float Roll
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float cp = cosf(Pitch);
    float sp = sinf(Pitch);

    float cy = cosf(Yaw);
    float sy = sinf(Yaw);

    float cr = cosf(Roll);
    float sr = sinf(Roll);

    XMMATRIX M;
    M.m[0][0] = cr * cy + sr * sp * sy;
    M.m[0][1] = sr * cp;
    M.m[0][2] = sr * sp * cy - cr * sy;
    M.m[0][3] = 0.0f;

    M.m[1][0] = cr * sp * sy - sr * cy;
    M.m[1][1] = cr * cp;
    M.m[1][2] = sr * sy + cr * sp * cy;
    M.m[1][3] = 0.0f;

    M.m[2][0] = cp * sy;
    M.m[2][1] = -sp;
    M.m[2][2] = cp * cy;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = 0.0f;
    M.m[3][3] = 1.0f;
    return M;
#else
    XMVECTOR Angles = XMVectorSet(Pitch, Yaw, Roll, 0.0f);
    return XMMatrixRotationRollPitchYawFromVector(Angles);
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationRollPitchYawFromVector
(
    FXMVECTOR Angles // <Pitch, Yaw, Roll, undefined>
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float cp = cosf(Angles.vector4_f32[0]);
    float sp = sinf(Angles.vector4_f32[0]);

    float cy = cosf(Angles.vector4_f32[1]);
    float sy = sinf(Angles.vector4_f32[1]);

    float cr = cosf(Angles.vector4_f32[2]);
    float sr = sinf(Angles.vector4_f32[2]);

    XMMATRIX M;
    M.m[0][0] = cr * cy + sr * sp * sy;
    M.m[0][1] = sr * cp;
    M.m[0][2] = sr * sp * cy - cr * sy;
    M.m[0][3] = 0.0f;

    M.m[1][0] = cr * sp * sy - sr * cy;
    M.m[1][1] = cr * cp;
    M.m[1][2] = sr * sy + cr * sp * cy;
    M.m[1][3] = 0.0f;

    M.m[2][0] = cp * sy;
    M.m[2][1] = -sp;
    M.m[2][2] = cp * cy;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = 0.0f;
    M.m[3][3] = 1.0f;
    return M;
#else
    static const XMVECTORF32  Sign = { { { 1.0f, -1.0f, -1.0f, 1.0f } } };

    XMVECTOR SinAngles, CosAngles;
    XMVectorSinCos(&SinAngles, &CosAngles, Angles);

    XMVECTOR P0 = XMVectorPermute<XM_PERMUTE_1X, XM_PERMUTE_0Z, XM_PERMUTE_1Z, XM_PERMUTE_1X>(SinAngles, CosAngles);
    XMVECTOR Y0 = XMVectorPermute<XM_PERMUTE_0Y, XM_PERMUTE_1X, XM_PERMUTE_1X, XM_PERMUTE_1Y>(SinAngles, CosAngles);
    XMVECTOR P1 = XMVectorPermute<XM_PERMUTE_1Z, XM_PERMUTE_0Z, XM_PERMUTE_1Z, XM_PERMUTE_0Z>(SinAngles, CosAngles);
    XMVECTOR Y1 = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_1Y, XM_PERMUTE_0Y, XM_PERMUTE_0Y>(SinAngles, CosAngles);
    XMVECTOR P2 = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_1Z, XM_PERMUTE_0Z, XM_PERMUTE_1Z>(SinAngles, CosAngles);
    XMVECTOR P3 = XMVectorPermute<XM_PERMUTE_0Y, XM_PERMUTE_0Y, XM_PERMUTE_1Y, XM_PERMUTE_1Y>(SinAngles, CosAngles);
    XMVECTOR Y2 = XMVectorSplatX(SinAngles);
    XMVECTOR NS = XMVectorNegate(SinAngles);

    XMVECTOR Q0 = XMVectorMultiply(P0, Y0);
    XMVECTOR Q1 = XMVectorMultiply(P1, Sign.v);
    Q1 = XMVectorMultiply(Q1, Y1);
    XMVECTOR Q2 = XMVectorMultiply(P2, Y2);
    Q2 = XMVectorMultiplyAdd(Q2, P3, Q1);

    XMVECTOR V0 = XMVectorPermute<XM_PERMUTE_1X, XM_PERMUTE_0Y, XM_PERMUTE_1Z, XM_PERMUTE_0W>(Q0, Q2);
    XMVECTOR V1 = XMVectorPermute<XM_PERMUTE_1Y, XM_PERMUTE_0Z, XM_PERMUTE_1W, XM_PERMUTE_0W>(Q0, Q2);
    XMVECTOR V2 = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_1X, XM_PERMUTE_0W, XM_PERMUTE_0W>(Q0, NS);

    XMMATRIX M;
    M.r[0] = XMVectorSelect(g_XMZero, V0, g_XMSelect1110.v);
    M.r[1] = XMVectorSelect(g_XMZero, V1, g_XMSelect1110.v);
    M.r[2] = XMVectorSelect(g_XMZero, V2, g_XMSelect1110.v);
    M.r[3] = g_XMIdentityR3;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationNormal
(
    FXMVECTOR NormalAxis,
    float     Angle
) noexcept
{
#if defined(_XM_NO_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    XMVECTOR A = XMVectorSet(fSinAngle, fCosAngle, 1.0f - fCosAngle, 0.0f);

    XMVECTOR C2 = XMVectorSplatZ(A);
    XMVECTOR C1 = XMVectorSplatY(A);
    XMVECTOR C0 = XMVectorSplatX(A);

    XMVECTOR N0 = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_X, XM_SWIZZLE_W>(NormalAxis);
    XMVECTOR N1 = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_W>(NormalAxis);

    XMVECTOR V0 = XMVectorMultiply(C2, N0);
    V0 = XMVectorMultiply(V0, N1);

    XMVECTOR R0 = XMVectorMultiply(C2, NormalAxis);
    R0 = XMVectorMultiplyAdd(R0, NormalAxis, C1);

    XMVECTOR R1 = XMVectorMultiplyAdd(C0, NormalAxis, V0);
    XMVECTOR R2 = XMVectorNegativeMultiplySubtract(C0, NormalAxis, V0);

    V0 = XMVectorSelect(A, R0, g_XMSelect1110.v);
    XMVECTOR V1 = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_1Y, XM_PERMUTE_1Z, XM_PERMUTE_0X>(R1, R2);
    XMVECTOR V2 = XMVectorPermute<XM_PERMUTE_0Y, XM_PERMUTE_1X, XM_PERMUTE_0Y, XM_PERMUTE_1X>(R1, R2);

    XMMATRIX M;
    M.r[0] = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_1X, XM_PERMUTE_1Y, XM_PERMUTE_0W>(V0, V1);
    M.r[1] = XMVectorPermute<XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_1W, XM_PERMUTE_0W>(V0, V1);
    M.r[2] = XMVectorPermute<XM_PERMUTE_1X, XM_PERMUTE_1Y, XM_PERMUTE_0Z, XM_PERMUTE_0W>(V0, V2);
    M.r[3] = g_XMIdentityR3.v;
    return M;

#elif defined(_XM_SSE_INTRINSICS_)
    float    fSinAngle;
    float    fCosAngle;
    XMScalarSinCos(&fSinAngle, &fCosAngle, Angle);

    XMVECTOR C2 = _mm_set_ps1(1.0f - fCosAngle);
    XMVECTOR C1 = _mm_set_ps1(fCosAngle);
    XMVECTOR C0 = _mm_set_ps1(fSinAngle);

    XMVECTOR N0 = XM_PERMUTE_PS(NormalAxis, _MM_SHUFFLE(3, 0, 2, 1));
    XMVECTOR N1 = XM_PERMUTE_PS(NormalAxis, _MM_SHUFFLE(3, 1, 0, 2));

    XMVECTOR V0 = _mm_mul_ps(C2, N0);
    V0 = _mm_mul_ps(V0, N1);

    XMVECTOR R0 = _mm_mul_ps(C2, NormalAxis);
    R0 = _mm_mul_ps(R0, NormalAxis);
    R0 = _mm_add_ps(R0, C1);

    XMVECTOR R1 = _mm_mul_ps(C0, NormalAxis);
    R1 = _mm_add_ps(R1, V0);
    XMVECTOR R2 = _mm_mul_ps(C0, NormalAxis);
    R2 = _mm_sub_ps(V0, R2);

    V0 = _mm_and_ps(R0, g_XMMask3);
    XMVECTOR V1 = _mm_shuffle_ps(R1, R2, _MM_SHUFFLE(2, 1, 2, 0));
    V1 = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 3, 2, 1));
    XMVECTOR V2 = _mm_shuffle_ps(R1, R2, _MM_SHUFFLE(0, 0, 1, 1));
    V2 = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 0, 2, 0));

    R2 = _mm_shuffle_ps(V0, V1, _MM_SHUFFLE(1, 0, 3, 0));
    R2 = XM_PERMUTE_PS(R2, _MM_SHUFFLE(1, 3, 2, 0));

    XMMATRIX M;
    M.r[0] = R2;

    R2 = _mm_shuffle_ps(V0, V1, _MM_SHUFFLE(3, 2, 3, 1));
    R2 = XM_PERMUTE_PS(R2, _MM_SHUFFLE(1, 3, 0, 2));
    M.r[1] = R2;

    V2 = _mm_shuffle_ps(V2, V0, _MM_SHUFFLE(3, 2, 1, 0));
    M.r[2] = V2;
    M.r[3] = g_XMIdentityR3.v;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationAxis
(
    FXMVECTOR Axis,
    float     Angle
) noexcept
{
    assert(!XMVector3Equal(Axis, XMVectorZero()));
    assert(!XMVector3IsInfinite(Axis));

    XMVECTOR Normal = XMVector3Normalize(Axis);
    return XMMatrixRotationNormal(Normal, Angle);
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixRotationQuaternion(FXMVECTOR Quaternion) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    float qx = Quaternion.vector4_f32[0];
    float qxx = qx * qx;

    float qy = Quaternion.vector4_f32[1];
    float qyy = qy * qy;

    float qz = Quaternion.vector4_f32[2];
    float qzz = qz * qz;

    float qw = Quaternion.vector4_f32[3];

    XMMATRIX M;
    M.m[0][0] = 1.f - 2.f * qyy - 2.f * qzz;
    M.m[0][1] = 2.f * qx * qy + 2.f * qz * qw;
    M.m[0][2] = 2.f * qx * qz - 2.f * qy * qw;
    M.m[0][3] = 0.f;

    M.m[1][0] = 2.f * qx * qy - 2.f * qz * qw;
    M.m[1][1] = 1.f - 2.f * qxx - 2.f * qzz;
    M.m[1][2] = 2.f * qy * qz + 2.f * qx * qw;
    M.m[1][3] = 0.f;

    M.m[2][0] = 2.f * qx * qz + 2.f * qy * qw;
    M.m[2][1] = 2.f * qy * qz - 2.f * qx * qw;
    M.m[2][2] = 1.f - 2.f * qxx - 2.f * qyy;
    M.m[2][3] = 0.f;

    M.m[3][0] = 0.f;
    M.m[3][1] = 0.f;
    M.m[3][2] = 0.f;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Constant1110 = { { { 1.0f, 1.0f, 1.0f, 0.0f } } };

    XMVECTOR Q0 = XMVectorAdd(Quaternion, Quaternion);
    XMVECTOR Q1 = XMVectorMultiply(Quaternion, Q0);

    XMVECTOR V0 = XMVectorPermute<XM_PERMUTE_0Y, XM_PERMUTE_0X, XM_PERMUTE_0X, XM_PERMUTE_1W>(Q1, Constant1110.v);
    XMVECTOR V1 = XMVectorPermute<XM_PERMUTE_0Z, XM_PERMUTE_0Z, XM_PERMUTE_0Y, XM_PERMUTE_1W>(Q1, Constant1110.v);
    XMVECTOR R0 = XMVectorSubtract(Constant1110, V0);
    R0 = XMVectorSubtract(R0, V1);

    V0 = XMVectorSwizzle<XM_SWIZZLE_X, XM_SWIZZLE_X, XM_SWIZZLE_Y, XM_SWIZZLE_W>(Quaternion);
    V1 = XMVectorSwizzle<XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_W>(Q0);
    V0 = XMVectorMultiply(V0, V1);

    V1 = XMVectorSplatW(Quaternion);
    XMVECTOR V2 = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_X, XM_SWIZZLE_W>(Q0);
    V1 = XMVectorMultiply(V1, V2);

    XMVECTOR R1 = XMVectorAdd(V0, V1);
    XMVECTOR R2 = XMVectorSubtract(V0, V1);

    V0 = XMVectorPermute<XM_PERMUTE_0Y, XM_PERMUTE_1X, XM_PERMUTE_1Y, XM_PERMUTE_0Z>(R1, R2);
    V1 = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_1Z, XM_PERMUTE_0X, XM_PERMUTE_1Z>(R1, R2);

    XMMATRIX M;
    M.r[0] = XMVectorPermute<XM_PERMUTE_0X, XM_PERMUTE_1X, XM_PERMUTE_1Y, XM_PERMUTE_0W>(R0, V0);
    M.r[1] = XMVectorPermute<XM_PERMUTE_1Z, XM_PERMUTE_0Y, XM_PERMUTE_1W, XM_PERMUTE_0W>(R0, V0);
    M.r[2] = XMVectorPermute<XM_PERMUTE_1X, XM_PERMUTE_1Y, XM_PERMUTE_0Z, XM_PERMUTE_0W>(R0, V1);
    M.r[3] = g_XMIdentityR3.v;
    return M;

#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32  Constant1110 = { { { 1.0f, 1.0f, 1.0f, 0.0f } } };

    XMVECTOR Q0 = _mm_add_ps(Quaternion, Quaternion);
    XMVECTOR Q1 = _mm_mul_ps(Quaternion, Q0);

    XMVECTOR V0 = XM_PERMUTE_PS(Q1, _MM_SHUFFLE(3, 0, 0, 1));
    V0 = _mm_and_ps(V0, g_XMMask3);
    XMVECTOR V1 = XM_PERMUTE_PS(Q1, _MM_SHUFFLE(3, 1, 2, 2));
    V1 = _mm_and_ps(V1, g_XMMask3);
    XMVECTOR R0 = _mm_sub_ps(Constant1110, V0);
    R0 = _mm_sub_ps(R0, V1);

    V0 = XM_PERMUTE_PS(Quaternion, _MM_SHUFFLE(3, 1, 0, 0));
    V1 = XM_PERMUTE_PS(Q0, _MM_SHUFFLE(3, 2, 1, 2));
    V0 = _mm_mul_ps(V0, V1);

    V1 = XM_PERMUTE_PS(Quaternion, _MM_SHUFFLE(3, 3, 3, 3));
    XMVECTOR V2 = XM_PERMUTE_PS(Q0, _MM_SHUFFLE(3, 0, 2, 1));
    V1 = _mm_mul_ps(V1, V2);

    XMVECTOR R1 = _mm_add_ps(V0, V1);
    XMVECTOR R2 = _mm_sub_ps(V0, V1);

    V0 = _mm_shuffle_ps(R1, R2, _MM_SHUFFLE(1, 0, 2, 1));
    V0 = XM_PERMUTE_PS(V0, _MM_SHUFFLE(1, 3, 2, 0));
    V1 = _mm_shuffle_ps(R1, R2, _MM_SHUFFLE(2, 2, 0, 0));
    V1 = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 0, 2, 0));

    Q1 = _mm_shuffle_ps(R0, V0, _MM_SHUFFLE(1, 0, 3, 0));
    Q1 = XM_PERMUTE_PS(Q1, _MM_SHUFFLE(1, 3, 2, 0));

    XMMATRIX M;
    M.r[0] = Q1;

    Q1 = _mm_shuffle_ps(R0, V0, _MM_SHUFFLE(3, 2, 3, 1));
    Q1 = XM_PERMUTE_PS(Q1, _MM_SHUFFLE(1, 3, 0, 2));
    M.r[1] = Q1;

    Q1 = _mm_shuffle_ps(V1, R0, _MM_SHUFFLE(3, 2, 1, 0));
    M.r[2] = Q1;
    M.r[3] = g_XMIdentityR3;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixTransformation2D
(
    FXMVECTOR ScalingOrigin,
    float     ScalingOrientation,
    FXMVECTOR Scaling,
    FXMVECTOR RotationOrigin,
    float     Rotation,
    GXMVECTOR Translation
) noexcept
{
    // M = Inverse(MScalingOrigin) * Transpose(MScalingOrientation) * MScaling * MScalingOrientation *
    //         MScalingOrigin * Inverse(MRotationOrigin) * MRotation * MRotationOrigin * MTranslation;

    XMVECTOR VScalingOrigin = XMVectorSelect(g_XMSelect1100.v, ScalingOrigin, g_XMSelect1100.v);
    XMVECTOR NegScalingOrigin = XMVectorNegate(VScalingOrigin);

    XMMATRIX MScalingOriginI = XMMatrixTranslationFromVector(NegScalingOrigin);
    XMMATRIX MScalingOrientation = XMMatrixRotationZ(ScalingOrientation);
    XMMATRIX MScalingOrientationT = XMMatrixTranspose(MScalingOrientation);
    XMVECTOR VScaling = XMVectorSelect(g_XMOne.v, Scaling, g_XMSelect1100.v);
    XMMATRIX MScaling = XMMatrixScalingFromVector(VScaling);
    XMVECTOR VRotationOrigin = XMVectorSelect(g_XMSelect1100.v, RotationOrigin, g_XMSelect1100.v);
    XMMATRIX MRotation = XMMatrixRotationZ(Rotation);
    XMVECTOR VTranslation = XMVectorSelect(g_XMSelect1100.v, Translation, g_XMSelect1100.v);

    XMMATRIX M = XMMatrixMultiply(MScalingOriginI, MScalingOrientationT);
    M = XMMatrixMultiply(M, MScaling);
    M = XMMatrixMultiply(M, MScalingOrientation);
    M.r[3] = XMVectorAdd(M.r[3], VScalingOrigin);
    M.r[3] = XMVectorSubtract(M.r[3], VRotationOrigin);
    M = XMMatrixMultiply(M, MRotation);
    M.r[3] = XMVectorAdd(M.r[3], VRotationOrigin);
    M.r[3] = XMVectorAdd(M.r[3], VTranslation);

    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixTransformation
(
    FXMVECTOR ScalingOrigin,
    FXMVECTOR ScalingOrientationQuaternion,
    FXMVECTOR Scaling,
    GXMVECTOR RotationOrigin,
    HXMVECTOR RotationQuaternion,
    HXMVECTOR Translation
) noexcept
{
    // M = Inverse(MScalingOrigin) * Transpose(MScalingOrientation) * MScaling * MScalingOrientation *
    //         MScalingOrigin * Inverse(MRotationOrigin) * MRotation * MRotationOrigin * MTranslation;

    XMVECTOR VScalingOrigin = XMVectorSelect(g_XMSelect1110.v, ScalingOrigin, g_XMSelect1110.v);
    XMVECTOR NegScalingOrigin = XMVectorNegate(ScalingOrigin);

    XMMATRIX MScalingOriginI = XMMatrixTranslationFromVector(NegScalingOrigin);
    XMMATRIX MScalingOrientation = XMMatrixRotationQuaternion(ScalingOrientationQuaternion);
    XMMATRIX MScalingOrientationT = XMMatrixTranspose(MScalingOrientation);
    XMMATRIX MScaling = XMMatrixScalingFromVector(Scaling);
    XMVECTOR VRotationOrigin = XMVectorSelect(g_XMSelect1110.v, RotationOrigin, g_XMSelect1110.v);
    XMMATRIX MRotation = XMMatrixRotationQuaternion(RotationQuaternion);
    XMVECTOR VTranslation = XMVectorSelect(g_XMSelect1110.v, Translation, g_XMSelect1110.v);

    XMMATRIX M;
    M = XMMatrixMultiply(MScalingOriginI, MScalingOrientationT);
    M = XMMatrixMultiply(M, MScaling);
    M = XMMatrixMultiply(M, MScalingOrientation);
    M.r[3] = XMVectorAdd(M.r[3], VScalingOrigin);
    M.r[3] = XMVectorSubtract(M.r[3], VRotationOrigin);
    M = XMMatrixMultiply(M, MRotation);
    M.r[3] = XMVectorAdd(M.r[3], VRotationOrigin);
    M.r[3] = XMVectorAdd(M.r[3], VTranslation);
    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixAffineTransformation2D
(
    FXMVECTOR Scaling,
    FXMVECTOR RotationOrigin,
    float     Rotation,
    FXMVECTOR Translation
) noexcept
{
    // M = MScaling * Inverse(MRotationOrigin) * MRotation * MRotationOrigin * MTranslation;

    XMVECTOR VScaling = XMVectorSelect(g_XMOne.v, Scaling, g_XMSelect1100.v);
    XMMATRIX MScaling = XMMatrixScalingFromVector(VScaling);
    XMVECTOR VRotationOrigin = XMVectorSelect(g_XMSelect1100.v, RotationOrigin, g_XMSelect1100.v);
    XMMATRIX MRotation = XMMatrixRotationZ(Rotation);
    XMVECTOR VTranslation = XMVectorSelect(g_XMSelect1100.v, Translation, g_XMSelect1100.v);

    XMMATRIX M;
    M = MScaling;
    M.r[3] = XMVectorSubtract(M.r[3], VRotationOrigin);
    M = XMMatrixMultiply(M, MRotation);
    M.r[3] = XMVectorAdd(M.r[3], VRotationOrigin);
    M.r[3] = XMVectorAdd(M.r[3], VTranslation);
    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixAffineTransformation
(
    FXMVECTOR Scaling,
    FXMVECTOR RotationOrigin,
    FXMVECTOR RotationQuaternion,
    GXMVECTOR Translation
) noexcept
{
    // M = MScaling * Inverse(MRotationOrigin) * MRotation * MRotationOrigin * MTranslation;

    XMMATRIX MScaling = XMMatrixScalingFromVector(Scaling);
    XMVECTOR VRotationOrigin = XMVectorSelect(g_XMSelect1110.v, RotationOrigin, g_XMSelect1110.v);
    XMMATRIX MRotation = XMMatrixRotationQuaternion(RotationQuaternion);
    XMVECTOR VTranslation = XMVectorSelect(g_XMSelect1110.v, Translation, g_XMSelect1110.v);

    XMMATRIX M;
    M = MScaling;
    M.r[3] = XMVectorSubtract(M.r[3], VRotationOrigin);
    M = XMMatrixMultiply(M, MRotation);
    M.r[3] = XMVectorAdd(M.r[3], VRotationOrigin);
    M.r[3] = XMVectorAdd(M.r[3], VTranslation);
    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixReflect(FXMVECTOR ReflectionPlane) noexcept
{
    assert(!XMVector3Equal(ReflectionPlane, XMVectorZero()));
    assert(!XMPlaneIsInfinite(ReflectionPlane));

    static const XMVECTORF32 NegativeTwo = { { { -2.0f, -2.0f, -2.0f, 0.0f } } };

    XMVECTOR P = XMPlaneNormalize(ReflectionPlane);
    XMVECTOR S = XMVectorMultiply(P, NegativeTwo);

    XMVECTOR A = XMVectorSplatX(P);
    XMVECTOR B = XMVectorSplatY(P);
    XMVECTOR C = XMVectorSplatZ(P);
    XMVECTOR D = XMVectorSplatW(P);

    XMMATRIX M;
    M.r[0] = XMVectorMultiplyAdd(A, S, g_XMIdentityR0.v);
    M.r[1] = XMVectorMultiplyAdd(B, S, g_XMIdentityR1.v);
    M.r[2] = XMVectorMultiplyAdd(C, S, g_XMIdentityR2.v);
    M.r[3] = XMVectorMultiplyAdd(D, S, g_XMIdentityR3.v);
    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixShadow
(
    FXMVECTOR ShadowPlane,
    FXMVECTOR LightPosition
) noexcept
{
    static const XMVECTORU32 Select0001 = { { { XM_SELECT_0, XM_SELECT_0, XM_SELECT_0, XM_SELECT_1 } } };

    assert(!XMVector3Equal(ShadowPlane, XMVectorZero()));
    assert(!XMPlaneIsInfinite(ShadowPlane));

    XMVECTOR P = XMPlaneNormalize(ShadowPlane);
    XMVECTOR Dot = XMPlaneDot(P, LightPosition);
    P = XMVectorNegate(P);
    XMVECTOR D = XMVectorSplatW(P);
    XMVECTOR C = XMVectorSplatZ(P);
    XMVECTOR B = XMVectorSplatY(P);
    XMVECTOR A = XMVectorSplatX(P);
    Dot = XMVectorSelect(Select0001.v, Dot, Select0001.v);

    XMMATRIX M;
    M.r[3] = XMVectorMultiplyAdd(D, LightPosition, Dot);
    Dot = XMVectorRotateLeft(Dot, 1);
    M.r[2] = XMVectorMultiplyAdd(C, LightPosition, Dot);
    Dot = XMVectorRotateLeft(Dot, 1);
    M.r[1] = XMVectorMultiplyAdd(B, LightPosition, Dot);
    Dot = XMVectorRotateLeft(Dot, 1);
    M.r[0] = XMVectorMultiplyAdd(A, LightPosition, Dot);
    return M;
}

//------------------------------------------------------------------------------
// View and projection initialization operations
//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixLookAtLH
(
    FXMVECTOR EyePosition,
    FXMVECTOR FocusPosition,
    FXMVECTOR UpDirection
) noexcept
{
    XMVECTOR EyeDirection = XMVectorSubtract(FocusPosition, EyePosition);
    return XMMatrixLookToLH(EyePosition, EyeDirection, UpDirection);
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixLookAtRH
(
    FXMVECTOR EyePosition,
    FXMVECTOR FocusPosition,
    FXMVECTOR UpDirection
) noexcept
{
    XMVECTOR NegEyeDirection = XMVectorSubtract(EyePosition, FocusPosition);
    return XMMatrixLookToLH(EyePosition, NegEyeDirection, UpDirection);
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixLookToLH
(
    FXMVECTOR EyePosition,
    FXMVECTOR EyeDirection,
    FXMVECTOR UpDirection
) noexcept
{
    assert(!XMVector3Equal(EyeDirection, XMVectorZero()));
    assert(!XMVector3IsInfinite(EyeDirection));
    assert(!XMVector3Equal(UpDirection, XMVectorZero()));
    assert(!XMVector3IsInfinite(UpDirection));

    XMVECTOR R2 = XMVector3Normalize(EyeDirection);

    XMVECTOR R0 = XMVector3Cross(UpDirection, R2);
    R0 = XMVector3Normalize(R0);

    XMVECTOR R1 = XMVector3Cross(R2, R0);

    XMVECTOR NegEyePosition = XMVectorNegate(EyePosition);

    XMVECTOR D0 = XMVector3Dot(R0, NegEyePosition);
    XMVECTOR D1 = XMVector3Dot(R1, NegEyePosition);
    XMVECTOR D2 = XMVector3Dot(R2, NegEyePosition);

    XMMATRIX M;
    M.r[0] = XMVectorSelect(D0, R0, g_XMSelect1110.v);
    M.r[1] = XMVectorSelect(D1, R1, g_XMSelect1110.v);
    M.r[2] = XMVectorSelect(D2, R2, g_XMSelect1110.v);
    M.r[3] = g_XMIdentityR3.v;

    M = XMMatrixTranspose(M);

    return M;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixLookToRH
(
    FXMVECTOR EyePosition,
    FXMVECTOR EyeDirection,
    FXMVECTOR UpDirection
) noexcept
{
    XMVECTOR NegEyeDirection = XMVectorNegate(EyeDirection);
    return XMMatrixLookToLH(EyePosition, NegEyeDirection, UpDirection);
}

//------------------------------------------------------------------------------

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable:28931, "PREfast noise: Esp:1266")
#endif

inline XMMATRIX XM_CALLCONV XMMatrixPerspectiveLH
(
    float ViewWidth,
    float ViewHeight,
    float NearZ,
    float FarZ
) noexcept
{
    assert(NearZ > 0.f && FarZ > 0.f);
    assert(!XMScalarNearEqual(ViewWidth, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(ViewHeight, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float TwoNearZ = NearZ + NearZ;
    float fRange = FarZ / (FarZ - NearZ);

    XMMATRIX M;
    M.m[0][0] = TwoNearZ / ViewWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = TwoNearZ / ViewHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = 1.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = -fRange * NearZ;
    M.m[3][3] = 0.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float TwoNearZ = NearZ + NearZ;
    float fRange = FarZ / (FarZ - NearZ);
    const float32x4_t Zero = vdupq_n_f32(0);
    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(TwoNearZ / ViewWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(TwoNearZ / ViewHeight, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, g_XMIdentityR3.v, 2);
    M.r[3] = vsetq_lane_f32(-fRange * NearZ, Zero, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float TwoNearZ = NearZ + NearZ;
    float fRange = FarZ / (FarZ - NearZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        TwoNearZ / ViewWidth,
        TwoNearZ / ViewHeight,
        fRange,
        -fRange * NearZ
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // TwoNearZ / ViewWidth,0,0,0
    M.r[0] = vTemp;
    // 0,TwoNearZ / ViewHeight,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // x=fRange,y=-fRange * NearZ,0,1.0f
    vValues = _mm_shuffle_ps(vValues, g_XMIdentityR3, _MM_SHUFFLE(3, 2, 3, 2));
    // 0,0,fRange,1.0f
    vTemp = _mm_setzero_ps();
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(3, 0, 0, 0));
    M.r[2] = vTemp;
    // 0,0,-fRange * NearZ,0
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(2, 1, 0, 0));
    M.r[3] = vTemp;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixPerspectiveRH
(
    float ViewWidth,
    float ViewHeight,
    float NearZ,
    float FarZ
) noexcept
{
    assert(NearZ > 0.f && FarZ > 0.f);
    assert(!XMScalarNearEqual(ViewWidth, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(ViewHeight, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float TwoNearZ = NearZ + NearZ;
    float fRange = FarZ / (NearZ - FarZ);

    XMMATRIX M;
    M.m[0][0] = TwoNearZ / ViewWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = TwoNearZ / ViewHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = -1.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = fRange * NearZ;
    M.m[3][3] = 0.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float TwoNearZ = NearZ + NearZ;
    float fRange = FarZ / (NearZ - FarZ);
    const float32x4_t Zero = vdupq_n_f32(0);

    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(TwoNearZ / ViewWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(TwoNearZ / ViewHeight, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, g_XMNegIdentityR3.v, 2);
    M.r[3] = vsetq_lane_f32(fRange * NearZ, Zero, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float TwoNearZ = NearZ + NearZ;
    float fRange = FarZ / (NearZ - FarZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        TwoNearZ / ViewWidth,
        TwoNearZ / ViewHeight,
        fRange,
        fRange * NearZ
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // TwoNearZ / ViewWidth,0,0,0
    M.r[0] = vTemp;
    // 0,TwoNearZ / ViewHeight,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // x=fRange,y=-fRange * NearZ,0,-1.0f
    vValues = _mm_shuffle_ps(vValues, g_XMNegIdentityR3, _MM_SHUFFLE(3, 2, 3, 2));
    // 0,0,fRange,-1.0f
    vTemp = _mm_setzero_ps();
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(3, 0, 0, 0));
    M.r[2] = vTemp;
    // 0,0,-fRange * NearZ,0
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(2, 1, 0, 0));
    M.r[3] = vTemp;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixPerspectiveFovLH
(
    float FovAngleY,
    float AspectRatio,
    float NearZ,
    float FarZ
) noexcept
{
    assert(NearZ > 0.f && FarZ > 0.f);
    assert(!XMScalarNearEqual(FovAngleY, 0.0f, 0.00001f * 2.0f));
    assert(!XMScalarNearEqual(AspectRatio, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float    SinFov;
    float    CosFov;
    XMScalarSinCos(&SinFov, &CosFov, 0.5f * FovAngleY);

    float Height = CosFov / SinFov;
    float Width = Height / AspectRatio;
    float fRange = FarZ / (FarZ - NearZ);

    XMMATRIX M;
    M.m[0][0] = Width;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = Height;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = 1.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = -fRange * NearZ;
    M.m[3][3] = 0.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float    SinFov;
    float    CosFov;
    XMScalarSinCos(&SinFov, &CosFov, 0.5f * FovAngleY);

    float fRange = FarZ / (FarZ - NearZ);
    float Height = CosFov / SinFov;
    float Width = Height / AspectRatio;
    const float32x4_t Zero = vdupq_n_f32(0);

    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(Width, Zero, 0);
    M.r[1] = vsetq_lane_f32(Height, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, g_XMIdentityR3.v, 2);
    M.r[3] = vsetq_lane_f32(-fRange * NearZ, Zero, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    float    SinFov;
    float    CosFov;
    XMScalarSinCos(&SinFov, &CosFov, 0.5f * FovAngleY);

    float fRange = FarZ / (FarZ - NearZ);
    // Note: This is recorded on the stack
    float Height = CosFov / SinFov;
    XMVECTOR rMem = {
        Height / AspectRatio,
        Height,
        fRange,
        -fRange * NearZ
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // Height / AspectRatio,0,0,0
    XMMATRIX M;
    M.r[0] = vTemp;
    // 0,Height,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // x=fRange,y=-fRange * NearZ,0,1.0f
    vTemp = _mm_setzero_ps();
    vValues = _mm_shuffle_ps(vValues, g_XMIdentityR3, _MM_SHUFFLE(3, 2, 3, 2));
    // 0,0,fRange,1.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(3, 0, 0, 0));
    M.r[2] = vTemp;
    // 0,0,-fRange * NearZ,0.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(2, 1, 0, 0));
    M.r[3] = vTemp;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixPerspectiveFovRH
(
    float FovAngleY,
    float AspectRatio,
    float NearZ,
    float FarZ
) noexcept
{
    assert(NearZ > 0.f && FarZ > 0.f);
    assert(!XMScalarNearEqual(FovAngleY, 0.0f, 0.00001f * 2.0f));
    assert(!XMScalarNearEqual(AspectRatio, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float    SinFov;
    float    CosFov;
    XMScalarSinCos(&SinFov, &CosFov, 0.5f * FovAngleY);

    float Height = CosFov / SinFov;
    float Width = Height / AspectRatio;
    float fRange = FarZ / (NearZ - FarZ);

    XMMATRIX M;
    M.m[0][0] = Width;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = Height;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = -1.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = fRange * NearZ;
    M.m[3][3] = 0.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float    SinFov;
    float    CosFov;
    XMScalarSinCos(&SinFov, &CosFov, 0.5f * FovAngleY);
    float fRange = FarZ / (NearZ - FarZ);
    float Height = CosFov / SinFov;
    float Width = Height / AspectRatio;
    const float32x4_t Zero = vdupq_n_f32(0);

    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(Width, Zero, 0);
    M.r[1] = vsetq_lane_f32(Height, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, g_XMNegIdentityR3.v, 2);
    M.r[3] = vsetq_lane_f32(fRange * NearZ, Zero, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    float    SinFov;
    float    CosFov;
    XMScalarSinCos(&SinFov, &CosFov, 0.5f * FovAngleY);
    float fRange = FarZ / (NearZ - FarZ);
    // Note: This is recorded on the stack
    float Height = CosFov / SinFov;
    XMVECTOR rMem = {
        Height / AspectRatio,
        Height,
        fRange,
        fRange * NearZ
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // Height / AspectRatio,0,0,0
    XMMATRIX M;
    M.r[0] = vTemp;
    // 0,Height,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // x=fRange,y=-fRange * NearZ,0,-1.0f
    vTemp = _mm_setzero_ps();
    vValues = _mm_shuffle_ps(vValues, g_XMNegIdentityR3, _MM_SHUFFLE(3, 2, 3, 2));
    // 0,0,fRange,-1.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(3, 0, 0, 0));
    M.r[2] = vTemp;
    // 0,0,fRange * NearZ,0.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(2, 1, 0, 0));
    M.r[3] = vTemp;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixPerspectiveOffCenterLH
(
    float ViewLeft,
    float ViewRight,
    float ViewBottom,
    float ViewTop,
    float NearZ,
    float FarZ
) noexcept
{
    assert(NearZ > 0.f && FarZ > 0.f);
    assert(!XMScalarNearEqual(ViewRight, ViewLeft, 0.00001f));
    assert(!XMScalarNearEqual(ViewTop, ViewBottom, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float TwoNearZ = NearZ + NearZ;
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = FarZ / (FarZ - NearZ);

    XMMATRIX M;
    M.m[0][0] = TwoNearZ * ReciprocalWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = TwoNearZ * ReciprocalHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = -(ViewLeft + ViewRight) * ReciprocalWidth;
    M.m[2][1] = -(ViewTop + ViewBottom) * ReciprocalHeight;
    M.m[2][2] = fRange;
    M.m[2][3] = 1.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = -fRange * NearZ;
    M.m[3][3] = 0.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float TwoNearZ = NearZ + NearZ;
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = FarZ / (FarZ - NearZ);
    const float32x4_t Zero = vdupq_n_f32(0);

    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(TwoNearZ * ReciprocalWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(TwoNearZ * ReciprocalHeight, Zero, 1);
    M.r[2] = XMVectorSet(-(ViewLeft + ViewRight) * ReciprocalWidth,
        -(ViewTop + ViewBottom) * ReciprocalHeight,
        fRange,
        1.0f);
    M.r[3] = vsetq_lane_f32(-fRange * NearZ, Zero, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float TwoNearZ = NearZ + NearZ;
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = FarZ / (FarZ - NearZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        TwoNearZ * ReciprocalWidth,
        TwoNearZ * ReciprocalHeight,
        -fRange * NearZ,
        0
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // TwoNearZ*ReciprocalWidth,0,0,0
    M.r[0] = vTemp;
    // 0,TwoNearZ*ReciprocalHeight,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // 0,0,fRange,1.0f
    M.r[2] = XMVectorSet(-(ViewLeft + ViewRight) * ReciprocalWidth,
        -(ViewTop + ViewBottom) * ReciprocalHeight,
        fRange,
        1.0f);
    // 0,0,-fRange * NearZ,0.0f
    vValues = _mm_and_ps(vValues, g_XMMaskZ);
    M.r[3] = vValues;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixPerspectiveOffCenterRH
(
    float ViewLeft,
    float ViewRight,
    float ViewBottom,
    float ViewTop,
    float NearZ,
    float FarZ
) noexcept
{
    assert(NearZ > 0.f && FarZ > 0.f);
    assert(!XMScalarNearEqual(ViewRight, ViewLeft, 0.00001f));
    assert(!XMScalarNearEqual(ViewTop, ViewBottom, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float TwoNearZ = NearZ + NearZ;
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = FarZ / (NearZ - FarZ);

    XMMATRIX M;
    M.m[0][0] = TwoNearZ * ReciprocalWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = TwoNearZ * ReciprocalHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = (ViewLeft + ViewRight) * ReciprocalWidth;
    M.m[2][1] = (ViewTop + ViewBottom) * ReciprocalHeight;
    M.m[2][2] = fRange;
    M.m[2][3] = -1.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = fRange * NearZ;
    M.m[3][3] = 0.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float TwoNearZ = NearZ + NearZ;
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = FarZ / (NearZ - FarZ);
    const float32x4_t Zero = vdupq_n_f32(0);

    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(TwoNearZ * ReciprocalWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(TwoNearZ * ReciprocalHeight, Zero, 1);
    M.r[2] = XMVectorSet((ViewLeft + ViewRight) * ReciprocalWidth,
        (ViewTop + ViewBottom) * ReciprocalHeight,
        fRange,
        -1.0f);
    M.r[3] = vsetq_lane_f32(fRange * NearZ, Zero, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float TwoNearZ = NearZ + NearZ;
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = FarZ / (NearZ - FarZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        TwoNearZ * ReciprocalWidth,
        TwoNearZ * ReciprocalHeight,
        fRange * NearZ,
        0
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // TwoNearZ*ReciprocalWidth,0,0,0
    M.r[0] = vTemp;
    // 0,TwoNearZ*ReciprocalHeight,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // 0,0,fRange,1.0f
    M.r[2] = XMVectorSet((ViewLeft + ViewRight) * ReciprocalWidth,
        (ViewTop + ViewBottom) * ReciprocalHeight,
        fRange,
        -1.0f);
    // 0,0,-fRange * NearZ,0.0f
    vValues = _mm_and_ps(vValues, g_XMMaskZ);
    M.r[3] = vValues;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixOrthographicLH
(
    float ViewWidth,
    float ViewHeight,
    float NearZ,
    float FarZ
) noexcept
{
    assert(!XMScalarNearEqual(ViewWidth, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(ViewHeight, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float fRange = 1.0f / (FarZ - NearZ);

    XMMATRIX M;
    M.m[0][0] = 2.0f / ViewWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = 2.0f / ViewHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = -fRange * NearZ;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float fRange = 1.0f / (FarZ - NearZ);

    const float32x4_t Zero = vdupq_n_f32(0);
    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(2.0f / ViewWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(2.0f / ViewHeight, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, Zero, 2);
    M.r[3] = vsetq_lane_f32(-fRange * NearZ, g_XMIdentityR3.v, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float fRange = 1.0f / (FarZ - NearZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        2.0f / ViewWidth,
        2.0f / ViewHeight,
        fRange,
        -fRange * NearZ
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // 2.0f / ViewWidth,0,0,0
    M.r[0] = vTemp;
    // 0,2.0f / ViewHeight,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // x=fRange,y=-fRange * NearZ,0,1.0f
    vTemp = _mm_setzero_ps();
    vValues = _mm_shuffle_ps(vValues, g_XMIdentityR3, _MM_SHUFFLE(3, 2, 3, 2));
    // 0,0,fRange,0.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(2, 0, 0, 0));
    M.r[2] = vTemp;
    // 0,0,-fRange * NearZ,1.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(3, 1, 0, 0));
    M.r[3] = vTemp;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixOrthographicRH
(
    float ViewWidth,
    float ViewHeight,
    float NearZ,
    float FarZ
) noexcept
{
    assert(!XMScalarNearEqual(ViewWidth, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(ViewHeight, 0.0f, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float fRange = 1.0f / (NearZ - FarZ);

    XMMATRIX M;
    M.m[0][0] = 2.0f / ViewWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = 2.0f / ViewHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = 0.0f;

    M.m[3][0] = 0.0f;
    M.m[3][1] = 0.0f;
    M.m[3][2] = fRange * NearZ;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float fRange = 1.0f / (NearZ - FarZ);

    const float32x4_t Zero = vdupq_n_f32(0);
    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(2.0f / ViewWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(2.0f / ViewHeight, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, Zero, 2);
    M.r[3] = vsetq_lane_f32(fRange * NearZ, g_XMIdentityR3.v, 2);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float fRange = 1.0f / (NearZ - FarZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        2.0f / ViewWidth,
        2.0f / ViewHeight,
        fRange,
        fRange * NearZ
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // 2.0f / ViewWidth,0,0,0
    M.r[0] = vTemp;
    // 0,2.0f / ViewHeight,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    M.r[1] = vTemp;
    // x=fRange,y=fRange * NearZ,0,1.0f
    vTemp = _mm_setzero_ps();
    vValues = _mm_shuffle_ps(vValues, g_XMIdentityR3, _MM_SHUFFLE(3, 2, 3, 2));
    // 0,0,fRange,0.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(2, 0, 0, 0));
    M.r[2] = vTemp;
    // 0,0,fRange * NearZ,1.0f
    vTemp = _mm_shuffle_ps(vTemp, vValues, _MM_SHUFFLE(3, 1, 0, 0));
    M.r[3] = vTemp;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixOrthographicOffCenterLH
(
    float ViewLeft,
    float ViewRight,
    float ViewBottom,
    float ViewTop,
    float NearZ,
    float FarZ
) noexcept
{
    assert(!XMScalarNearEqual(ViewRight, ViewLeft, 0.00001f));
    assert(!XMScalarNearEqual(ViewTop, ViewBottom, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = 1.0f / (FarZ - NearZ);

    XMMATRIX M;
    M.m[0][0] = ReciprocalWidth + ReciprocalWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = ReciprocalHeight + ReciprocalHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = 0.0f;

    M.m[3][0] = -(ViewLeft + ViewRight) * ReciprocalWidth;
    M.m[3][1] = -(ViewTop + ViewBottom) * ReciprocalHeight;
    M.m[3][2] = -fRange * NearZ;
    M.m[3][3] = 1.0f;
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = 1.0f / (FarZ - NearZ);
    const float32x4_t Zero = vdupq_n_f32(0);
    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(ReciprocalWidth + ReciprocalWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(ReciprocalHeight + ReciprocalHeight, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, Zero, 2);
    M.r[3] = XMVectorSet(-(ViewLeft + ViewRight) * ReciprocalWidth,
        -(ViewTop + ViewBottom) * ReciprocalHeight,
        -fRange * NearZ,
        1.0f);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float fReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float fReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = 1.0f / (FarZ - NearZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        fReciprocalWidth,
        fReciprocalHeight,
        fRange,
        1.0f
    };
    XMVECTOR rMem2 = {
        -(ViewLeft + ViewRight),
        -(ViewTop + ViewBottom),
        -NearZ,
        1.0f
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // fReciprocalWidth*2,0,0,0
    vTemp = _mm_add_ss(vTemp, vTemp);
    M.r[0] = vTemp;
    // 0,fReciprocalHeight*2,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    vTemp = _mm_add_ps(vTemp, vTemp);
    M.r[1] = vTemp;
    // 0,0,fRange,0.0f
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskZ);
    M.r[2] = vTemp;
    // -(ViewLeft + ViewRight)*fReciprocalWidth,-(ViewTop + ViewBottom)*fReciprocalHeight,fRange*-NearZ,1.0f
    vValues = _mm_mul_ps(vValues, rMem2);
    M.r[3] = vValues;
    return M;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMatrixOrthographicOffCenterRH
(
    float ViewLeft,
    float ViewRight,
    float ViewBottom,
    float ViewTop,
    float NearZ,
    float FarZ
) noexcept
{
    assert(!XMScalarNearEqual(ViewRight, ViewLeft, 0.00001f));
    assert(!XMScalarNearEqual(ViewTop, ViewBottom, 0.00001f));
    assert(!XMScalarNearEqual(FarZ, NearZ, 0.00001f));

#if defined(_XM_NO_INTRINSICS_)

    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = 1.0f / (NearZ - FarZ);

    XMMATRIX M;
    M.m[0][0] = ReciprocalWidth + ReciprocalWidth;
    M.m[0][1] = 0.0f;
    M.m[0][2] = 0.0f;
    M.m[0][3] = 0.0f;

    M.m[1][0] = 0.0f;
    M.m[1][1] = ReciprocalHeight + ReciprocalHeight;
    M.m[1][2] = 0.0f;
    M.m[1][3] = 0.0f;

    M.m[2][0] = 0.0f;
    M.m[2][1] = 0.0f;
    M.m[2][2] = fRange;
    M.m[2][3] = 0.0f;

    M.r[3] = XMVectorSet(-(ViewLeft + ViewRight) * ReciprocalWidth,
        -(ViewTop + ViewBottom) * ReciprocalHeight,
        fRange * NearZ,
        1.0f);
    return M;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float ReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float ReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = 1.0f / (NearZ - FarZ);
    const float32x4_t Zero = vdupq_n_f32(0);
    XMMATRIX M;
    M.r[0] = vsetq_lane_f32(ReciprocalWidth + ReciprocalWidth, Zero, 0);
    M.r[1] = vsetq_lane_f32(ReciprocalHeight + ReciprocalHeight, Zero, 1);
    M.r[2] = vsetq_lane_f32(fRange, Zero, 2);
    M.r[3] = XMVectorSet(-(ViewLeft + ViewRight) * ReciprocalWidth,
        -(ViewTop + ViewBottom) * ReciprocalHeight,
        fRange * NearZ,
        1.0f);
    return M;
#elif defined(_XM_SSE_INTRINSICS_)
    XMMATRIX M;
    float fReciprocalWidth = 1.0f / (ViewRight - ViewLeft);
    float fReciprocalHeight = 1.0f / (ViewTop - ViewBottom);
    float fRange = 1.0f / (NearZ - FarZ);
    // Note: This is recorded on the stack
    XMVECTOR rMem = {
        fReciprocalWidth,
        fReciprocalHeight,
        fRange,
        1.0f
    };
    XMVECTOR rMem2 = {
        -(ViewLeft + ViewRight),
        -(ViewTop + ViewBottom),
        NearZ,
        1.0f
    };
    // Copy from memory to SSE register
    XMVECTOR vValues = rMem;
    XMVECTOR vTemp = _mm_setzero_ps();
    // Copy x only
    vTemp = _mm_move_ss(vTemp, vValues);
    // fReciprocalWidth*2,0,0,0
    vTemp = _mm_add_ss(vTemp, vTemp);
    M.r[0] = vTemp;
    // 0,fReciprocalHeight*2,0,0
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskY);
    vTemp = _mm_add_ps(vTemp, vTemp);
    M.r[1] = vTemp;
    // 0,0,fRange,0.0f
    vTemp = vValues;
    vTemp = _mm_and_ps(vTemp, g_XMMaskZ);
    M.r[2] = vTemp;
    // -(ViewLeft + ViewRight)*fReciprocalWidth,-(ViewTop + ViewBottom)*fReciprocalHeight,fRange*-NearZ,1.0f
    vValues = _mm_mul_ps(vValues, rMem2);
    M.r[3] = vValues;
    return M;
#endif
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

/****************************************************************************
 *
 * XMMATRIX operators and methods
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMMATRIX::XMMATRIX
(
    float m00, float m01, float m02, float m03,
    float m10, float m11, float m12, float m13,
    float m20, float m21, float m22, float m23,
    float m30, float m31, float m32, float m33
) noexcept
{
    r[0] = XMVectorSet(m00, m01, m02, m03);
    r[1] = XMVectorSet(m10, m11, m12, m13);
    r[2] = XMVectorSet(m20, m21, m22, m23);
    r[3] = XMVectorSet(m30, m31, m32, m33);
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMMATRIX::XMMATRIX(const float* pArray) noexcept
{
    assert(pArray != nullptr);
    r[0] = XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray));
    r[1] = XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray + 4));
    r[2] = XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray + 8));
    r[3] = XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray + 12));
}

//------------------------------------------------------------------------------

inline XMMATRIX XMMATRIX::operator- () const noexcept
{
    XMMATRIX R;
    R.r[0] = XMVectorNegate(r[0]);
    R.r[1] = XMVectorNegate(r[1]);
    R.r[2] = XMVectorNegate(r[2]);
    R.r[3] = XMVectorNegate(r[3]);
    return R;
}

//------------------------------------------------------------------------------

inline XMMATRIX& XM_CALLCONV XMMATRIX::operator+= (FXMMATRIX M) noexcept
{
    r[0] = XMVectorAdd(r[0], M.r[0]);
    r[1] = XMVectorAdd(r[1], M.r[1]);
    r[2] = XMVectorAdd(r[2], M.r[2]);
    r[3] = XMVectorAdd(r[3], M.r[3]);
    return *this;
}

//------------------------------------------------------------------------------

inline XMMATRIX& XM_CALLCONV XMMATRIX::operator-= (FXMMATRIX M) noexcept
{
    r[0] = XMVectorSubtract(r[0], M.r[0]);
    r[1] = XMVectorSubtract(r[1], M.r[1]);
    r[2] = XMVectorSubtract(r[2], M.r[2]);
    r[3] = XMVectorSubtract(r[3], M.r[3]);
    return *this;
}

//------------------------------------------------------------------------------

inline XMMATRIX& XM_CALLCONV XMMATRIX::operator*=(FXMMATRIX M) noexcept
{
    *this = XMMatrixMultiply(*this, M);
    return *this;
}

//------------------------------------------------------------------------------

inline XMMATRIX& XMMATRIX::operator*= (float S) noexcept
{
    r[0] = XMVectorScale(r[0], S);
    r[1] = XMVectorScale(r[1], S);
    r[2] = XMVectorScale(r[2], S);
    r[3] = XMVectorScale(r[3], S);
    return *this;
}

//------------------------------------------------------------------------------

inline XMMATRIX& XMMATRIX::operator/= (float S) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTOR vS = XMVectorReplicate(S);
    r[0] = XMVectorDivide(r[0], vS);
    r[1] = XMVectorDivide(r[1], vS);
    r[2] = XMVectorDivide(r[2], vS);
    r[3] = XMVectorDivide(r[3], vS);
    return *this;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    float32x4_t vS = vdupq_n_f32(S);
    r[0] = vdivq_f32(r[0], vS);
    r[1] = vdivq_f32(r[1], vS);
    r[2] = vdivq_f32(r[2], vS);
    r[3] = vdivq_f32(r[3], vS);
#else
    // 2 iterations of Newton-Raphson refinement of reciprocal
    float32x2_t vS = vdup_n_f32(S);
    float32x2_t R0 = vrecpe_f32(vS);
    float32x2_t S0 = vrecps_f32(R0, vS);
    R0 = vmul_f32(S0, R0);
    S0 = vrecps_f32(R0, vS);
    R0 = vmul_f32(S0, R0);
    float32x4_t Reciprocal = vcombine_f32(R0, R0);
    r[0] = vmulq_f32(r[0], Reciprocal);
    r[1] = vmulq_f32(r[1], Reciprocal);
    r[2] = vmulq_f32(r[2], Reciprocal);
    r[3] = vmulq_f32(r[3], Reciprocal);
#endif
    return *this;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 vS = _mm_set_ps1(S);
    r[0] = _mm_div_ps(r[0], vS);
    r[1] = _mm_div_ps(r[1], vS);
    r[2] = _mm_div_ps(r[2], vS);
    r[3] = _mm_div_ps(r[3], vS);
    return *this;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMATRIX::operator+ (FXMMATRIX M) const noexcept
{
    XMMATRIX R;
    R.r[0] = XMVectorAdd(r[0], M.r[0]);
    R.r[1] = XMVectorAdd(r[1], M.r[1]);
    R.r[2] = XMVectorAdd(r[2], M.r[2]);
    R.r[3] = XMVectorAdd(r[3], M.r[3]);
    return R;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMATRIX::operator- (FXMMATRIX M) const noexcept
{
    XMMATRIX R;
    R.r[0] = XMVectorSubtract(r[0], M.r[0]);
    R.r[1] = XMVectorSubtract(r[1], M.r[1]);
    R.r[2] = XMVectorSubtract(r[2], M.r[2]);
    R.r[3] = XMVectorSubtract(r[3], M.r[3]);
    return R;
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV XMMATRIX::operator*(FXMMATRIX M) const noexcept
{
    return XMMatrixMultiply(*this, M);
}

//------------------------------------------------------------------------------

inline XMMATRIX XMMATRIX::operator* (float S) const noexcept
{
    XMMATRIX R;
    R.r[0] = XMVectorScale(r[0], S);
    R.r[1] = XMVectorScale(r[1], S);
    R.r[2] = XMVectorScale(r[2], S);
    R.r[3] = XMVectorScale(r[3], S);
    return R;
}

//------------------------------------------------------------------------------

inline XMMATRIX XMMATRIX::operator/ (float S) const noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTOR vS = XMVectorReplicate(S);
    XMMATRIX R;
    R.r[0] = XMVectorDivide(r[0], vS);
    R.r[1] = XMVectorDivide(r[1], vS);
    R.r[2] = XMVectorDivide(r[2], vS);
    R.r[3] = XMVectorDivide(r[3], vS);
    return R;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    float32x4_t vS = vdupq_n_f32(S);
    XMMATRIX R;
    R.r[0] = vdivq_f32(r[0], vS);
    R.r[1] = vdivq_f32(r[1], vS);
    R.r[2] = vdivq_f32(r[2], vS);
    R.r[3] = vdivq_f32(r[3], vS);
#else
    // 2 iterations of Newton-Raphson refinement of reciprocal
    float32x2_t vS = vdup_n_f32(S);
    float32x2_t R0 = vrecpe_f32(vS);
    float32x2_t S0 = vrecps_f32(R0, vS);
    R0 = vmul_f32(S0, R0);
    S0 = vrecps_f32(R0, vS);
    R0 = vmul_f32(S0, R0);
    float32x4_t Reciprocal = vcombine_f32(R0, R0);
    XMMATRIX R;
    R.r[0] = vmulq_f32(r[0], Reciprocal);
    R.r[1] = vmulq_f32(r[1], Reciprocal);
    R.r[2] = vmulq_f32(r[2], Reciprocal);
    R.r[3] = vmulq_f32(r[3], Reciprocal);
#endif
    return R;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 vS = _mm_set_ps1(S);
    XMMATRIX R;
    R.r[0] = _mm_div_ps(r[0], vS);
    R.r[1] = _mm_div_ps(r[1], vS);
    R.r[2] = _mm_div_ps(r[2], vS);
    R.r[3] = _mm_div_ps(r[3], vS);
    return R;
#endif
}

//------------------------------------------------------------------------------

inline XMMATRIX XM_CALLCONV operator*
(
    float S,
    FXMMATRIX M
) noexcept
{
    XMMATRIX R;
    R.r[0] = XMVectorScale(M.r[0], S);
    R.r[1] = XMVectorScale(M.r[1], S);
    R.r[2] = XMVectorScale(M.r[2], S);
    R.r[3] = XMVectorScale(M.r[3], S);
    return R;
}

/****************************************************************************
 *
 * XMFLOAT3X3 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMFLOAT3X3::XMFLOAT3X3(const float* pArray) noexcept
{
    assert(pArray != nullptr);
    for (size_t Row = 0; Row < 3; Row++)
    {
        for (size_t Column = 0; Column < 3; Column++)
        {
            m[Row][Column] = pArray[Row * 3 + Column];
        }
    }
}

/****************************************************************************
 *
 * XMFLOAT4X3 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMFLOAT4X3::XMFLOAT4X3(const float* pArray) noexcept
{
    assert(pArray != nullptr);

    m[0][0] = pArray[0];
    m[0][1] = pArray[1];
    m[0][2] = pArray[2];

    m[1][0] = pArray[3];
    m[1][1] = pArray[4];
    m[1][2] = pArray[5];

    m[2][0] = pArray[6];
    m[2][1] = pArray[7];
    m[2][2] = pArray[8];

    m[3][0] = pArray[9];
    m[3][1] = pArray[10];
    m[3][2] = pArray[11];
}

/****************************************************************************
*
* XMFLOAT3X4 operators
*
****************************************************************************/

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMFLOAT3X4::XMFLOAT3X4(const float* pArray) noexcept
{
    assert(pArray != nullptr);

    m[0][0] = pArray[0];
    m[0][1] = pArray[1];
    m[0][2] = pArray[2];
    m[0][3] = pArray[3];

    m[1][0] = pArray[4];
    m[1][1] = pArray[5];
    m[1][2] = pArray[6];
    m[1][3] = pArray[7];

    m[2][0] = pArray[8];
    m[2][1] = pArray[9];
    m[2][2] = pArray[10];
    m[2][3] = pArray[11];
}

/****************************************************************************
 *
 * XMFLOAT4X4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMFLOAT4X4::XMFLOAT4X4(const float* pArray) noexcept
{
    assert(pArray != nullptr);

    m[0][0] = pArray[0];
    m[0][1] = pArray[1];
    m[0][2] = pArray[2];
    m[0][3] = pArray[3];

    m[1][0] = pArray[4];
    m[1][1] = pArray[5];
    m[1][2] = pArray[6];
    m[1][3] = pArray[7];

    m[2][0] = pArray[8];
    m[2][1] = pArray[9];
    m[2][2] = pArray[10];
    m[2][3] = pArray[11];

    m[3][0] = pArray[12];
    m[3][1] = pArray[13];
    m[3][2] = pArray[14];
    m[3][3] = pArray[15];
}

