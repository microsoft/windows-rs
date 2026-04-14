/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* amp_math.h
*
* C++ AMP Math Library
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#ifndef _SILENCE_AMP_DEPRECATION_WARNINGS
#error <amp_math.h> is part of C++ AMP which is deprecated by Microsoft and will be REMOVED. \
You can define _SILENCE_AMP_DEPRECATION_WARNINGS to acknowledge that you have received this warning.
#endif // _SILENCE_AMP_DEPRECATION_WARNINGS

#include <cmath>
#include <amprt.h>

#define _AMP_MATH_H

extern "C"
{

//=============================================================================
// fast_math intrinsics.
//=============================================================================
float __dp_d3d_absf(float) __GPU_ONLY;
float __dp_d3d_acosf(float) __GPU_ONLY;
float __dp_d3d_asinf(float) __GPU_ONLY;
float __dp_d3d_atanf(float) __GPU_ONLY;
float __dp_d3d_atan2f(float, float) __GPU_ONLY;
float __dp_d3d_ceilf(float) __GPU_ONLY;
float __dp_d3d_cosf(float) __GPU_ONLY;
float __dp_d3d_coshf(float) __GPU_ONLY;
float __dp_d3d_expf(float) __GPU_ONLY;
float __dp_d3d_exp2f(float) __GPU_ONLY;
float __dp_d3d_floorf(float) __GPU_ONLY;
float __dp_d3d_fmodf(float, float) __GPU_ONLY;
float __dp_d3d_fracf(float) __GPU_ONLY;
float __dp_d3d_frexpf(float, _Out_ float *) __GPU_ONLY;
int __dp_d3d_isfinitef(float)__GPU_ONLY;
int __dp_d3d_isinff(float) __GPU_ONLY;
int __dp_d3d_isnanf(float) __GPU_ONLY;
float __dp_d3d_ldexpf(float, float) __GPU_ONLY;
float __dp_d3d_logf(float) __GPU_ONLY;
float __dp_d3d_log10f(float) __GPU_ONLY;
float __dp_d3d_log2f(float) __GPU_ONLY;
float __dp_d3d_maxf(float, float) __GPU_ONLY;
float __dp_d3d_minf(float, float) __GPU_ONLY;
float __dp_d3d_modff(float, _Out_ float *) __GPU_ONLY;
float __dp_d3d_powf(float, float) __GPU_ONLY;
float __dp_d3d_roundf(float) __GPU_ONLY;
float __dp_d3d_rsqrtf(float) __GPU_ONLY;
int __dp_d3d_signf(float) __GPU_ONLY;
float __dp_d3d_sincosf(float, _Out_ float *) __GPU_ONLY;
float __dp_d3d_sinf(float) __GPU_ONLY;
float __dp_d3d_sinhf(float) __GPU_ONLY;
float __dp_d3d_sqrtf(float) __GPU_ONLY;
float __dp_d3d_tanf(float) __GPU_ONLY;
float __dp_d3d_tanhf(float) __GPU_ONLY;
float __dp_d3d_truncf(float) __GPU_ONLY;

//=============================================================================
// Single-precision precise_math intrinsics.
//=============================================================================
float __dp_math_acosf(float) __GPU_ONLY;
float __dp_math_acoshf(float) __GPU_ONLY;
float __dp_math_asinf(float) __GPU_ONLY;
float __dp_math_asinhf(float) __GPU_ONLY;
float __dp_math_atanf(float) __GPU_ONLY;
float __dp_math_atan2f(float, float) __GPU_ONLY;
float __dp_math_atanhf(float) __GPU_ONLY;
float __dp_math_cbrtf(float) __GPU_ONLY;
float __dp_math_ceilf(float) __GPU_ONLY;
float __dp_math_copysignf(float, float) __GPU_ONLY;
float __dp_math_cosf(float) __GPU_ONLY;
float __dp_math_coshf(float) __GPU_ONLY;
float __dp_math_cospif(float) __GPU_ONLY;
float __dp_math_erff(float) __GPU_ONLY;
float __dp_math_erfcf(float) __GPU_ONLY;
float __dp_math_erfinvf(float) __GPU_ONLY;
float __dp_math_erfcinvf(float) __GPU_ONLY;
float __dp_math_expf(float) __GPU_ONLY;
float __dp_math_exp2f(float) __GPU_ONLY;
float __dp_math_exp10f(float) __GPU_ONLY;
float __dp_math_expm1f(float) __GPU_ONLY;
float __dp_math_fabsf(float) __GPU_ONLY;
float __dp_math_fdimf(float, float) __GPU_ONLY;
float __dp_math_floorf(float) __GPU_ONLY;
float __dp_math_fmaf(float, float, float) __GPU_ONLY;
float __dp_math_fmaxf(float, float) __GPU_ONLY;
float __dp_math_fminf(float, float) __GPU_ONLY;
float __dp_math_fmodf(float, float) __GPU_ONLY;
int   __dp_math_fpclassifyf(float) __GPU_ONLY;
float __dp_math_frexpf(float, _Out_ int *) __GPU_ONLY;
float __dp_math_hypotf(float, float) __GPU_ONLY;
int   __dp_math_ilogbf(float) __GPU_ONLY;
int   __dp_math_isfinitef(float) __GPU_ONLY;
int   __dp_math_isinff(float) __GPU_ONLY;
int   __dp_math_isnanf(float) __GPU_ONLY;
int   __dp_math_isnormalf(float) __GPU_ONLY;
float __dp_math_ldexpf(float, int) __GPU_ONLY;
float __dp_math_lgammaf(float, _Out_ int *) __GPU_ONLY;
float __dp_math_logf(float) __GPU_ONLY;
float __dp_math_log10f(float) __GPU_ONLY;
float __dp_math_log2f(float) __GPU_ONLY;
float __dp_math_log1pf(float) __GPU_ONLY;
float __dp_math_logbf(float) __GPU_ONLY;
float __dp_math_modff(float, _Out_ float *) __GPU_ONLY;
float __dp_math_nanf(int) __GPU_ONLY;
float __dp_math_nearbyintf(float) __GPU_ONLY;
float __dp_math_nextafterf(float, float) __GPU_ONLY;
float __dp_math_phif(float) __GPU_ONLY;
float __dp_math_powf(float, float) __GPU_ONLY;
float __dp_math_probitf(float) __GPU_ONLY;
float __dp_math_rcbrtf(float) __GPU_ONLY;
float __dp_math_remainderf(float, float) __GPU_ONLY;
float __dp_math_remquof(float, float, _Out_ int *) __GPU_ONLY;
float __dp_math_roundf(float) __GPU_ONLY;
float __dp_math_rsqrtf(float) __GPU_ONLY;
float __dp_math_scalbf(float, float) __GPU_ONLY;
float __dp_math_scalbnf(float, int) __GPU_ONLY;
int   __dp_math_signbitf(float) __GPU_ONLY;
float __dp_math_sinf(float) __GPU_ONLY;
float __dp_math_sincosf(float, _Out_ float *) __GPU_ONLY;
float __dp_math_sinpif(float) __GPU_ONLY;
float __dp_math_sinhf(float) __GPU_ONLY;
float __dp_math_sqrtf(float) __GPU_ONLY;
float __dp_math_tanf(float) __GPU_ONLY;
float __dp_math_tanhf(float) __GPU_ONLY;
float __dp_math_tanpif(float) __GPU_ONLY;
float __dp_math_tgammaf(float) __GPU_ONLY;
float __dp_math_truncf(float) __GPU_ONLY;

//=============================================================================
// Double-precision precise_math intrinsics.
//=============================================================================
double __dp_math_acos(double) __GPU_ONLY;
double __dp_math_acosh(double) __GPU_ONLY;
double __dp_math_asin(double) __GPU_ONLY;
double __dp_math_asinh(double) __GPU_ONLY;
double __dp_math_atan(double) __GPU_ONLY;
double __dp_math_atan2(double, double) __GPU_ONLY;
double __dp_math_atanh(double) __GPU_ONLY;
double __dp_math_cbrt(double) __GPU_ONLY;
double __dp_math_ceil(double) __GPU_ONLY;
double __dp_math_copysign(double, double) __GPU_ONLY;
double __dp_math_cos(double) __GPU_ONLY;
double __dp_math_cosh(double) __GPU_ONLY;
double __dp_math_cospi(double) __GPU_ONLY;
double __dp_math_erf(double) __GPU_ONLY;
double __dp_math_erfc(double) __GPU_ONLY;
double __dp_math_erfinv(double) __GPU_ONLY;
double __dp_math_erfcinv(double) __GPU_ONLY;
double __dp_math_exp(double) __GPU_ONLY;
double __dp_math_exp2(double) __GPU_ONLY;
double __dp_math_exp10(double) __GPU_ONLY;
double __dp_math_expm1(double) __GPU_ONLY;
double __dp_math_fabs(double) __GPU_ONLY;
double __dp_math_fdim(double, double) __GPU_ONLY;
double __dp_math_floor(double) __GPU_ONLY;
double __dp_math_fma(double, double, double) __GPU_ONLY;
double __dp_math_fmax(double, double) __GPU_ONLY;
double __dp_math_fmin(double, double) __GPU_ONLY;
double __dp_math_fmod(double, double) __GPU_ONLY;
int    __dp_math_fpclassify(double) __GPU_ONLY;
double __dp_math_frexp(double, _Out_ int *) __GPU_ONLY;
double __dp_math_hypot(double, double) __GPU_ONLY;
int    __dp_math_ilogb(double) __GPU_ONLY;
int    __dp_math_isfinite(double) __GPU_ONLY;
int    __dp_math_isinf(double) __GPU_ONLY;
int    __dp_math_isnan(double) __GPU_ONLY;
int    __dp_math_isnormal(double) __GPU_ONLY;
double __dp_math_ldexp(double, int) __GPU_ONLY;
double __dp_math_lgamma(double, _Out_ int *) __GPU_ONLY;
double __dp_math_log(double) __GPU_ONLY;
double __dp_math_log10(double) __GPU_ONLY;
double __dp_math_log2(double) __GPU_ONLY;
double __dp_math_log1p(double) __GPU_ONLY;
double __dp_math_logb(double) __GPU_ONLY;
double __dp_math_modf(double, _Out_ double *) __GPU_ONLY;
double __dp_math_nan(int) __GPU_ONLY;
double __dp_math_nearbyint(double) __GPU_ONLY;
double __dp_math_nextafter(double, double) __GPU_ONLY;
double __dp_math_phi(double) __GPU_ONLY;
double __dp_math_pow(double, double) __GPU_ONLY;
double __dp_math_probit(double) __GPU_ONLY;
double __dp_math_rcbrt(double) __GPU_ONLY;
double __dp_math_remainder(double, double) __GPU_ONLY;
double __dp_math_remquo(double, double, _Out_ int *) __GPU_ONLY;
double __dp_math_round(double) __GPU_ONLY;
double __dp_math_rsqrt(double) __GPU_ONLY;
double __dp_math_scalb(double, double) __GPU_ONLY;
double __dp_math_scalbn(double, int) __GPU_ONLY;
int    __dp_math_signbit(double) __GPU_ONLY;
double __dp_math_sin(double) __GPU_ONLY;
double __dp_math_sincos(double, _Out_ double *) __GPU_ONLY;
double __dp_math_sinpi(double) __GPU_ONLY;
double __dp_math_sinh(double) __GPU_ONLY;
double __dp_math_sqrt(double) __GPU_ONLY;
double __dp_math_tan(double) __GPU_ONLY;
double __dp_math_tanh(double) __GPU_ONLY;
double __dp_math_tanpi(double) __GPU_ONLY;
double __dp_math_tgamma(double) __GPU_ONLY;
double __dp_math_trunc(double) __GPU_ONLY;
}

namespace Concurrency
{
/// <summary>
///    Functions in the fast_math namespace have lower accuracy, and support
///    only single-precision float.
/// </summary>
namespace fast_math
{
	/// <summary>
	///     Returns the absolute value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the absolute value of the argument
	/// </returns>
	inline float fabsf(float _X) __GPU_ONLY
	{
		return __dp_d3d_absf(_X);
	}

	/// <summary>
	///     Returns the absolute value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the absolute value of the argument
	/// </returns>
	inline float fabs(float _X) __GPU_ONLY
	{
		return __dp_d3d_absf(_X);
	}

	/// <summary>
	///     Calculates the arccosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arccosine value of the argument
	/// </returns>
	inline float acosf(float _X) __GPU_ONLY
	{
		return __dp_d3d_acosf(_X);
	}

	/// <summary>
	///     Calculates the arccosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arccosine value of the argument
	/// </returns>
	inline float acos(float _X) __GPU_ONLY
	{
		return __dp_d3d_acosf(_X);
	}

	/// <summary>
	///     Calculates the arcsine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arcsine value of the argument
	/// </returns>
	inline float asinf(float _X) __GPU_ONLY
	{
		return __dp_d3d_asinf(_X);
	}

	/// <summary>
	///     Calculates the arcsine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arcsine value of the argument
	/// </returns>
	inline float asin(float _X) __GPU_ONLY
	{
		return __dp_d3d_asinf(_X);
	}

	/// <summary>
	///     Calculates the arctangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of the argument
	/// </returns>
	inline float atanf(float _X) __GPU_ONLY
	{
		return __dp_d3d_atanf(_X);
	}

	/// <summary>
	///     Calculates the arctangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of the argument
	/// </returns>
	inline float atan(float _X) __GPU_ONLY
	{
		return __dp_d3d_atanf(_X);
	}

	/// <summary>
	///     Calculates the arctangent of _Y/_X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of _Y/_X
	/// </returns>
	inline float atan2f(float _Y, float _X) __GPU_ONLY
	{
		return __dp_d3d_atan2f(_Y, _X);
	}

	/// <summary>
	///     Calculates the arctangent of _Y/_X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of _Y/_X
	/// </returns>
	inline float atan2(float _Y, float _X) __GPU_ONLY
	{
		return __dp_d3d_atan2f(_Y, _X);
	}

	/// <summary>
	///     Calculates the ceiling of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the ceiling of the argument
	/// </returns>
	inline float ceilf(float _X) __GPU_ONLY
	{
		return __dp_d3d_ceilf(_X);
	}

	/// <summary>
	///     Calculates the ceiling of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the ceiling of the argument
	/// </returns>
	inline float ceil(float _X) __GPU_ONLY
	{
		return __dp_d3d_ceilf(_X);
	}

	/// <summary>
	///     Calculates the cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of the argument
	/// </returns>
	inline float cosf(float _X) __GPU_ONLY
	{
		return __dp_d3d_cosf(_X);
	}

	/// <summary>
	///     Calculates the cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of the argument
	/// </returns>
	inline float cos(float _X) __GPU_ONLY
	{
		return __dp_d3d_cosf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic cosine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic cosine value of the argument
	/// </returns>
	inline float coshf(float _X) __GPU_ONLY
	{
		return __dp_d3d_coshf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic cosine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic cosine value of the argument
	/// </returns>
	inline float cosh(float _X) __GPU_ONLY
	{
		return __dp_d3d_coshf(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument
	/// </returns>
	inline float expf(float _X) __GPU_ONLY
	{
		return __dp_d3d_expf(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument
	/// </returns>
	inline float exp(float _X) __GPU_ONLY
	{
		return __dp_d3d_expf(_X);
	}

	/// <summary>
	///     Calculates the base-2 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-2 exponential of the argument
	/// </returns>
	inline float exp2f(float _X) __GPU_ONLY
	{
		return __dp_d3d_exp2f(_X);
	}

	/// <summary>
	///     Calculates the base-2 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-2 exponential of the argument
	/// </returns>
	inline float exp2(float _X) __GPU_ONLY
	{
		return __dp_d3d_exp2f(_X);
	}

	/// <summary>
	///     Calculates the floor of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floor of the argument
	/// </returns>
	inline float floorf(float _X) __GPU_ONLY
	{
		return __dp_d3d_floorf(_X);
	}

	/// <summary>
	///     Calculates the floor of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floor of the argument
	/// </returns>
	inline float floor(float _X) __GPU_ONLY
	{
		return __dp_d3d_floorf(_X);
	}

	/// <summary>
	///     Determine the maximum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the maximum numeric value of the arguments
	/// </returns>
	inline float fmaxf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_maxf(_X, _Y);
	}

	/// <summary>
	///     Determine the maximum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the maximum numeric value of the arguments
	/// </returns>
	inline float fmax(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_maxf(_X, _Y);
	}

	/// <summary>
	///     Determine the minimum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the minimum numeric value of the arguments
	/// </returns>
	inline float fminf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_minf(_X, _Y);
	}

	/// <summary>
	///     Determine the minimum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the minimum numeric value of the arguments
	/// </returns>
	inline float fmin(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_minf(_X, _Y);
	}

	/// <summary>
	///     Calculates the floating-point remainder of _X/_Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floating-point remainder of _X/_Y
	/// </returns>
	inline float fmodf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_fmodf(_X, _Y);
	}

	/// <summary>
	///     Calculates the floating-point remainder of _X/_Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floating-point remainder of _X/_Y
	/// </returns>
	inline float fmod(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_fmodf(_X, _Y);
	}

	/// <summary>
	///     Gets the mantissa and exponent of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Exp">
	///     Returns the integer exponent of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the mantissa _X
	/// </returns>
	inline float frexpf(float _X, _Out_ int * _Exp) __GPU_ONLY
	{
	    float _FExp = 0.0f;
		float _M = __dp_d3d_frexpf(_X, &_FExp);
		*_Exp = static_cast<int>(_FExp);
		// Currently, the mantissa returned by d3d's frexp is always positive
		// Fetch the sign bit from _X to match cmath frexp
		*reinterpret_cast<unsigned int*>(&_M) = *reinterpret_cast<unsigned int*>(&_M) | (*reinterpret_cast<unsigned int*>(&_X) & 0x80000000);
		return _M;
	}

	/// <summary>
	///     Gets the mantissa and exponent of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Exp">
	///     Returns the integer exponent of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the mantissa _X
	/// </returns>
	inline float frexp(float _X, _Out_ int * _Exp) __GPU_ONLY
	{
        return frexpf(_X, _Exp);
	}

	/// <summary>
	///     Determines whether the argument has a finite value
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a finite value
	/// </returns>
	inline int isfinite(float _X) __GPU_ONLY
	{
		return __dp_d3d_isfinitef(_X);
	}

	/// <summary>
	///     Determines whether the argument is an infinity
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has an infinite value
	/// </returns>
	inline int isinf(float _X) __GPU_ONLY
	{
		return __dp_d3d_isinff(_X);
	}

	/// <summary>
	///     Determines whether the argument is a NaN
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a NaN value
	/// </returns>
	inline int isnan(float _X) __GPU_ONLY
	{
		return __dp_d3d_isnanf(_X);
	}

	/// <summary>
	///     Computes a real number from the mantissa and exponent
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, mantissa
	/// </param>
	/// <param name="_Exp">
	///     Integer value, exponent
	/// </param>
	/// <returns>
	///     Returns _X * 2^_Exp
	/// </returns>
	inline float ldexpf(float _X, int _Exp) __GPU_ONLY
	{
	    float _FExp = static_cast<float>(_Exp);
		return __dp_d3d_ldexpf(_X, _FExp);
	}

	/// <summary>
	///     Computes a real number from the mantissa and exponent
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, mantissa
	/// </param>
	/// <param name="_Exp">
	///     Integer value, exponent
	/// </param>
	/// <returns>
	///     Returns _X * 2^_Exp
	/// </returns>
	inline float ldexp(float _X, int _Exp) __GPU_ONLY
	{
		return ldexpf(_X, _Exp);
	}

	/// <summary>
	///     Calculates the base-e logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of the argument
	/// </returns>
	inline float logf(float _X) __GPU_ONLY
	{
		return __dp_d3d_logf(_X);
	}

	/// <summary>
	///     Calculates the base-e logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of the argument
	/// </returns>
	inline float log(float _X) __GPU_ONLY
	{
		return __dp_d3d_logf(_X);
	}

	/// <summary>
	///     Calculates the base-10 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline float log10f(float _X) __GPU_ONLY
	{
		return __dp_d3d_log10f(_X);
	}

	/// <summary>
	///     Calculates the base-10 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline float log10(float _X) __GPU_ONLY
	{
		return __dp_d3d_log10f(_X);
	}

	/// <summary>
	///     Calculates the base-2 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-2 logarithm of the argument
	/// </returns>
	inline float log2f(float _X) __GPU_ONLY
	{
		return __dp_d3d_log2f(_X);
	}

	/// <summary>
	///     Calculates the base-2 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-2 logarithm of the argument
	/// </returns>
	inline float log2(float _X) __GPU_ONLY
	{
		return __dp_d3d_log2f(_X);
	}

	/// <summary>
	///     Splits _X into fractional and integer parts.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Ip">
	///     Returns the integer portion of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed fractional portion of _X
	/// </returns>
	inline float modff(float _X, float * _Ip) __GPU_ONLY
	{
		return __dp_d3d_modff(_X, _Ip);
	}

	/// <summary>
	///     Splits _X into fractional and integer parts.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Ip">
	///     Returns the integer portion of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed fractional portion of _X
	/// </returns>
	inline float modf(float _X, float * _Ip) __GPU_ONLY
	{
		return __dp_d3d_modff(_X, _Ip);
	}

	/// <summary>
	///     Calculates _X raised to the power of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, base
	/// </param>
	/// <param name="_Y">
	///     Floating-point value, exponent
	/// </param>
	/// <returns>
	///     Returns the value of _X raised to the power of _Y
	/// </returns>
	inline float powf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_powf(_X, _Y);
	}

	/// <summary>
	///     Calculates _X raised to the power of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, base
	/// </param>
	/// <param name="_Y">
	///     Floating-point value, exponent
	/// </param>
	/// <returns>
	///     Returns the value of _X raised to the power of _Y
	/// </returns>
	inline float pow(float _X, float _Y) __GPU_ONLY
	{
		return __dp_d3d_powf(_X, _Y);
	}

	/// <summary>
	///     Rounds _X to the nearest integer
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the nearest integer of _X
	/// </returns>
	inline float roundf(float _X) __GPU_ONLY
	{
		return __dp_d3d_roundf(_X);
	}

	/// <summary>
	///     Rounds _X to the nearest integer
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the nearest integer of _X
	/// </returns>
	inline float round(float _X) __GPU_ONLY
	{
		return __dp_d3d_roundf(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the square root of the argument
	/// </returns>
	inline float rsqrtf(float _X) __GPU_ONLY
	{
		return __dp_d3d_rsqrtf(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the square root of the argument
	/// </returns>
	inline float rsqrt(float _X) __GPU_ONLY
	{
		return __dp_d3d_rsqrtf(_X);
	}

	/// <summary>
	///     Returns the sign of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sign of the argument
	/// </returns>
	inline int signbitf(float _X) __GPU_ONLY
	{
		return __dp_d3d_signf(_X);
	}

	/// <summary>
	///     Returns the sign of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sign of the argument
	/// </returns>
	inline int signbit(float _X) __GPU_ONLY
	{
		return __dp_d3d_signf(_X);
	}

	/// <summary>
	///     Calculates the sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of the argument
	/// </returns>
	inline float sinf(float _X) __GPU_ONLY
	{
		return __dp_d3d_sinf(_X);
	}

	/// <summary>
	///     Calculates the sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of the argument
	/// </returns>
	inline float sin(float _X) __GPU_ONLY
	{
		return __dp_d3d_sinf(_X);
	}

	/// <summary>
	///     Calculates sine and cosine value of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_S">
	///     Returns the sine value of _X
	/// </param>
	/// <param name="_C">
	///     Returns the cosine value of _X
	/// </param>
	inline void sincosf(float _X, float * _S, float * _C) __GPU_ONLY
	{
		*_C = __dp_d3d_sincosf(_X, _S);
	}

	/// <summary>
	///     Calculates sine and cosine value of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_S">
	///     Returns the sine value of _X
	/// </param>
	/// <param name="_C">
	///     Returns the cosine value of _X
	/// </param>
	inline void sincos(float _X, float * _S, float * _C) __GPU_ONLY
	{
		*_C = __dp_d3d_sincosf(_X, _S);
	}

	/// <summary>
	///     Calculates the hyperbolic sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic sine value of the argument
	/// </returns>
	inline float sinhf(float _X) __GPU_ONLY
	{
		return __dp_d3d_sinhf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic sine value of the argument
	/// </returns>
	inline float sinh(float _X) __GPU_ONLY
	{
		return __dp_d3d_sinhf(_X);
	}

	/// <summary>
	///     Calculates the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the argument
	/// </returns>
	inline float sqrtf(float _X) __GPU_ONLY
	{
		return __dp_d3d_sqrtf(_X);
	}

	/// <summary>
	///     Calculates the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the argument
	/// </returns>
	inline float sqrt(float _X) __GPU_ONLY
	{
		return __dp_d3d_sqrtf(_X);
	}

	/// <summary>
	///     Calculates the tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of the argument
	/// </returns>
	inline float tanf(float _X) __GPU_ONLY
	{
		return __dp_d3d_tanf(_X);
	}

	/// <summary>
	///     Calculates the tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of the argument
	/// </returns>
	inline float tan(float _X) __GPU_ONLY
	{
		return __dp_d3d_tanf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic tangent value of the argument
	/// </returns>
	inline float tanhf(float _X) __GPU_ONLY
	{
		return __dp_d3d_tanhf(_X);
	}
	/// <summary>
	///     Calculates the hyperbolic tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic tangent value of the argument
	/// </returns>
	inline float tanh(float _X) __GPU_ONLY
	{
		return __dp_d3d_tanhf(_X);
	}

	/// <summary>
	///     Truncates the argument to the integer component
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the integer component of the argument
	/// </returns>
	inline float truncf(float _X) __GPU_ONLY
	{
		return __dp_d3d_truncf(_X);
	}

	/// <summary>
	///     Truncates the argument to the integer component
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the integer component of the argument
	/// </returns>
	inline float trunc(float _X) __GPU_ONLY
	{
		return __dp_d3d_truncf(_X);
	}

    //=============================================================================
    // Import CMATH C++ functions:
    //=============================================================================
    using std::acosf;
    using std::asinf;
    using std::atanf;
    using std::atan2f;
    using std::ceilf;
    using std::cosf;
    using std::coshf;
    using std::expf;
    using std::fabsf;
    using std::floorf;
    using std::fmodf;
    using std::frexpf;
    using std::ldexpf;
    using std::logf;
    using std::log10f;
    using std::modff;
    using std::powf;
    using std::sinf;
    using std::sinhf;
    using std::sqrtf;
    using std::tanf;
    using std::tanhf;

    using std::acos;
    using std::asin;
    using std::atan;
    using std::atan2;
    using std::ceil;
    using std::cos;
    using std::cosh;
    using std::exp;
    using std::fabs;
    using std::floor;
    using std::fmod;
    using std::frexp;
    using std::ldexp;
    using std::log;
    using std::log10;
    using std::modf;
    using std::pow;
    using std::sin;
    using std::sinh;
    using std::sqrt;
    using std::tan;
    using std::tanh;
    using std::exp2;
    using std::exp2f;
    using std::fmax;
    using std::fmaxf;
    using std::fmin;
    using std::fminf;
    using std::log2;
    using std::log2f;
    using std::round;
    using std::roundf;
    using std::trunc;
    using std::truncf;

} // namespace fast_math


/// <summary>
///    Functions in the precise_math namespace have higher accuracy, but require
///    double-precision support, which not all accelerators do.
/// </summary>
namespace precise_math
{
	/// <summary>
	///     Calculates the arccosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arccosine value of the argument
	/// </returns>
	inline float acosf(float _X) __GPU_ONLY
	{
		return __dp_math_acosf(_X);
	}

	/// <summary>
	///     Calculates the arccosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arccosine value of the argument
	/// </returns>
	inline float acos(float _X) __GPU_ONLY
	{
		return __dp_math_acosf(_X);
	}

	/// <summary>
	///     Calculates the arccosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arccosine value of the argument
	/// </returns>
	inline double acos(double _X) __GPU_ONLY
	{
		return __dp_math_acos(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic cosine value of the argument
	/// </returns>
	inline float acoshf(float _X) __GPU_ONLY
	{
		return __dp_math_acoshf(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic cosine value of the argument
	/// </returns>
	inline float acosh(float _X) __GPU_ONLY
	{
		return __dp_math_acoshf(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic cosine value of the argument
	/// </returns>
	inline double acosh(double _X) __GPU_ONLY
	{
		return __dp_math_acosh(_X);
	}


	/// <summary>
	///     Calculates the arcsine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arcsine value of the argument
	/// </returns>
	inline float asinf(float _X) __GPU_ONLY
	{
		return __dp_math_asinf(_X);
	}

	/// <summary>
	///     Calculates the arcsine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arcsine value of the argument
	/// </returns>
	inline float asin(float _X) __GPU_ONLY
	{
		return __dp_math_asinf(_X);
	}

	/// <summary>
	///     Calculates the arcsine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arcsine value of the argument
	/// </returns>
	inline double asin(double _X) __GPU_ONLY
	{
		return __dp_math_asin(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic sine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic sine value of the argument
	/// </returns>
	inline float asinhf(float _X) __GPU_ONLY
	{
		return __dp_math_asinhf(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic sine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic sine value of the argument
	/// </returns>
	inline float asinh(float _X) __GPU_ONLY
	{
		return __dp_math_asinhf(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic sine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic sine value of the argument
	/// </returns>
	inline double asinh(double _X) __GPU_ONLY
	{
		return __dp_math_asinh(_X);
	}

	/// <summary>
	///     Calculates the arctangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of the argument
	/// </returns>
	inline float atanf(float _X) __GPU_ONLY
	{
		return __dp_math_atanf(_X);
	}

	/// <summary>
	///     Calculates the arctangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of the argument
	/// </returns>
	inline float atan(float _X) __GPU_ONLY
	{
		return __dp_math_atanf(_X);
	}

	/// <summary>
	///     Calculates the arctangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of the argument
	/// </returns>
	inline double atan(double _X) __GPU_ONLY
	{
		return __dp_math_atan(_X);
	}

	/// <summary>
	///     Calculates the arctangent of _Y/_X
	/// </summary>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of _Y/_X
	/// </returns>
	inline float atan2f(float _Y, float _X) __GPU_ONLY
	{
		return __dp_math_atan2f(_Y, _X);
	}

	/// <summary>
	///     Calculates the arctangent of _Y/_X
	/// </summary>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of _Y/_X
	/// </returns>
	inline float atan2(float _Y, float _X) __GPU_ONLY
	{
		return __dp_math_atan2f(_Y, _X);
	}

	/// <summary>
	///     Calculates the arctangent of _Y/_X
	/// </summary>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the arctangent value of _Y/_X
	/// </returns>
	inline double atan2(double _Y, double _X) __GPU_ONLY
	{
		return __dp_math_atan2(_Y, _X);
	}


	/// <summary>
	///     Calculates the inverse hyperbolic tangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic tangent value of the argument
	/// </returns>
	inline float atanhf(float _X) __GPU_ONLY
	{
		return __dp_math_atanhf(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic tangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic tangent value of the argument
	/// </returns>
	inline float atanh(float _X) __GPU_ONLY
	{
		return __dp_math_atanhf(_X);
	}

	/// <summary>
	///     Calculates the inverse hyperbolic tangent of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse hyperbolic tangent value of the argument
	/// </returns>
	inline double atanh(double _X) __GPU_ONLY
	{
		return __dp_math_atanh(_X);
	}

	/// <summary>
	///     Computes the real cube root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the real cube root of the argument
	/// </returns>
	inline float cbrtf(float _X) __GPU_ONLY
	{
		return __dp_math_cbrtf(_X);
	}

	/// <summary>
	///     Computes the real cube root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the real cube root of the argument
	/// </returns>
	inline float cbrt(float _X) __GPU_ONLY
	{
		return __dp_math_cbrtf(_X);
	}

	/// <summary>
	///     Computes the real cube root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the real cube root of the argument
	/// </returns>
	inline double cbrt(double _X) __GPU_ONLY
	{
		return __dp_math_cbrt(_X);
	}

	/// <summary>
	///     Calculates the ceiling of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the ceiling of the argument
	/// </returns>
	inline float ceilf(float _X) __GPU_ONLY
	{
		return __dp_math_ceilf(_X);
	}

	/// <summary>
	///     Calculates the ceiling of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the ceiling of the argument
	/// </returns>
	inline float ceil(float _X) __GPU_ONLY
	{
		return __dp_math_ceilf(_X);
	}

	/// <summary>
	///     Calculates the ceiling of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the ceiling of the argument
	/// </returns>
	inline double ceil(double _X) __GPU_ONLY
	{
		return __dp_math_ceil(_X);
	}

	/// <summary>
	///     Produces a value with the magnitude of _X and the sign of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a value with the magnitude of _X and the sign of _Y
	/// </returns>
	inline float copysignf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_copysignf(_X, _Y);
	}

	/// <summary>
	///     Produces a value with the magnitude of _X and the sign of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a value with the magnitude of _X and the sign of _Y
	/// </returns>
	inline float copysign(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_copysignf(_X, _Y);
	}

	/// <summary>
	///     Produces a value with the magnitude of _X and the sign of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a value with the magnitude of _X and the sign of _Y
	/// </returns>
	inline double copysign(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_copysign(_X, _Y);
	}

	/// <summary>
	///     Calculates the cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of the argument
	/// </returns>
	inline float cosf(float _X) __GPU_ONLY
	{
		return __dp_math_cosf(_X);
	}

	/// <summary>
	///     Calculates the cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of the argument
	/// </returns>
	inline float cos(float _X) __GPU_ONLY
	{
		return __dp_math_cosf(_X);
	}

	/// <summary>
	///     Calculates the cosine of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of the argument
	/// </returns>
	inline double cos(double _X) __GPU_ONLY
	{
		return __dp_math_cos(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic cosine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic cosine value of the argument
	/// </returns>
	inline float coshf(float _X) __GPU_ONLY
	{
		return __dp_math_coshf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic cosine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic cosine value of the argument
	/// </returns>
	inline float cosh(float _X) __GPU_ONLY
	{
		return __dp_math_coshf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic cosine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic cosine value of the argument
	/// </returns>
	inline double cosh(double _X) __GPU_ONLY
	{
		return __dp_math_cosh(_X);
	}

	/// <summary>
	///     Calculates the cosine value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of pi * _X
	/// </returns>
	inline float cospif(float _X) __GPU_ONLY
	{
		return __dp_math_cospif(_X);
	}

	/// <summary>
	///     Calculates the cosine value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of pi * _X
	/// </returns>
	inline float cospi(float _X) __GPU_ONLY
	{
		return __dp_math_cospif(_X);
	}

	/// <summary>
	///     Calculates the cosine value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cosine value of pi * _X
	/// </returns>
	inline double cospi(double _X) __GPU_ONLY
	{
		return __dp_math_cospi(_X);
	}

	/// <summary>
	///     Computes the error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the error function of _X
	/// </returns>
	inline float erff(float _X) __GPU_ONLY
	{
		return __dp_math_erff(_X);
	}

	/// <summary>
	///     Computes the error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the error function of _X
	/// </returns>
	inline float erf(float _X) __GPU_ONLY
	{
		return __dp_math_erff(_X);
	}

	/// <summary>
	///     Computes the error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the error function of _X
	/// </returns>
	inline double erf(double _X) __GPU_ONLY
	{
		return __dp_math_erf(_X);
	}

	/// <summary>
	///     Computes the complementary error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the complementary error function of _X
	/// </returns>
	inline float erfcf(float _X) __GPU_ONLY
	{
		return __dp_math_erfcf(_X);
	}

	/// <summary>
	///     Computes the complementary error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the complementary error function of _X
	/// </returns>
	inline float erfc(float _X) __GPU_ONLY
	{
		return __dp_math_erfcf(_X);
	}

	/// <summary>
	///     Computes the complementary error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the complementary error function of _X
	/// </returns>
	inline double erfc(double _X) __GPU_ONLY
	{
		return __dp_math_erfc(_X);
	}

	/// <summary>
	///     Computes the inverse error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse error function of _X
	/// </returns>
	inline float erfinvf(float _X) __GPU_ONLY
	{
		return __dp_math_erfinvf(_X);
	}

	/// <summary>
	///     Computes the inverse error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse error function of _X
	/// </returns>
	inline float erfinv(float _X) __GPU_ONLY
	{
		return __dp_math_erfinvf(_X);
	}

	/// <summary>
	///     Computes the inverse error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse error function of _X
	/// </returns>
	inline double erfinv(double _X) __GPU_ONLY
	{
		return __dp_math_erfinv(_X);
	}

	/// <summary>
	///     Computes the inverse complementary error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse complementary error function of _X
	/// </returns>
	inline float erfcinvf(float _X) __GPU_ONLY
	{
		return __dp_math_erfcinvf(_X);
	}

	/// <summary>
	///     Computes the inverse complementary error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse complementary error function of _X
	/// </returns>
	inline float erfcinv(float _X) __GPU_ONLY
	{
		return __dp_math_erfcinvf(_X);
	}

	/// <summary>
	///     Computes the inverse complementary error function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse complementary error function of _X
	/// </returns>
	inline double erfcinv(double _X) __GPU_ONLY
	{
		return __dp_math_erfcinv(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument
	/// </returns>
	inline float expf(float _X) __GPU_ONLY
	{
		return __dp_math_expf(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument
	/// </returns>
	inline float exp(float _X) __GPU_ONLY
	{
		return __dp_math_expf(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument
	/// </returns>
	inline double exp(double _X) __GPU_ONLY
	{
		return __dp_math_exp(_X);
	}

	/// <summary>
	///     Calculates the base-2 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-2 exponential of the argument
	/// </returns>
	inline float exp2f(float _X) __GPU_ONLY
	{
		return __dp_math_exp2f(_X);
	}

	/// <summary>
	///     Calculates the base-2 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-2 exponential of the argument
	/// </returns>
	inline float exp2(float _X) __GPU_ONLY
	{
		return __dp_math_exp2f(_X);
	}

	/// <summary>
	///     Calculates the base-2 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-2 exponential of the argument
	/// </returns>
	inline double exp2(double _X) __GPU_ONLY
	{
		return __dp_math_exp2(_X);
	}

	/// <summary>
	///     Calculates the base-10 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 exponential of the argument
	/// </returns>
	inline float exp10f(float _X) __GPU_ONLY
	{
		return __dp_math_exp10f(_X);
	}

	/// <summary>
	///     Calculates the base-10 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 exponential of the argument
	/// </returns>
	inline float exp10(float _X) __GPU_ONLY
	{
		return __dp_math_exp10f(_X);
	}

	/// <summary>
	///     Calculates the base-10 exponential of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 exponential of the argument
	/// </returns>
	inline double exp10(double _X) __GPU_ONLY
	{
		return __dp_math_exp10(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument, minus 1
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument, minus 1
	/// </returns>
	inline float expm1f(float _X) __GPU_ONLY
	{
		return __dp_math_expm1f(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument, minus 1
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument, minus 1
	/// </returns>
	inline float expm1(float _X) __GPU_ONLY
	{
		return __dp_math_expm1f(_X);
	}

	/// <summary>
	///     Calculates the base-e exponential of the argument, minus 1
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e exponential of the argument, minus 1
	/// </returns>
	inline double expm1(double _X) __GPU_ONLY
	{
		return __dp_math_expm1(_X);
	}

	/// <summary>
	///     Returns the absolute value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the absolute value of the argument
	/// </returns>
	inline float fabsf(float _X) __GPU_ONLY
	{
		return __dp_math_fabsf(_X);
	}

	/// <summary>
	///     Returns the absolute value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the absolute value of the argument
	/// </returns>
	inline float fabs(float _X) __GPU_ONLY
	{
		return __dp_math_fabsf(_X);
	}

	/// <summary>
	///     Returns the absolute value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the absolute value of the argument
	/// </returns>
	inline double fabs(double _X) __GPU_ONLY
	{
		return __dp_math_fabs(_X);
	}

	/// <summary>
	///     Determines the positive difference between the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X - _Y if _X &gt; _Y; +0, otherwise
	/// </returns>
	inline float fdimf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fdimf(_X, _Y);
	}


	/// <summary>
	///     Determines the positive difference between the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X - _Y if _X &gt; _Y; +0, otherwise
	/// </returns>
	inline float fdim(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fdimf(_X, _Y);
	}


	/// <summary>
	///     Determines the positive difference between the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X - _Y if _X &gt; _Y; +0, otherwise
	/// </returns>
	inline double fdim(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_fdim(_X, _Y);
	}

	/// <summary>
	///     Calculates the floor of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floor of the argument
	/// </returns>
	inline float floorf(float _X) __GPU_ONLY
	{
		return __dp_math_floorf(_X);
	}

	/// <summary>
	///     Calculates the floor of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floor of the argument
	/// </returns>
	inline float floor(float _X) __GPU_ONLY
	{
		return __dp_math_floorf(_X);
	}

	/// <summary>
	///     Calculates the floor of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floor of the argument
	/// </returns>
	inline double floor(double _X) __GPU_ONLY
	{
		return __dp_math_floor(_X);
	}

	/// <summary>
	///     Compute (_X * _Y) + _Z, rounded as one ternary operation
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_Z">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns (_X * _Y) + _Z, rounded as one ternary operation
	/// </returns>
	inline float fmaf(float _X, float _Y, float _Z) __GPU_ONLY
	{
		return __dp_math_fmaf(_X, _Y, _Z);
	}

	/// <summary>
	///     Compute (_X * _Y) + _Z, rounded as one ternary operation
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_Z">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns (_X * _Y) + _Z, rounded as one ternary operation
	/// </returns>
	inline float fma(float _X, float _Y, float _Z) __GPU_ONLY
	{
		return __dp_math_fmaf(_X, _Y, _Z);
	}

	/// <summary>
	///     Compute (_X * _Y) + _Z, rounded as one ternary operation
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_Z">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns (_X * _Y) + _Z, rounded as one ternary operation
	/// </returns>
	inline double fma(double _X, double _Y, double _Z) __GPU_ONLY
	{
		return __dp_math_fma(_X, _Y, _Z);
	}

	/// <summary>
	///     Determine the maximum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the maximum numeric value of the arguments
	/// </returns>
	inline float fmaxf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fmaxf(_X, _Y);
	}

	/// <summary>
	///     Determine the maximum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the maximum numeric value of the arguments
	/// </returns>
	inline float fmax(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fmaxf(_X, _Y);
	}

	/// <summary>
	///     Determine the maximum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the maximum numeric value of the arguments
	/// </returns>
	inline double fmax(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_fmax(_X, _Y);
	}

	/// <summary>
	///     Determine the minimum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the minimum numeric value of the arguments
	/// </returns>
	inline float fminf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fminf(_X, _Y);
	}

	/// <summary>
	///     Determine the minimum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the minimum numeric value of the arguments
	/// </returns>
	inline float fmin(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fminf(_X, _Y);
	}

	/// <summary>
	///     Determine the minimum numeric value of the arguments
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Return the minimum numeric value of the arguments
	/// </returns>
	inline double fmin(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_fmin(_X, _Y);
	}

	/// <summary>
	///     Calculates the floating-point remainder of _X/_Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floating-point remainder of _X/_Y
	/// </returns>
	inline float fmodf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fmodf(_X, _Y);
	}

	/// <summary>
	///     Calculates the floating-point remainder of _X/_Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floating-point remainder of _X/_Y
	/// </returns>
	inline float fmod(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_fmodf(_X, _Y);
	}

	/// <summary>
	///     Calculates the floating-point remainder of _X/_Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the floating-point remainder of _X/_Y
	/// </returns>
	inline double fmod(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_fmod(_X, _Y);
	}


	/// <summary>
	///     Classifies the argument value as NaN, infinite, normal, subnormal, zero
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the value of the number classification macro appropriate to the value of the argument.
	/// </returns>
	inline int fpclassify(float _X) __GPU_ONLY
	{
		return __dp_math_fpclassifyf(_X);
	}

	/// <summary>
	///     Classifies the argument value as NaN, infinite, normal, subnormal, zero
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the value of the number classification macro appropriate to the value of the argument.
	/// </returns>
	inline int fpclassify(double _X) __GPU_ONLY
	{
		return __dp_math_fpclassify(_X);
	}

	/// <summary>
	///     Gets the mantissa and exponent of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Exp">
	///     Returns the integer exponent of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the mantissa _X
	/// </returns>
	inline float frexpf(float _X, _Out_ int * _Exp) __GPU_ONLY
	{
		return __dp_math_frexpf(_X, _Exp);
	}

	/// <summary>
	///     Gets the mantissa and exponent of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Exp">
	///     Returns the integer exponent of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the mantissa _X
	/// </returns>
	inline float frexp(float _X, _Out_ int * _Exp) __GPU_ONLY
	{
		return __dp_math_frexpf(_X, _Exp);
	}

	/// <summary>
	///     Gets the mantissa and exponent of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Exp">
	///     Returns the integer exponent of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the mantissa _X
	/// </returns>
	inline double frexp(double _X, _Out_ int * _Exp) __GPU_ONLY
	{
		return __dp_math_frexp(_X, _Exp);
	}

	/// <summary>
	///     Computes the square root of the sum of the squares of _X and _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the sum of the squares of _X and _Y
	/// </returns>
	inline float hypotf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_hypotf(_X, _Y);
	}

	/// <summary>
	///     Computes the square root of the sum of the squares of _X and _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the sum of the squares of _X and _Y
	/// </returns>
	inline float hypot(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_hypotf(_X, _Y);
	}

	/// <summary>
	///     Computes the square root of the sum of the squares of _X and _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the sum of the squares of _X and _Y
	/// </returns>
	inline double hypot(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_hypot(_X, _Y);
	}

	/// <summary>
	///     Extract the exponent of _X as a signed int value
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the exponent of _X as a signed int value
	/// </returns>
	inline int ilogbf(float _X) __GPU_ONLY
	{
		return __dp_math_ilogbf(_X);
	}

	/// <summary>
	///     Extract the exponent of _X as a signed int value
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the exponent of _X as a signed int value
	/// </returns>
	inline int ilogb(float _X) __GPU_ONLY
	{
		return __dp_math_ilogbf(_X);
	}

	/// <summary>
	///     Extract the exponent of _X as a signed int value
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the exponent of _X as a signed int value
	/// </returns>
	inline int ilogb(double _X) __GPU_ONLY
	{
		return __dp_math_ilogb(_X);
	}

	/// <summary>
	///     Determines whether the argument has a finite value
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a finite value
	/// </returns>
	inline int isfinite(float _X) __GPU_ONLY
	{
		return __dp_math_isfinitef(_X);
	}

	/// <summary>
	///     Determines whether the argument has a finite value
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a finite value
	/// </returns>
	inline int isfinite(double _X) __GPU_ONLY
	{
		return __dp_math_isfinite(_X);
	}

	/// <summary>
	///     Determines whether the argument is an infinity
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has an infinite value
	/// </returns>
	inline int isinf(float _X) __GPU_ONLY
	{
		return __dp_math_isinff(_X);
	}

	/// <summary>
	///     Determines whether the argument is an infinity
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has an infinite value
	/// </returns>
	inline int isinf(double _X) __GPU_ONLY
	{
		return __dp_math_isinf(_X);
	}

	/// <summary>
	///     Determines whether the argument is a NaN
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a NaN value
	/// </returns>
	inline int isnan(float _X) __GPU_ONLY
	{
		return __dp_math_isnanf(_X);
	}

	/// <summary>
	///     Determines whether the argument is a NaN
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a NaN value
	/// </returns>
	inline int isnan(double _X) __GPU_ONLY
	{
		return __dp_math_isnan(_X);
	}

	/// <summary>
	///     Determines whether the argument is a normal
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a normal value
	/// </returns>
	inline int isnormal(float _X) __GPU_ONLY
	{
		return __dp_math_isnormalf(_X);
	}

	/// <summary>
	///     Determines whether the argument is a normal
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the argument has a normal value
	/// </returns>
	inline int isnormal(double _X) __GPU_ONLY
	{
		return __dp_math_isnormal(_X);
	}

	/// <summary>
	///     Computes a real number from the mantissa and exponent
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, mantissa
	/// </param>
	/// <param name="_Exp">
	///     Integer value, exponent
	/// </param>
	/// <returns>
	///     Returns _X * 2^_Exp
	/// </returns>
	inline float ldexpf(float _X, int _Exp) __GPU_ONLY
	{
		return __dp_math_ldexpf(_X, _Exp);
	}

	/// <summary>
	///     Computes a real number from the mantissa and exponent
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, mantissa
	/// </param>
	/// <param name="_Exp">
	///     Integer value, exponent
	/// </param>
	/// <returns>
	///     Returns _X * 2^_Exp
	/// </returns>
	inline float ldexp(float _X, int _Exp) __GPU_ONLY
	{
		return __dp_math_ldexpf(_X, _Exp);
	}

	/// <summary>
	///     Computes a real number from the mantissa and exponent
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, mantissa
	/// </param>
	/// <param name="_Exp">
	///     Integer value, exponent
	/// </param>
	/// <returns>
	///     Returns _X * 2^_Exp
	/// </returns>
	inline double ldexp(double _X, int _Exp) __GPU_ONLY
	{
		return __dp_math_ldexp(_X, _Exp);
	}

	/// <summary>
	///     Computes the natural logarithm of the absolute value of gamma of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Sign">
	///     Returns the sign
	/// </param>
	/// <returns>
	///     Returns the natural logarithm of the absolute value of gamma of the argument
	/// </returns>
	inline float lgammaf(float _X, _Out_ int * _Sign) __GPU_ONLY
	{
		return __dp_math_lgammaf(_X, _Sign);
	}

	/// <summary>
	///     Computes the natural logarithm of the absolute value of gamma of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Sign">
	///     Returns the sign
	/// </param>
	/// <returns>
	///     Returns the natural logarithm of the absolute value of gamma of the argument
	/// </returns>
	inline float lgamma(float _X, _Out_ int * _Sign) __GPU_ONLY
	{
		return __dp_math_lgammaf(_X, _Sign);
	}

	/// <summary>
	///     Computes the natural logarithm of the absolute value of gamma of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Sign">
	///     Returns the sign
	/// </param>
	/// <returns>
	///     Returns the natural logarithm of the absolute value of gamma of the argument
	/// </returns>
	inline double lgamma(double _X, _Out_ int * _Sign) __GPU_ONLY
	{
		return __dp_math_lgamma(_X, _Sign);
	}

	/// <summary>
	///     Calculates the base-e logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of the argument
	/// </returns>
	inline float logf(float _X) __GPU_ONLY
	{
		return __dp_math_logf(_X);
	}

	/// <summary>
	///     Calculates the base-e logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of the argument
	/// </returns>
	inline float log(float _X) __GPU_ONLY
	{
		return __dp_math_logf(_X);
	}

	/// <summary>
	///     Calculates the base-e logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of the argument
	/// </returns>
	inline double log(double _X) __GPU_ONLY
	{
		return __dp_math_log(_X);
	}

	/// <summary>
	///     Calculates the base-10 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline float log10f(float _X) __GPU_ONLY
	{
		return __dp_math_log10f(_X);
	}

	/// <summary>
	///     Calculates the base-10 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline float log10(float _X) __GPU_ONLY
	{
		return __dp_math_log10f(_X);
	}

	/// <summary>
	///     Calculates the base-10 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline double log10(double _X) __GPU_ONLY
	{
		return __dp_math_log10(_X);
	}

	/// <summary>
	///     Calculates the base-2 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline float log2f(float _X) __GPU_ONLY
	{
		return __dp_math_log2f(_X);
	}

	/// <summary>
	///     Calculates the base-2 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline float log2(float _X) __GPU_ONLY
	{
		return __dp_math_log2f(_X);
	}

	/// <summary>
	///     Calculates the base-2 logarithm of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-10 logarithm of the argument
	/// </returns>
	inline double log2(double _X) __GPU_ONLY
	{
		return __dp_math_log2(_X);
	}

	/// <summary>
	///     Calculates the base-e logarithm of 1 plus the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of 1 plus the argument
	/// </returns>
	inline float log1pf(float _X) __GPU_ONLY
	{
		return __dp_math_log1pf(_X);
	}

	/// <summary>
	///     Calculates the base-e logarithm of 1 plus the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of 1 plus the argument
	/// </returns>
	inline float log1p(float _X) __GPU_ONLY
	{
		return __dp_math_log1pf(_X);
	}

	/// <summary>
	///     Calculates the base-e logarithm of 1 plus the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the base-e logarithm of 1 plus the argument
	/// </returns>
	inline double log1p(double _X) __GPU_ONLY
	{
		return __dp_math_log1p(_X);
	}

	/// <summary>
	///     Extracts the exponent of _X, as a signed integer value in floating-point format
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed exponent of _X
	/// </returns>
	inline float logbf(float _X) __GPU_ONLY
	{
		return __dp_math_logbf(_X);
	}

	/// <summary>
	///     Extracts the exponent of _X, as a signed integer value in floating-point format
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed exponent of _X
	/// </returns>
	inline float logb(float _X) __GPU_ONLY
	{
		return __dp_math_logbf(_X);
	}

	/// <summary>
	///     Extracts the exponent of _X, as a signed integer value in floating-point format
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed exponent of _X
	/// </returns>
	inline double logb(double _X) __GPU_ONLY
	{
		return __dp_math_logb(_X);
	}

	/// <summary>
	///     Splits _X into fractional and integer parts.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Iptr">
	///     Returns the integer portion of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed fractional portion of _X
	/// </returns>
	inline float modff(float _X, _Out_ float * _Iptr) __GPU_ONLY
	{
		return __dp_math_modff(_X, _Iptr);
	}

	/// <summary>
	///     Splits _X into fractional and integer parts.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Iptr">
	///     Returns the integer portion of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed fractional portion of _X
	/// </returns>
	inline float modf(float _X, _Out_ float * _Iptr) __GPU_ONLY
	{
		return __dp_math_modff(_X, _Iptr);
	}

	/// <summary>
	///     Splits _X into fractional and integer parts.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Iptr">
	///     Returns the integer portion of _X in floating-point value
	/// </param>
	/// <returns>
	///     Returns the signed fractional portion of _X
	/// </returns>
	inline double modf(double _X, _Out_ double * _Iptr) __GPU_ONLY
	{
		return __dp_math_modf(_X, _Iptr);
	}

	/// <summary>
	///     Returns a quiet NaN
	/// </summary>
	/// <param name="_X">
	///     Integer value
	/// </param>
	/// <returns>
	///     Returns a quiet NaN, if available, with the content indicated in _X
	/// </returns>
	inline float nanf(int _X) __GPU_ONLY
	{
		return __dp_math_nanf(_X);
	}

	/// <summary>
	///     Returns a quiet NaN
	/// </summary>
	/// <param name="_X">
	///     Integer value
	/// </param>
	/// <returns>
	///     Returns a quiet NaN, if available, with the content indicated in _X
	/// </returns>
	inline double nan(int _X) __GPU_ONLY
	{
		return __dp_math_nan(_X);
	}


	/// <summary>
	///     Rounds the argument to an integer value in
	///     floating-point format, using the current rounding direction.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the rounded integer value.
	/// </returns>
	inline float nearbyintf(float _X) __GPU_ONLY
	{
		return __dp_math_nearbyintf(_X);
	}

	/// <summary>
	///     Rounds the argument to an integer value in
	///     floating-point format, using the current rounding direction.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the rounded integer value.
	/// </returns>
	inline float nearbyint(float _X) __GPU_ONLY
	{
		return __dp_math_nearbyintf(_X);
	}

	/// <summary>
	///     Rounds the argument to an integer value in
	///     floating-point format, using the current rounding direction.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the rounded integer value.
	/// </returns>
	inline double nearbyint(double _X) __GPU_ONLY
	{
		return __dp_math_nearbyint(_X);
	}


	/// <summary>
	///     Determine the next representable value, in the type of the function,
	///     after _X in the direction of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the next representable value, in the type of the function,
	///     after _X in the direction of _Y
	/// </returns>
	inline float nextafterf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_nextafterf(_X, _Y);
	}

	/// <summary>
	///     Determine the next representable value, in the type of the function,
	///     after _X in the direction of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the next representable value, in the type of the function,
	///     after _X in the direction of _Y
	/// </returns>
	inline float nextafter(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_nextafterf(_X, _Y);
	}

	/// <summary>
	///     Determine the next representable value, in the type of the function,
	///     after _X in the direction of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the next representable value, in the type of the function,
	///     after _X in the direction of _Y
	/// </returns>
	inline double nextafter(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_nextafter(_X, _Y);
	}

	/// <summary>
	///     Returns the cumulative distribution function of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cumulative distribution function of the argument
	/// </returns>
	inline float phif(float _X) __GPU_ONLY
	{
		return __dp_math_phif(_X);
	}

	/// <summary>
	///     Returns the cumulative distribution function of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cumulative distribution function of the argument
	/// </returns>
	inline float phi(float _X) __GPU_ONLY
	{
		return __dp_math_phif(_X);
	}

	/// <summary>
	///     Returns the cumulative distribution function of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the cumulative distribution function of the argument
	/// </returns>
	inline double phi(double _X) __GPU_ONLY
	{
		return __dp_math_phi(_X);
	}

	/// <summary>
	///     Calculates _X raised to the power of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, base
	/// </param>
	/// <param name="_Y">
	///     Floating-point value, exponent
	/// </param>
	/// <returns>
	///     Returns the value of _X raised to the power of _Y
	/// </returns>
	inline float powf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_powf(_X, _Y);
	}

	/// <summary>
	///     Calculates _X raised to the power of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, base
	/// </param>
	/// <param name="_Y">
	///     Floating-point value, exponent
	/// </param>
	/// <returns>
	///     Returns the value of _X raised to the power of _Y
	/// </returns>
	inline float pow(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_powf(_X, _Y);
	}

	/// <summary>
	///     Calculates _X raised to the power of _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value, base
	/// </param>
	/// <param name="_Y">
	///     Floating-point value, exponent
	/// </param>
	/// <returns>
	///     Returns the value of _X raised to the power of _Y
	/// </returns>
	inline double pow(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_pow(_X, _Y);
	}


	/// <summary>
	///     Returns the inverse cumulative distribution function of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse cumulative distribution function of the argument
	/// </returns>
	inline float probitf(float _X) __GPU_ONLY
	{
		return __dp_math_probitf(_X);
	}

	/// <summary>
	///     Returns the inverse cumulative distribution function of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse cumulative distribution function of the argument
	/// </returns>
	inline float probit(float _X) __GPU_ONLY
	{
		return __dp_math_probitf(_X);
	}

	/// <summary>
	///     Returns the inverse cumulative distribution function of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the inverse cumulative distribution function of the argument
	/// </returns>
	inline double probit(double _X) __GPU_ONLY
	{
		return __dp_math_probit(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the cube root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the cube root of the argument
	/// </returns>
	inline float rcbrtf(float _X) __GPU_ONLY
	{
		return __dp_math_rcbrtf(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the cube root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the cube root of the argument
	/// </returns>
	inline float rcbrt(float _X) __GPU_ONLY
	{
		return __dp_math_rcbrtf(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the cube root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the cube root of the argument
	/// </returns>
	inline double rcbrt(double _X) __GPU_ONLY
	{
		return __dp_math_rcbrt(_X);
	}

	/// <summary>
	///     Computes the remainder: _X REM _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X REM _Y
	/// </returns>
	inline float remainderf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_remainderf(_X, _Y);
	}

	/// <summary>
	///     Computes the remainder: _X REM _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X REM _Y
	/// </returns>
	inline float remainder(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_remainderf(_X, _Y);
	}

	/// <summary>
	///     Computes the remainder: _X REM _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X REM _Y
	/// </returns>
	inline double remainder(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_remainder(_X, _Y);
	}

	/// <summary>
	///     Computes the same remainder as _X REM _Y. Also calculates
	///     the lower 24 bits of the integral quotient _X/_Y, and
	///     gives that value the same sign as _X/_Y. It stores this
	///     signed value in the integer pointed to by _Quo.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_Quo">
	///     Pointer to an integer value
	/// </param>
	/// <returns>
	///     Returns the remainder
	/// </returns>
	inline float remquof(float _X, float _Y, _Out_ int * _Quo) __GPU_ONLY
	{
		return __dp_math_remquof(_X, _Y, _Quo);
	}

	/// <summary>
	///     Computes the same remainder as _X REM _Y. Also calculates
	///     the lower 24 bits of the integral quotient _X/_Y, and
	///     gives that value the same sign as _X/_Y. It stores this
	///     signed value in the integer pointed to by _Quo.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_Quo">
	///     Pointer to an integer value
	/// </param>
	/// <returns>
	///     Returns the remainder
	/// </returns>
	inline float remquo(float _X, float _Y, _Out_ int * _Quo) __GPU_ONLY
	{
		return __dp_math_remquof(_X, _Y, _Quo);
	}

	/// <summary>
	///     Computes the same remainder as _X REM _Y. Also calculates
	///     the lower 31 bits of the integral quotient _X/_Y, and
	///     gives that value the same sign as _X/_Y. It stores this
	///     signed value in the integer pointed to by _Quo.
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <param name="_Quo">
	///     Pointer to an integer value
	/// </param>
	/// <returns>
	///     Returns the remainder
	/// </returns>
	inline double remquo(double _X, double _Y, _Out_ int * _Quo) __GPU_ONLY
	{
		return __dp_math_remquo(_X, _Y, _Quo);
	}

	/// <summary>
	///     Rounds _X to the nearest integer
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the nearest integer of _X
	/// </returns>
	inline float roundf(float _X) __GPU_ONLY
	{
		return __dp_math_roundf(_X);
	}

	/// <summary>
	///     Rounds _X to the nearest integer
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the nearest integer of _X
	/// </returns>
	inline float round(float _X) __GPU_ONLY
	{
		return __dp_math_roundf(_X);
	}

	/// <summary>
	///     Rounds _X to the nearest integer
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the nearest integer of _X
	/// </returns>
	inline double round(double _X) __GPU_ONLY
	{
		return __dp_math_round(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the square root of the argument
	/// </returns>
	inline float rsqrtf(float _X) __GPU_ONLY
	{
		return __dp_math_rsqrtf(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the square root of the argument
	/// </returns>
	inline float rsqrt(float _X) __GPU_ONLY
	{
		return __dp_math_rsqrtf(_X);
	}

	/// <summary>
	///     Returns the reciprocal of the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the reciprocal of the square root of the argument
	/// </returns>
	inline double rsqrt(double _X) __GPU_ONLY
	{
		return __dp_math_rsqrt(_X);
	}

	/// <summary>
	///     Multiplies _X by FLT_RADIX to the power _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X * (FLT_RADIX ^ _Y)
	/// </returns>
	inline float scalbf(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_scalbf(_X, _Y);
	}

	/// <summary>
	///     Multiplies _X by FLT_RADIX to the power _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X * (FLT_RADIX ^ _Y)
	/// </returns>
	inline float scalb(float _X, float _Y) __GPU_ONLY
	{
		return __dp_math_scalbf(_X, _Y);
	}

	/// <summary>
	///     Multiplies _X by FLT_RADIX to the power _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns _X * (FLT_RADIX ^ _Y)
	/// </returns>
	inline double scalb(double _X, double _Y) __GPU_ONLY
	{
		return __dp_math_scalb(_X, _Y);
	}

	/// <summary>
	///     Multiplies _X by FLT_RADIX to the power _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Integer value
	/// </param>
	/// <returns>
	///     Returns _X * (FLT_RADIX ^ _Y)
	/// </returns>
	inline float scalbnf(float _X, int _Y) __GPU_ONLY
	{
		return __dp_math_scalbnf(_X, _Y);
	}

	/// <summary>
	///     Multiplies _X by FLT_RADIX to the power _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Integer value
	/// </param>
	/// <returns>
	///     Returns _X * (FLT_RADIX ^ _Y)
	/// </returns>
	inline float scalbn(float _X, int _Y) __GPU_ONLY
	{
		return __dp_math_scalbnf(_X, _Y);
	}

	/// <summary>
	///     Multiplies _X by FLT_RADIX to the power _Y
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_Y">
	///     Integer value
	/// </param>
	/// <returns>
	///     Returns _X * (FLT_RADIX ^ _Y)
	/// </returns>
	inline double scalbn(double _X, int _Y) __GPU_ONLY
	{
		return __dp_math_scalbn(_X, _Y);
	}

	/// <summary>
	///     Determines whether the sign of _X is negative
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the sign of _X
	///     is negative
	/// </returns>
	inline int signbitf(float _X) __GPU_ONLY
	{
		return __dp_math_signbitf(_X);
	}

	/// <summary>
	///     Determines whether the sign of _X is negative
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the sign of _X
	///     is negative
	/// </returns>
	inline int signbit(float _X) __GPU_ONLY
	{
		return __dp_math_signbitf(_X);
	}

	/// <summary>
	///     Determines whether the sign of _X is negative
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns a nonzero value if and only if the sign of _X
	///     is negative
	/// </returns>
	inline int signbit(double _X) __GPU_ONLY
	{
		return __dp_math_signbit(_X);
	}

	/// <summary>
	///     Calculates the sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of the argument
	/// </returns>
	inline float sinf(float _X) __GPU_ONLY
	{
		return __dp_math_sinf(_X);
	}

	/// <summary>
	///     Calculates the sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of the argument
	/// </returns>
	inline float sin(float _X) __GPU_ONLY
	{
		return __dp_math_sinf(_X);
	}

	/// <summary>
	///     Calculates the sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of the argument
	/// </returns>
	inline double sin(double _X) __GPU_ONLY
	{
		return __dp_math_sin(_X);
	}

	/// <summary>
	///     Calculates sine and cosine value of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_S">
	///     Returns the sine value of _X
	/// </param>
	/// <param name="_C">
	///     Returns the cosine value of _X
	/// </param>
	inline void sincosf(float _X, _Out_ float * _S, _Out_ float * _C) __GPU_ONLY
	{
		*_C = __dp_math_sincosf(_X, _S);
	}

	/// <summary>
	///     Calculates sine and cosine value of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_S">
	///     Returns the sine value of _X
	/// </param>
	/// <param name="_C">
	///     Returns the cosine value of _X
	/// </param>
	inline void sincos(float _X, _Out_ float * _S, _Out_ float * _C) __GPU_ONLY
	{
		*_C = __dp_math_sincosf(_X, _S);
	}

	/// <summary>
	///     Calculates sine and cosine value of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <param name="_S">
	///     Returns the sine value of _X
	/// </param>
	/// <param name="_C">
	///     Returns the cosine value of _X
	/// </param>
	inline void sincos(double _X, _Out_ double * _S, _Out_ double * _C) __GPU_ONLY
	{
		*_C = __dp_math_sincos(_X, _S);
	}

	/// <summary>
	///     Calculates the hyperbolic sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic sine value of the argument
	/// </returns>
	inline float sinhf(float _X) __GPU_ONLY
	{
		return __dp_math_sinhf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic sine value of the argument
	/// </returns>
	inline float sinh(float _X) __GPU_ONLY
	{
		return __dp_math_sinhf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic sine value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic sine value of the argument
	/// </returns>
	inline double sinh(double _X) __GPU_ONLY
	{
		return __dp_math_sinh(_X);
	}

	/// <summary>
	///     Calculates the sine value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of pi * _X
	/// </returns>
	inline float sinpif(float _X) __GPU_ONLY
	{
		return __dp_math_sinpif(_X);
	}

	/// <summary>
	///     Calculates the sine value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of pi * _X
	/// </returns>
	inline float sinpi(float _X) __GPU_ONLY
	{
		return __dp_math_sinpif(_X);
	}

	/// <summary>
	///     Calculates the sine value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the sine value of pi * _X
	/// </returns>
	inline double sinpi(double _X) __GPU_ONLY
	{
		return __dp_math_sinpi(_X);
	}

	/// <summary>
	///     Calculates the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the argument
	/// </returns>
	inline float sqrtf(float _X) __GPU_ONLY
	{
		return __dp_math_sqrtf(_X);
	}

	/// <summary>
	///     Calculates the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the argument
	/// </returns>
	inline float sqrt(float _X) __GPU_ONLY
	{
		return __dp_math_sqrtf(_X);
	}

	/// <summary>
	///     Calculates the square root of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the square root of the argument
	/// </returns>
	inline double sqrt(double _X) __GPU_ONLY
	{
		return __dp_math_sqrt(_X);
	}

	/// <summary>
	///     Computes the gamma function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the result of gamma function of _X
	/// </returns>
	inline float tgammaf(float _X) __GPU_ONLY
	{
		return __dp_math_tgammaf(_X);
	}

	/// <summary>
	///     Computes the gamma function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the result of gamma function of _X
	/// </returns>
	inline float tgamma(float _X) __GPU_ONLY
	{
		return __dp_math_tgammaf(_X);
	}

	/// <summary>
	///     Computes the gamma function of _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the result of gamma function of _X
	/// </returns>
	inline double tgamma(double _X) __GPU_ONLY
	{
		return __dp_math_tgamma(_X);
	}

	/// <summary>
	///     Calculates the tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of the argument
	/// </returns>
	inline float tanf(float _X) __GPU_ONLY
	{
		return __dp_math_tanf(_X);
	}

	/// <summary>
	///     Calculates the tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of the argument
	/// </returns>
	inline float tan(float _X) __GPU_ONLY
	{
		return __dp_math_tanf(_X);
	}

	/// <summary>
	///     Calculates the tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of the argument
	/// </returns>
	inline double tan(double _X) __GPU_ONLY
	{
		return __dp_math_tan(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic tangent value of the argument
	/// </returns>
	inline float tanhf(float _X) __GPU_ONLY
	{
		return __dp_math_tanhf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic tangent value of the argument
	/// </returns>
	inline float tanh(float _X) __GPU_ONLY
	{
		return __dp_math_tanhf(_X);
	}

	/// <summary>
	///     Calculates the hyperbolic tangent value of the argument
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the hyperbolic tangent value of the argument
	/// </returns>
	inline double tanh(double _X) __GPU_ONLY
	{
		return __dp_math_tanh(_X);
	}

	/// <summary>
	///     Calculates the tangent value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of pi * _X
	/// </returns>
	inline float tanpif(float _X) __GPU_ONLY
	{
		return __dp_math_tanpif(_X);
	}

	/// <summary>
	///     Calculates the tangent value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of pi * _X
	/// </returns>
	inline float tanpi(float _X) __GPU_ONLY
	{
		return __dp_math_tanpif(_X);
	}

	/// <summary>
	///     Calculates the tangent value of pi * _X
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the tangent value of pi * _X
	/// </returns>
	inline double tanpi(double _X) __GPU_ONLY
	{
		return __dp_math_tanpi(_X);
	}

	/// <summary>
	///     Truncates the argument to the integer component
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the integer component of the argument
	/// </returns>
	inline float truncf(float _X) __GPU_ONLY
	{
		return __dp_math_truncf(_X);
	}

	/// <summary>
	///     Truncates the argument to the integer component
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the integer component of the argument
	/// </returns>
	inline float trunc(float _X) __GPU_ONLY
	{
		return __dp_math_truncf(_X);
	}

	/// <summary>
	///     Truncates the argument to the integer component
	/// </summary>
	/// <param name="_X">
	///     Floating-point value
	/// </param>
	/// <returns>
	///     Returns the integer component of the argument
	/// </returns>
	inline double trunc(double _X) __GPU_ONLY
	{
		return __dp_math_trunc(_X);
	}

    //=============================================================================
    // Import CMATH C++ functions:
    //=============================================================================
    using std::acosf;
    using std::asinf;
    using std::atanf;
    using std::atan2f;
    using std::ceilf;
    using std::cosf;
    using std::coshf;
    using std::expf;
    using std::fabsf;
    using std::floorf;
    using std::fmodf;
    using std::frexpf;
    using std::hypotf;
    using std::ldexpf;
    using std::logf;
    using std::log10f;
    using std::modff;
    using std::powf;
    using std::sinf;
    using std::sinhf;
    using std::sqrtf;
    using std::tanf;
    using std::tanhf;

    using std::acos;
    using std::asin;
    using std::atan;
    using std::atan2;
    using std::ceil;
    using std::cos;
    using std::cosh;
    using std::exp;
    using std::fabs;
    using std::floor;
    using std::fmod;
    using std::frexp;
    using std::hypot;
    using std::ldexp;
    using std::log;
    using std::log10;
    using std::modf;
    using std::pow;
    using std::sin;
    using std::sinh;
    using std::sqrt;
    using std::tan;
    using std::tanh;

    using std::acosh;
    using std::acoshf;
    using std::asinh;
    using std::asinhf;
    using std::atanh;
    using std::atanhf;
    using std::cbrt;
    using std::cbrtf;
    using std::copysign;
    using std::copysignf;
    using std::erf;
    using std::erfc;
    using std::erfcf;
    using std::erff;
    using std::exp2;
    using std::exp2f;
    using std::expm1;
    using std::expm1f;
    using std::fdim;
    using std::fdimf;
    using std::fma;
    using std::fmaf;
    using std::fmax;
    using std::fmaxf;
    using std::fmin;
    using std::fminf;
    using std::ilogb;
    using std::ilogbf;
    using std::log1p;
    using std::log1pf;
    using std::log2;
    using std::log2f;
    using std::logb;
    using std::logbf;
    using std::nearbyint;
    using std::nearbyintf;
    using std::nextafter;
    using std::nextafterf;
    using std::remainder;
    using std::remainderf;
    using std::remquo;
    using std::remquof;
    using std::round;
    using std::roundf;
    using std::scalbn;
    using std::scalbnf;
    using std::tgamma;
    using std::tgammaf;
    using std::trunc;
    using std::truncf;
} // namespace precise_math

} // namespace Concurrency
