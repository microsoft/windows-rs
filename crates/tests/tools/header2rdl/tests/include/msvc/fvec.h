/*
 *  Copyright (C) 1985-2015 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/*
 *  Definition of a C++ class interface to Streaming SIMD Extension intrinsics.
 *
 *
 *  File name : fvec.h  Fvec class definitions
 *
 *  Concept: A C++ abstraction of Streaming SIMD Extensions designed to improve
 *
 *  programmer productivity.  Speed and accuracy are sacrificed for utility.
 *
 *  Facilitates an easy transition to compiler intrinsics
 *
 *  or assembly language.
 *
 *  F32vec4:    4 packed single precision
 *              32-bit floating point numbers
*/

#ifndef _FVEC_H_INCLUDED
#define _FVEC_H_INCLUDED
#include <vcruntime.h>
#if _VCRT_COMPILER_PREPROCESSOR

#if !defined __cplusplus
    #error ERROR: This file is only supported in C++ compilations!
#endif  /* !defined __cplusplus */

#if defined (_M_CEE_PURE)
    #error ERROR: This file is not supported in the pure mode!
#else  /* defined (_M_CEE_PURE) */

#include <intrin.h> /* SSE Intrinsic function definition include file */
#include <ivec.h>

#ifndef _VEC_ASSERT
#ifdef NDEBUG
        #define _VEC_ASSERT(_Expression) ((void)0)
#else  /* NDEBUG */
#ifdef __cplusplus
            extern "C" {
#endif  /* __cplusplus */

        _ACRTIMP void __cdecl _wassert(_In_z_ const wchar_t * _Message, _In_z_ const wchar_t * _File, _In_ unsigned _Line);

#ifdef __cplusplus
            }
#endif  /* __cplusplus */

        #define _VEC_ASSERT(_Expression) (void)( (!!(_Expression)) || (_wassert(_CRT_WIDE(#_Expression), _CRT_WIDE(__FILE__), __LINE__), 0) )
#endif  /* NDEBUG */
#endif  /* _VEC_ASSERT */

/* Define _ENABLE_VEC_DEBUG to enable std::ostream inserters for debug output */
#if defined (_ENABLE_VEC_DEBUG)
    #include <iostream>
#endif  /* defined (_ENABLE_VEC_DEBUG) */

#pragma pack(push,16) /* Must ensure class & union 16-B aligned */

const union
{
    int i[4];
    __m128 m;
} __f32vec4_abs_mask_cheat = {0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff};

#define _f32vec4_abs_mask ((F32vec4)__f32vec4_abs_mask_cheat.m)

class F32vec4
{
protected:
     __m128 vec;
public:

    /* Constructors: __m128, 4 floats, 1 float */
    F32vec4() {}

    /* initialize 4 SP FP with __m128 data type */
    F32vec4(__m128 _M)                   { vec = _M;}

    /* initialize 4 SP FPs with 4 floats */
    F32vec4(float _F3, float _F2, float _F1, float _F0)     { vec= _mm_set_ps(_F3,_F2,_F1,_F0); }

    /* Explicitly initialize each of 4 SP FPs with same float */
    explicit F32vec4(float _F)   { vec = _mm_set_ps1(_F); }

    /* Explicitly initialize each of 4 SP FPs with same double */
    explicit F32vec4(double _D)  { vec = _mm_set_ps1((float)_D); }

    /* Assignment operations */

    F32vec4& operator =(float _F) { vec = _mm_set_ps1(_F); return *this; }

    F32vec4& operator =(double _D)
    {
        vec = _mm_set_ps1((float)_D);
        return *this;
    }

    /* Conversion functions */
    operator  __m128() const    { return vec; }     /* Convert to __m128 */

    /* Logical Operators */
    friend F32vec4 operator &(const F32vec4 &_A, const F32vec4 &_B) { return _mm_and_ps(_A,_B); }
    friend F32vec4 operator |(const F32vec4 &_A, const F32vec4 &_B) { return _mm_or_ps(_A,_B); }
    friend F32vec4 operator ^(const F32vec4 &_A, const F32vec4 &_B) { return _mm_xor_ps(_A,_B); }

    /* Arithmetic Operators */
    friend F32vec4 operator +(const F32vec4 &_A, const F32vec4 &_B) { return _mm_add_ps(_A,_B); }
    friend F32vec4 operator -(const F32vec4 &_A, const F32vec4 &_B) { return _mm_sub_ps(_A,_B); }
    friend F32vec4 operator *(const F32vec4 &_A, const F32vec4 &_B) { return _mm_mul_ps(_A,_B); }
    friend F32vec4 operator /(const F32vec4 &_A, const F32vec4 &_B) { return _mm_div_ps(_A,_B); }

    F32vec4& operator =(const F32vec4 &_A) { vec = _A.vec; return *this; }
    F32vec4& operator =(const __m128 &_Avec) { vec = _Avec; return *this; }
    F32vec4& operator +=(const F32vec4 &_A) { return *this = _mm_add_ps(vec,_A); }
    F32vec4& operator -=(const F32vec4 &_A) { return *this = _mm_sub_ps(vec,_A); }
    F32vec4& operator *=(const F32vec4 &_A) { return *this = _mm_mul_ps(vec,_A); }
    F32vec4& operator /=(const F32vec4 &_A) { return *this = _mm_div_ps(vec,_A); }
    F32vec4& operator &=(const F32vec4 &_A) { return *this = _mm_and_ps(vec,_A); }
    F32vec4& operator |=(const F32vec4 &_A) { return *this = _mm_or_ps(vec,_A); }
    F32vec4& operator ^=(const F32vec4 &_A) { return *this = _mm_xor_ps(vec,_A); }

    /* Horizontal Add */
    friend float add_horizontal(const F32vec4 &_A)
    {
        F32vec4 _Ftemp = _mm_add_ps(_A, _mm_movehl_ps(_A, _A));
        _Ftemp = _mm_add_ss(_Ftemp, _mm_shuffle_ps(_Ftemp, _Ftemp, 1));
        return _mm_cvtss_f32(_Ftemp);
    }

    /* Square Root */
    friend F32vec4 sqrt(const F32vec4 &_A)       { return _mm_sqrt_ps(_A); }
    /* Reciprocal */
    friend F32vec4 rcp(const F32vec4 &_A)        { return _mm_rcp_ps(_A); }
    /* Reciprocal Square Root */
    friend F32vec4 rsqrt(const F32vec4 &_A)      { return _mm_rsqrt_ps(_A); }

    /* NewtonRaphson Reciprocal
       [2 * rcpps(x) - (x * rcpps(x) * rcpps(x))] */
    friend F32vec4 rcp_nr(const F32vec4 &_A)
    {
        F32vec4 _Ra0 = _mm_rcp_ps(_A);
        return _mm_sub_ps(_mm_add_ps(_Ra0, _Ra0), _mm_mul_ps(_mm_mul_ps(_Ra0, _A), _Ra0));
    }

    /*  NewtonRaphson Reciprocal Square Root
        0.5 * rsqrtps * (3 - x * rsqrtps(x) * rsqrtps(x)) */
#pragma warning(push)
#pragma warning(disable : 4640)
    friend F32vec4 rsqrt_nr(const F32vec4 &_A)
    {
        static const F32vec4 fvecf0pt5(0.5f);
        static const F32vec4 fvecf3pt0(3.0f);
        F32vec4 _Ra0 = _mm_rsqrt_ps(_A);
        return (fvecf0pt5 * _Ra0) * (fvecf3pt0 - (_A * _Ra0) * _Ra0);
    }
#pragma warning(pop)

    /* Compares: Mask is returned  */
    /* Macros expand to all compare intrinsics.  Example:
    friend F32vec4 cmpeq(const F32vec4 &_A, const F32vec4 &_B)
    { return _mm_cmpeq_ps(_A,_B);} */
    #define Fvec32s4_COMP(op) \
    friend F32vec4 cmp##op (const F32vec4 &_A, const F32vec4 &_B) { return _mm_cmp##op##_ps(_A,_B); }
        Fvec32s4_COMP(eq)                   /* expanded to cmpeq(_A,_B) */
        Fvec32s4_COMP(lt)                   /* expanded to cmplt(_A,_B) */
        Fvec32s4_COMP(le)                   /* expanded to cmple(_A,_B) */
        Fvec32s4_COMP(gt)                   /* expanded to cmpgt(_A,_B) */
        Fvec32s4_COMP(ge)                   /* expanded to cmpge(_A,_B) */
        Fvec32s4_COMP(neq)                  /* expanded to cmpneq(_A,_B) */
        Fvec32s4_COMP(nlt)                  /* expanded to cmpnlt(_A,_B) */
        Fvec32s4_COMP(nle)                  /* expanded to cmpnle(_A,_B) */
        Fvec32s4_COMP(ngt)                  /* expanded to cmpngt(_A,_B) */
        Fvec32s4_COMP(nge)                  /* expanded to cmpnge(_A,_B) */
    #undef Fvec32s4_COMP

    /* Min and Max */
    friend F32vec4 simd_min(const F32vec4 &_A, const F32vec4 &_B) { return _mm_min_ps(_A,_B); }
    friend F32vec4 simd_max(const F32vec4 &_A, const F32vec4 &_B) { return _mm_max_ps(_A,_B); }

    /* Absolute value */
    friend F32vec4 abs(const F32vec4 &_A) {return _mm_and_ps(_A, _f32vec4_abs_mask); }

    /* Debug Features */
#if defined (_ENABLE_VEC_DEBUG)
    /* Output */
    friend std::ostream & operator<<(std::ostream & _Os, const F32vec4 &_A)
    {
    /* To use: cout << "Elements of F32vec4 fvec are: " << fvec; */
      float *_Fp = (float*)&_A;
      _Os << "[3]:" << *(_Fp+3)
            << " [2]:" << *(_Fp+2)
            << " [1]:" << *(_Fp+1)
            << " [0]:" << *_Fp;
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */
    /* Element Access Only, no modifications to elements*/
    const float& operator[](int _I) const
    {
        /* Assert enabled only during debug /DDEBUG */
        _VEC_ASSERT((0 <= _I) && (_I <= 3));          /* User should only access elements 0-3 */
        float *_Fp = (float*)&vec;
        return *(_Fp+ _I);
    }
    /* Element Access and Modification*/
    float& operator[](int _I)
    {
        /* Assert enabled only during debug /DDEBUG */
        _VEC_ASSERT((0 <= _I) && (_I <= 3));          /* User should only access elements 0-3 */
        float *_Fp = (float*)&vec;
        return *(_Fp+ _I);
    }
};

                        /* Miscellaneous */

/* Interleave low order data elements of a and b into destination */
inline F32vec4 unpack_low(const F32vec4 &_A, const F32vec4 &_B)
{ return _mm_unpacklo_ps(_A, _B); }

/* Interleave high order data elements of a and b into target */
inline F32vec4 unpack_high(const F32vec4 &_A, const F32vec4 &_B)
{ return _mm_unpackhi_ps(_A, _B); }

/* Move Mask to Integer returns 4 bit mask formed of most significant bits of a */
inline int move_mask(const F32vec4 &_A)
{ return _mm_movemask_ps(_A);}

                        /* Data Motion Functions */

/* Load Unaligned loadu_ps: Unaligned */
inline void loadu(F32vec4 &_A, float *_P)
{ _A = _mm_loadu_ps(_P); }

/* Store Temporal storeu_ps: Unaligned */
inline void storeu(float *_P, const F32vec4 &_A)
{ _mm_storeu_ps(_P, _A); }

                        /* Cacheability Support */

/* Non-Temporal Store */
inline void store_nta(float *_P, const F32vec4 &_A)
{ _mm_stream_ps(_P,_A);}

                        /* Conditional Selects:*/
/*(a OP b)? c : d; where OP is any compare operator
Macros expand to conditional selects which use all compare intrinsics.
Example:
friend F32vec4 select_eq(const F32vec4 &_A, const F32vec4 &_B, const F32vec4 &_C, const F32vec4 &_D)
{
    F32vec4 _Mask = _mm_cmpeq_ps(_A,_B);
    return( (_Mask & _C) | F32vec4((_mm_andnot_ps(_Mask,_D))));
}
*/

#define Fvec32s4_SELECT(op) \
inline F32vec4 select_##op (const F32vec4 &_A, const F32vec4 &_B, const F32vec4 &_C, const F32vec4 &_D)        \
{                                                               \
    F32vec4 _Mask = _mm_cmp##op##_ps(_A,_B);                       \
    return( (_Mask & _C) | F32vec4((_mm_andnot_ps(_Mask,_D)))); \
}
Fvec32s4_SELECT(eq)         /* generates select_eq(_A,_B) */
Fvec32s4_SELECT(lt)         /* generates select_lt(_A,_B) */
Fvec32s4_SELECT(le)         /* generates select_le(_A,_B) */
Fvec32s4_SELECT(gt)         /* generates select_gt(_A,_B) */
Fvec32s4_SELECT(ge)         /* generates select_ge(_A,_B) */
Fvec32s4_SELECT(neq)        /* generates select_neq(_A,_B) */
Fvec32s4_SELECT(nlt)        /* generates select_nlt(_A,_B) */
Fvec32s4_SELECT(nle)        /* generates select_nle(_A,_B) */
Fvec32s4_SELECT(ngt)        /* generates select_ngt(_A,_B) */
Fvec32s4_SELECT(nge)        /* generates select_nge(_A,_B) */
#undef Fvec32s4_SELECT

/* Streaming SIMD Extensions Integer Intrinsic Functions */

#if defined(_M_IX86)
/* Max and Min */
inline Is16vec4 simd_max(const Is16vec4 &_A, const Is16vec4 &_B)      { return _m_pmaxsw(_A,_B);}
inline Is16vec4 simd_min(const Is16vec4 &_A, const Is16vec4 &_B)      { return _m_pminsw(_A,_B);}
inline Iu8vec8 simd_max(const Iu8vec8 &_A, const Iu8vec8 &_B)         { return _m_pmaxub(_A,_B);}
inline Iu8vec8 simd_min(const Iu8vec8 &_A, const Iu8vec8 &_B)         { return _m_pminub(_A,_B);}

/* Average */
inline Iu16vec4 simd_avg(const Iu16vec4 &_A, const Iu16vec4 &_B)      { return _mm_avg_pu16(_A,_B); }
inline Iu8vec8 simd_avg(const Iu8vec8 &_A, const Iu8vec8 &_B)         { return _mm_avg_pu8(_A,_B); }

/* Move ByteMask To Int: returns mask formed from most sig bits of each vec of a */
inline int move_mask(const I8vec8 &_A)                               { return _m_pmovmskb(_A);}

/* Packed Multiply High Unsigned */
inline Iu16vec4 mul_high(const Iu16vec4 &_A, const Iu16vec4 &_B)      { return _m_pmulhuw(_A,_B); }

/* Byte Mask Write: Write bytes if most significant bit in each corresponding byte is set */
inline void mask_move(const I8vec8 &_A, const I8vec8 &_B, char *_Addr) { _m_maskmovq(_A, _B, _Addr); }

/* Data Motion: Store Non Temporal */
inline void store_nta(__m64 *_P, const M64 &_A) { _mm_stream_pi(_P,_A); }

/* Conversions between ivec <-> fvec */

/* Convert first element of F32vec4 to int with truncation */
inline int F32vec4ToInt(const F32vec4 &_A)
{

    return _mm_cvtt_ss2si(_A);

}

/* Convert two lower SP FP values of a to Is32vec2 with truncation */
inline Is32vec2 F32vec4ToIs32vec2 (const F32vec4 &_A)
{

    __m64 _Result;
    _Result = _mm_cvtt_ps2pi(_A);
    return Is32vec2(_Result);

}

/* Convert the 32-bit int i to an SP FP value; the upper three SP FP values are passed through from a. */
inline F32vec4 IntToF32vec4(const F32vec4 &_A, int _I)
{

    __m128 _Result;
    _Result = _mm_cvt_si2ss(_A, _I);
    return F32vec4(_Result);

}

/* Convert the two 32-bit integer values in b to two SP FP values; the upper two SP FP values are passed from a. */
inline F32vec4 Is32vec2ToF32vec4(const F32vec4 &_A, const Is32vec2 &_B)
{

    __m128 _Result;
    _Result = _mm_cvt_pi2ps(_A,_B);
    return F32vec4(_Result);
}
#endif

class F32vec1
{
protected:
     __m128 vec;
public:

    /* Constructors: 1 float */
    F32vec1() {}

    F32vec1(int _I)      { vec = _mm_cvt_si2ss(vec, _I);};

    /* Initialize each of 4 SP FPs with same float */
    explicit F32vec1(float _F)   { vec = _mm_set_ss(_F); }

    /* Initialize each of 4 SP FPs with same float */
    explicit F32vec1(double _D)  { vec = _mm_set_ss((float) _D); }

    /* initialize with __m128 data type */
    F32vec1(__m128 _M)   { vec = _M; }

    /* Conversion functions */
    operator  __m128() const    { return vec; }     /* Convert to float */

    /* Logical Operators */
    friend F32vec1 operator &(const F32vec1 &_A, const F32vec1 &_B) { return _mm_and_ps(_A,_B); }
    friend F32vec1 operator |(const F32vec1 &_A, const F32vec1 &_B) { return _mm_or_ps(_A,_B); }
    friend F32vec1 operator ^(const F32vec1 &_A, const F32vec1 &_B) { return _mm_xor_ps(_A,_B); }

    /* Arithmetic Operators */
    friend F32vec1 operator +(const F32vec1 &_A, const F32vec1 &_B) { return _mm_add_ss(_A,_B); }
    friend F32vec1 operator -(const F32vec1 &_A, const F32vec1 &_B) { return _mm_sub_ss(_A,_B); }
    friend F32vec1 operator *(const F32vec1 &_A, const F32vec1 &_B) { return _mm_mul_ss(_A,_B); }
    friend F32vec1 operator /(const F32vec1 &_A, const F32vec1 &_B) { return _mm_div_ss(_A,_B); }

    F32vec1& operator +=(const F32vec1 &_A) { return *this = _mm_add_ss(vec,_A); }
    F32vec1& operator -=(const F32vec1 &_A) { return *this = _mm_sub_ss(vec,_A); }
    F32vec1& operator *=(const F32vec1 &_A) { return *this = _mm_mul_ss(vec,_A); }
    F32vec1& operator /=(const F32vec1 &_A) { return *this = _mm_div_ss(vec,_A); }
    F32vec1& operator &=(const F32vec1 &_A) { return *this = _mm_and_ps(vec,_A); }
    F32vec1& operator |=(const F32vec1 &_A) { return *this = _mm_or_ps(vec,_A); }
    F32vec1& operator ^=(const F32vec1 &_A) { return *this = _mm_xor_ps(vec,_A); }


    /* Square Root */
    friend F32vec1 sqrt(const F32vec1 &_A)       { return _mm_sqrt_ss(_A); }
    /* Reciprocal */
    friend F32vec1 rcp(const F32vec1 &_A)        { return _mm_rcp_ss(_A); }
    /* Reciprocal Square Root */
    friend F32vec1 rsqrt(const F32vec1 &_A)      { return _mm_rsqrt_ss(_A); }

    /* NewtonRaphson Reciprocal
       [2 * rcpss(x) - (x * rcpss(x) * rcpss(x))] */
    friend F32vec1 rcp_nr(const F32vec1 &_A)
    {
        F32vec1 _Ra0 = _mm_rcp_ss(_A);
        return _mm_sub_ss(_mm_add_ss(_Ra0, _Ra0), _mm_mul_ss(_mm_mul_ss(_Ra0, _A), _Ra0));
    }

    /*  NewtonRaphson Reciprocal Square Root
        0.5 * rsqrtss * (3 - x * rsqrtss(x) * rsqrtss(x)) */
#pragma warning(push)
#pragma warning(disable : 4640)
    friend F32vec1 rsqrt_nr(const F32vec1 &_A)
    {
        static const F32vec1 fvecf0pt5(0.5f);
        static const F32vec1 fvecf3pt0(3.0f);
        F32vec1 _Ra0 = _mm_rsqrt_ss(_A);
        return (fvecf0pt5 * _Ra0) * (fvecf3pt0 - (_A * _Ra0) * _Ra0);
    }
#pragma warning(pop)

    /* Compares: Mask is returned  */
    /* Macros expand to all compare intrinsics.  Example:
    friend F32vec1 cmpeq(const F32vec1 &_A, const F32vec1 &_B)
    { return _mm_cmpeq_ss(_A,_B);} */
    #define Fvec32s1_COMP(op) \
    friend F32vec1 cmp##op (const F32vec1 &_A, const F32vec1 &_B) { return _mm_cmp##op##_ss(_A,_B); }
        Fvec32s1_COMP(eq)                   /* expanded to cmpeq(_A,_B) */
        Fvec32s1_COMP(lt)                   /* expanded to cmplt(_A,_B) */
        Fvec32s1_COMP(le)                   /* expanded to cmple(_A,_B) */
        Fvec32s1_COMP(gt)                   /* expanded to cmpgt(_A,_B) */
        Fvec32s1_COMP(ge)                   /* expanded to cmpge(_A,_B) */
        Fvec32s1_COMP(neq)                  /* expanded to cmpneq(_A,_B) */
        Fvec32s1_COMP(nlt)                  /* expanded to cmpnlt(_A,_B) */
        Fvec32s1_COMP(nle)                  /* expanded to cmpnle(_A,_B) */
        Fvec32s1_COMP(ngt)                  /* expanded to cmpngt(_A,_B) */
        Fvec32s1_COMP(nge)                  /* expanded to cmpnge(_A,_B) */
    #undef Fvec32s1_COMP

    /* Min and Max */
    friend F32vec1 simd_min(const F32vec1 &_A, const F32vec1 &_B) { return _mm_min_ss(_A,_B); }
    friend F32vec1 simd_max(const F32vec1 &_A, const F32vec1 &_B) { return _mm_max_ss(_A,_B); }

    /* Debug Features */
#if defined (_ENABLE_VEC_DEBUG)
    /* Output */
    friend std::ostream & operator<<(std::ostream & _Os, const F32vec1 &_A)
    {
    /* To use: cout << "Elements of F32vec1 fvec are: " << fvec; */
      float *_Fp = (float*)&_A;
	  _Os << "float:" << *_Fp;
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

};

                        /* Conditional Selects:*/
/*(_A OP _B)? _C : _D; where OP is any compare operator
Macros expand to conditional selects which use all compare intrinsics.
Example:
friend F32vec1 select_eq(const F32vec1 &_A, const F32vec1 &_B, const F32vec1 &_C, const F32vec1 &_D)
{
    F32vec1 _Mask = _mm_cmpeq_ss(_A,_B);
    return( (_Mask & _C) | F32vec1((_mm_andnot_ps(_Mask,_D))));
}
*/

#define Fvec32s1_SELECT(op) \
inline F32vec1 select_##op (const F32vec1 &_A, const F32vec1 &_B, const F32vec1 &_C, const F32vec1 &_D)        \
{                                                      \
    F32vec1 _Mask = _mm_cmp##op##_ss(_A,_B);                                          \
    return( (_Mask & _C) | F32vec1((_mm_andnot_ps(_Mask,_D))));                                            \
}
Fvec32s1_SELECT(eq)         /* generates select_eq(_A,_B) */
Fvec32s1_SELECT(lt)         /* generates select_lt(_A,_B) */
Fvec32s1_SELECT(le)         /* generates select_le(_A,_B) */
Fvec32s1_SELECT(gt)         /* generates select_gt(_A,_B) */
Fvec32s1_SELECT(ge)         /* generates select_ge(_A,_B) */
Fvec32s1_SELECT(neq)        /* generates select_neq(_A,_B) */
Fvec32s1_SELECT(nlt)        /* generates select_nlt(_A,_B) */
Fvec32s1_SELECT(nle)        /* generates select_nle(_A,_B) */
Fvec32s1_SELECT(ngt)        /* generates select_ngt(_A,_B) */
Fvec32s1_SELECT(nge)        /* generates select_nge(_A,_B) */
#undef Fvec32s1_SELECT

/* Conversions between ivec <-> fvec */

/* Convert F32vec1 to int */
inline int F32vec1ToInt(const F32vec1 &_A)
{
    return _mm_cvtt_ss2si(_A);
}



#pragma pack(pop) /* 16-B aligned */

#endif  /* defined (_M_CEE_PURE) */

#endif  /* _VCRT_COMPILER_PREPROCESSOR */
#endif  /* _FVEC_H_INCLUDED */
