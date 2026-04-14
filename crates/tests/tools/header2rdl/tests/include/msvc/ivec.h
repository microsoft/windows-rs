/*
 *  Copyright (C) 1985-2015 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/*
 *  Definition of a C++ class interface to MMX(TM) instruction intrinsics.
 *
 */

#ifndef _IVEC_H_INCLUDED
#define _IVEC_H_INCLUDED
#include <vcruntime.h>
#if _VCRT_COMPILER_PREPROCESSOR

#if !defined __cplusplus
    #error ERROR: This file is only supported in C++ compilations!
#endif  /* !defined __cplusplus */

#if defined (_M_CEE_PURE)
    #error ERROR: This file is not supported in the pure mode!
#else  /* defined (_M_CEE_PURE) */

#include <intrin.h>

#ifndef _VEC_ASSERT
    #include <corecrt.h>

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

/*
 * Define _SILENCE_IVEC_C4799 to disable warning C4799 inside this header.
 * Be careful that any code that uses these functions properly executes EMMS
 * or _m_empty() after using any MMX instruction and before using the x87 NDP.
 */
#if defined (_SILENCE_IVEC_C4799)
    #pragma warning(push)
    #pragma warning(disable: 4799)
#endif  /* defined (_SILENCE_IVEC_C4799) */

/*
 * Define _ENABLE_VEC_DEBUG to enable std::ostream inserters for debug output
 */
#if defined (_ENABLE_VEC_DEBUG)
    #include <iostream>
#endif  /* defined (_ENABLE_VEC_DEBUG) */

class I8vec8;           /* 8 elements, each element a signed or unsigned char data type */
class Is8vec8;          /* 8 elements, each element a signed char data type */
class Iu8vec8;          /* 8 elements, each element an unsigned char data type */
class I16vec4;          /* 4 elements, each element a signed or unsigned short */
class Is16vec4;         /* 4 elements, each element a signed short */
class Iu16vec4;         /* 4 elements, each element an unsigned short */
class I32vec2;          /* 2 elements, each element a signed or unsigned long */
class Is32vec2;         /* 2 elements, each element a signed long */
class Iu32vec2;         /* 2 elements, each element a unsigned long */
class I64vec1;          /* 1 element, a __m64 data type - Base I64vec1 class  */

#define _MM_8UB(element,vector) (*((unsigned char*)&##vector + ##element))
#define _MM_8B(element,vector) (*((signed char*)&##vector + ##element))

#define _MM_4UW(element,vector) (*((unsigned short*)&##vector + ##element))
#define _MM_4W(element,vector) (*((short*)&##vector + ##element))

#define _MM_2UDW(element,vector) (*((unsigned int*)&##vector + ##element))
#define _MM_2DW(element,vector) (*((int*)&##vector + ##element))

#define _MM_QW (*((__int64*)&vec))

#if defined(_M_IX86)
/* M64 Class:
 * 1 element, a __m64 data type
 * Contructors & Logical Operations
 */
class M64
{
protected:
        __m64 vec;

public:
    M64()                                   { }
    M64(__m64 _Mm)                           { vec = _Mm; }
    M64(__int64 _Mm)                         { vec = _mm_set_pi32((int)(_Mm >> 32), (int)_Mm); }
    M64(int _I)                              { vec = _m_from_int(_I); }

    operator __m64() const                  { return vec; }

    /* Logical Operations */
    M64& operator&=(const M64 &_A)                   { return *this = (M64) _m_pand(vec,_A); }
    M64& operator|=(const M64 &_A)                   { return *this = (M64) _m_por(vec,_A); }
    M64& operator^=(const M64 &_A)                   { return *this = (M64) _m_pxor(vec,_A); }

};

const  union {__int64 m1; __m64 m2;} __mmx_all_ones_cheat =
    {-1};

#define _mmx_all_ones ((M64)__mmx_all_ones_cheat.m2)

inline M64 operator&(const M64 &_A, const M64 &_B)    { return _m_pand(_A,_B); }
inline M64 operator|(const M64 &_A, const M64 &_B)    { return _m_por(_A,_B); }
inline M64 operator^(const M64 &_A, const M64 &_B)    { return _m_pxor(_A,_B); }
inline M64 andnot(const M64 &_A, const M64 &_B)       { return _m_pandn(_A,_B); }

/* I64vec1 Class:
 * 1 element, a __m64 data type
 * Contains Operations which can operate on any __m64 data type
 */

class I64vec1 : public M64
{
public:
    I64vec1()                               { }
    I64vec1(__m64 _Mm) : M64(_Mm)             { }
    explicit I64vec1(int _I) : M64(_I)        { }
    explicit I64vec1(__int64 _Mm) : M64(_Mm)  { }

    I64vec1& operator= (const M64 &_A) { return *this = (I64vec1) _A; }
    I64vec1& operator&=(const M64 &_A) { return *this = (I64vec1) _m_pand(vec,_A); }
    I64vec1& operator|=(const M64 &_A) { return *this = (I64vec1) _m_por(vec,_A); }
    I64vec1& operator^=(const M64 &_A) { return *this = (I64vec1) _m_pxor(vec,_A); }

    /* Shift Logical Operations */
    I64vec1 operator<<(const M64 &_A)                { return _m_psllq(vec,_A); }
    I64vec1 operator<<(int _Count)                   { return _m_psllqi(vec, _Count); }
    I64vec1& operator<<=(const M64 &_A)              { return *this = (I64vec1) _m_psllq(vec,_A); }
    I64vec1& operator<<=(int _Count)                 { return *this = (I64vec1) _m_psllqi(vec, _Count); }
    I64vec1 operator>>(const M64 &_A)                { return _m_psrlq(vec,_A); }
    I64vec1 operator>>(int _Count)                   { return _m_psrlqi(vec, _Count); }
    I64vec1& operator>>=(const M64 &_A)              { return *this = (I64vec1) _m_psrlq(vec,_A); }
    I64vec1& operator>>=(int _Count)                 { return *this = (I64vec1) _m_psrlqi(vec, _Count); }
};

/* I32vec2 Class:
 * 2 elements, each element either a signed or unsigned int
 */
class I32vec2 : public M64
{
public:
    I32vec2() { }
    I32vec2(__m64 _Mm) : M64(_Mm) { }
    I32vec2(int _I0, int _I1) { vec = _mm_set_pi32(_I0, _I1); }
    explicit I32vec2(int _I) : M64 (_I) { }
    explicit I32vec2(__int64 _I): M64(_I) {}

    /* Assignment Operator */
    I32vec2& operator= (const M64 &_A) { return *this = (I32vec2)_A; }

    /* Logical Assignment Operators */
    I32vec2& operator&=(const M64 &_A) { return *this = (I32vec2) _m_pand(vec,_A); }
    I32vec2& operator|=(const M64 &_A) { return *this = (I32vec2) _m_por(vec,_A); }
    I32vec2& operator^=(const M64 &_A) { return *this = (I32vec2) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    I32vec2& operator +=(const I32vec2 &_A)          { return *this = (I32vec2) _m_paddd(vec,_A); }
    I32vec2& operator -=(const I32vec2 &_A)          { return *this = (I32vec2) _m_psubd(vec,_A); }

    /* Shift Logical Operators */
    I32vec2 operator<<(const I32vec2 &_A)            { return _m_pslld(vec,_A); }
    I32vec2 operator<<(int _Count)                   { return _m_pslldi(vec,_Count); }
    I32vec2& operator<<=(const I32vec2 &_A)          { return *this = (I32vec2) _m_pslld(vec,_A); }
    I32vec2& operator<<=(int _Count)                 { return *this = (I32vec2) _m_pslldi(vec,_Count); }

};

/* Compare For Equality */
inline I32vec2 cmpeq(const I32vec2 &_A, const I32vec2 &_B)        { return _m_pcmpeqd(_A,_B); }
inline I32vec2 cmpneq(const I32vec2 &_A, const I32vec2 &_B)       { return _m_pandn(_m_pcmpeqd(_A,_B), _mmx_all_ones); }
/* Unpacks */
inline I32vec2 unpack_low(const I32vec2 &_A, const I32vec2 &_B)   {return _m_punpckldq(_A,_B); }
inline I32vec2 unpack_high(const I32vec2 &_A, const I32vec2 &_B)  {return _m_punpckhdq(_A,_B); }

/* Is32vec2 Class:
 * 2 elements, each element a signed int
 */
class Is32vec2 : public I32vec2
{
public:
    Is32vec2() { }
    Is32vec2(__m64 _Mm) : I32vec2(_Mm) { }
    Is32vec2(signed int _I0, signed int _I1) : I32vec2(_I0, _I1) {}
    explicit Is32vec2(int _I) : I32vec2 (_I)      {}
    explicit Is32vec2(__int64 _I): I32vec2(_I)    {}

    /* Assignment Operator */
    Is32vec2& operator= (const M64 &_A)      { return *this = (Is32vec2)_A; }

    /* Logical Assignment Operators */
    Is32vec2& operator&=(const M64 &_A)      { return *this = (Is32vec2) _m_pand(vec,_A); }
    Is32vec2& operator|=(const M64 &_A)      { return *this = (Is32vec2) _m_por(vec,_A); }
    Is32vec2& operator^=(const M64 &_A)      { return *this = (Is32vec2) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Is32vec2& operator +=(const I32vec2 &_A) { return *this = (Is32vec2) _m_paddd(vec,_A); }
    Is32vec2& operator -=(const I32vec2 &_A) { return *this = (Is32vec2) _m_psubd(vec,_A); }

    /* Shift Logical Operators */
    Is32vec2 operator<<(const M64 &_A)       { return _m_pslld(vec,_A); }
    Is32vec2 operator<<(int _Count)          { return _m_pslldi(vec,_Count); }
    Is32vec2& operator<<=(const M64 &_A)     { return *this = (Is32vec2) _m_pslld(vec,_A); }
    Is32vec2& operator<<=(int _Count)        { return *this = (Is32vec2) _m_pslldi(vec,_Count); }
    /* Shift Arithmetic Operations */
    Is32vec2 operator>>(const M64 &_A)       { return _m_psrad(vec, _A); }
    Is32vec2 operator>>(int _Count)          { return _m_psradi(vec, _Count); }
    Is32vec2& operator>>=(const M64 &_A)     { return *this = (Is32vec2) _m_psrad(vec, _A); }
    Is32vec2& operator>>=(int _Count)        { return *this = (Is32vec2) _m_psradi(vec, _Count); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Is32vec2 &_A)
    {
        _Os << " [1]:" << _MM_2DW(1,_A)
        << " [0]:" << _MM_2DW(0,_A);
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const int& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 2);  /* Only 2 elements to access */
        return _MM_2DW(_I,vec);
    }

    /* Element Access and Assignment for Debug */
    int& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 2);  /* Only 2 elements to access */
        return _MM_2DW(_I,vec);
    }
};

/* Compares */
inline Is32vec2 cmpeq(const Is32vec2 &_A, const Is32vec2 &_B)         { return _m_pcmpeqd(_A,_B); }
inline Is32vec2 cmpneq(const Is32vec2 &_A, const Is32vec2 &_B)        { return _m_pandn(_m_pcmpeqd(_A,_B), _mmx_all_ones); }
inline Is32vec2 cmpgt(const Is32vec2 &_A, const Is32vec2 &_B)         { return _m_pcmpgtd(_A,_B); }
inline Is32vec2 cmplt(const Is32vec2 &_A, const Is32vec2 &_B)         { return _m_pcmpgtd(_B,_A); }
inline Is32vec2 cmple(const Is32vec2 &_A, const Is32vec2 &_B)         { return _m_pandn(_m_pcmpgtd(_A,_B), _mmx_all_ones); }
inline Is32vec2 cmpge(const Is32vec2 &_A, const Is32vec2 &_B)         { return _m_pandn(_m_pcmpgtd(_B,_A), _mmx_all_ones); }
/* Unpacks & Pack */
inline Is32vec2 unpack_low(const Is32vec2 &_A, const Is32vec2 &_B)    { return _m_punpckldq(_A,_B); }
inline Is32vec2 unpack_high(const Is32vec2 &_A, const Is32vec2 &_B)   { return _m_punpckhdq(_A,_B); }

/* Iu32vec2 Class:
 * 2 elements, each element unsigned int
 */
class Iu32vec2 : public I32vec2
{
public:
    Iu32vec2() { }
    Iu32vec2(__m64 _Mm) : I32vec2(_Mm) { }
    Iu32vec2(unsigned int _I0, unsigned int _I1) : I32vec2(_I0, _I1) {}
    explicit Iu32vec2(int _I) : I32vec2 (_I)      { }
    explicit Iu32vec2(__int64 _I) : I32vec2 (_I)  { }

    /* Assignment Operator */
    Iu32vec2& operator= (const M64 &_A)      { return *this = (Iu32vec2) _A; }

    /* Logical Assignment Operators */
    Iu32vec2& operator&=(const M64 &_A)      { return *this = (Iu32vec2) _m_pand(vec,_A); }
    Iu32vec2& operator|=(const M64 &_A)      { return *this = (Iu32vec2) _m_por(vec,_A); }
    Iu32vec2& operator^=(const M64 &_A)      { return *this = (Iu32vec2) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Iu32vec2& operator +=(const I32vec2 &_A) { return *this = (Iu32vec2) _m_paddd(vec,_A); }
    Iu32vec2& operator -=(const I32vec2 &_A) { return *this = (Iu32vec2) _m_psubd(vec,_A); }

    /* Shift Logical Operators */
    Iu32vec2 operator<<(const M64 &_A)       { return _m_pslld(vec,_A); }
    Iu32vec2 operator<<(int _Count)          { return _m_pslldi(vec,_Count); }
    Iu32vec2& operator<<=(const M64 &_A)     { return *this = (Iu32vec2) _m_pslld(vec,_A); }
    Iu32vec2& operator<<=(int _Count)        { return *this = (Iu32vec2) _m_pslldi(vec,_Count); }
    Iu32vec2 operator>>(const M64 &_A)       { return _m_psrld(vec,_A); }
    Iu32vec2 operator>>(int _Count)          { return _m_psrldi(vec,_Count); }
    Iu32vec2& operator>>=(const M64 &_A)     { return *this = (Iu32vec2) _m_psrld(vec,_A); }
    Iu32vec2& operator>>=(int _Count)        { return *this = (Iu32vec2) _m_psrldi(vec,_Count); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Iu32vec2 &_A)
    {
        _Os << "[1]:" << _MM_2UDW(1,_A)
        << " [0]:" << _MM_2UDW(0,_A);
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const unsigned int& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 2);  /* Only 2 elements to access */
        return _MM_2UDW(_I,vec);
    }

    /* Element Access and Assignment for Debug */
    unsigned int& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 2);  /* Only 2 elements to access */
        return _MM_2UDW(_I,vec);
    }
};

/* Compares For Equality / Inequality */
inline Iu32vec2 cmpeq(const Iu32vec2 &_A, const Iu32vec2 &_B)         { return _m_pcmpeqd(_A,_B); }
inline Iu32vec2 cmpneq(const Iu32vec2 &_A, const Iu32vec2 &_B)        { return _m_pandn(_m_pcmpeqd(_A,_B), _mmx_all_ones); }
/* Unpacks */
inline Iu32vec2 unpack_low(const Iu32vec2 &_A, const Iu32vec2 &_B)    {return _m_punpckldq(_A,_B); }
inline Iu32vec2 unpack_high(const Iu32vec2 &_A, const Iu32vec2 &_B)   {return _m_punpckhdq(_A,_B); }

/* I16vec4 Class:
 * 4 elements, each element either a signed or unsigned short
 */
class I16vec4 : public M64
{
public:
    I16vec4() { }
    I16vec4(__m64 _Mm) : M64(_Mm) { }
    I16vec4(short _I0, short _I1, short _I2, short _I3)
    {
        vec = _mm_set_pi16(_I0, _I1, _I2, _I3);
    }
    explicit I16vec4(__int64 _I) : M64 (_I) { }
    explicit I16vec4(int _I) : M64 (_I) { }

    /* Assignment Operator */
    I16vec4& operator= (const M64 &_A)               { return *this = (I16vec4) _A; }

    /* Addition & Subtraction Assignment Operators */
    I16vec4& operator&=(const M64 &_A)               { return *this = (I16vec4) _m_pand(vec,_A); }
    I16vec4& operator|=(const M64 &_A)               { return *this = (I16vec4) _m_por(vec,_A); }
    I16vec4& operator^=(const M64 &_A)               { return *this = (I16vec4) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    I16vec4& operator +=(const I16vec4 &_A)          { return *this = (I16vec4)_m_paddw(vec,_A); }
    I16vec4& operator -=(const I16vec4 &_A)          { return *this = (I16vec4)_m_psubw(vec,_A); }
    I16vec4& operator *=(const I16vec4 &_A)          { return *this = (I16vec4)_m_pmullw(vec,_A); }

    /* Shift Logical Operators */
    I16vec4 operator<<(const I16vec4 &_A)            { return _m_psllw(vec,_A); }
    I16vec4 operator<<(int _Count)                   { return _m_psllwi(vec,_Count); }
    I16vec4& operator<<=(const I16vec4 &_A)          { return *this = (I16vec4)_m_psllw(vec,_A); }
    I16vec4& operator<<=(int _Count)                 { return *this = (I16vec4)_m_psllwi(vec,_Count); }
};

inline I16vec4 operator*(const I16vec4 &_A, const I16vec4 &_B)    { return _m_pmullw(_A,_B); }
inline I16vec4 cmpeq(const I16vec4 &_A, const I16vec4 &_B)        { return _m_pcmpeqw(_A,_B); }
inline I16vec4 cmpneq(const I16vec4 &_A, const I16vec4 &_B)       { return _m_pandn(_m_pcmpeqw(_A,_B), _mmx_all_ones); }

inline I16vec4 unpack_low(const I16vec4 &_A, const I16vec4 &_B)   { return _m_punpcklwd(_A,_B); }
inline I16vec4 unpack_high(const I16vec4 &_A, const I16vec4 &_B)  { return _m_punpckhwd(_A,_B); }

/* Is16vec4 Class:
 * 4 elements, each element signed short
 */
class Is16vec4 : public I16vec4
{
public:
    Is16vec4() { }
    Is16vec4(__m64 _Mm) : I16vec4(_Mm) { }
    Is16vec4(short _I0, short _I1, short _I2, short _I3) : I16vec4(_I0, _I1, _I2, _I3) { }
    explicit Is16vec4(__int64 _I) : I16vec4 (_I)  { }
    explicit Is16vec4(int _I) : I16vec4 (_I)      { }

    /* Assignment Operator */
    Is16vec4& operator= (const M64 &_A)      { return *this = (Is16vec4) _A; }

    /* Addition & Subtraction Assignment Operators */
    Is16vec4& operator&=(const M64 &_A)      { return *this = (Is16vec4) _m_pand(vec,_A); }
    Is16vec4& operator|=(const M64 &_A)      { return *this = (Is16vec4) _m_por(vec,_A); }
    Is16vec4& operator^=(const M64 &_A)      { return *this = (Is16vec4) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Is16vec4& operator +=(const I16vec4 &_A) { return *this = (Is16vec4)_m_paddw(vec,_A); }
    Is16vec4& operator -=(const I16vec4 &_A) { return *this = (Is16vec4)_m_psubw(vec,_A); }
    Is16vec4& operator *=(const I16vec4 &_A) { return *this = (Is16vec4)_m_pmullw(vec,_A); }

    /* Shift Logical Operators */
    Is16vec4 operator<<(const M64 &_A)       { return _m_psllw(vec,_A); }
    Is16vec4 operator<<(int _Count)          { return _m_psllwi(vec,_Count); }
    Is16vec4& operator<<=(const M64 &_A)     { return *this = (Is16vec4)_m_psllw(vec,_A); }
    Is16vec4& operator<<=(int _Count)        { return *this = (Is16vec4)_m_psllwi(vec,_Count); }
    /* Shift Arithmetic Operations */
    Is16vec4 operator>>(const M64 &_A)       { return _m_psraw(vec,_A); }
    Is16vec4 operator>>(int _Count)          { return _m_psrawi(vec,_Count); }
    Is16vec4& operator>>=(const M64 &_A)     { return *this = (Is16vec4) _m_psraw(vec,_A); }
    Is16vec4& operator>>=(int _Count)        { return *this = (Is16vec4) _m_psrawi(vec,_Count); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Is16vec4 &_A)
    {
        _Os << "[3]:" << _MM_4W(3,_A)
            << " [2]:" << _MM_4W(2,_A)
            << " [1]:" << _MM_4W(1,_A)
            << " [0]:" << _MM_4W(0,_A);
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const short& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4W(_I,vec);
    }

    /* Element Access for Debug */
    short& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4W(_I,vec);
    }
};

inline Is16vec4 operator*(const Is16vec4 &_A, const Is16vec4 &_B)     { return _m_pmullw(_A,_B); }

/* Compares */
inline Is16vec4 cmpeq(const Is16vec4 &_A, const Is16vec4 &_B)         { return _m_pcmpeqw(_A,_B); }
inline Is16vec4 cmpneq(const Is16vec4 &_A, const Is16vec4 &_B)        { return _m_pandn(_m_pcmpeqw(_A,_B), _mmx_all_ones); }
inline Is16vec4 cmpgt(const Is16vec4 &_A, const Is16vec4 &_B)         { return _m_pcmpgtw(_A,_B); }
inline Is16vec4 cmplt(const Is16vec4 &_A, const Is16vec4 &_B)         { return _m_pcmpgtw(_B,_A); }
inline Is16vec4 cmple(const Is16vec4 &_A, const Is16vec4 &_B)         { return _m_pandn(_m_pcmpgtw(_A,_B), _mmx_all_ones); }
inline Is16vec4 cmpge(const Is16vec4 &_A, const Is16vec4 &_B)         { return _m_pandn(_m_pcmpgtw(_B,_A), _mmx_all_ones); }
/* Unpacks */
inline Is16vec4 unpack_low(const Is16vec4 &_A, const Is16vec4 &_B)    { return _m_punpcklwd(_A,_B); }
inline Is16vec4 unpack_high(const Is16vec4 &_A, const Is16vec4 &_B)   { return _m_punpckhwd(_A,_B); }

inline Is16vec4 sat_add(const Is16vec4 &_A, const Is16vec4 &_B)       { return _m_paddsw(_A,_B); }
inline Is16vec4 sat_sub(const Is16vec4 &_A, const Is16vec4 &_B)       { return _m_psubsw(_A,_B); }
inline Is16vec4 mul_high(const Is16vec4 &_A, const Is16vec4 &_B)      { return _m_pmulhw(_A,_B); }
inline Is32vec2 mul_add(const Is16vec4 &_A, const Is16vec4 &_B)       { return _m_pmaddwd(_A,_B);}


/* Iu16vec4 Class:
 * 4 elements, each element unsigned short
 */
class Iu16vec4 : public I16vec4
{
public:
    Iu16vec4() { }
    Iu16vec4(__m64 _Mm) : I16vec4(_Mm) { }
    Iu16vec4(unsigned short _Ui0, unsigned short _Ui1,
        unsigned short _Ui2, unsigned short _Ui3)
        : I16vec4(_Ui0, _Ui1, _Ui2, _Ui3) { }
    explicit Iu16vec4(__int64 _I) : I16vec4 (_I) { }
    explicit Iu16vec4(int _I) : I16vec4 (_I) { }

    /* Assignment Operator */
    Iu16vec4& operator= (const M64 &_A)      { return *this = (Iu16vec4) _A; }

    /* Logical Assignment Operators */
    Iu16vec4& operator&=(const M64 &_A)      { return *this = (Iu16vec4) _m_pand(vec,_A); }
    Iu16vec4& operator|=(const M64 &_A)      { return *this = (Iu16vec4) _m_por(vec,_A); }
    Iu16vec4& operator^=(const M64 &_A)      { return *this = (Iu16vec4) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Iu16vec4& operator +=(const I16vec4 &_A) { return *this = (Iu16vec4)_m_paddw(vec,_A); }
    Iu16vec4& operator -=(const I16vec4 &_A) { return *this = (Iu16vec4)_m_psubw(vec,_A); }
    Iu16vec4& operator *=(const I16vec4 &_A) { return *this = (Iu16vec4)_m_pmullw(vec,_A); }

    /* Shift Logical Operators */
    Iu16vec4 operator<<(const M64 &_A)               { return _m_psllw(vec,_A); }
    Iu16vec4 operator<<(int _Count)                  { return _m_psllwi(vec,_Count); }
    Iu16vec4& operator<<=(const M64 &_A)             { return *this = (Iu16vec4)_m_psllw(vec,_A); }
    Iu16vec4& operator<<=(int _Count)                { return *this = (Iu16vec4)_m_psllwi(vec,_Count); }
    Iu16vec4 operator>>(const M64 &_A)               { return _m_psrlw(vec,_A); }
    Iu16vec4 operator>>(int _Count)                  { return _m_psrlwi(vec,_Count); }
    Iu16vec4& operator>>=(const M64 &_A)             { return *this = (Iu16vec4) _m_psrlw(vec,_A); }
    Iu16vec4& operator>>=(int _Count)                { return *this = (Iu16vec4) _m_psrlwi(vec,_Count); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Iu16vec4 &_A)
    {
        _Os << "[3]:" << _MM_4UW(3,_A)
            << " [2]:" << _MM_4UW(2,_A)
            << " [1]:" << _MM_4UW(1,_A)
            << " [0]:" << _MM_4UW(0,_A);
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const unsigned short& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4UW(_I,vec);
    }

    /* Element Access and Assignment for Debug */
    unsigned short& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 4);  /* Only 4 elements to access */
        return _MM_4UW(_I,vec);
    }
};

inline Iu16vec4 operator*(const Iu16vec4 &_A, const Iu16vec4 &_B)     { return _m_pmullw(_A,_B); }
inline Iu16vec4 cmpeq(const Iu16vec4 &_A, const Iu16vec4 &_B)         { return _m_pcmpeqw(_A,_B); }
inline Iu16vec4 cmpneq(const Iu16vec4 &_A, const Iu16vec4 &_B)        { return _m_pandn(_m_pcmpeqw(_A,_B), _mmx_all_ones); }

inline Iu16vec4 sat_add(const Iu16vec4 &_A, const Iu16vec4 &_B)   { return _m_paddusw(_A,_B); }
inline Iu16vec4 sat_sub(const Iu16vec4 &_A, const Iu16vec4 &_B)   { return _m_psubusw(_A,_B); }

inline Iu16vec4 unpack_low(const Iu16vec4 &_A, const Iu16vec4 &_B)    { return _m_punpcklwd(_A,_B); }
inline Iu16vec4 unpack_high(const Iu16vec4 &_A, const Iu16vec4 &_B)   { return _m_punpckhwd(_A,_B); }

/* I8vec8 Class:
 * 8 elements, each element either unsigned or signed char
 */
class I8vec8 : public M64
{
public:
    I8vec8() { }
    I8vec8(__m64 _Mm) : M64(_Mm) { }
    I8vec8(char _S0, char _S1, char _S2, char _S3, char _S4, char _S5, char _S6, char _S7)
    {
        vec = _mm_set_pi8(_S0, _S1, _S2, _S3, _S4, _S5, _S6, _S7);
    }
    explicit I8vec8(__int64 _I) : M64 (_I) { }
    explicit I8vec8(int _I) : M64 (_I) { }

    /* Assignment Operator */
    I8vec8& operator= (const M64 &_A)        { return *this = (I8vec8) _A; }

    /* Logical Assignment Operators */
    I8vec8& operator&=(const M64 &_A)        { return *this = (I8vec8) _m_pand(vec,_A); }
    I8vec8& operator|=(const M64 &_A)        { return *this = (I8vec8) _m_por(vec,_A); }
    I8vec8& operator^=(const M64 &_A)        { return *this = (I8vec8) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    I8vec8& operator +=(const I8vec8 &_A)    { return *this = (I8vec8) _m_paddb(vec,_A); }
    I8vec8& operator -=(const I8vec8 &_A)    { return *this = (I8vec8) _m_psubb(vec,_A); }
};


inline I8vec8 cmpeq(const I8vec8 &_A, const I8vec8 &_B)       { return _m_pcmpeqb(_A,_B); }
inline I8vec8 cmpneq(const I8vec8 &_A, const I8vec8 &_B)      { return _m_pandn(_m_pcmpeqb(_A,_B), _mmx_all_ones); }

inline I8vec8 unpack_low(const I8vec8 &_A, const I8vec8 &_B)  { return _m_punpcklbw(_A,_B); }
inline I8vec8 unpack_high(const I8vec8 &_A, const I8vec8 &_B) { return _m_punpckhbw(_A,_B); }

/* Is8vec8 Class:
 * 8 elements, each element signed char
 */
class Is8vec8 : public I8vec8
{
public:
    Is8vec8() { }
    Is8vec8(__m64 _Mm) : I8vec8(_Mm) { }
    Is8vec8(signed char _S0, signed char _S1, signed char _S2, signed char _S3,
        signed char _S4, signed char _S5, signed char _S6, signed char _S7)
        : I8vec8(_S0, _S1, _S2, _S3, _S4, _S5, _S6, _S7) { }
    explicit Is8vec8(__int64 _I) : I8vec8 (_I) { }
    explicit Is8vec8(int _I) : I8vec8 (_I) { }

    /* Assignment Operator */
    Is8vec8& operator= (const M64 &_A)       { return *this = (Is8vec8) _A; }

    /* Logical Assignment Operators */
    Is8vec8& operator&=(const M64 &_A)       { return *this = (Is8vec8) _m_pand(vec,_A); }
    Is8vec8& operator|=(const M64 &_A)       { return *this = (Is8vec8) _m_por(vec,_A); }
    Is8vec8& operator^=(const M64 &_A)       { return *this = (Is8vec8) _m_pxor(vec,_A); }

    /* Addition & Subtraction Assignment Operators */
    Is8vec8& operator +=(const I8vec8 &_A)   { return *this = (Is8vec8) _m_paddb(vec,_A); }
    Is8vec8& operator -=(const I8vec8 &_A)   { return *this = (Is8vec8) _m_psubb(vec,_A); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator<< (std::ostream &_Os, const Is8vec8 &_A)
    {
        _Os << "[7]:" << short(_MM_8B(7,_A))
            << " [6]:" << short(_MM_8B(6,_A))
            << " [5]:" << short(_MM_8B(5,_A))
            << " [4]:" << short(_MM_8B(4,_A))
            << " [3]:" << short(_MM_8B(3,_A))
            << " [2]:" << short(_MM_8B(2,_A))
            << " [1]:" << short(_MM_8B(1,_A))
            << " [0]:" << short(_MM_8B(0,_A));
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const signed char& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8B(_I,vec);
    }

    /* Element Access and Assignment for Debug */
    signed char& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8B(_I,vec);
    }
};

/* Additional Is8vec8 functions: compares, unpacks, sat add/sub */
inline Is8vec8 cmpeq(const Is8vec8 &_A, const Is8vec8 &_B)        { return _m_pcmpeqb(_A,_B); }
inline Is8vec8 cmpneq(const Is8vec8 &_A, const Is8vec8 &_B)       { return _m_pandn(_m_pcmpeqb(_A,_B), _mmx_all_ones); }
inline Is8vec8 cmpgt(const Is8vec8 &_A, const Is8vec8 &_B)        { return _m_pcmpgtb(_A,_B); }
inline Is8vec8 cmplt(const Is8vec8 &_A, const Is8vec8 &_B)        { return _m_pcmpgtb(_B,_A); }
inline Is8vec8 cmple(const Is8vec8 &_A, const Is8vec8 &_B)        { return _m_pandn(_m_pcmpgtb(_A,_B), _mmx_all_ones); }
inline Is8vec8 cmpge(const Is8vec8 &_A, const Is8vec8 &_B)        { return _m_pandn(_m_pcmpgtb(_B,_A), _mmx_all_ones); }

inline Is8vec8 unpack_low(const Is8vec8 &_A, const Is8vec8 &_B)   { return _m_punpcklbw(_A,_B); }
inline Is8vec8 unpack_high(const Is8vec8 &_A, const Is8vec8 &_B)  { return _m_punpckhbw(_A,_B); }

inline Is8vec8 sat_add(const Is8vec8 &_A, const Is8vec8 &_B)      { return _m_paddsb(_A,_B); }
inline Is8vec8 sat_sub(const Is8vec8 &_A, const Is8vec8 &_B)      { return _m_psubsb(_A,_B); }

/* Iu8vec8 Class:
 * 8 elements, each element unsigned char
 */
class Iu8vec8 : public I8vec8
{
public:
    Iu8vec8() { }
    Iu8vec8(__m64 _Mm) : I8vec8(_Mm) { }
    Iu8vec8(unsigned char _S0, unsigned char _S1, unsigned char _S2,
        unsigned char _S3, unsigned char _S4, unsigned char _S5,
        unsigned char _S6, unsigned char _S7)
        : I8vec8(_S0, _S1, _S2, _S3, _S4, _S5, _S6, _S7) { }
    explicit Iu8vec8(__int64 _I) : I8vec8 (_I) { }
    explicit Iu8vec8(int _I) : I8vec8 (_I) { }

    /* Assignment Operator */
    Iu8vec8& operator= (const M64 &_A)       { return *this = (Iu8vec8) _A; }
    /* Logical Assignment Operators */
    Iu8vec8& operator&=(const M64 &_A)       { return *this = (Iu8vec8) _m_pand(vec,_A); }
    Iu8vec8& operator|=(const M64 &_A)       { return *this = (Iu8vec8) _m_por(vec,_A); }
    Iu8vec8& operator^=(const M64 &_A)       { return *this = (Iu8vec8) _m_pxor(vec,_A); }
    /* Addition & Subtraction Assignment Operators */
    Iu8vec8& operator +=(const I8vec8 &_A)   { return *this = (Iu8vec8) _m_paddb(vec,_A); }
    Iu8vec8& operator -=(const I8vec8 &_A)   { return *this = (Iu8vec8) _m_psubb(vec,_A); }

#if defined (_ENABLE_VEC_DEBUG)
    /* Output for Debug */
    friend std::ostream& operator << (std::ostream &_Os, const Iu8vec8 &_A)
    {
        _Os << "[7]:"  << (unsigned short) (_MM_8UB(7,_A))
            << " [6]:" << (unsigned short) (_MM_8UB(6,_A))
            << " [5]:" << (unsigned short) (_MM_8UB(5,_A))
            << " [4]:" << (unsigned short) (_MM_8UB(4,_A))
            << " [3]:" << (unsigned short) (_MM_8UB(3,_A))
            << " [2]:" << (unsigned short) (_MM_8UB(2,_A))
            << " [1]:" << (unsigned short) (_MM_8UB(1,_A))
            << " [0]:" << (unsigned short) (_MM_8UB(0,_A));
        return _Os;
    }
#endif  /* defined (_ENABLE_VEC_DEBUG) */

    /* Element Access for Debug, No data modified */
    const unsigned char& operator[](int _I)const
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8UB(_I,vec);
    }

    /* Element Access for Debug */
    unsigned char& operator[](int _I)
    {
        _VEC_ASSERT(static_cast<unsigned int>(_I) < 8);  /* Only 8 elements to access */
        return _MM_8UB(_I,vec);
    }
};

/* Additional Iu8vec8 functions: cmpeq,cmpneq, unpacks, sat add/sub */
inline Iu8vec8 cmpeq(const Iu8vec8 &_A, const Iu8vec8 &_B)        { return _m_pcmpeqb(_A,_B); }
inline Iu8vec8 cmpneq(const Iu8vec8 &_A, const Iu8vec8 &_B)       { return _m_pandn(_m_pcmpeqb(_A,_B), _mmx_all_ones); }

inline Iu8vec8 unpack_low(const Iu8vec8 &_A, const Iu8vec8 &_B)   { return _m_punpcklbw(_A,_B); }
inline Iu8vec8 unpack_high(const Iu8vec8 &_A, const Iu8vec8 &_B)  { return _m_punpckhbw(_A,_B); }

inline Iu8vec8 sat_add(const Iu8vec8 &_A, const Iu8vec8 &_B)      { return _m_paddusb(_A,_B); }
inline Iu8vec8 sat_sub(const Iu8vec8 &_A, const Iu8vec8 &_B)      { return _m_psubusb(_A,_B); }

inline Is16vec4 pack_sat(const Is32vec2 &_A, const Is32vec2 &_B)      { return _m_packssdw(_A,_B); }
inline Is8vec8 pack_sat(const Is16vec4 &_A, const Is16vec4 &_B)       { return _m_packsswb(_A,_B); }
inline Iu8vec8 packu_sat(const Is16vec4 &_A, const Is16vec4 &_B)  { return _m_packuswb(_A,_B); }

 /********************************* Logicals ****************************************/
#define IVEC_LOGICALS(vect,element) \
inline I##vect##vec##element operator& (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _m_pand( _A,_B); } \
inline I##vect##vec##element operator| (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _m_por( _A,_B); } \
inline I##vect##vec##element operator^ (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _m_pxor( _A,_B); } \
inline I##vect##vec##element andnot (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _m_pandn( _A,_B); }

IVEC_LOGICALS(8,8)
IVEC_LOGICALS(u8,8)
IVEC_LOGICALS(s8,8)
IVEC_LOGICALS(16,4)
IVEC_LOGICALS(u16,4)
IVEC_LOGICALS(s16,4)
IVEC_LOGICALS(32,2)
IVEC_LOGICALS(u32,2)
IVEC_LOGICALS(s32,2)
IVEC_LOGICALS(64,1)
#undef IVEC_LOGICALS

 /********************************* Add & Sub ****************************************/
#define IVEC_ADD_SUB(vect,element,opsize) \
inline I##vect##vec##element operator+ (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _m_padd##opsize( _A,_B); } \
inline I##vect##vec##element operator- (const I##vect##vec##element &_A, const I##vect##vec##element &_B) \
{ return _m_psub##opsize( _A,_B); }

IVEC_ADD_SUB(8,8, b)
IVEC_ADD_SUB(u8,8, b)
IVEC_ADD_SUB(s8,8, b)
IVEC_ADD_SUB(16,4, w)
IVEC_ADD_SUB(u16,4, w)
IVEC_ADD_SUB(s16,4, w)
IVEC_ADD_SUB(32,2, d)
IVEC_ADD_SUB(u32,2, d)
IVEC_ADD_SUB(s32,2, d)
#undef IVEC_ADD_SUB

 /************************* Conditional Select ********************************
 *      version of: retval = (a OP b)? c : d;                                *
 *      Where OP is one of the possible comparision operators.               *
 *      Example: r = select_eq(a,b,c,d);                                     *
 *      if "member at position x of the vector a" ==                         *
 *         "member at position x of vector b"                                *
 *      assign the corresponding member in r from c, else assign from d.     *
 ************************* Conditional Select ********************************/

#define IVEC_SELECT(vect12,vect34,element,selop)               \
    inline I##vect34##vec##element select_##selop (            \
        const I##vect12##vec##element &_A,                      \
        const I##vect12##vec##element &_B,                      \
        const I##vect34##vec##element &_C,                      \
        const I##vect34##vec##element &_D)                      \
{                                                              \
    I##vect12##vec##element _Mask = cmp##selop(_A,_B);            \
    return( (I##vect34##vec##element)(_Mask &_C ) |             \
            (I##vect34##vec##element)((_m_pandn(_Mask, _D ))));  \
}

IVEC_SELECT(8,s8,8,eq)
IVEC_SELECT(8,u8,8,eq)
IVEC_SELECT(8,8,8,eq)
IVEC_SELECT(8,s8,8,neq)
IVEC_SELECT(8,u8,8,neq)
IVEC_SELECT(8,8,8,neq)

IVEC_SELECT(16,s16,4,eq)
IVEC_SELECT(16,u16,4,eq)
IVEC_SELECT(16,16,4,eq)
IVEC_SELECT(16,s16,4,neq)
IVEC_SELECT(16,u16,4,neq)
IVEC_SELECT(16,16,4,neq)

IVEC_SELECT(32,s32,2,eq)
IVEC_SELECT(32,u32,2,eq)
IVEC_SELECT(32,32,2,eq)
IVEC_SELECT(32,s32,2,neq)
IVEC_SELECT(32,u32,2,neq)
IVEC_SELECT(32,32,2,neq)


IVEC_SELECT(s8,s8,8,gt)
IVEC_SELECT(s8,u8,8,gt)
IVEC_SELECT(s8,8,8,gt)
IVEC_SELECT(s8,s8,8,lt)
IVEC_SELECT(s8,u8,8,lt)
IVEC_SELECT(s8,8,8,lt)
IVEC_SELECT(s8,s8,8,le)
IVEC_SELECT(s8,u8,8,le)
IVEC_SELECT(s8,8,8,le)
IVEC_SELECT(s8,s8,8,ge)
IVEC_SELECT(s8,u8,8,ge)
IVEC_SELECT(s8,8,8,ge)

IVEC_SELECT(s16,s16,4,gt)
IVEC_SELECT(s16,u16,4,gt)
IVEC_SELECT(s16,16,4,gt)
IVEC_SELECT(s16,s16,4,lt)
IVEC_SELECT(s16,u16,4,lt)
IVEC_SELECT(s16,16,4,lt)
IVEC_SELECT(s16,s16,4,le)
IVEC_SELECT(s16,u16,4,le)
IVEC_SELECT(s16,16,4,le)
IVEC_SELECT(s16,s16,4,ge)
IVEC_SELECT(s16,u16,4,ge)
IVEC_SELECT(s16,16,4,ge)

IVEC_SELECT(s32,s32,2,gt)
IVEC_SELECT(s32,u32,2,gt)
IVEC_SELECT(s32,32,2,gt)
IVEC_SELECT(s32,s32,2,lt)
IVEC_SELECT(s32,u32,2,lt)
IVEC_SELECT(s32,32,2,lt)
IVEC_SELECT(s32,s32,2,le)
IVEC_SELECT(s32,u32,2,le)
IVEC_SELECT(s32,32,2,le)
IVEC_SELECT(s32,s32,2,ge)
IVEC_SELECT(s32,u32,2,ge)
IVEC_SELECT(s32,32,2,ge)


#undef IVEC_SELECT

inline static void empty(void)      { _m_empty(); }

#endif  // defined(_M_IX86)

#if defined (_SILENCE_IVEC_C4799)
    #pragma warning(pop)
#endif  /* defined (_SILENCE_IVEC_C4799) */

#endif  /* defined (_M_CEE_PURE) */

#endif  /* _VCRT_COMPILER_PREPROCESSOR */
#endif  /* _IVEC_H_INCLUDED */
