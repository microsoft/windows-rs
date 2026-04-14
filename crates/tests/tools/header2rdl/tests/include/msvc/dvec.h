/*
 *  Copyright (C) 1985-2015 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/*
 *  Definition of a C++ class interface to Intel(R) Pentium(R) 4 processor SSE2 intrinsics.
 *
 *  File name : dvec.h  class definitions
 *
 *  Concept: A C++ abstraction of Intel(R) Pentium(R) 4 processor SSE2
 *      designed to improve programmer productivity.  Speed and accuracy are
 *      sacrificed for utility.  Facilitates an easy transition to compiler
 *      intrinsics or assembly language.
 *
 */

#ifndef _DVEC_H_INCLUDED
#define _DVEC_H_INCLUDED
#include <vcruntime.h>
#if _VCRT_COMPILER_PREPROCESSOR

#if !defined __cplusplus
    #error ERROR: This file is only supported in C++ compilations!
#endif  /* !defined __cplusplus */

#if defined (_M_CEE_PURE)
    #error ERROR: This file is not supported in the pure mode!
#else  /* defined (_M_CEE_PURE) */

#include <intrin.h> /* SSE2 intrinsic function definition include file */
#include <fvec.h>

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

#pragma pack(push,_CRT_PACKING)

/* Define _ENABLE_VEC_DEBUG to enable std::ostream inserters for debug output */
#if defined (_ENABLE_VEC_DEBUG)
    #include <iostream>
#endif  /* defined (_ENABLE_VEC_DEBUG) */

#pragma pack(push,16) /* Must ensure class & union 16-B aligned */

const union
{
    int i[4];
    __m128d m;
} __f64vec2_abs_mask_cheat = {-1, 0x7fffffff, -1, 0x7fffffff};

#define _f64vec2_abs_mask ((F64vec2)__f64vec2_abs_mask_cheat.m)

/* EMM Functionality Intrinsics */

class I8vec16;          /* 16 elements, each element a signed or unsigned char data type */
class Is8vec16;         /* 16 elements, each element a signed char data type */
class Iu8vec16;         /* 16 elements, each element an unsigned char data type */
class I16vec8;          /* 8 elements, each element a signed or unsigned short */
class Is16vec8;         /* 8 elements, each element a signed short */
class Iu16vec8;         /* 8 elements, each element an unsigned short */
class I32vec4;          /* 4 elements, each element a signed or unsigned long */
class Is32vec4;         /* 4 elements, each element a signed long */
class Iu32vec4;         /* 4 elements, each element a unsigned long */
class I64vec2;          /* 2 element, each a __m64 data type */
class I128vec1;         /* 1 element, a __m128i data type */

#define _MM_16UB(element,vector) (*((unsigned char*)&##vector + ##element))
#define _MM_16B(element,vector) (*((signed char*)&##vector + ##element))

#define _MM_8UW(element,vector) (*((unsigned short*)&##vector + ##element))
#define _MM_8W(element,vector) (*((short*)&##vector + ##element))

#define _MM_4UDW(element,vector) (*((unsigned int*)&##vector + ##element))
#define _MM_4DW(element,vector) (*((int*)&##vector + ##element))

#define _MM_2QW(element,vector) (*((__int64*)&##vector + ##element))


/* We need a m128i constant, keeping performance in mind*/

#pragma warning(push)
#pragma warning(disable : 4640)
inline const __m128i get_mask128()
{
    static const __m128i _Mask128 = _mm_set1_epi64x(0xffffffffffffffffi64);
    return _Mask128;
}
#pragma warning(pop)


//DEVDIV Remove alias created in public\sdk\inc\winnt.h
#ifdef M128
#undef M128
#endif  /* M128 */
#ifdef PM128
#undef PM128
#endif  /* PM128 */
//end DEVDIV

/* M128 Class:
 * 1 element, a __m128i data type
 * Contructors & Logical Operations
 */

class M128
{
protected:
        __m128i vec;

public:
    M128()                                  { }
    M128(__m128i _Mm)                        { vec = _Mm; }

    operator __m128i() const                    { return vec; }

    /* Logical Operations */
    M128& operator&=(const M128 &_A)                 { return *this = (M128) _mm_and_si128(vec,_A); }
    M128& operator|=(const M128 &_A)                 { return *this = (M128) _mm_or_si128(vec,_A); }
    M128& operator^=(const M128 &_A)                 { return *this = (M128) _mm_xor_si128(vec,_A); }

};

inline M128 operator&(const M128 &_A, const M128 &_B) { return _mm_and_si128(_A,_B); }
inline M128 operator|(const M128 &_A, const M128 &_B) { return _mm_or_si128(_A,_B); }
inline M128 operator^(const M128 &_A, const M128 &_B) { return _mm_xor_si128(_A,_B); }
inline M128 andnot(const M128 &_A, const M128 &_B)    { return _mm_andnot_si128(_A,_B); }

/* I128vec1 Class:
 * 1 element, a __m128i data type
 * Contains Operations which can operate on any __m6128i data type
 */

class I128vec1 : public M128
{
public:
    I128vec1()                              { }
    I128vec1(__m128i _Mm) : M128(_Mm)             { }

    I128vec1& operator= (const M128 &_A) { return *this = (I128vec1) _A; }
    I128vec1& operator&=(const M128 &_A) { return *this = (I128vec1) _mm_and_si128(vec,_A); }
    I128vec1& operator|=(const M128 &_A) { return *this = (I128vec1) _mm_or_si128(vec,_A); }
    I128vec1& operator^=(const M128 &_A) { return *this = (I128vec1) _mm_xor_si128(vec,_A); }

};

/* I64vec2 Class:
 * 2 elements, each element signed or unsigned 64-bit integer
 */
class I64vec2 : public M128
{
public:
    I64vec2() { }
    I64vec2(__m128i _Mm) : M128(_Mm) { }

    I64vec2(__m64 _Q1, __m64 _Q0)
    {
        _MM_2QW(0,vec) = *(__int64*)&_Q0;
        _MM_2QW(1,vec) = *(__int64*)&_Q1;
    }

    /* Assignment Operator */
    I64vec2& operator= (const M128 &_A) { return *this = (I64vec2) _A; }

    /* Logical Assignment Operators */
    I64vec2& operator&=(const M128 &_A) { return *this = (I64vec2) _mm_and_si128(vec,_A); }
    I64vec2& operator|=(const M128 &_A) { return *this = (I64vec2) _mm_or_si128(vec,_A); }
    I64vec2& operator^=(const M128 &_A) { return *this = (I64vec2) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    I64vec2& operator +=(const I64vec2 &_A)          { return *this = (I64vec2) _mm_add_epi64(vec,_A); }
    I64vec2& operator -=(const I64vec2 &_A)          { return *this = (I64vec2) _mm_sub_epi64(vec,_A); }

    /* Shift Logical Operators */
    I64vec2 operator<<(const I64vec2 &_A)            { return _mm_sll_epi64(vec,_A); }
    I64vec2 operator<<(int _Count)                   { return _mm_slli_epi64(vec,_Count); }
    I64vec2& operator<<=(const I64vec2 &_A)          { return *this = (I64vec2) _mm_sll_epi64(vec,_A); }
    I64vec2& operator<<=(int _Count)                 { return *this = (I64vec2) _mm_slli_epi64(vec,_Count); }
    I64vec2 operator>>(const I64vec2 &_A)            { return _mm_srl_epi64(vec,_A); }
    I64vec2 operator>>(int _Count)                   { return _mm_srli_epi64(vec,_Count); }
    I64vec2& operator>>=(const I64vec2 &_A)          { return *this = (I64vec2) _mm_srl_epi64(vec,_A); }
    I64vec2& operator>>=(int _Count)                 { return *this = (I64vec2) _mm_srli_epi64(vec,_Count); }

    /* Element Access for Debug, No data modified */
    const __int64& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 2);  /* Only 2 elements to access */
        return _MM_2QW(_I,vec);
    }

    /* Element Access and Assignment for Debug */
    __int64& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 2);  /* Only 2 elements to access */
        return _MM_2QW(_I,vec);
    }


};

/* Unpacks */
inline I64vec2 unpack_low(const I64vec2 &_A, const I64vec2 &_B)   {return _mm_unpacklo_epi64(_A,_B); }
inline I64vec2 unpack_high(const I64vec2 &_A, const I64vec2 &_B)  {return _mm_unpackhi_epi64(_A,_B); }

/* I32vec4 Class:
 * 4 elements, each element either a signed or unsigned int
 */
class I32vec4 : public M128
{
public:
    I32vec4() { }
    I32vec4(__m128i _Mm) : M128(_Mm) { }
    I32vec4(int _I3, int _I2, int _I1, int _I0) {vec = _mm_set_epi32(_I3, _I2, _I1, _I0);}

    /* Assignment Operator */
    I32vec4& operator= (const M128 &_A)              { return *this = (I32vec4) _A; }

    /* Logicals Operators */
    I32vec4& operator&=(const M128 &_A)              { return *this = (I32vec4) _mm_and_si128(vec,_A); }
    I32vec4& operator|=(const M128 &_A)              { return *this = (I32vec4) _mm_or_si128(vec,_A); }
    I32vec4& operator^=(const M128 &_A)              { return *this = (I32vec4) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    I32vec4& operator +=(const I32vec4 &_A)          { return *this = (I32vec4)_mm_add_epi32(vec,_A); }
    I32vec4& operator -=(const I32vec4 &_A)          { return *this = (I32vec4)_mm_sub_epi32(vec,_A); }

    /* Shift Logical Operators */
    I32vec4 operator<<(const I32vec4 &_A)            { return _mm_sll_epi32(vec,_A); }
    I32vec4 operator<<(int _Count)                   { return _mm_slli_epi32(vec,_Count); }
    I32vec4& operator<<=(const I32vec4 &_A)          { return *this = (I32vec4)_mm_sll_epi32(vec,_A); }
    I32vec4& operator<<=(int _Count)                 { return *this = (I32vec4)_mm_slli_epi32(vec,_Count); }

};

inline I32vec4 cmpeq(const I32vec4 &_A, const I32vec4 &_B)        { return _mm_cmpeq_epi32(_A,_B); }
inline I32vec4 cmpneq(const I32vec4 &_A, const I32vec4 &_B)       { return _mm_andnot_si128(_mm_cmpeq_epi32(_A,_B), get_mask128()); }

inline I32vec4 unpack_low(const I32vec4 &_A, const I32vec4 &_B)   { return _mm_unpacklo_epi32(_A,_B); }
inline I32vec4 unpack_high(const I32vec4 &_A, const I32vec4 &_B)  { return _mm_unpackhi_epi32(_A,_B); }

/* Is32vec4 Class:
 * 4 elements, each element signed integer
 */
class Is32vec4 : public I32vec4
{
public:
    Is32vec4() { }
    Is32vec4(__m128i _Mm) : I32vec4(_Mm) { }
    Is32vec4(int _I3, int _I2, int _I1, int _I0) : I32vec4(_I3, _I2, _I1, _I0){}

    /* Assignment Operator */
    Is32vec4& operator= (const M128 &_A)     { return *this = (Is32vec4) _A; }

    /* Logical Operators */
    Is32vec4& operator&=(const M128 &_A)     { return *this = (Is32vec4) _mm_and_si128(vec,_A); }
    Is32vec4& operator|=(const M128 &_A)     { return *this = (Is32vec4) _mm_or_si128(vec,_A); }
    Is32vec4& operator^=(const M128 &_A)     { return *this = (Is32vec4) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Is32vec4& operator +=(const I32vec4 &_A) { return *this = (Is32vec4)_mm_add_epi32(vec,_A); }
    Is32vec4& operator -=(const I32vec4 &_A) { return *this = (Is32vec4)_mm_sub_epi32(vec,_A); }

    /* Shift Logical Operators */
    Is32vec4 operator<<(const M128 &_A)      { return _mm_sll_epi32(vec,_A); }
    Is32vec4 operator<<(int _Count)          { return _mm_slli_epi32(vec,_Count); }
    Is32vec4& operator<<=(const M128 &_A)    { return *this = (Is32vec4)_mm_sll_epi32(vec,_A); }
    Is32vec4& operator<<=(int _Count)        { return *this = (Is32vec4)_mm_slli_epi32(vec,_Count); }
    /* Shift Arithmetic Operations */
    Is32vec4 operator>>(const M128 &_A)      { return _mm_sra_epi32(vec,_A); }
    Is32vec4 operator>>(int _Count)          { return _mm_srai_epi32(vec,_Count); }
    Is32vec4& operator>>=(const M128 &_A)    { return *this = (Is32vec4) _mm_sra_epi32(vec,_A); }
    Is32vec4& operator>>=(int _Count)        { return *this = (Is32vec4) _mm_srai_epi32(vec,_Count); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Is32vec4 &_A)
    {
        _Os << "[3]:" << _MM_4DW(3,_A)
            << " [2]:" << _MM_4DW(2,_A)
            << " [1]:" << _MM_4DW(1,_A)
            << " [0]:" << _MM_4DW(0,_A);
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const int& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4DW(_I,vec);
    }

    /* Element Access for Debug */
    int& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4DW(_I,vec);
    }
};

/* Compares */
inline Is32vec4 cmpeq(const Is32vec4 &_A, const Is32vec4 &_B)             { return _mm_cmpeq_epi32(_A,_B); }
inline Is32vec4 cmpneq(const Is32vec4 &_A, const Is32vec4 &_B)            { return _mm_andnot_si128(_mm_cmpeq_epi32(_A,_B), get_mask128()); }
inline Is32vec4 cmpgt(const Is32vec4 &_A, const Is32vec4 &_B)             { return _mm_cmpgt_epi32(_A,_B); }
inline Is32vec4 cmplt(const Is32vec4 &_A, const Is32vec4 &_B)             { return _mm_cmpgt_epi32(_B,_A); }

/* Unpacks */
inline Is32vec4 unpack_low(const Is32vec4 &_A, const Is32vec4 &_B)        { return _mm_unpacklo_epi32(_A,_B); }
inline Is32vec4 unpack_high(const Is32vec4 &_A, const Is32vec4 &_B)       { return _mm_unpackhi_epi32(_A,_B); }

/* Iu32vec4 Class:
 * 4 elements, each element unsigned int
 */
class Iu32vec4 : public I32vec4
{
public:
    Iu32vec4() { }
    Iu32vec4(__m128i _Mm) : I32vec4(_Mm) { }
    Iu32vec4(unsigned int _Ui3, unsigned int _Ui2, unsigned int _Ui1, unsigned int _Ui0)
        : I32vec4(_Ui3, _Ui2, _Ui1, _Ui0) { }

    /* Assignment Operator */
    Iu32vec4& operator= (const M128 &_A)     { return *this = (Iu32vec4) _A; }

    /* Logical Assignment Operators */
    Iu32vec4& operator&=(const M128 &_A)     { return *this = (Iu32vec4) _mm_and_si128(vec,_A); }
    Iu32vec4& operator|=(const M128 &_A)     { return *this = (Iu32vec4) _mm_or_si128(vec,_A); }
    Iu32vec4& operator^=(const M128 &_A)     { return *this = (Iu32vec4) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Iu32vec4& operator +=(const I32vec4 &_A) { return *this = (Iu32vec4)_mm_add_epi32(vec,_A); }
    Iu32vec4& operator -=(const I32vec4 &_A) { return *this = (Iu32vec4)_mm_sub_epi32(vec,_A); }

    /* Shift Logical Operators */
    Iu32vec4 operator<<(const M128 &_A)              { return _mm_sll_epi32(vec,_A); }
    Iu32vec4 operator<<(int _Count)                  { return _mm_slli_epi32(vec,_Count); }
    Iu32vec4& operator<<=(const M128 &_A)            { return *this = (Iu32vec4)_mm_sll_epi32(vec,_A); }
    Iu32vec4& operator<<=(int _Count)                { return *this = (Iu32vec4)_mm_slli_epi32(vec,_Count); }
    Iu32vec4 operator>>(const M128 &_A)              { return _mm_srl_epi32(vec,_A); }
    Iu32vec4 operator>>(int _Count)                  { return _mm_srli_epi32(vec,_Count); }
    Iu32vec4& operator>>=(const M128 &_A)            { return *this = (Iu32vec4) _mm_srl_epi32(vec,_A); }
    Iu32vec4& operator>>=(int _Count)                { return *this = (Iu32vec4) _mm_srli_epi32(vec,_Count); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Iu32vec4 &_A)
    {
        _Os << "[3]:" << _MM_4UDW(3,_A)
            << " [2]:" << _MM_4UDW(2,_A)
            << " [1]:" << _MM_4UDW(1,_A)
            << " [0]:" << _MM_4UDW(0,_A);
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const unsigned int& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4UDW(_I,vec);
    }

    /* Element Access and Assignment for Debug */
    unsigned int& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4UDW(_I,vec);
    }
};

inline I64vec2 operator*(const Iu32vec4 &_A, const Iu32vec4 &_B) { return _mm_mul_epu32(_A,_B); }
inline Iu32vec4 cmpeq(const Iu32vec4 &_A, const Iu32vec4 &_B)     { return _mm_cmpeq_epi32(_A,_B); }
inline Iu32vec4 cmpneq(const Iu32vec4 &_A, const Iu32vec4 &_B)    { return _mm_andnot_si128(_mm_cmpeq_epi32(_A,_B), get_mask128()); }

inline Iu32vec4 unpack_low(const Iu32vec4 &_A, const Iu32vec4 &_B)    { return _mm_unpacklo_epi32(_A,_B); }
inline Iu32vec4 unpack_high(const Iu32vec4 &_A, const Iu32vec4 &_B)   { return _mm_unpackhi_epi32(_A,_B); }

/* I16vec8 Class:
 * 8 elements, each element either unsigned or signed short
 */
class I16vec8 : public M128
{
public:
    I16vec8() { }
    I16vec8(__m128i _Mm) : M128(_Mm) { }
    I16vec8(short _S7, short _S6, short _S5, short _S4, short _S3, short _S2, short _S1, short _S0)
    {
        vec = _mm_set_epi16(_S7, _S6, _S5, _S4, _S3, _S2, _S1, _S0);
    }

    /* Assignment Operator */
    I16vec8& operator= (const M128 &_A)      { return *this = (I16vec8) _A; }

    /* Logical Assignment Operators */
    I16vec8& operator&=(const M128 &_A)      { return *this = (I16vec8) _mm_and_si128(vec,_A); }
    I16vec8& operator|=(const M128 &_A)      { return *this = (I16vec8) _mm_or_si128(vec,_A); }
    I16vec8& operator^=(const M128 &_A)      { return *this = (I16vec8) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    I16vec8& operator +=(const I16vec8 &_A)  { return *this = (I16vec8) _mm_add_epi16(vec,_A); }
    I16vec8& operator -=(const I16vec8 &_A)  { return *this = (I16vec8) _mm_sub_epi16(vec,_A); }
    I16vec8& operator *=(const I16vec8 &_A)  { return *this = (I16vec8) _mm_mullo_epi16(vec,_A); }

    /* Shift Logical Operators */
    I16vec8 operator<<(const M128 &_A)               { return _mm_sll_epi16(vec,_A); }
    I16vec8 operator<<(int _Count)               { return _mm_slli_epi16(vec,_Count); }
    I16vec8& operator<<=(const M128 &_A)             { return *this = (I16vec8)_mm_sll_epi16(vec,_A); }
    I16vec8& operator<<=(int _Count)                 { return *this = (I16vec8)_mm_slli_epi16(vec,_Count); }

};


inline I16vec8 operator*(const I16vec8 &_A, const I16vec8 &_B)    { return _mm_mullo_epi16(_A,_B); }

inline I16vec8 cmpeq(const I16vec8 &_A, const I16vec8 &_B)        { return _mm_cmpeq_epi16(_A,_B); }
inline I16vec8 cmpneq(const I16vec8 &_A, const I16vec8 &_B)       { return _mm_andnot_si128(_mm_cmpeq_epi16(_A,_B), get_mask128()); }

inline I16vec8 unpack_low(const I16vec8 &_A, const I16vec8 &_B)   { return _mm_unpacklo_epi16(_A,_B); }
inline I16vec8 unpack_high(const I16vec8 &_A, const I16vec8 &_B)  { return _mm_unpackhi_epi16(_A,_B); }

/* Is16vec8 Class:
 * 8 elements, each element signed short
 */
class Is16vec8 : public I16vec8
{
public:
    Is16vec8() { }
    Is16vec8(__m128i _Mm) : I16vec8(_Mm) { }
    Is16vec8(signed short _S7, signed short _S6, signed short _S5,
        signed short _S4, signed short _S3, signed short _S2,
        signed short _S1, signed short _S0)
        : I16vec8(_S7, _S6, _S5, _S4, _S3, _S2, _S1, _S0) { }

    /* Assignment Operator */
    Is16vec8& operator= (const M128 &_A)     { return *this = (Is16vec8) _A; }

    /* Logical Assignment Operators */
    Is16vec8& operator&=(const M128 &_A)     { return *this = (Is16vec8) _mm_and_si128(vec,_A); }
    Is16vec8& operator|=(const M128 &_A)     { return *this = (Is16vec8) _mm_or_si128(vec,_A); }
    Is16vec8& operator^=(const M128 &_A)     { return *this = (Is16vec8) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Is16vec8& operator +=(const I16vec8 &_A) { return *this = (Is16vec8) _mm_add_epi16(vec,_A); }
    Is16vec8& operator -=(const I16vec8 &_A) { return *this = (Is16vec8) _mm_sub_epi16(vec,_A); }
    Is16vec8& operator *=(const I16vec8 &_A) { return *this = (Is16vec8) _mm_mullo_epi16(vec,_A); }

    /* Shift Logical Operators */
    Is16vec8 operator<<(const M128 &_A)              { return _mm_sll_epi16(vec,_A); }
    Is16vec8 operator<<(int _Count)              { return _mm_slli_epi16(vec,_Count); }
    Is16vec8& operator<<=(const M128 &_A)            { return *this = (Is16vec8)_mm_sll_epi16(vec,_A); }
    Is16vec8& operator<<=(int _Count)                { return *this = (Is16vec8)_mm_slli_epi16(vec,_Count); }
    /* Shift Arithmetic Operators */
    Is16vec8 operator>>(const M128 &_A)              { return _mm_sra_epi16(vec,_A); }
    Is16vec8 operator>>(int _Count)              { return _mm_srai_epi16(vec,_Count); }
    Is16vec8& operator>>=(const M128 &_A)            { return *this = (Is16vec8)_mm_sra_epi16(vec,_A); }
    Is16vec8& operator>>=(int _Count)                { return *this = (Is16vec8)_mm_srai_epi16(vec,_Count); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Is16vec8 &_A)
    {
        _Os << "[7]:" << _MM_8W(7,_A)
            << " [6]:" << _MM_8W(6,_A)
            << " [5]:" << _MM_8W(5,_A)
            << " [4]:" << _MM_8W(4,_A)
            << " [3]:" << _MM_8W(3,_A)
            << " [2]:" << _MM_8W(2,_A)
            << " [1]:" << _MM_8W(1,_A)
            << " [0]:" << _MM_8W(0,_A);
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const signed short& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8W(_I,vec);
    }

    /* Element Access and Assignment for Debug */
    signed short& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8W(_I,vec);
    }
};

inline Is16vec8 operator*(const Is16vec8 &_A, const Is16vec8 &_B) { return _mm_mullo_epi16(_A,_B); }


/* Additional Is16vec8 functions: compares, unpacks, sat add/sub */
inline Is16vec8 cmpeq(const Is16vec8 &_A, const Is16vec8 &_B)     { return _mm_cmpeq_epi16(_A,_B); }
inline Is16vec8 cmpneq(const Is16vec8 &_A, const Is16vec8 &_B)    { return _mm_andnot_si128(_mm_cmpeq_epi16(_A,_B), get_mask128()); }
inline Is16vec8 cmpgt(const Is16vec8 &_A, const Is16vec8 &_B)     { return _mm_cmpgt_epi16(_A,_B); }
inline Is16vec8 cmplt(const Is16vec8 &_A, const Is16vec8 &_B)     { return _mm_cmpgt_epi16(_B,_A); }

inline Is16vec8 unpack_low(const Is16vec8 &_A, const Is16vec8 &_B)    { return _mm_unpacklo_epi16(_A,_B); }
inline Is16vec8 unpack_high(const Is16vec8 &_A, const Is16vec8 &_B)   { return _mm_unpackhi_epi16(_A,_B); }

inline Is16vec8 mul_high(const Is16vec8 &_A, const Is16vec8 &_B)  { return _mm_mulhi_epi16(_A,_B); }
inline Is32vec4 mul_add(const Is16vec8 &_A, const Is16vec8 &_B)   { return _mm_madd_epi16(_A,_B);}

inline Is16vec8 sat_add(const Is16vec8 &_A, const Is16vec8 &_B)   { return _mm_adds_epi16(_A,_B); }
inline Is16vec8 sat_sub(const Is16vec8 &_A, const Is16vec8 &_B)   { return _mm_subs_epi16(_A,_B); }

inline Is16vec8 simd_max(const Is16vec8 &_A, const Is16vec8 &_B)  { return _mm_max_epi16(_A,_B); }
inline Is16vec8 simd_min(const Is16vec8 &_A, const Is16vec8 &_B)  { return _mm_min_epi16(_A,_B); }


/* Iu16vec8 Class:
 * 8 elements, each element unsigned short
 */
class Iu16vec8 : public I16vec8
{
public:
    Iu16vec8() { }
    Iu16vec8(__m128i _Mm) : I16vec8(_Mm) { }
    Iu16vec8(unsigned short _S7, unsigned short _S6, unsigned short _S5,
        unsigned short _S4, unsigned short _S3, unsigned short _S2,
        unsigned short _S1, unsigned short _S0)
        : I16vec8(_S7, _S6, _S5, _S4, _S3, _S2, _S1, _S0) { }

    /* Assignment Operator */
    Iu16vec8& operator= (const M128 &_A)     { return *this = (Iu16vec8) _A; }
    /* Logical Assignment Operators */
    Iu16vec8& operator&=(const M128 &_A)     { return *this = (Iu16vec8) _mm_and_si128(vec,_A); }
    Iu16vec8& operator|=(const M128 &_A)     { return *this = (Iu16vec8) _mm_or_si128(vec,_A); }
    Iu16vec8& operator^=(const M128 &_A)     { return *this = (Iu16vec8) _mm_xor_si128(vec,_A); }
    /* Addition & Subtraction Assignment Operators */
    Iu16vec8& operator +=(const I16vec8 &_A) { return *this = (Iu16vec8) _mm_add_epi16(vec,_A); }
    Iu16vec8& operator -=(const I16vec8 &_A) { return *this = (Iu16vec8) _mm_sub_epi16(vec,_A); }
    Iu16vec8& operator *=(const I16vec8 &_A) { return *this = (Iu16vec8) _mm_mullo_epi16(vec,_A); }

    /* Shift Logical Operators */
    Iu16vec8 operator<<(const M128 &_A)              { return _mm_sll_epi16(vec,_A); }
    Iu16vec8 operator<<(int _Count)                  { return _mm_slli_epi16(vec,_Count); }
    Iu16vec8& operator<<=(const M128 &_A)            { return *this = (Iu16vec8)_mm_sll_epi16(vec,_A); }
    Iu16vec8& operator<<=(int _Count)                { return *this = (Iu16vec8)_mm_slli_epi16(vec,_Count); }
    Iu16vec8 operator>>(const M128 &_A)              { return _mm_srl_epi16(vec,_A); }
    Iu16vec8 operator>>(int _Count)                  { return _mm_srli_epi16(vec,_Count); }
    Iu16vec8& operator>>=(const M128 &_A)            { return *this = (Iu16vec8) _mm_srl_epi16(vec,_A); }
    Iu16vec8& operator>>=(int _Count)                { return *this = (Iu16vec8) _mm_srli_epi16(vec,_Count); }


#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator << (std::ostream &_Os, const Iu16vec8 &_A)
    {
        _Os << "[7]:"  << (unsigned short)(_MM_8UW(7,_A))
           << " [6]:" << (unsigned short)(_MM_8UW(6,_A))
           << " [5]:" << (unsigned short)(_MM_8UW(5,_A))
           << " [4]:" << (unsigned short)(_MM_8UW(4,_A))
           << " [3]:" << (unsigned short)(_MM_8UW(3,_A))
           << " [2]:" << (unsigned short)(_MM_8UW(2,_A))
           << " [1]:" << (unsigned short)(_MM_8UW(1,_A))
           << " [0]:" << (unsigned short)(_MM_8UW(0,_A));
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const unsigned short& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8UW(_I,vec);
    }

    /* Element Access for Debug */
    unsigned short& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8UW(_I,vec);
    }
};

inline Iu16vec8 operator*(const Iu16vec8 &_A, const Iu16vec8 &_B) { return _mm_mullo_epi16(_A,_B); }

/* Additional Iu16vec8 functions: cmpeq,cmpneq, unpacks, sat add/sub */
inline Iu16vec8 cmpeq(const Iu16vec8 &_A, const Iu16vec8 &_B)     { return _mm_cmpeq_epi16(_A,_B); }
inline Iu16vec8 cmpneq(const Iu16vec8 &_A, const Iu16vec8 &_B)    { return _mm_andnot_si128(_mm_cmpeq_epi16(_A,_B), get_mask128()); }

inline Iu16vec8 unpack_low(const Iu16vec8 &_A, const Iu16vec8 &_B)    { return _mm_unpacklo_epi16(_A,_B); }
inline Iu16vec8 unpack_high(const Iu16vec8 &_A, const Iu16vec8 &_B) { return _mm_unpackhi_epi16(_A,_B); }

inline Iu16vec8 sat_add(const Iu16vec8 &_A, const Iu16vec8 &_B)   { return _mm_adds_epu16(_A,_B); }
inline Iu16vec8 sat_sub(const Iu16vec8 &_A, const Iu16vec8 &_B)   { return _mm_subs_epu16(_A,_B); }

inline Iu16vec8 simd_avg(const Iu16vec8 &_A, const Iu16vec8 &_B)  { return _mm_avg_epu16(_A,_B); }
inline I16vec8 mul_high(const Iu16vec8 &_A, const Iu16vec8 &_B)   { return _mm_mulhi_epu16(_A,_B); }

/* I8vec16 Class:
 * 16 elements, each element either unsigned or signed char
 */
class I8vec16 : public M128
{
public:
    I8vec16() { }
    I8vec16(__m128i _Mm) : M128(_Mm) { }
    I8vec16(char _S15, char _S14, char _S13, char _S12, char _S11, char _S10,
        char _S9, char _S8, char _S7, char _S6, char _S5, char _S4,
        char _S3, char _S2, char _S1, char _S0)
    {
        vec = _mm_set_epi8(_S15, _S14, _S13, _S12, _S11, _S10, _S9, _S8, _S7, _S6, _S5, _S4, _S3, _S2, _S1, _S0);
    }

    /* Assignment Operator */
    I8vec16& operator= (const M128 &_A)      { return *this = (I8vec16) _A; }

    /* Logical Assignment Operators */
    I8vec16& operator&=(const M128 &_A)      { return *this = (I8vec16) _mm_and_si128(vec,_A); }
    I8vec16& operator|=(const M128 &_A)      { return *this = (I8vec16) _mm_or_si128(vec,_A); }
    I8vec16& operator^=(const M128 &_A)      { return *this = (I8vec16) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    I8vec16& operator +=(const I8vec16 &_A)  { return *this = (I8vec16) _mm_add_epi8(vec,_A); }
    I8vec16& operator -=(const I8vec16 &_A)  { return *this = (I8vec16) _mm_sub_epi8(vec,_A); }

};

inline I8vec16 cmpeq(const I8vec16 &_A, const I8vec16 &_B)        { return _mm_cmpeq_epi8(_A,_B); }
inline I8vec16 cmpneq(const I8vec16 &_A, const I8vec16 &_B)       { return _mm_andnot_si128(_mm_cmpeq_epi8(_A,_B), get_mask128()); }

inline I8vec16 unpack_low(const I8vec16 &_A, const I8vec16 &_B)   { return _mm_unpacklo_epi8(_A,_B); }
inline I8vec16 unpack_high(const I8vec16 &_A, const I8vec16 &_B)  { return _mm_unpackhi_epi8(_A,_B); }

/* Is8vec16 Class:
 * 16 elements, each element a signed char
 */
class Is8vec16 : public I8vec16
{
public:
    Is8vec16() { }
    Is8vec16(__m128i _Mm) : I8vec16(_Mm) { }
    Is8vec16(char _S15, char _S14, char _S13, char _S12, char _S11, char _S10,
        char _S9, char _S8, char _S7, char _S6, char _S5, char _S4,
        char _S3, char _S2, char _S1, char _S0)
        : I8vec16(_S15, _S14, _S13, _S12, _S11, _S10, _S9, _S8,
        _S7, _S6, _S5, _S4, _S3, _S2, _S1, _S0) { }

    /* Assignment Operator */
    Is8vec16& operator= (const M128 &_A)     { return *this = (Is8vec16) _A; }

    /* Logical Assignment Operators */
    Is8vec16& operator&=(const M128 &_A)     { return *this = (Is8vec16) _mm_and_si128(vec,_A); }
    Is8vec16& operator|=(const M128 &_A)     { return *this = (Is8vec16) _mm_or_si128(vec,_A); }
    Is8vec16& operator^=(const M128 &_A)     { return *this = (Is8vec16) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Is8vec16& operator +=(const I8vec16 &_A) { return *this = (Is8vec16) _mm_add_epi8(vec,_A); }
    Is8vec16& operator -=(const I8vec16 &_A) { return *this = (Is8vec16) _mm_sub_epi8(vec,_A); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator << (std::ostream &_Os, const Is8vec16 &_A)
    {
        _Os << "[15]:"  << short(_MM_16B(15,_A))
            << " [14]:" << short(_MM_16B(14,_A))
            << " [13]:" << short(_MM_16B(13,_A))
            << " [12]:" << short(_MM_16B(12,_A))
            << " [11]:" << short(_MM_16B(11,_A))
            << " [10]:" << short(_MM_16B(10,_A))
            << " [9]:" << short(_MM_16B(9,_A))
            << " [8]:" << short(_MM_16B(8,_A))
              << " [7]:" << short(_MM_16B(7,_A))
            << " [6]:" << short(_MM_16B(6,_A))
            << " [5]:" << short(_MM_16B(5,_A))
            << " [4]:" << short(_MM_16B(4,_A))
            << " [3]:" << short(_MM_16B(3,_A))
            << " [2]:" << short(_MM_16B(2,_A))
            << " [1]:" << short(_MM_16B(1,_A))
            << " [0]:" << short(_MM_16B(0,_A));
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const signed char& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 16); /* Only 16 elements to access */
        return _MM_16B(_I,vec);
    }

    /* Element Access for Debug */
    signed char& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 16); /* Only 16 elements to access */
        return _MM_16B(_I,vec);
    }

};

inline Is8vec16 cmpeq(const Is8vec16 &_A, const Is8vec16 &_B)     { return _mm_cmpeq_epi8(_A,_B); }
inline Is8vec16 cmpneq(const Is8vec16 &_A, const Is8vec16 &_B)    { return _mm_andnot_si128(_mm_cmpeq_epi8(_A,_B), get_mask128()); }
inline Is8vec16 cmpgt(const Is8vec16 &_A, const Is8vec16 &_B)     { return _mm_cmpgt_epi8(_A,_B); }
inline Is8vec16 cmplt(const Is8vec16 &_A, const Is8vec16 &_B)     { return _mm_cmplt_epi8(_A,_B); }

inline Is8vec16 unpack_low(const Is8vec16 &_A, const Is8vec16 &_B)    { return _mm_unpacklo_epi8(_A,_B); }
inline Is8vec16 unpack_high(const Is8vec16 &_A, const Is8vec16 &_B) { return _mm_unpackhi_epi8(_A,_B); }

inline Is8vec16 sat_add(const Is8vec16 &_A, const Is8vec16 &_B)   { return _mm_adds_epi8(_A,_B); }
inline Is8vec16 sat_sub(const Is8vec16 &_A, const Is8vec16 &_B)   { return _mm_subs_epi8(_A,_B); }

/* Iu8vec16 Class:
 * 16 elements, each element a unsigned char
 */
class Iu8vec16 : public I8vec16
{
public:
    Iu8vec16() { }
    Iu8vec16(__m128i _Mm) : I8vec16(_Mm) { }
    Iu8vec16(unsigned char _U15, unsigned char _U14, unsigned char _U13,
        unsigned char _U12, unsigned char _U11, unsigned char _U10,
        unsigned char _U9, unsigned char _U8, unsigned char _U7,
        unsigned char _U6, unsigned char _U5, unsigned char _U4,
        unsigned char _U3, unsigned char _U2, unsigned char _U1,
        unsigned char _U0)
        : I8vec16(_U15, _U14, _U13, _U12, _U11, _U10, _U9, _U8,
        _U7, _U6, _U5, _U4, _U3, _U2, _U1, _U0) { }

    /* Assignment Operator */
    Iu8vec16& operator= (const M128 &_A)     { return *this = (Iu8vec16) _A; }

    /* Logical Assignment Operators */
    Iu8vec16& operator&=(const M128 &_A)     { return *this = (Iu8vec16) _mm_and_si128(vec,_A); }
    Iu8vec16& operator|=(const M128 &_A)     { return *this = (Iu8vec16) _mm_or_si128(vec,_A); }
    Iu8vec16& operator^=(const M128 &_A)     { return *this = (Iu8vec16) _mm_xor_si128(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Iu8vec16& operator +=(const I8vec16 &_A) { return *this = (Iu8vec16) _mm_add_epi8(vec,_A); }
    Iu8vec16& operator -=(const I8vec16 &_A) { return *this = (Iu8vec16) _mm_sub_epi8(vec,_A); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator << (std::ostream &_Os, const Iu8vec16 &_A)
    {
        _Os << "[15]:"  << (unsigned char)(_MM_16UB(15,_A))
            << " [14]:" << (unsigned char)(_MM_16UB(14,_A))
            << " [13]:" << (unsigned char)(_MM_16UB(13,_A))
            << " [12]:" << (unsigned char)(_MM_16UB(12,_A))
            << " [11]:" << (unsigned char)(_MM_16UB(11,_A))
            << " [10]:" << (unsigned char)(_MM_16UB(10,_A))
            << " [9]:" << (unsigned char)(_MM_16UB(9,_A))
            << " [8]:" << (unsigned char)(_MM_16UB(8,_A))
            << " [7]:" << (unsigned char)(_MM_16UB(7,_A))
            << " [6]:" << (unsigned char)(_MM_16UB(6,_A))
            << " [5]:" << (unsigned char)(_MM_16UB(5,_A))
            << " [4]:" << (unsigned char)(_MM_16UB(4,_A))
            << " [3]:" << (unsigned char)(_MM_16UB(3,_A))
            << " [2]:" << (unsigned char)(_MM_16UB(2,_A))
            << " [1]:" << (unsigned char)(_MM_16UB(1,_A))
            << " [0]:" << (unsigned char)(_MM_16UB(0,_A));
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const unsigned char& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 16); /* Only 16 elements to access */
        return _MM_16UB(_I,vec);
    }

    /* Element Access for Debug */
    unsigned char& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 16); /* Only 16 elements to access */
        return _MM_16UB(_I,vec);
    }

};

inline Iu8vec16 cmpeq(const Iu8vec16 &_A, const Iu8vec16 &_B)     { return _mm_cmpeq_epi8(_A,_B); }
inline Iu8vec16 cmpneq(const Iu8vec16 &_A, const Iu8vec16 &_B)    { return _mm_andnot_si128(_mm_cmpeq_epi8(_A,_B), get_mask128()); }

inline Iu8vec16 unpack_low(const Iu8vec16 &_A, const Iu8vec16 &_B)    { return _mm_unpacklo_epi8(_A,_B); }
inline Iu8vec16 unpack_high(const Iu8vec16 &_A, const Iu8vec16 &_B) { return _mm_unpackhi_epi8(_A,_B); }

inline Iu8vec16 sat_add(const Iu8vec16 &_A, const Iu8vec16 &_B)   { return _mm_adds_epu8(_A,_B); }
inline Iu8vec16 sat_sub(const Iu8vec16 &_A, const Iu8vec16 &_B)   { return _mm_subs_epu8(_A,_B); }

inline I64vec2 sum_abs(const Iu8vec16 &_A, const Iu8vec16 &_B)    { return _mm_sad_epu8(_A,_B); }

inline Iu8vec16 simd_avg(const Iu8vec16 &_A, const Iu8vec16 &_B)  { return _mm_avg_epu8(_A,_B); }
inline Iu8vec16 simd_max(const Iu8vec16 &_A, const Iu8vec16 &_B)  { return _mm_max_epu8(_A,_B); }
inline Iu8vec16 simd_min(const Iu8vec16 &_A, const Iu8vec16 &_B)  { return _mm_min_epu8(_A,_B); }

/* Pack & Saturates */

inline Is16vec8 pack_sat(const Is32vec4 &_A, const Is32vec4 &_B)  { return _mm_packs_epi32(_A,_B); }
inline Is8vec16 pack_sat(const Is16vec8 &_A, const Is16vec8 &_B)  { return _mm_packs_epi16(_A,_B); }
inline Iu8vec16 packu_sat(const Is16vec8 &_A, const Is16vec8 &_B) { return _mm_packus_epi16(_A,_B);}

 /********************************* Logicals ****************************************/
#define IVEC128_LOGICALS(vect,element) \
inline I##vect##vec##element operator& (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _mm_and_si128( _A,_B); } \
inline I##vect##vec##element operator| (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _mm_or_si128( _A,_B); } \
inline I##vect##vec##element operator^ (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _mm_xor_si128( _A,_B); } \
inline I##vect##vec##element andnot (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _mm_andnot_si128( _A,_B); }

IVEC128_LOGICALS(8,16)
IVEC128_LOGICALS(u8,16)
IVEC128_LOGICALS(s8,16)
IVEC128_LOGICALS(16,8)
IVEC128_LOGICALS(u16,8)
IVEC128_LOGICALS(s16,8)
IVEC128_LOGICALS(32,4)
IVEC128_LOGICALS(u32,4)
IVEC128_LOGICALS(s32,4)
IVEC128_LOGICALS(64,2)
IVEC128_LOGICALS(128,1)
#undef IVEC128_LOGICALS

 /********************************* Add & Sub ****************************************/
#define IVEC128_ADD_SUB(vect,element,opsize) \
inline I##vect##vec##element operator+ (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _mm_add_##opsize( _A,_B); } \
inline I##vect##vec##element operator- (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _mm_sub_##opsize( _A,_B); }

IVEC128_ADD_SUB(8,16, epi8)
IVEC128_ADD_SUB(u8,16, epi8)
IVEC128_ADD_SUB(s8,16, epi8)
IVEC128_ADD_SUB(16,8, epi16)
IVEC128_ADD_SUB(u16,8, epi16)
IVEC128_ADD_SUB(s16,8, epi16)
IVEC128_ADD_SUB(32,4, epi32)
IVEC128_ADD_SUB(u32,4, epi32)
IVEC128_ADD_SUB(s32,4, epi32)
IVEC128_ADD_SUB(64,2, epi64)
#undef IVEC128_ADD_SUB

 /************************* Conditional Select ********************************
 *  version of: retval = (a OP b)? c : d;                                    *
 *  Where OP is one of the possible comparision operators.                   *
 *  Example: r = select_eq(a,b,c,d);                                         *
 *      if "member at position x of the vector a" ==                         *
 *         "member at position x of vector b"                                *
 *  assign the corresponding member in r from c, else assign from d.         *
 ************************* Conditional Select ********************************/

#define IVEC128_SELECT(vect12,vect34,element,selop)                 \
    inline I##vect34##vec##element select_##selop (                 \
        const I##vect12##vec##element &_A,                           \
        const I##vect12##vec##element &_B,                           \
        const I##vect34##vec##element &_C,                           \
        const I##vect34##vec##element &_D)                           \
{                                                                   \
    I##vect12##vec##element _Mask = cmp##selop(_A,_B);                 \
    return ( I##vect34##vec##element (_Mask & _C ) |                  \
        I##vect34##vec##element ((_mm_andnot_si128(_Mask, _D ))));    \
}

IVEC128_SELECT(8,s8,16,eq)
IVEC128_SELECT(8,u8,16,eq)
IVEC128_SELECT(8,8,16,eq)
IVEC128_SELECT(8,s8,16,neq)
IVEC128_SELECT(8,u8,16,neq)
IVEC128_SELECT(8,8,16,neq)

IVEC128_SELECT(16,s16,8,eq)
IVEC128_SELECT(16,u16,8,eq)
IVEC128_SELECT(16,16,8,eq)
IVEC128_SELECT(16,s16,8,neq)
IVEC128_SELECT(16,u16,8,neq)
IVEC128_SELECT(16,16,8,neq)

IVEC128_SELECT(32,s32,4,eq)
IVEC128_SELECT(32,u32,4,eq)
IVEC128_SELECT(32,32,4,eq)
IVEC128_SELECT(32,s32,4,neq)
IVEC128_SELECT(32,u32,4,neq)
IVEC128_SELECT(32,32,4,neq)

IVEC128_SELECT(s8,s8,16,gt)
IVEC128_SELECT(s8,u8,16,gt)
IVEC128_SELECT(s8,8,16,gt)
IVEC128_SELECT(s8,s8,16,lt)
IVEC128_SELECT(s8,u8,16,lt)
IVEC128_SELECT(s8,8,16,lt)

IVEC128_SELECT(s16,s16,8,gt)
IVEC128_SELECT(s16,u16,8,gt)
IVEC128_SELECT(s16,16,8,gt)
IVEC128_SELECT(s16,s16,8,lt)
IVEC128_SELECT(s16,u16,8,lt)
IVEC128_SELECT(s16,16,8,lt)


#undef IVEC128_SELECT


class F64vec2
{
protected:
     __m128d vec;
public:

    /* Constructors: __m128d, 2 doubles */
    F64vec2() {}

    /* initialize 2 DP FP with __m128d data type */
    F64vec2(__m128d _M)                  { vec = _M;}

    /* initialize 2 DP FPs with 2 doubles */
    F64vec2(double _D1, double _D0)                       { vec= _mm_set_pd(_D1,_D0); }

    /* Explicitly initialize each of 2 DP FPs with same double */
    explicit F64vec2(double _D)  { vec = _mm_set1_pd(_D); }

    /* Conversion functions */
    operator  __m128d() const   { return vec; }     /* Convert to __m128d */

    /* Logical Operators */
    friend F64vec2 operator &(const F64vec2 &_A, const F64vec2 &_B) { return _mm_and_pd(_A,_B); }
    friend F64vec2 operator |(const F64vec2 &_A, const F64vec2 &_B) { return _mm_or_pd(_A,_B); }
    friend F64vec2 operator ^(const F64vec2 &_A, const F64vec2 &_B) { return _mm_xor_pd(_A,_B); }

    /* Arithmetic Operators */
    friend F64vec2 operator +(const F64vec2 &_A, const F64vec2 &_B) { return _mm_add_pd(_A,_B); }
    friend F64vec2 operator -(const F64vec2 &_A, const F64vec2 &_B) { return _mm_sub_pd(_A,_B); }
    friend F64vec2 operator *(const F64vec2 &_A, const F64vec2 &_B) { return _mm_mul_pd(_A,_B); }
    friend F64vec2 operator /(const F64vec2 &_A, const F64vec2 &_B) { return _mm_div_pd(_A,_B); }

    F64vec2& operator +=(const F64vec2 &_A) { return *this = _mm_add_pd(vec,_A); }
    F64vec2& operator -=(const F64vec2 &_A) { return *this = _mm_sub_pd(vec,_A); }
    F64vec2& operator *=(const F64vec2 &_A) { return *this = _mm_mul_pd(vec,_A); }
    F64vec2& operator /=(const F64vec2 &_A) { return *this = _mm_div_pd(vec,_A); }
    F64vec2& operator &=(const F64vec2 &_A) { return *this = _mm_and_pd(vec,_A); }
    F64vec2& operator |=(const F64vec2 &_A) { return *this = _mm_or_pd(vec,_A); }
    F64vec2& operator ^=(const F64vec2 &_A) { return *this = _mm_xor_pd(vec,_A); }

    /* Horizontal Add */
    friend double add_horizontal(const F64vec2 &_A)
    {
        F64vec2 _Ftemp = _mm_add_sd(_A,_mm_shuffle_pd(_A, _A, 1));
        return _mm_cvtsd_f64(_Ftemp);
    }

    /* And Not */
    friend F64vec2 andnot(const F64vec2 &_A, const F64vec2 &_B) { return _mm_andnot_pd(_A,_B); }

    /* Square Root */
    friend F64vec2 sqrt(const F64vec2 &_A)       { return _mm_sqrt_pd(_A); }

    /* Compares: Mask is returned  */
    /* Macros expand to all compare intrinsics.  Example:
            friend F64vec2 cmpeq(const F64vec2 &_A, const F64vec2 &_B)
            { return _mm_cmpeq_ps(_A,_B);} */
    #define F64vec2_COMP(op) \
    friend F64vec2 cmp##op (const F64vec2 &_A, const F64vec2 &_B) { return _mm_cmp##op##_pd(_A,_B); }
        F64vec2_COMP(eq)                    /* expanded to cmpeq(_A,_B) */
        F64vec2_COMP(lt)                    /* expanded to cmplt(_A,_B) */
        F64vec2_COMP(le)                    /* expanded to cmple(_A,_B) */
        F64vec2_COMP(gt)                    /* expanded to cmpgt(_A,_B) */
        F64vec2_COMP(ge)                    /* expanded to cmpge(_A,_B) */
        F64vec2_COMP(ngt)                   /* expanded to cmpngt(_A,_B) */
        F64vec2_COMP(nge)                   /* expanded to cmpnge(_A,_B) */
        F64vec2_COMP(neq)                   /* expanded to cmpneq(_A,_B) */
        F64vec2_COMP(nlt)                   /* expanded to cmpnlt(_A,_B) */
        F64vec2_COMP(nle)                   /* expanded to cmpnle(_A,_B) */
    #undef F64vec2_COMP

    /* Min and Max */
    friend F64vec2 simd_min(const F64vec2 &_A, const F64vec2 &_B) { return _mm_min_pd(_A,_B); }
    friend F64vec2 simd_max(const F64vec2 &_A, const F64vec2 &_B) { return _mm_max_pd(_A,_B); }

    /* Absolute value */
    friend F64vec2 abs(const F64vec2 &_A)
    {
        return _mm_and_pd(_A, _f64vec2_abs_mask);
    }

        /* Compare lower DP FP values */
    #define F64vec2_COMI(op) \
    friend int comi##op (const F64vec2 &_A, const F64vec2 &_B) { return _mm_comi##op##_sd(_A,_B); }
        F64vec2_COMI(eq)                    /* expanded to comieq(_A,_B) */
        F64vec2_COMI(lt)                    /* expanded to comilt(_A,_B) */
        F64vec2_COMI(le)                    /* expanded to comile(_A,_B) */
        F64vec2_COMI(gt)                    /* expanded to comigt(_A,_B) */
        F64vec2_COMI(ge)                    /* expanded to comige(_A,_B) */
        F64vec2_COMI(neq)                   /* expanded to comineq(_A,_B) */
    #undef F64vec2_COMI

        /* Compare lower DP FP values */
    #define F64vec2_UCOMI(op) \
    friend int ucomi##op (const F64vec2 &_A, const F64vec2 &_B) { return _mm_ucomi##op##_sd(_A,_B); }
        F64vec2_UCOMI(eq)                   /* expanded to ucomieq(_A,_B) */
        F64vec2_UCOMI(lt)                   /* expanded to ucomilt(_A,_B) */
        F64vec2_UCOMI(le)                   /* expanded to ucomile(_A,_B) */
        F64vec2_UCOMI(gt)                   /* expanded to ucomigt(_A,_B) */
        F64vec2_UCOMI(ge)                   /* expanded to ucomige(_A,_B) */
        F64vec2_UCOMI(neq)                  /* expanded to ucomineq(_A,_B) */
    #undef F64vec2_UCOMI

    /* Debug Features */
#if defined (_ENABLE_VEC_DEBUG)
    /* Output */
    friend std::ostream & operator<<(std::ostream & _Os, const F64vec2 &_A)
    {
    /* To use: cout << "Elements of F64vec2 fvec are: " << fvec; */
      double *_Dp = (double*)&_A;
      _Os <<   "[1]:" << *(_Dp+1)
            << " [0]:" << *_Dp;
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */
    /* Element Access Only, no modifications to elements*/
    const double& operator[](int _I) const
    {
        /* Assert enabled only during debug /DDEBUG */
        _VEC_ASSERT((0 <= _I) && (_I <= 1));          /* User should only access elements 0-1 */
        double *_Dp = (double*)&vec;
        return *(_Dp+ _I);
    }
    /* Element Access and Modification*/
    double& operator[](int _I)
    {
        /* Assert enabled only during debug /DDEBUG */
        _VEC_ASSERT((0 <= _I) && (_I <= 1));          /* User should only access elements 0-1 */
        double *_Dp = (double*)&vec;
        return *(_Dp+ _I);
    }
};

                        /* Miscellaneous */

/* Interleave low order data elements of a and b into destination */
inline F64vec2 unpack_low(const F64vec2 &_A, const F64vec2 &_B)
{ return _mm_unpacklo_pd(_A, _B); }

/* Interleave high order data elements of a and b into target */
inline F64vec2 unpack_high(const F64vec2 &_A, const F64vec2 &_B)
{ return _mm_unpackhi_pd(_A, _B); }

/* Move Mask to Integer returns 4 bit mask formed of most significant bits of a */
inline int move_mask(const F64vec2 &_A)
{ return _mm_movemask_pd(_A);}

                        /* Data Motion Functions */

/* Load Unaligned loadu_pd: Unaligned */
inline void loadu(F64vec2 &_A, double *_P)
{ _A = _mm_loadu_pd(_P); }

/* Store Temporal storeu_pd: Unaligned */
inline void storeu(double *_P, const F64vec2 &_A)
{ _mm_storeu_pd(_P, _A); }

                        /* Cacheability Support */

/* Non-Temporal Store */
inline void store_nta(double *_P, F64vec2 &_A)
{ _mm_stream_pd(_P,_A);}

#define F64vec2_SELECT(op) \
inline F64vec2 select_##op (const F64vec2 &_A, const F64vec2 &_B, const F64vec2 &_C, const F64vec2 &_D) \
{                                                           \
    F64vec2 _Mask = _mm_cmp##op##_pd(_A,_B);                   \
    return( (_Mask & _C) | F64vec2((_mm_andnot_pd(_Mask,_D)))); \
}
F64vec2_SELECT(eq)      /* generates select_eq(_A,_B) */
F64vec2_SELECT(lt)      /* generates select_lt(_A,_B) */
F64vec2_SELECT(le)      /* generates select_le(_A,_B) */
F64vec2_SELECT(gt)      /* generates select_gt(_A,_B) */
F64vec2_SELECT(ge)      /* generates select_ge(_A,_B) */
F64vec2_SELECT(neq)     /* generates select_neq(_A,_B) */
F64vec2_SELECT(nlt)     /* generates select_nlt(_A,_B) */
F64vec2_SELECT(nle)     /* generates select_nle(_A,_B) */
#undef F64vec2_SELECT

/* Convert the lower DP FP value of a to a 32 bit signed integer using Truncate*/
inline int F64vec2ToInt(const F64vec2 &_A)
{

    return _mm_cvttsd_si32(_A);

}

/* Convert the 4 SP FP values of a to DP FP values */
inline F64vec2 F32vec4ToF64vec2(const F32vec4 &_A)
{
    return _mm_cvtps_pd(_A);
}

/* Convert the 2 DP FP values of a to SP FP values */
inline F32vec4 F64vec2ToF32vec4(const F64vec2 &_A)
{
    return _mm_cvtpd_ps(_A);
}

/* Convert the signed int in b to a DP FP value.  Upper DP FP value in a passed through */
inline F64vec2 IntToF64vec2(const F64vec2 &_A, int _B)
{
    return _mm_cvtsi32_sd(_A,_B);
}

#pragma pack(pop) /* 16-B aligned */

 /******************************************************************************/
 /************** Interface classes for Intel(R) AVX intrinsics *****************/
 /******************************************************************************/

/*
 * class F32vec8
 *
 * Represents 256-bit vector composed of 8 single precision floating point elements.
 */
class F32vec8
{
protected:
    __m256 vec;

public:

    /* Constructors: __m256, 8 floats, 1 float */
    F32vec8() {}

    /* initialize 8 SP FP with __m256 data type */
    F32vec8(__m256 _M) { vec = _M; }

    /* initialize 8 SP FPs with 8 floats */
    F32vec8(float _F7, float _F6, float _F5, float _F4, float _F3, float _F2, float _F1, float _F0)
    {
        vec = _mm256_set_ps(_F7, _F6, _F5, _F4, _F3, _F2,_F1, _F0);
    }

    /* Explicitly initialize each of 8 SP FPs with same float */
    explicit F32vec8(float _F)   { vec = _mm256_set1_ps(_F); }

    /* Explicitly initialize each of 8 SP FPs with same double */
    explicit F32vec8(double _D)  { vec = _mm256_set1_ps((float) _D); }

    /* Assignment operations */
    F32vec8& operator =(float _F)
    {
        vec = _mm256_set1_ps(_F);
        return *this;
    }

    F32vec8& operator =(double _D)
    {
        vec = _mm256_set1_ps((float) _D);
        return *this;
    }

    /* Conversion functions */
    operator  __m256() const { return vec; }

    /* Logical Operators */
    friend F32vec8 operator &(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_and_ps(_A,_B); }
    friend F32vec8 operator |(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_or_ps(_A,_B); }
    friend F32vec8 operator ^(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_xor_ps(_A,_B); }

    /* Arithmetic Operators */
    friend F32vec8 operator +(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_add_ps(_A,_B); }
    friend F32vec8 operator -(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_sub_ps(_A,_B); }
    friend F32vec8 operator *(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_mul_ps(_A,_B); }
    friend F32vec8 operator /(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_div_ps(_A,_B); }

    F32vec8& operator +=(const F32vec8 &_A) { return *this = _mm256_add_ps(vec,_A); }
    F32vec8& operator -=(const F32vec8 &_A) { return *this = _mm256_sub_ps(vec,_A); }
    F32vec8& operator *=(const F32vec8 &_A) { return *this = _mm256_mul_ps(vec,_A); }
    F32vec8& operator /=(const F32vec8 &_A) { return *this = _mm256_div_ps(vec,_A); }
    F32vec8& operator &=(const F32vec8 &_A) { return *this = _mm256_and_ps(vec,_A); }
    F32vec8& operator |=(const F32vec8 &_A) { return *this = _mm256_or_ps(vec,_A); }
    F32vec8& operator ^=(const F32vec8 &_A) { return *this = _mm256_xor_ps(vec,_A); }

    /* Horizontal Add */
    friend float add_horizontal(const F32vec8 &_A)
    {
        F32vec8 _Temp = _mm256_add_ps(_A, _mm256_permute_ps(_A, 0xee));
        _Temp = _mm256_add_ps(_Temp, _mm256_movehdup_ps(_Temp));
        return _mm_cvtss_f32(_mm_add_ss(_mm256_castps256_ps128(_Temp), _mm256_extractf128_ps(_Temp,1)));
    }

    /* And Not */
    friend F32vec8 andnot(const F32vec8 &_A, const F32vec8 &_B) { return _mm256_andnot_ps(_A,_B); }

    /* Square Root */
    friend F32vec8 sqrt(const F32vec8 &_A)   { return _mm256_sqrt_ps(_A); }

    /* Reciprocal */
    friend F32vec8 rcp(const F32vec8 &_A)    { return _mm256_rcp_ps(_A); }

    /* Reciprocal Square Root */
    friend F32vec8 rsqrt(const F32vec8 &_A)  { return _mm256_rsqrt_ps(_A); }

    /*
     * NewtonRaphson Reciprocal
     * [2 * rcpps(x) - (x * rcpps(x) * rcpps(x))]
     */
    friend F32vec8 rcp_nr(const F32vec8 &_A)
    {
        F32vec8 _Ra0 = _mm256_rcp_ps(_A);
        return _mm256_sub_ps(_mm256_add_ps(_Ra0, _Ra0), _mm256_mul_ps(_mm256_mul_ps(_Ra0, _A), _Ra0));
    }

    /*
     * NewtonRaphson Reciprocal Square Root
     * 0.5 * rsqrtps * (3 - x * rsqrtps(x) * rsqrtps(x))
     */
    friend F32vec8 rsqrt_nr(const F32vec8 &_A)
    {
#pragma warning(push)
#pragma warning(disable:4640)
        static const F32vec8 fvecf0pt5(0.5f);
        static const F32vec8 fvecf3pt0(3.0f);
#pragma warning(pop)
        F32vec8 _Ra0 = _mm256_rsqrt_ps(_A);
        return (fvecf0pt5 * _Ra0) * (fvecf3pt0 - (_A * _Ra0) * _Ra0);

    }

    /* Compares: Mask is returned */
    friend F32vec8 cmp_eq(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_EQ_OQ); }
    friend F32vec8 cmp_lt(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_LT_OS); }
    friend F32vec8 cmp_le(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_LE_OS); }
    friend F32vec8 cmp_gt(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_GT_OS); }
    friend F32vec8 cmp_ge(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_GE_OS); }
    friend F32vec8 cmp_neq(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_NEQ_UQ); }
    friend F32vec8 cmp_nlt(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_NLT_US); }
    friend F32vec8 cmp_nle(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_NLE_US); }
    friend F32vec8 cmp_ngt(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_NGT_US); }
    friend F32vec8 cmp_nge(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_cmp_ps(_A, _B, _CMP_NGE_US); }

    /* Min and Max */
    friend F32vec8 simd_min(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_min_ps(_A,_B); }
    friend F32vec8 simd_max(const F32vec8 &_A, const F32vec8 &_B)
        { return _mm256_max_ps(_A,_B); }

    /* Absolute value */
    friend F32vec8 abs(const F32vec8 &_A)
    {
        static const union
        {
            int i[8];
            __m256 m;
        } __f32vec8_abs_mask = { 0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff,
                                 0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff};
        return _mm256_and_ps(_A, __f32vec8_abs_mask.m);
    }

    /* Debug Features */
#if defined (_ENABLE_VEC_DEBUG)
    /* Output */
    friend DVEC_STD ostream & operator<<(DVEC_STD ostream &_Os, const F32vec8 &_A)
    {
        /* To use: cout << "Elements of F32vec8 fvec are: " << fvec; */
        float *_Fp = (float*) &_A;
        _Os <<  "[7]:" << *(_Fp+7)
           << " [6]:" << *(_Fp+6)
           << " [5]:" << *(_Fp+5)
           << " [4]:" << *(_Fp+4)
           << " [3]:" << *(_Fp+3)
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
        _VEC_ASSERT((0 <= _I) && (_I <= 7));
        float *_Fp = (float*)&vec;
        return *(_Fp+ _I);
    }

    /* Element Access and Modification*/
    float& operator[](int _I)
    {
        /* Assert enabled only during debug /DDEBUG */
        _VEC_ASSERT((0 <= _I) && (_I <= 7));
        float *_Fp = (float*)&vec;
        return *(_Fp+ _I);
    }
};

            /* Miscellaneous */

/* Interleave low order data elements of a and b into destination */
inline F32vec8 unpack_low(const F32vec8 &_A, const F32vec8 &_B){
    return _mm256_unpacklo_ps(_A, _B); }

/* Interleave high order data elements of a and b into target */
inline F32vec8 unpack_high(const F32vec8 &_A, const F32vec8 &_B){
    return _mm256_unpackhi_ps(_A, _B); }

/* Move Mask to Integer returns 8 bit mask formed of most significant bits of a */
inline int move_mask(const F32vec8 &_A){
    return _mm256_movemask_ps(_A); }

            /* Data Motion Functions */

/* Load Unaligned loadu_ps: Unaligned */
inline void loadu(F32vec8 &_A, const float *_P){
    _A = _mm256_loadu_ps(_P); }

/* Store Unaligned storeu_ps: Unaligned */
inline void storeu(float *_P, const F32vec8 &_A){
    _mm256_storeu_ps(_P, _A); }

            /* Cacheability Support */

/* Non-Temporal Store */
inline void store_nta(float *_P, const F32vec8 &_A){
    _mm256_stream_ps(_P, _A); }

            /* Conditional moves */

/* Masked load */
inline void maskload(F32vec8 &_A, const float *_P, const F32vec8 &_M){
    _A = _mm256_maskload_ps(_P, _mm256_castps_si256(_M)); }

inline void maskload(F32vec4 &_A, const float *_P, const F32vec4 &_M){
    _A = _mm_maskload_ps(_P, _mm_castps_si128(_M)); }

/* Masked store */
inline void maskstore(float *_P, const F32vec8 &_A, const F32vec8 &_M){
    _mm256_maskstore_ps(_P, _mm256_castps_si256(_M), _A); }

inline void maskstore(float *_P, const F32vec4 &_A, const F32vec4 &_M){
    _mm_maskstore_ps(_P, _mm_castps_si128(_M), _A); }

            /* Conditional Selects */

inline F32vec8 select_eq(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_EQ_OQ)); }

inline F32vec8 select_lt(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_LT_OS)); }

inline F32vec8 select_le(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_LE_OS)); }

inline F32vec8 select_gt(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_GT_OS)); }

inline F32vec8 select_ge(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_GE_OS)); }

inline F32vec8 select_neq(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_NEQ_UQ)); }

inline F32vec8 select_nlt(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_NLT_US)); }

inline F32vec8 select_nle(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_NLE_US)); }

inline F32vec8 select_ngt(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_NGT_US)); }

inline F32vec8 select_nge(const F32vec8 &_A, const F32vec8 &_B, const F32vec8 &_C, const F32vec8 &_D){
    return _mm256_blendv_ps(_D, _C, _mm256_cmp_ps(_A, _B, _CMP_NGE_US)); }

/*
 * class F64vec4
 *
 * Represents 256-bit vector composed of 4 double precision floating point elements.
 */
class F64vec4
{
protected:
    __m256d vec;

public:

    /* Constructors: __m256d, 4 doubles */
    F64vec4() {}

    /* initialize 4 DP FP with __m256d data type */
    F64vec4(__m256d m) { vec = m; }

    /* initialize 4 DP FPs with 4 doubles */
    F64vec4(double _D3, double _D2, double _D1, double _D0)
    {
        vec = _mm256_set_pd(_D3,_D2,_D1,_D0);
    }

    /* Explicitly initialize each of 4 DP FPs with same double */
    explicit F64vec4(double _D) { vec = _mm256_set1_pd(_D); }

    /* Conversion functions */
    operator  __m256d() const { return vec; }

    /* Logical Operators */
    friend F64vec4 operator &(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_and_pd(_A,_B); }
    friend F64vec4 operator |(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_or_pd(_A,_B); }
    friend F64vec4 operator ^(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_xor_pd(_A,_B); }

    /* Arithmetic Operators */
    friend F64vec4 operator +(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_add_pd(_A,_B); }
    friend F64vec4 operator -(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_sub_pd(_A,_B); }
    friend F64vec4 operator *(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_mul_pd(_A,_B); }
    friend F64vec4 operator /(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_div_pd(_A,_B); }

    F64vec4& operator +=(const F64vec4 &_A) { return *this = _mm256_add_pd(vec,_A); }
    F64vec4& operator -=(const F64vec4 &_A) { return *this = _mm256_sub_pd(vec,_A); }
    F64vec4& operator *=(const F64vec4 &_A) { return *this = _mm256_mul_pd(vec,_A); }
    F64vec4& operator /=(const F64vec4 &_A) { return *this = _mm256_div_pd(vec,_A); }
    F64vec4& operator &=(const F64vec4 &_A) { return *this = _mm256_and_pd(vec,_A); }
    F64vec4& operator |=(const F64vec4 &_A) { return *this = _mm256_or_pd(vec,_A); }
    F64vec4& operator ^=(const F64vec4 &_A) { return *this = _mm256_xor_pd(vec,_A); }

    /* Horizontal Add */
    friend double add_horizontal(const F64vec4 &_A)
    {
        F64vec4 _Temp = _mm256_add_pd(_A, _mm256_permute_pd(_A,0x05));
        return _mm_cvtsd_f64(_mm_add_sd(_mm256_castpd256_pd128(_Temp), _mm256_extractf128_pd(_Temp,1)));
    }

    /* And Not */
    friend F64vec4 andnot(const F64vec4 &_A, const F64vec4 &_B) { return _mm256_andnot_pd(_A,_B); }

    /* Square Root */
    friend F64vec4 sqrt(const F64vec4 &_A) { return _mm256_sqrt_pd(_A); }

    /* Compares: Mask is returned  */
    friend F64vec4 cmp_eq(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_EQ_OQ); }
    friend F64vec4 cmp_lt(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_LT_OS); }
    friend F64vec4 cmp_le(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_LE_OS); }
    friend F64vec4 cmp_gt(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_GT_OS); }
    friend F64vec4 cmp_ge(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_GE_OS); }
    friend F64vec4 cmp_neq(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_NEQ_UQ); }
    friend F64vec4 cmp_nlt(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_NLT_US); }
    friend F64vec4 cmp_nle(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_NLE_US); }
    friend F64vec4 cmp_ngt(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_NGT_US); }
    friend F64vec4 cmp_nge(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_cmp_pd(_A, _B, _CMP_NGE_US); }

    /* Min and Max */
    friend F64vec4 simd_min(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_min_pd(_A,_B); }
    friend F64vec4 simd_max(const F64vec4 &_A, const F64vec4 &_B)
        { return _mm256_max_pd(_A,_B); }

    /* Absolute value */
    friend F64vec4 abs(const F64vec4 &_A)
    {
        static const union
        {
            int i[8];
            __m256d m;
        } __f64vec4_abs_mask = { -1, 0x7fffffff, -1, 0x7fffffff,
                                 -1, 0x7fffffff, -1, 0x7fffffff};
        return _mm256_and_pd(_A, __f64vec4_abs_mask.m);
    }

    /* Debug Features */
#if defined (_ENABLE_VEC_DEBUG)
    /* Output */
    friend DVEC_STD ostream & operator<<(DVEC_STD ostream &_Os, const F64vec4 &_A)
    {
        /* To use: cout << "Elements of F64vec4 fvec are: " << fvec; */
        double *_Dp = (double*) &_A;
        _Os <<  "[3]:" << *(_Dp+3)
        << " [2]:" << *(_Dp+2)
        << " [3]:" << *(_Dp+1)
        << " [0]:" << *_Dp;
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access Only, no modifications to elements */
    const double& operator[](int _I) const
    {
        /* Assert enabled only during debug /DDEBUG */
        _VEC_ASSERT((0 <= _I) && (_I <= 3));
        double *_Dp = (double*)&vec;
        return *(_Dp+ _I);
    }
    /* Element Access and Modification*/
    double& operator[](int _I)
    {
        /* Assert enabled only during debug /DDEBUG */
        _VEC_ASSERT((0 <= _I) && (_I <= 3));
        double *_Dp = (double*)&vec;
        return *(_Dp+ _I);
    }
};

            /* Miscellaneous */

/* Interleave low order data elements of a and b into destination */
inline F64vec4 unpack_low(const F64vec4 &_A, const F64vec4 &_B){
    return _mm256_unpacklo_pd(_A, _B); }

/* Interleave high order data elements of a and b into target */
inline F64vec4 unpack_high(const F64vec4 &_A, const F64vec4 &_B){
    return _mm256_unpackhi_pd(_A, _B); }

/* Move Mask to Integer returns 4 bit mask formed of most significant bits of a */
inline int move_mask(const F64vec4 &_A){
    return _mm256_movemask_pd(_A); }

            /* Data Motion Functions */

/* Load Unaligned loadu_pd: Unaligned */
inline void loadu(F64vec4 &_A, double *_P){
    _A = _mm256_loadu_pd(_P); }

/* Store Unaligned storeu_pd: Unaligned */
inline void storeu(double *_P, const F64vec4 &_A){
    _mm256_storeu_pd(_P, _A); }

            /* Cacheability Support */

/* Non-Temporal Store */
inline void store_nta(double *_P, const F64vec4 &_A){
    _mm256_stream_pd(_P, _A); }

            /* Conditional moves */

/* Masked load */
inline void maskload(F64vec4 &_A, const double *_P, const F64vec4 &_M){
    _A = _mm256_maskload_pd(_P, _mm256_castpd_si256(_M)); }

inline void maskload(F64vec2 &_A, const double *_P, const F64vec2 &_M){
    _A = _mm_maskload_pd(_P, _mm_castpd_si128(_M)); }

/* Masked store */
inline void maskstore(double *_P, const F64vec4 &_A, const F64vec4 &_M){
    _mm256_maskstore_pd(_P, _mm256_castpd_si256(_M), _A); }

inline void maskstore(double *_P, const F64vec2 &_A, const F64vec2 &_M){
    _mm_maskstore_pd(_P, _mm_castpd_si128(_M), _A); }

            /* Conditional Selects */

inline F64vec4 select_eq(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_EQ_OQ)); }

inline F64vec4 select_lt(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_LT_OS)); }

inline F64vec4 select_le(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_LE_OS)); }

inline F64vec4 select_gt(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_GT_OS)); }

inline F64vec4 select_ge(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_GE_OS)); }

inline F64vec4 select_neq(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_NEQ_UQ)); }

inline F64vec4 select_nlt(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_NLT_US)); }

inline F64vec4 select_nle(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_NLE_US)); }

inline F64vec4 select_ngt(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_NGT_US)); }

inline F64vec4 select_nge(const F64vec4 &_A, const F64vec4 &_B, const F64vec4 &_C, const F64vec4 &_D){
    return _mm256_blendv_pd(_D, _C, _mm256_cmp_pd(_A, _B, _CMP_NGE_US)); }

            /* Conversion Functions */

/* Convert the 4 SP FP values of a to 4 DP FP values */
inline F64vec4 F32vec4ToF64vec4(const F32vec4 &_A){
    return _mm256_cvtps_pd(_A); }

/* Convert the 4 DP FP values of a to 4 SP FP values */
inline F32vec4 F64vec4ToF32vec8(const F64vec4 &_A){
    return _mm256_cvtpd_ps(_A); }

#undef DVEC_DEFINE_OUTPUT_OPERATORS
#undef DVEC_STD

#pragma pack(pop)

#endif /* defined (_M_CEE_PURE) */
#endif /* _VCRT_COMPILER_PREPROCESSOR */
#endif /* _DVEC_H_INCLUDED */
