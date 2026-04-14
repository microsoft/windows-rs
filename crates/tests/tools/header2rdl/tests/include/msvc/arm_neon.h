/***
*   arm_neon.h - declarations/definitions for ARM NEON specific intrinsics
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*       This include file contains the declarations for ARM NEON intrinsic functions
*
****/

#pragma once

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined (_M_ARM64EC)
#include <arm64_neon.h>
#else  /* defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) */

#include <stdint.h>
#include <sal.h>

#if !defined (_M_ARM)
#error This header is specific to ARM targets
#endif  /* !defined (_M_ARM) */

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

#if defined (__cplusplus)
extern "C" {
#endif  /* defined (__cplusplus) */


///////////////////////////////////////////////////////////////////////////////
//
#if !defined (_ADVSIMD_ALIGN)
#if defined (__midl)
#define _ADVSIMD_ALIGN(x)
#else  /* defined (__midl) */
#define _ADVSIMD_ALIGN(x) __declspec(align(x))
#endif  /* defined (__midl) */
#endif  /* !defined (_ADVSIMD_ALIGN) */

#ifndef DUMMYNEONSTRUCT
#define DUMMYNEONSTRUCT s
#endif  /* DUMMYNEONSTRUCT */

///////////////////////////////////////////////////////////////////////////////
//
// ARM Advanced SIMD 64bit type
//
typedef union __declspec(intrin_type) _ADVSIMD_ALIGN(8) __n64
{
    unsigned __int64    n64_u64[1];
    unsigned __int32    n64_u32[2];
    unsigned __int16    n64_u16[4];
    unsigned __int8     n64_u8[8];
    __int64             n64_i64[1];
    __int32             n64_i32[2];
    __int16             n64_i16[4];
    __int8              n64_i8[8];
    float               n64_f32[2];
} __n64;


///////////////////////////////////////////////////////////////////////////////
//
// ARM Advanced SIMD 128bit type
//
typedef union __declspec(intrin_type) _ADVSIMD_ALIGN(8) __n128
{
     unsigned __int64   n128_u64[2];
     unsigned __int32   n128_u32[4];
     unsigned __int16   n128_u16[8];
     unsigned __int8    n128_u8[16];
     __int64            n128_i64[2];
     __int32            n128_i32[4];
     __int16            n128_i16[8];
     __int8             n128_i8[16];
     float              n128_f32[4];

    struct
    {
        __n64  low64;
        __n64  high64;
    } DUMMYNEONSTRUCT;

} __n128;

typedef struct __n64x2
{
    __n64 val[2];
} __n64x2;

typedef struct __n64x3
{
    __n64 val[3];
} __n64x3;

typedef struct __n64x4
{
    __n64 val[4];
} __n64x4;

typedef struct __n128x2
{
    __n128 val[2];
} __n128x2;

typedef struct __n128x3
{
    __n128 val[3];
} __n128x3;

typedef struct __n128x4
{
    __n128 val[4];
} __n128x4;

///////////////////////////////////////////////////////////////////////////////
//
typedef unsigned __int8  poly8_t;
typedef unsigned __int16 poly16_t;

typedef float float32_t;

#if !defined(_ARM_USE_NEW_NEON_INTRINSICS)
// Once a version is determined, this should default based on _MSC_FULL_VER
#define _ARM_USE_DEPRECATED_NEON_INTRINSICS
#endif

#if defined(_ARM_USE_DEPRECATED_NEON_INTRINSICS)

///////////////////////////////////////////////////////////////////////////////
//
__inline _Post_equal_to_(p) __n64 *__int8ToN64(_Pre_notnull_ _Const_ int8_t *p)       { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__int16ToN64(_Pre_notnull_ _Const_ int16_t *p)     { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__int32ToN64(_Pre_notnull_ _Const_ int32_t *p)     { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__int64ToN64(_Pre_notnull_ _Const_ int64_t *p)     { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__uint8ToN64(_Pre_notnull_ _Const_ uint8_t *p)     { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__uint16ToN64(_Pre_notnull_ _Const_ uint16_t *p)   { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__uint32ToN64(_Pre_notnull_ _Const_ uint32_t *p)   { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__uint64ToN64(_Pre_notnull_ _Const_ uint64_t *p)   { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__poly8ToN64(_Pre_notnull_ _Const_ poly8_t *p)     { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__poly16ToN64(_Pre_notnull_ _Const_ poly16_t *p)   { return (__n64 *)p; }
__inline _Post_equal_to_(p) __n64 *__float32ToN64(_Pre_notnull_ _Const_ float32_t *p) { return (__n64 *)p; }

__inline _Post_equal_to_(p) const __n64 *__int8ToN64_c(_Pre_notnull_ _Const_ const int8_t *p)       { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__int16ToN64_c(_Pre_notnull_ _Const_ const int16_t *p)     { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__int32ToN64_c(_Pre_notnull_ _Const_ const int32_t *p)     { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__int64ToN64_c(_Pre_notnull_ _Const_ const int64_t *p)     { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__uint8ToN64_c(_Pre_notnull_ _Const_ const uint8_t *p)     { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__uint16ToN64_c(_Pre_notnull_ _Const_ const uint16_t *p)   { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__uint32ToN64_c(_Pre_notnull_ _Const_ const uint32_t *p)   { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__uint64ToN64_c(_Pre_notnull_ _Const_ const uint64_t *p)   { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__poly8ToN64_c(_Pre_notnull_ _Const_ const poly8_t *p)     { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__poly16ToN64_c(_Pre_notnull_ _Const_ const poly16_t *p)   { return (const __n64 *)p; }
__inline _Post_equal_to_(p) const __n64 *__float32ToN64_c(_Pre_notnull_ _Const_ const float32_t *p) { return (const __n64 *)p; }

__inline int32_t __int8ToInt32(int8_t i)      { return (int32_t)i; }
__inline int32_t __int16ToInt32(int16_t i)    { return (int32_t)i; }
__inline int32_t __int32ToInt32(int32_t i)    { return (int32_t)i; }
__inline int64_t __int64ToInt64(int64_t i)    { return (int64_t)i; }

__inline int32_t __uint8ToInt32(uint8_t i)    { return (int32_t)i; }
__inline int32_t __uint16ToInt32(uint16_t i)  { return (int32_t)i; }
__inline int32_t __uint32ToInt32(uint32_t i)  { return (int32_t)i; }
__inline int64_t __uint64ToInt64(uint64_t i)  { return (int64_t)i; }

__inline int32_t __poly8ToInt32(poly8_t i)    { return (int32_t)i; }
__inline int32_t __poly16ToInt32(poly16_t i)  { return (int32_t)i; }

///////////////////////////////////////////////////////////////////////////////
//
#define vshll_n_s8(Dm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) <= 8,  "invalid shift amount"), ((shift_amount) == 8)  ? __internal_vshll_n_t2_s8((Dm))  : __internal_vshll_n_t1_s8((Dm),  (shift_amount)) )
#define vshll_n_s16(Dm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) <= 16, "invalid shift amount"), ((shift_amount) == 16) ? __internal_vshll_n_t2_s16((Dm)) : __internal_vshll_n_t1_s16((Dm), (shift_amount)) )
#define vshll_n_s32(Dm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) <= 32, "invalid shift amount"), ((shift_amount) == 32) ? __internal_vshll_n_t2_s32((Dm)) : __internal_vshll_n_t1_s32((Dm), (shift_amount)) )
#define vshll_n_u8(Dm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) <= 8,  "invalid shift amount"), ((shift_amount) == 8)  ? __internal_vshll_n_t2_u8((Dm))  : __internal_vshll_n_t1_u8((Dm),  (shift_amount)) )
#define vshll_n_u16(Dm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) <= 16, "invalid shift amount"), ((shift_amount) == 16) ? __internal_vshll_n_t2_u16((Dm)) : __internal_vshll_n_t1_u16((Dm), (shift_amount)) )
#define vshll_n_u32(Dm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) <= 32, "invalid shift amount"), ((shift_amount) == 32) ? __internal_vshll_n_t2_u32((Dm)) : __internal_vshll_n_t1_u32((Dm), (shift_amount)) )

#endif


///////////////////////////////////////////////////////////////////////////////
//
// { +++ auto-generated code begins (explicit types)

typedef __n64    float32x2_t;
typedef __n64x2  float32x2x2_t;
typedef __n64x3  float32x2x3_t;
typedef __n64x4  float32x2x4_t;
typedef __n64    int8x8_t;
typedef __n64x2  int8x8x2_t;
typedef __n64x3  int8x8x3_t;
typedef __n64x4  int8x8x4_t;
typedef __n64    int16x4_t;
typedef __n64x2  int16x4x2_t;
typedef __n64x3  int16x4x3_t;
typedef __n64x4  int16x4x4_t;
typedef __n64    int32x2_t;
typedef __n64x2  int32x2x2_t;
typedef __n64x3  int32x2x3_t;
typedef __n64x4  int32x2x4_t;
typedef __n64    int64x1_t;
typedef __n64x2  int64x1x2_t;
typedef __n64x3  int64x1x3_t;
typedef __n64x4  int64x1x4_t;
typedef __n64    poly8x8_t;
typedef __n64x2  poly8x8x2_t;
typedef __n64x3  poly8x8x3_t;
typedef __n64x4  poly8x8x4_t;
typedef __n64    poly16x4_t;
typedef __n64x2  poly16x4x2_t;
typedef __n64x3  poly16x4x3_t;
typedef __n64x4  poly16x4x4_t;
typedef __n64    uint8x8_t;
typedef __n64x2  uint8x8x2_t;
typedef __n64x3  uint8x8x3_t;
typedef __n64x4  uint8x8x4_t;
typedef __n64    uint16x4_t;
typedef __n64x2  uint16x4x2_t;
typedef __n64x3  uint16x4x3_t;
typedef __n64x4  uint16x4x4_t;
typedef __n64    uint32x2_t;
typedef __n64x2  uint32x2x2_t;
typedef __n64x3  uint32x2x3_t;
typedef __n64x4  uint32x2x4_t;
typedef __n64    uint64x1_t;
typedef __n64x2  uint64x1x2_t;
typedef __n64x3  uint64x1x3_t;
typedef __n64x4  uint64x1x4_t;
typedef __n128   float32x4_t;
typedef __n128x2 float32x4x2_t;
typedef __n128x3 float32x4x3_t;
typedef __n128x4 float32x4x4_t;
typedef __n128   int8x16_t;
typedef __n128x2 int8x16x2_t;
typedef __n128x3 int8x16x3_t;
typedef __n128x4 int8x16x4_t;
typedef __n128   int16x8_t;
typedef __n128x2 int16x8x2_t;
typedef __n128x3 int16x8x3_t;
typedef __n128x4 int16x8x4_t;
typedef __n128   int32x4_t;
typedef __n128x2 int32x4x2_t;
typedef __n128x3 int32x4x3_t;
typedef __n128x4 int32x4x4_t;
typedef __n128   int64x2_t;
typedef __n128x2 int64x2x2_t;
typedef __n128x3 int64x2x3_t;
typedef __n128x4 int64x2x4_t;
typedef __n128   poly8x16_t;
typedef __n128x2 poly8x16x2_t;
typedef __n128x3 poly8x16x3_t;
typedef __n128x4 poly8x16x4_t;
typedef __n128   poly16x8_t;
typedef __n128x2 poly16x8x2_t;
typedef __n128x3 poly16x8x3_t;
typedef __n128x4 poly16x8x4_t;
typedef __n128   uint8x16_t;
typedef __n128x2 uint8x16x2_t;
typedef __n128x3 uint8x16x3_t;
typedef __n128x4 uint8x16x4_t;
typedef __n128   uint16x8_t;
typedef __n128x2 uint16x8x2_t;
typedef __n128x3 uint16x8x3_t;
typedef __n128x4 uint16x8x4_t;
typedef __n128   uint32x4_t;
typedef __n128x2 uint32x4x2_t;
typedef __n128x3 uint32x4x3_t;
typedef __n128x4 uint32x4x4_t;
typedef __n128   uint64x2_t;
typedef __n128x2 uint64x2x2_t;
typedef __n128x3 uint64x2x3_t;
typedef __n128x4 uint64x2x4_t;

// } +++ auto-generated code ends (explicit types)

#if defined(_ARM_USE_DEPRECATED_NEON_INTRINSICS)

///////////////////////////////////////////////////////////////////////////////
//
// { +++ auto-generated code begins (prototypes)

__n64x2 __neon_DdDm_acc2(unsigned int _Enc, __n64, __n64);
__n64x2 __neon_Dx2Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64x2 __neon_Dx2Adr_acc(unsigned int _Enc, __n64x2, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64x3 __neon_Dx3Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64x3 __neon_Dx3Adr_acc(unsigned int _Enc, __n64x3, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64x4 __neon_Dx4Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64x4 __neon_Dx4Adr_acc(unsigned int _Enc, __n64x4, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64 __neon_DdDm(unsigned int _Enc, __n64);
__n64 __neon_DdDx2Dm(unsigned int _Enc, __n64x2, __n64);
__n64 __neon_DdDx2Dm_acc(unsigned int _Enc, __n64, __n64x2, __n64);
__n64 __neon_DdDx3Dm(unsigned int _Enc, __n64x3, __n64);
__n64 __neon_DdDx3Dm_acc(unsigned int _Enc, __n64, __n64x3, __n64);
__n64 __neon_DdDx4Dm(unsigned int _Enc, __n64x4, __n64);
__n64 __neon_DdDx4Dm_acc(unsigned int _Enc, __n64, __n64x4, __n64);
__n64 __neon_DdDm_acc(unsigned int _Enc, __n64, __n64);
__n64 __neon_DdDnDm(unsigned int _Enc, __n64, __n64);
__n64 __neon_DdDnDm_acc(unsigned int _Enc, __n64, __n64, __n64);
__n64 __neon_DdDnDmx(unsigned int _Enc, __n64, __n64);
__n64 __neon_DdDnDmx_acc(unsigned int _Enc, __n64, __n64, __n64);
__n64 __neon_DdDnFt(unsigned int, __n64, float);
__n64 __neon_DdDnFt_acc(unsigned int, __n64, __n64, float);
__n64 __neon_DdFt(unsigned int _Enc, float);
__n64 __neon_DdFt_acc(unsigned int _Enc, __n64, float);
__n64 __neon_D1Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64 __neon_D1Adr_acc(unsigned int _Enc, __n64, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n64 __neon_DdQm(unsigned int _Enc, __n128);
__n64 __neon_DdQm_high(unsigned int _Enc, __n128);
__n64 __neon_DdQm_low(unsigned int _Enc, __n128);
__n64 __neon_DdQnQm(unsigned int _Enc, __n128, __n128);
__n64 __neon_DdRt(unsigned int _Enc, int);
__n64 __neon_DdRtRt2(unsigned int _Enc, __int64);
__n64 __neon_DdRtRt2_acc(unsigned int _Enc, __n64, __int64);
__n64 __neon_DdRt_acc(unsigned int _Enc, __n64, int);
float __neon_FtDn(unsigned int _Enc, __n64);
float __neon_FtQn(unsigned int _Enc, __n128);
__n128x2 __neon_Qx2Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128x2 __neon_Qx2Adr_acc(unsigned int _Enc, __n128x2, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128x2 __neon_QdQm_acc2(unsigned int _Enc, __n128, __n128);
__n128x2 __neon_QdQm_acc3(unsigned int _Enc, __n128, __n128);
__n128x3 __neon_Qx3Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128x3 __neon_Qx3Adr_acc(unsigned int _Enc, __n128x3, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128x4 __neon_Qx4Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128x4 __neon_Qx4Adr_acc(unsigned int _Enc, __n128x4, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128 __neon_QdDm(unsigned int _Enc, __n64);
__n128 __neon_QdDnDm(unsigned int _Enc, __n64, __n64);
__n128 __neon_QdDnDm_acc(unsigned int _Enc, __n128, __n64, __n64);
__n128 __neon_QdDnDm_merge(unsigned int _Enc, __n64, __n64);
__n128 __neon_QdDnDmx(unsigned int _Enc, __n64, __n64);
__n128 __neon_QdDnDmx_acc(unsigned int _Enc, __n128, __n64, __n64);
__n128 __neon_QdFt(unsigned int _Enc, float);
__n128 __neon_QdFt_acc(unsigned int _Enc, __n128, float);
__n128 __neon_Q1Adr(unsigned int _Enc, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128 __neon_Q1Adr_acc(unsigned int _Enc, __n128, _In_reads_bytes_(_Inexpressible_(_Enc)) const __n64*);
__n128 __neon_QdQm(unsigned int _Enc, __n128);
__n128 __neon_QdQm_acc(unsigned int _Enc, __n128, __n128);
__n128 __neon_QdQnDm(unsigned int _Enc, __n128, __n64);
__n128 __neon_QdQnDmx(unsigned int _Enc, __n128, __n64);
__n128 __neon_QdQnDmx_acc(unsigned int _Enc, __n128, __n128, __n64);
__n128 __neon_QdQnFt(unsigned int, __n128, float);
__n128 __neon_QdQnFt_acc(unsigned int, __n128, __n128, float);
__n128 __neon_QdQnQm(unsigned int _Enc, __n128, __n128);
__n128 __neon_QdQnQm_acc(unsigned int _Enc, __n128, __n128, __n128);
__n128 __neon_QdRt(unsigned int _Enc, int);
__n128 __neon_QdRtRt2_acc(unsigned int _Enc, __n128, __int64);
__n128 __neon_QdRtRt2_dup(unsigned int _Enc, __int64);
__n128 __neon_QdRt_acc(unsigned int _Enc, __n128, int);
__int64 __neon_RtRt2Dm(unsigned int _Enc, __n64);
__int64 __neon_RtRt2Qm(unsigned int _Enc, __n128);
int __neon_RtDn(unsigned int _Enc, __n64);
int __neon_RtQn(unsigned int _Enc, __n128);
void __neon_AdrD1(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n64);
void __neon_AdrDx2(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n64x2);
void __neon_AdrDx2x(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n64x2);
void __neon_AdrDx3(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n64x3);
void __neon_AdrDx3x(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n64x3);
void __neon_AdrDx4(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n64x4);
void __neon_AdrDx4x(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n64x4);
void __neon_AdrQ1(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n128);
void __neon_AdrQx2(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n128x2);
void __neon_AdrQx2x(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n128x2);
void __neon_AdrQx3(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n128x3);
void __neon_AdrQx3x(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n128x3);
void __neon_AdrQx4(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n128x4);
void __neon_AdrQx4x(unsigned int _Enc, _Out_writes_bytes_(_Inexpressible_(_Enc)) __n64*, __n128x4);

// } +++ auto-generated code ends (prototypes)

#endif

#if defined (__cplusplus)
}
#endif  /* defined (__cplusplus) */

#if defined(_ARM_USE_DEPRECATED_NEON_INTRINSICS)

///////////////////////////////////////////////////////////////////////////////
//
// VLDx/VSTx alignment specifications
//


#define _NEON_ALIGN16(a)         \
    (                            \
    ((a) == 8) ? 0 :             \
    ((a) == 16) ? 1 :            \
    -1)

#define _NEON_ALIGN32(a)         \
    (                            \
    ((a) == 8) ? 0 :             \
    ((a) == 32) ? 1 :            \
    -1)

#define _NEON_ALIGN64(a)         \
    (                            \
    ((a) == 8) ? 0 :             \
    ((a) == 64) ? 1 :            \
    -1)

#define _NEON_ALIGN64_128(a)     \
    (                            \
    ((a) == 8) ? 0 :             \
    ((a) == 64) ? 1 :            \
    ((a) == 128) ? 2 :           \
    -1)


#define _NEON_ALIGN64_128_256(a) \
    (                            \
    ((a) == 8) ? 0 :             \
    ((a) == 64) ? 1 :            \
    ((a) == 128) ? 2 :           \
    ((a) == 256) ? 3 :           \
    -1)


///////////////////////////////////////////////////////////////////////////////
//
// { +++ auto-generated code begins (encoding macros)

#define _NENC_0(x)                               ((x) & 0x1)
#define _NENC_11_8(x)                            (((x) << 8) & 0xf00)
#define _NENC_12(x)                              (((x) << 12) & 0x1000)
#define _NENC_16(x)                              (((x) << 16) & 0x10000)
#define _NENC_18_16(x)                           (((x) << 16) & 0x70000)
#define _NENC_19(x)                              (((x) << 19) & 0x80000)
#define _NENC_19_16(x)                           (((x) << 16) & 0xf0000)
#define _NENC_19_17(x)                           (((x) << 17) & 0xe0000)
#define _NENC_19_18(x)                           (((x) << 18) & 0xc0000)
#define _NENC_20_16(x)                           (((x) << 16) & 0x1f0000)
#define _NENC_21(x)                              (((x) << 21) & 0x200000)
#define _NENC_21_16(x)                           (((x) << 16) & 0x3f0000)
#define _NENC_21x6(x)                            (((x) << 6) & 0x40 | ((x) << 20) & 0x200000)
#define _NENC_21x6_5(x)                          (((x) << 5) & 0x60 | ((x) << 19) & 0x200000)
#define _NENC_4(x)                               (((x) << 4) & 0x10)
#define _NENC_5(x)                               (((x) << 5) & 0x20)
#define _NENC_5_4(x)                             (((x) << 4) & 0x30)
#define _NENC_5x3(x)                             (((x) << 3) & 0x8 | ((x) << 4) & 0x20)
#define _NENC_7(x)                               (((x) << 7) & 0x80)
#define _NENC_7_5(x)                             (((x) << 5) & 0xe0)
#define _NENC_7_6(x)                             (((x) << 6) & 0xc0)

// } +++ auto-generated code ends (encoding macros)


///////////////////////////////////////////////////////////////////////////////
//
// { +++ auto-generated code begins (Neon macros)

// AES
#define aesd_p8(Qm)                              ( __neon_QdQm( 0xf3b00340, (Qm)) )
#define aesd_s8(Qm)                              ( __neon_QdQm( 0xf3b00340, (Qm)) )
#define aesd_u8(Qm)                              ( __neon_QdQm( 0xf3b00340, (Qm)) )
#define aese_p8(Qm)                              ( __neon_QdQm( 0xf3b00300, (Qm)) )
#define aese_s8(Qm)                              ( __neon_QdQm( 0xf3b00300, (Qm)) )
#define aese_u8(Qm)                              ( __neon_QdQm( 0xf3b00300, (Qm)) )
#define aesimc_p8(Qm)                            ( __neon_QdQm( 0xf3b003c0, (Qm)) )
#define aesimc_s8(Qm)                            ( __neon_QdQm( 0xf3b003c0, (Qm)) )
#define aesimc_u8(Qm)                            ( __neon_QdQm( 0xf3b003c0, (Qm)) )
#define aesmc_p8(Qm)                             ( __neon_QdQm( 0xf3b00380, (Qm)) )
#define aesmc_s8(Qm)                             ( __neon_QdQm( 0xf3b00380, (Qm)) )
#define aesmc_u8(Qm)                             ( __neon_QdQm( 0xf3b00380, (Qm)) )

// SHA (2-operand)
#define sha1h_f32(Qm)                            ( __neon_QdQm( 0xf3b902c0, (Qm)) )
#define sha1h_s32(Qm)                            ( __neon_QdQm( 0xf3b902c0, (Qm)) )
#define sha1h_u32(Qm)                            ( __neon_QdQm( 0xf3b902c0, (Qm)) )
#define sha1su1_f32(Qm)                          ( __neon_QdQm( 0xf3ba0380, (Qm)) )
#define sha1su1_s32(Qm)                          ( __neon_QdQm( 0xf3ba0380, (Qm)) )
#define sha1su1_u32(Qm)                          ( __neon_QdQm( 0xf3ba0380, (Qm)) )
#define sha256su0_f32(Qm)                        ( __neon_QdQm( 0xf3ba03c0, (Qm)) )
#define sha256su0_s32(Qm)                        ( __neon_QdQm( 0xf3ba03c0, (Qm)) )
#define sha256su0_u32(Qm)                        ( __neon_QdQm( 0xf3ba03c0, (Qm)) )

// SHA (3-operand)
#define sha1c_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000c40, (Qn), (Qm)) )
#define sha1c_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000c40, (Qn), (Qm)) )
#define sha1c_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000c40, (Qn), (Qm)) )
#define sha1m_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200c40, (Qn), (Qm)) )
#define sha1m_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200c40, (Qn), (Qm)) )
#define sha1m_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200c40, (Qn), (Qm)) )
#define sha1p_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100c40, (Qn), (Qm)) )
#define sha1p_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100c40, (Qn), (Qm)) )
#define sha1p_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100c40, (Qn), (Qm)) )
#define sha1su0_f32(Qn, Qm)                      ( __neon_QdQnQm( 0xf2300c40, (Qn), (Qm)) )
#define sha1su0_s32(Qn, Qm)                      ( __neon_QdQnQm( 0xf2300c40, (Qn), (Qm)) )
#define sha1su0_u32(Qn, Qm)                      ( __neon_QdQnQm( 0xf2300c40, (Qn), (Qm)) )
#define sha256h_f32(Qn, Qm)                      ( __neon_QdQnQm( 0xf3000c40, (Qn), (Qm)) )
#define sha256h_s32(Qn, Qm)                      ( __neon_QdQnQm( 0xf3000c40, (Qn), (Qm)) )
#define sha256h_u32(Qn, Qm)                      ( __neon_QdQnQm( 0xf3000c40, (Qn), (Qm)) )
#define sha256h2_f32(Qn, Qm)                     ( __neon_QdQnQm( 0xf3100c40, (Qn), (Qm)) )
#define sha256h2_s32(Qn, Qm)                     ( __neon_QdQnQm( 0xf3100c40, (Qn), (Qm)) )
#define sha256h2_u32(Qn, Qm)                     ( __neon_QdQnQm( 0xf3100c40, (Qn), (Qm)) )
#define sha256su1_f32(Qn, Qm)                    ( __neon_QdQnQm( 0xf3200c40, (Qn), (Qm)) )
#define sha256su1_s32(Qn, Qm)                    ( __neon_QdQnQm( 0xf3200c40, (Qn), (Qm)) )
#define sha256su1_u32(Qn, Qm)                    ( __neon_QdQnQm( 0xf3200c40, (Qn), (Qm)) )

// VABA, VABAL
#define vaba_s16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2100710, (Dd), (Dn), (Dm)) )
#define vaba_s32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2200710, (Dd), (Dn), (Dm)) )
#define vaba_s8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf2000710, (Dd), (Dn), (Dm)) )
#define vaba_u16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100710, (Dd), (Dn), (Dm)) )
#define vaba_u32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200710, (Dd), (Dn), (Dm)) )
#define vaba_u8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3000710, (Dd), (Dn), (Dm)) )
#define vabal_s16(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf2900500, (Qd), (Dn), (Dm)) )
#define vabal_s32(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf2a00500, (Qd), (Dn), (Dm)) )
#define vabal_s8(Qd, Dn, Dm)                     ( __neon_QdDnDm_acc( 0xf2800500, (Qd), (Dn), (Dm)) )
#define vabal_u16(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf3900500, (Qd), (Dn), (Dm)) )
#define vabal_u32(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf3a00500, (Qd), (Dn), (Dm)) )
#define vabal_u8(Qd, Dn, Dm)                     ( __neon_QdDnDm_acc( 0xf3800500, (Qd), (Dn), (Dm)) )
#define vabaq_s16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2100750, (Qd), (Qn), (Qm)) )
#define vabaq_s32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2200750, (Qd), (Qn), (Qm)) )
#define vabaq_s8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf2000750, (Qd), (Qn), (Qm)) )
#define vabaq_u16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100750, (Qd), (Qn), (Qm)) )
#define vabaq_u32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200750, (Qd), (Qn), (Qm)) )
#define vabaq_u8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3000750, (Qd), (Qn), (Qm)) )

// VABD (floating point)
#define vabd_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200d00, (Dn), (Dm)) )
#define vabdq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200d40, (Qn), (Qm)) )

// VABD[L] (integer)
#define vabd_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100700, (Dn), (Dm)) )
#define vabd_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200700, (Dn), (Dm)) )
#define vabd_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000700, (Dn), (Dm)) )
#define vabd_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100700, (Dn), (Dm)) )
#define vabd_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200700, (Dn), (Dm)) )
#define vabd_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000700, (Dn), (Dm)) )
#define vabdl_s16(Dn, Dm)                        ( __neon_QdDnDm( 0xf2900700, (Dn), (Dm)) )
#define vabdl_s32(Dn, Dm)                        ( __neon_QdDnDm( 0xf2a00700, (Dn), (Dm)) )
#define vabdl_s8(Dn, Dm)                         ( __neon_QdDnDm( 0xf2800700, (Dn), (Dm)) )
#define vabdl_u16(Dn, Dm)                        ( __neon_QdDnDm( 0xf3900700, (Dn), (Dm)) )
#define vabdl_u32(Dn, Dm)                        ( __neon_QdDnDm( 0xf3a00700, (Dn), (Dm)) )
#define vabdl_u8(Dn, Dm)                         ( __neon_QdDnDm( 0xf3800700, (Dn), (Dm)) )
#define vabdq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100740, (Qn), (Qm)) )
#define vabdq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200740, (Qn), (Qm)) )
#define vabdq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000740, (Qn), (Qm)) )
#define vabdq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100740, (Qn), (Qm)) )
#define vabdq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200740, (Qn), (Qm)) )
#define vabdq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000740, (Qn), (Qm)) )

// VABS, VNEG
#define vabs_f32(Dm)                             ( __neon_DdDm( 0xf3b90700, (Dm)) )
#define vabs_s16(Dm)                             ( __neon_DdDm( 0xf3b50300, (Dm)) )
#define vabs_s32(Dm)                             ( __neon_DdDm( 0xf3b90300, (Dm)) )
#define vabs_s8(Dm)                              ( __neon_DdDm( 0xf3b10300, (Dm)) )
#define vneg_f32(Dm)                             ( __neon_DdDm( 0xf3b90780, (Dm)) )
#define vneg_s16(Dm)                             ( __neon_DdDm( 0xf3b50380, (Dm)) )
#define vneg_s32(Dm)                             ( __neon_DdDm( 0xf3b90380, (Dm)) )
#define vneg_s8(Dm)                              ( __neon_DdDm( 0xf3b10380, (Dm)) )
#define vabsq_f32(Qm)                            ( __neon_QdQm( 0xf3b90740, (Qm)) )
#define vabsq_s16(Qm)                            ( __neon_QdQm( 0xf3b50340, (Qm)) )
#define vabsq_s32(Qm)                            ( __neon_QdQm( 0xf3b90340, (Qm)) )
#define vabsq_s8(Qm)                             ( __neon_QdQm( 0xf3b10340, (Qm)) )
#define vnegq_f32(Qm)                            ( __neon_QdQm( 0xf3b907c0, (Qm)) )
#define vnegq_s16(Qm)                            ( __neon_QdQm( 0xf3b503c0, (Qm)) )
#define vnegq_s32(Qm)                            ( __neon_QdQm( 0xf3b903c0, (Qm)) )
#define vnegq_s8(Qm)                             ( __neon_QdQm( 0xf3b103c0, (Qm)) )

// VACGE, VACGT, VACLE, VACLT
#define vacge_f32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3000e10, (Dn), (Dm)) )
#define vacgt_f32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200e10, (Dn), (Dm)) )
#define vacle_f32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3000e10, (Dm), (Dn)) )
#define vaclt_f32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200e10, (Dm), (Dn)) )
#define vacgeq_f32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3000e50, (Qn), (Qm)) )
#define vacgtq_f32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200e50, (Qn), (Qm)) )
#define vacleq_f32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3000e50, (Qm), (Qn)) )
#define vacltq_f32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200e50, (Qm), (Qn)) )

// VADD
#define vadd_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000d00, (Dn), (Dm)) )
#define vadd_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100800, (Dn), (Dm)) )
#define vadd_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200800, (Dn), (Dm)) )
#define vadd_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300800, (Dn), (Dm)) )
#define vadd_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000800, (Dn), (Dm)) )
#define vadd_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100800, (Dn), (Dm)) )
#define vadd_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200800, (Dn), (Dm)) )
#define vadd_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300800, (Dn), (Dm)) )
#define vadd_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000800, (Dn), (Dm)) )
#define vaddq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000d40, (Qn), (Qm)) )
#define vaddq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100840, (Qn), (Qm)) )
#define vaddq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200840, (Qn), (Qm)) )
#define vaddq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300840, (Qn), (Qm)) )
#define vaddq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000840, (Qn), (Qm)) )
#define vaddq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100840, (Qn), (Qm)) )
#define vaddq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200840, (Qn), (Qm)) )
#define vaddq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300840, (Qn), (Qm)) )
#define vaddq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000840, (Qn), (Qm)) )

// VADDHN, VRADDHN
#define vaddhn_s16(Qn, Qm)                       ( __neon_DdQnQm( 0xf2800400, (Qn), (Qm)) )
#define vaddhn_s32(Qn, Qm)                       ( __neon_DdQnQm( 0xf2900400, (Qn), (Qm)) )
#define vaddhn_s64(Qn, Qm)                       ( __neon_DdQnQm( 0xf2a00400, (Qn), (Qm)) )
#define vaddhn_u16(Qn, Qm)                       ( __neon_DdQnQm( 0xf2800400, (Qn), (Qm)) )
#define vaddhn_u32(Qn, Qm)                       ( __neon_DdQnQm( 0xf2900400, (Qn), (Qm)) )
#define vaddhn_u64(Qn, Qm)                       ( __neon_DdQnQm( 0xf2a00400, (Qn), (Qm)) )
#define vraddhn_s16(Qn, Qm)                      ( __neon_DdQnQm( 0xf3800400, (Qn), (Qm)) )
#define vraddhn_s32(Qn, Qm)                      ( __neon_DdQnQm( 0xf3900400, (Qn), (Qm)) )
#define vraddhn_s64(Qn, Qm)                      ( __neon_DdQnQm( 0xf3a00400, (Qn), (Qm)) )
#define vraddhn_u16(Qn, Qm)                      ( __neon_DdQnQm( 0xf3800400, (Qn), (Qm)) )
#define vraddhn_u32(Qn, Qm)                      ( __neon_DdQnQm( 0xf3900400, (Qn), (Qm)) )
#define vraddhn_u64(Qn, Qm)                      ( __neon_DdQnQm( 0xf3a00400, (Qn), (Qm)) )

// VADDL, VADDW
#define vaddl_s16(Dn, Dm)                        ( __neon_QdDnDm( 0xf2900000, (Dn), (Dm)) )
#define vaddl_s32(Dn, Dm)                        ( __neon_QdDnDm( 0xf2a00000, (Dn), (Dm)) )
#define vaddl_s8(Dn, Dm)                         ( __neon_QdDnDm( 0xf2800000, (Dn), (Dm)) )
#define vaddl_u16(Dn, Dm)                        ( __neon_QdDnDm( 0xf3900000, (Dn), (Dm)) )
#define vaddl_u32(Dn, Dm)                        ( __neon_QdDnDm( 0xf3a00000, (Dn), (Dm)) )
#define vaddl_u8(Dn, Dm)                         ( __neon_QdDnDm( 0xf3800000, (Dn), (Dm)) )
#define vaddw_s16(Qn, Dm)                        ( __neon_QdQnDm( 0xf2900100, (Qn), (Dm)) )
#define vaddw_s32(Qn, Dm)                        ( __neon_QdQnDm( 0xf2a00100, (Qn), (Dm)) )
#define vaddw_s8(Qn, Dm)                         ( __neon_QdQnDm( 0xf2800100, (Qn), (Dm)) )
#define vaddw_u16(Qn, Dm)                        ( __neon_QdQnDm( 0xf3900100, (Qn), (Dm)) )
#define vaddw_u32(Qn, Dm)                        ( __neon_QdQnDm( 0xf3a00100, (Qn), (Dm)) )
#define vaddw_u8(Qn, Dm)                         ( __neon_QdQnDm( 0xf3800100, (Qn), (Dm)) )

// VAND, VORR
#define vand_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vand_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vand_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vand_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vand_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vand_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vand_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vand_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000110, (Dn), (Dm)) )
#define vorr_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vorr_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vorr_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vorr_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vorr_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vorr_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vorr_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vorr_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2200110, (Dn), (Dm)) )
#define vandq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vandq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vandq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vandq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vandq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vandq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vandq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vandq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000150, (Qn), (Qm)) )
#define vorrq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )
#define vorrq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )
#define vorrq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )
#define vorrq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )
#define vorrq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )
#define vorrq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )
#define vorrq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )
#define vorrq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2200150, (Qn), (Qm)) )

// VBIF, VBIT, VBSL
#define vbif_f32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_p16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_p8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_s16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_s32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_s64(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_s8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_u16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_u32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_u64(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbif_u8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3300110, (Dd), (Dn), (Dm)) )
#define vbit_f32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_p16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_p8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_s16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_s32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_s64(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_s8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_u16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_u32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_u64(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbit_u8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3200110, (Dd), (Dn), (Dm)) )
#define vbsl_f32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_p16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_p8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_s16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_s32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_s64(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_s8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_u16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_u32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_u64(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbsl_u8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3100110, (Dd), (Dn), (Dm)) )
#define vbifq_f32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_p16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_p8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_s16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_s32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_s64(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_s8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_u16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_u32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_u64(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbifq_u8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3300150, (Qd), (Qn), (Qm)) )
#define vbitq_f32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_p16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_p8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_s16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_s32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_s64(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_s8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_u16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_u32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_u64(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbitq_u8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3200150, (Qd), (Qn), (Qm)) )
#define vbslq_f32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_p16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_p8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_s16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_s32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_s64(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_s8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_u16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_u32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_u64(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )
#define vbslq_u8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3100150, (Qd), (Qn), (Qm)) )

// VCEQ (immediate #0)
#define vceq_z_f32_ex(Dm)                        ( __neon_DdDm( 0xf3b90500, (Dm)) )
#define vceq_z_s16_ex(Dm)                        ( __neon_DdDm( 0xf3b50100, (Dm)) )
#define vceq_z_s32_ex(Dm)                        ( __neon_DdDm( 0xf3b90100, (Dm)) )
#define vceq_z_s8_ex(Dm)                         ( __neon_DdDm( 0xf3b10100, (Dm)) )
#define vceq_z_u16_ex(Dm)                        ( __neon_DdDm( 0xf3b50100, (Dm)) )
#define vceq_z_u32_ex(Dm)                        ( __neon_DdDm( 0xf3b90100, (Dm)) )
#define vceq_z_u8_ex(Dm)                         ( __neon_DdDm( 0xf3b10100, (Dm)) )
#define vceqq_z_f32_ex(Qm)                       ( __neon_QdQm( 0xf3b90540, (Qm)) )
#define vceqq_z_s16_ex(Qm)                       ( __neon_QdQm( 0xf3b50140, (Qm)) )
#define vceqq_z_s32_ex(Qm)                       ( __neon_QdQm( 0xf3b90140, (Qm)) )
#define vceqq_z_s8_ex(Qm)                        ( __neon_QdQm( 0xf3b10140, (Qm)) )
#define vceqq_z_u16_ex(Qm)                       ( __neon_QdQm( 0xf3b50140, (Qm)) )
#define vceqq_z_u32_ex(Qm)                       ( __neon_QdQm( 0xf3b90140, (Qm)) )
#define vceqq_z_u8_ex(Qm)                        ( __neon_QdQm( 0xf3b10140, (Qm)) )

// VCEQ (register)
#define vceq_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000e00, (Dn), (Dm)) )
#define vceq_p8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000810, (Dn), (Dm)) )
#define vceq_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100810, (Dn), (Dm)) )
#define vceq_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200810, (Dn), (Dm)) )
#define vceq_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000810, (Dn), (Dm)) )
#define vceq_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100810, (Dn), (Dm)) )
#define vceq_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200810, (Dn), (Dm)) )
#define vceq_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000810, (Dn), (Dm)) )
#define vceqq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000e40, (Qn), (Qm)) )
#define vceqq_p8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000850, (Qn), (Qm)) )
#define vceqq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100850, (Qn), (Qm)) )
#define vceqq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200850, (Qn), (Qm)) )
#define vceqq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000850, (Qn), (Qm)) )
#define vceqq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100850, (Qn), (Qm)) )
#define vceqq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200850, (Qn), (Qm)) )
#define vceqq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000850, (Qn), (Qm)) )

// VCGE (immediate #0)
#define vcge_z_f32_ex(Dm)                        ( __neon_DdDm( 0xf3b90480, (Dm)) )
#define vcge_z_s16_ex(Dm)                        ( __neon_DdDm( 0xf3b50080, (Dm)) )
#define vcge_z_s32_ex(Dm)                        ( __neon_DdDm( 0xf3b90080, (Dm)) )
#define vcge_z_s8_ex(Dm)                         ( __neon_DdDm( 0xf3b10080, (Dm)) )
#define vcgeq_z_f32_ex(Qm)                       ( __neon_QdQm( 0xf3b904c0, (Qm)) )
#define vcgeq_z_s16_ex(Qm)                       ( __neon_QdQm( 0xf3b500c0, (Qm)) )
#define vcgeq_z_s32_ex(Qm)                       ( __neon_QdQm( 0xf3b900c0, (Qm)) )
#define vcgeq_z_s8_ex(Qm)                        ( __neon_QdQm( 0xf3b100c0, (Qm)) )

// VCGE, VCLE (register)
#define vcge_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000e00, (Dn), (Dm)) )
#define vcge_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100310, (Dn), (Dm)) )
#define vcge_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200310, (Dn), (Dm)) )
#define vcge_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000310, (Dn), (Dm)) )
#define vcge_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100310, (Dn), (Dm)) )
#define vcge_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200310, (Dn), (Dm)) )
#define vcge_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000310, (Dn), (Dm)) )
#define vcle_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000e00, (Dm), (Dn)) )
#define vcle_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100310, (Dm), (Dn)) )
#define vcle_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200310, (Dm), (Dn)) )
#define vcle_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000310, (Dm), (Dn)) )
#define vcle_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100310, (Dm), (Dn)) )
#define vcle_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200310, (Dm), (Dn)) )
#define vcle_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000310, (Dm), (Dn)) )
#define vcgeq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000e40, (Qn), (Qm)) )
#define vcgeq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100350, (Qn), (Qm)) )
#define vcgeq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200350, (Qn), (Qm)) )
#define vcgeq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000350, (Qn), (Qm)) )
#define vcgeq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100350, (Qn), (Qm)) )
#define vcgeq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200350, (Qn), (Qm)) )
#define vcgeq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000350, (Qn), (Qm)) )
#define vcleq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000e40, (Qm), (Qn)) )
#define vcleq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100350, (Qm), (Qn)) )
#define vcleq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200350, (Qm), (Qn)) )
#define vcleq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000350, (Qm), (Qn)) )
#define vcleq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100350, (Qm), (Qn)) )
#define vcleq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200350, (Qm), (Qn)) )
#define vcleq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000350, (Qm), (Qn)) )

// VCGT (immediate #0)
#define vcgt_z_f32_ex(Dm)                        ( __neon_DdDm( 0xf3b90400, (Dm)) )
#define vcgt_z_s16_ex(Dm)                        ( __neon_DdDm( 0xf3b50000, (Dm)) )
#define vcgt_z_s32_ex(Dm)                        ( __neon_DdDm( 0xf3b90000, (Dm)) )
#define vcgt_z_s8_ex(Dm)                         ( __neon_DdDm( 0xf3b10000, (Dm)) )
#define vcgtq_z_f32_ex(Qm)                       ( __neon_QdQm( 0xf3b90440, (Qm)) )
#define vcgtq_z_s16_ex(Qm)                       ( __neon_QdQm( 0xf3b50040, (Qm)) )
#define vcgtq_z_s32_ex(Qm)                       ( __neon_QdQm( 0xf3b90040, (Qm)) )
#define vcgtq_z_s8_ex(Qm)                        ( __neon_QdQm( 0xf3b10040, (Qm)) )

// VCGT, VCLT (register)
#define vcgt_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200e00, (Dn), (Dm)) )
#define vcgt_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100300, (Dn), (Dm)) )
#define vcgt_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200300, (Dn), (Dm)) )
#define vcgt_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000300, (Dn), (Dm)) )
#define vcgt_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100300, (Dn), (Dm)) )
#define vcgt_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200300, (Dn), (Dm)) )
#define vcgt_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000300, (Dn), (Dm)) )
#define vclt_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200e00, (Dm), (Dn)) )
#define vclt_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100300, (Dm), (Dn)) )
#define vclt_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200300, (Dm), (Dn)) )
#define vclt_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000300, (Dm), (Dn)) )
#define vclt_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100300, (Dm), (Dn)) )
#define vclt_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200300, (Dm), (Dn)) )
#define vclt_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000300, (Dm), (Dn)) )
#define vcgtq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200e40, (Qn), (Qm)) )
#define vcgtq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100340, (Qn), (Qm)) )
#define vcgtq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200340, (Qn), (Qm)) )
#define vcgtq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000340, (Qn), (Qm)) )
#define vcgtq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100340, (Qn), (Qm)) )
#define vcgtq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200340, (Qn), (Qm)) )
#define vcgtq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000340, (Qn), (Qm)) )
#define vcltq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200e40, (Qm), (Qn)) )
#define vcltq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100340, (Qm), (Qn)) )
#define vcltq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200340, (Qm), (Qn)) )
#define vcltq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000340, (Qm), (Qn)) )
#define vcltq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100340, (Qm), (Qn)) )
#define vcltq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200340, (Qm), (Qn)) )
#define vcltq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000340, (Qm), (Qn)) )

// VCLE (immediate #0)
#define vcle_z_f32_ex(Dm)                        ( __neon_DdDm( 0xf3b90580, (Dm)) )
#define vcle_z_s16_ex(Dm)                        ( __neon_DdDm( 0xf3b50180, (Dm)) )
#define vcle_z_s32_ex(Dm)                        ( __neon_DdDm( 0xf3b90180, (Dm)) )
#define vcle_z_s8_ex(Dm)                         ( __neon_DdDm( 0xf3b10180, (Dm)) )
#define vcleq_z_f32_ex(Qm)                       ( __neon_QdQm( 0xf3b905c0, (Qm)) )
#define vcleq_z_s16_ex(Qm)                       ( __neon_QdQm( 0xf3b501c0, (Qm)) )
#define vcleq_z_s32_ex(Qm)                       ( __neon_QdQm( 0xf3b901c0, (Qm)) )
#define vcleq_z_s8_ex(Qm)                        ( __neon_QdQm( 0xf3b101c0, (Qm)) )

// VCLS, VCLZ
#define vcls_s16(Dm)                             ( __neon_DdDm( 0xf3b40400, (Dm)) )
#define vcls_s32(Dm)                             ( __neon_DdDm( 0xf3b80400, (Dm)) )
#define vcls_s8(Dm)                              ( __neon_DdDm( 0xf3b00400, (Dm)) )
#define vclz_s16(Dm)                             ( __neon_DdDm( 0xf3b40480, (Dm)) )
#define vclz_s32(Dm)                             ( __neon_DdDm( 0xf3b80480, (Dm)) )
#define vclz_s8(Dm)                              ( __neon_DdDm( 0xf3b00480, (Dm)) )
#define vclz_u16(Dm)                             ( __neon_DdDm( 0xf3b40480, (Dm)) )
#define vclz_u32(Dm)                             ( __neon_DdDm( 0xf3b80480, (Dm)) )
#define vclz_u8(Dm)                              ( __neon_DdDm( 0xf3b00480, (Dm)) )
#define vclsq_s16(Qm)                            ( __neon_QdQm( 0xf3b40440, (Qm)) )
#define vclsq_s32(Qm)                            ( __neon_QdQm( 0xf3b80440, (Qm)) )
#define vclsq_s8(Qm)                             ( __neon_QdQm( 0xf3b00440, (Qm)) )
#define vclzq_s16(Qm)                            ( __neon_QdQm( 0xf3b404c0, (Qm)) )
#define vclzq_s32(Qm)                            ( __neon_QdQm( 0xf3b804c0, (Qm)) )
#define vclzq_s8(Qm)                             ( __neon_QdQm( 0xf3b004c0, (Qm)) )
#define vclzq_u16(Qm)                            ( __neon_QdQm( 0xf3b404c0, (Qm)) )
#define vclzq_u32(Qm)                            ( __neon_QdQm( 0xf3b804c0, (Qm)) )
#define vclzq_u8(Qm)                             ( __neon_QdQm( 0xf3b004c0, (Qm)) )

// VCLT (immediate #0)
#define vclt_z_f32_ex(Dm)                        ( __neon_DdDm( 0xf3b90600, (Dm)) )
#define vclt_z_s16_ex(Dm)                        ( __neon_DdDm( 0xf3b50200, (Dm)) )
#define vclt_z_s32_ex(Dm)                        ( __neon_DdDm( 0xf3b90200, (Dm)) )
#define vclt_z_s8_ex(Dm)                         ( __neon_DdDm( 0xf3b10200, (Dm)) )
#define vcltq_z_f32_ex(Qm)                       ( __neon_QdQm( 0xf3b90640, (Qm)) )
#define vcltq_z_s16_ex(Qm)                       ( __neon_QdQm( 0xf3b50240, (Qm)) )
#define vcltq_z_s32_ex(Qm)                       ( __neon_QdQm( 0xf3b90240, (Qm)) )
#define vcltq_z_s8_ex(Qm)                        ( __neon_QdQm( 0xf3b10240, (Qm)) )

// VCNT
#define vcnt_p8(Dm)                              ( __neon_DdDm( 0xf3b00500, (Dm)) )
#define vcnt_s8(Dm)                              ( __neon_DdDm( 0xf3b00500, (Dm)) )
#define vcnt_u8(Dm)                              ( __neon_DdDm( 0xf3b00500, (Dm)) )
#define vcntq_p8(Qm)                             ( __neon_QdQm( 0xf3b00540, (Qm)) )
#define vcntq_s8(Qm)                             ( __neon_QdQm( 0xf3b00540, (Qm)) )
#define vcntq_u8(Qm)                             ( __neon_QdQm( 0xf3b00540, (Qm)) )

// VCOMBINE (combine 2x64bit into a 128bit register)
#define vcombine_f32(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_p16(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_p8(Dn, Dm)                      ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_s16(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_s32(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_s64(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_s8(Dn, Dm)                      ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_u16(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_u32(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_u64(Dn, Dm)                     ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )
#define vcombine_u8(Dn, Dm)                      ( __neon_QdDnDm_merge( 0x00000000, (Dn), (Dm)) )

// VCREATE (ARM core register pair to Neon 64bit register)
#define vcreate_f32(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_p16(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_p8(R64t)                         ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_s16(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_s32(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_s64(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_s8(R64t)                         ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_u16(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_u32(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_u64(R64t)                        ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vcreate_u8(R64t)                         ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )

// VCVT (between floating-point and fixed-point)
#define vcvt_n_f32_s32(Dm, fbits)                ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_DdDm( 0xf2800e10 | _NENC_21_16(64 - (fbits)), (Dm)) )
#define vcvt_n_f32_u32(Dm, fbits)                ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_DdDm( 0xf3800e10 | _NENC_21_16(64 - (fbits)), (Dm)) )
#define vcvt_n_s32_f32(Dm, fbits)                ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_DdDm( 0xf2800f10 | _NENC_21_16(64 - (fbits)), (Dm)) )
#define vcvt_n_u32_f32(Dm, fbits)                ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_DdDm( 0xf3800f10 | _NENC_21_16(64 - (fbits)), (Dm)) )
#define vcvtq_n_f32_s32(Qm, fbits)               ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_QdQm( 0xf2800e50 | _NENC_21_16(64 - (fbits)), (Qm)) )
#define vcvtq_n_f32_u32(Qm, fbits)               ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_QdQm( 0xf3800e50 | _NENC_21_16(64 - (fbits)), (Qm)) )
#define vcvtq_n_s32_f32(Qm, fbits)               ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_QdQm( 0xf2800f50 | _NENC_21_16(64 - (fbits)), (Qm)) )
#define vcvtq_n_u32_f32(Qm, fbits)               ( __static_assert((fbits) >= 1 && (fbits) <= 32, "invalid fbits value"), __neon_QdQm( 0xf3800f50 | _NENC_21_16(64 - (fbits)), (Qm)) )

// VCVT (between floating-point and integer with directed rounding)
#define vcvta_s32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0000, (Dm)) )
#define vcvta_u32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0080, (Dm)) )
#define vcvtm_s32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0300, (Dm)) )
#define vcvtm_u32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0380, (Dm)) )
#define vcvtn_s32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0100, (Dm)) )
#define vcvtn_u32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0180, (Dm)) )
#define vcvtp_s32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0200, (Dm)) )
#define vcvtp_u32_f32(Dm)                        ( __neon_DdDm( 0xf3bb0280, (Dm)) )
#define vcvtaq_s32_f32(Qm)                       ( __neon_QdQm( 0xf3bb0040, (Qm)) )
#define vcvtaq_u32_f32(Qm)                       ( __neon_QdQm( 0xf3bb00c0, (Qm)) )
#define vcvtmq_s32_f32(Qm)                       ( __neon_QdQm( 0xf3bb0340, (Qm)) )
#define vcvtmq_u32_f32(Qm)                       ( __neon_QdQm( 0xf3bb03c0, (Qm)) )
#define vcvtnq_s32_f32(Qm)                       ( __neon_QdQm( 0xf3bb0140, (Qm)) )
#define vcvtnq_u32_f32(Qm)                       ( __neon_QdQm( 0xf3bb01c0, (Qm)) )
#define vcvtpq_s32_f32(Qm)                       ( __neon_QdQm( 0xf3bb0240, (Qm)) )
#define vcvtpq_u32_f32(Qm)                       ( __neon_QdQm( 0xf3bb02c0, (Qm)) )

// VCVT (between floating-point and integer)
#define vcvt_f32_s32(Dm)                         ( __neon_DdDm( 0xf3bb0600, (Dm)) )
#define vcvt_f32_u32(Dm)                         ( __neon_DdDm( 0xf3bb0680, (Dm)) )
#define vcvt_s32_f32(Dm)                         ( __neon_DdDm( 0xf3bb0700, (Dm)) )
#define vcvt_u32_f32(Dm)                         ( __neon_DdDm( 0xf3bb0780, (Dm)) )
#define vcvtq_f32_s32(Qm)                        ( __neon_QdQm( 0xf3bb0640, (Qm)) )
#define vcvtq_f32_u32(Qm)                        ( __neon_QdQm( 0xf3bb06c0, (Qm)) )
#define vcvtq_s32_f32(Qm)                        ( __neon_QdQm( 0xf3bb0740, (Qm)) )
#define vcvtq_u32_f32(Qm)                        ( __neon_QdQm( 0xf3bb07c0, (Qm)) )

// VDUP (scalar)
#define vdup_lane_f32(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDm( 0xf3b40c00 | _NENC_19(lane), (Dm)) )
#define vdup_lane_p16(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDm( 0xf3b20c00 | _NENC_19_18(lane), (Dm)) )
#define vdup_lane_p8(Dm, lane)                   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_DdDm( 0xf3b10c00 | _NENC_19_17(lane), (Dm)) )
#define vdup_lane_s16(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDm( 0xf3b20c00 | _NENC_19_18(lane), (Dm)) )
#define vdup_lane_s32(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDm( 0xf3b40c00 | _NENC_19(lane), (Dm)) )
#define vdup_lane_s8(Dm, lane)                   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_DdDm( 0xf3b10c00 | _NENC_19_17(lane), (Dm)) )
#define vdup_lane_u16(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDm( 0xf3b20c00 | _NENC_19_18(lane), (Dm)) )
#define vdup_lane_u32(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDm( 0xf3b40c00 | _NENC_19(lane), (Dm)) )
#define vdup_lane_u8(Dm, lane)                   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_DdDm( 0xf3b10c00 | _NENC_19_17(lane), (Dm)) )
#define vdupq_lane_f32(Dm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDm( 0xf3b40c40 | _NENC_19(lane), (Dm)) )
#define vdupq_lane_p16(Dm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDm( 0xf3b20c40 | _NENC_19_18(lane), (Dm)) )
#define vdupq_lane_p8(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_QdDm( 0xf3b10c40 | _NENC_19_17(lane), (Dm)) )
#define vdupq_lane_s16(Dm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDm( 0xf3b20c40 | _NENC_19_18(lane), (Dm)) )
#define vdupq_lane_s32(Dm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDm( 0xf3b40c40 | _NENC_19(lane), (Dm)) )
#define vdupq_lane_s8(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_QdDm( 0xf3b10c40 | _NENC_19_17(lane), (Dm)) )
#define vdupq_lane_u16(Dm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDm( 0xf3b20c40 | _NENC_19_18(lane), (Dm)) )
#define vdupq_lane_u32(Dm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDm( 0xf3b40c40 | _NENC_19(lane), (Dm)) )
#define vdupq_lane_u8(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_QdDm( 0xf3b10c40 | _NENC_19_17(lane), (Dm)) )

// VDUP, VMOV (ARM core register to Neon register)
#define vdup_n_f32(Ft)                           ( __neon_DdFt( 0xee800b10, (Ft)) )
#define vmov_n_f32(Ft)                           ( __neon_DdFt( 0xee800b10, (Ft)) )
#define vdup_n_p16(Rt)                           ( __neon_DdRt( 0xee800b30, __poly16ToInt32(Rt)) )
#define vdup_n_p8(Rt)                            ( __neon_DdRt( 0xeec00b10, __poly8ToInt32(Rt)) )
#define vdup_n_s16(Rt)                           ( __neon_DdRt( 0xee800b30, __int16ToInt32(Rt)) )
#define vdup_n_s32(Rt)                           ( __neon_DdRt( 0xee800b10, __int32ToInt32(Rt)) )
#define vdup_n_s8(Rt)                            ( __neon_DdRt( 0xeec00b10, __int8ToInt32(Rt)) )
#define vdup_n_u16(Rt)                           ( __neon_DdRt( 0xee800b30, __uint16ToInt32(Rt)) )
#define vdup_n_u32(Rt)                           ( __neon_DdRt( 0xee800b10, __uint32ToInt32(Rt)) )
#define vdup_n_u8(Rt)                            ( __neon_DdRt( 0xeec00b10, __uint8ToInt32(Rt)) )
#define vmov_n_p16(Rt)                           ( __neon_DdRt( 0xee800b30, __poly16ToInt32(Rt)) )
#define vmov_n_p8(Rt)                            ( __neon_DdRt( 0xeec00b10, __poly8ToInt32(Rt)) )
#define vmov_n_s16(Rt)                           ( __neon_DdRt( 0xee800b30, __int16ToInt32(Rt)) )
#define vmov_n_s32(Rt)                           ( __neon_DdRt( 0xee800b10, __int32ToInt32(Rt)) )
#define vmov_n_s8(Rt)                            ( __neon_DdRt( 0xeec00b10, __int8ToInt32(Rt)) )
#define vmov_n_u16(Rt)                           ( __neon_DdRt( 0xee800b30, __uint16ToInt32(Rt)) )
#define vmov_n_u32(Rt)                           ( __neon_DdRt( 0xee800b10, __uint32ToInt32(Rt)) )
#define vmov_n_u8(Rt)                            ( __neon_DdRt( 0xeec00b10, __uint8ToInt32(Rt)) )
#define vdupq_n_f32(Ft)                          ( __neon_QdFt( 0xeea00b10, (Ft)) )
#define vmovq_n_f32(Ft)                          ( __neon_QdFt( 0xeea00b10, (Ft)) )
#define vdupq_n_p16(Rt)                          ( __neon_QdRt( 0xeea00b30, __poly16ToInt32(Rt)) )
#define vdupq_n_p8(Rt)                           ( __neon_QdRt( 0xeee00b10, __poly8ToInt32(Rt)) )
#define vdupq_n_s16(Rt)                          ( __neon_QdRt( 0xeea00b30, __int16ToInt32(Rt)) )
#define vdupq_n_s32(Rt)                          ( __neon_QdRt( 0xeea00b10, __int32ToInt32(Rt)) )
#define vdupq_n_s8(Rt)                           ( __neon_QdRt( 0xeee00b10, __int8ToInt32(Rt)) )
#define vdupq_n_u16(Rt)                          ( __neon_QdRt( 0xeea00b30, __uint16ToInt32(Rt)) )
#define vdupq_n_u32(Rt)                          ( __neon_QdRt( 0xeea00b10, __uint32ToInt32(Rt)) )
#define vdupq_n_u8(Rt)                           ( __neon_QdRt( 0xeee00b10, __uint8ToInt32(Rt)) )
#define vmovq_n_p16(Rt)                          ( __neon_QdRt( 0xeea00b30, __poly16ToInt32(Rt)) )
#define vmovq_n_p8(Rt)                           ( __neon_QdRt( 0xeee00b10, __poly8ToInt32(Rt)) )
#define vmovq_n_s16(Rt)                          ( __neon_QdRt( 0xeea00b30, __int16ToInt32(Rt)) )
#define vmovq_n_s32(Rt)                          ( __neon_QdRt( 0xeea00b10, __int32ToInt32(Rt)) )
#define vmovq_n_s8(Rt)                           ( __neon_QdRt( 0xeee00b10, __int8ToInt32(Rt)) )
#define vmovq_n_u16(Rt)                          ( __neon_QdRt( 0xeea00b30, __uint16ToInt32(Rt)) )
#define vmovq_n_u32(Rt)                          ( __neon_QdRt( 0xeea00b10, __uint32ToInt32(Rt)) )
#define vmovq_n_u8(Rt)                           ( __neon_QdRt( 0xeee00b10, __uint8ToInt32(Rt)) )

// VDUP.64, VMOV.64 (ARM core register pair to Neon registers)
#define vdup_n_s64(R64t)                         ( __neon_DdRtRt2( 0xec400b10, __int64ToInt64(R64t)) )
#define vdup_n_u64(R64t)                         ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vmov_n_s64(R64t)                         ( __neon_DdRtRt2( 0xec400b10, __int64ToInt64(R64t)) )
#define vmov_n_u64(R64t)                         ( __neon_DdRtRt2( 0xec400b10, __uint64ToInt64(R64t)) )
#define vdupq_n_s64(R64t)                        ( __neon_QdRtRt2_dup( 0xec400b10, __int64ToInt64(R64t)) )
#define vdupq_n_u64(R64t)                        ( __neon_QdRtRt2_dup( 0xec400b10, __uint64ToInt64(R64t)) )
#define vmovq_n_s64(R64t)                        ( __neon_QdRtRt2_dup( 0xec400b10, __int64ToInt64(R64t)) )
#define vmovq_n_u64(R64t)                        ( __neon_QdRtRt2_dup( 0xec400b10, __uint64ToInt64(R64t)) )

// VEOR, VBIC, VORN
#define vbic_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define vbic_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define vbic_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define vbic_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define vbic_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define vbic_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define vbic_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define vbic_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2100110, (Dn), (Dm)) )
#define veor_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define veor_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define veor_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define veor_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define veor_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define veor_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define veor_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define veor_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000110, (Dn), (Dm)) )
#define vorn_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vorn_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vorn_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vorn_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vorn_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vorn_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vorn_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vorn_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2300110, (Dn), (Dm)) )
#define vbicq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define vbicq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define vbicq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define vbicq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define vbicq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define vbicq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define vbicq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define vbicq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2100150, (Qn), (Qm)) )
#define veorq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define veorq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define veorq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define veorq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define veorq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define veorq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define veorq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define veorq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000150, (Qn), (Qm)) )
#define vornq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )
#define vornq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )
#define vornq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )
#define vornq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )
#define vornq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )
#define vornq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )
#define vornq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )
#define vornq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2300150, (Qn), (Qm)) )

// VEXT
#define vext_f32(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 2, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 4), (Dn), (Dm)) )
#define vext_p16(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 4, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 2), (Dn), (Dm)) )
#define vext_p8(Dn, Dm, pos)                     ( __static_assert((pos) >= 0 && (pos) < 8, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8(pos), (Dn), (Dm)) )
#define vext_s16(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 4, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 2), (Dn), (Dm)) )
#define vext_s32(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 2, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 4), (Dn), (Dm)) )
#define vext_s64(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 1, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 8), (Dn), (Dm)) )
#define vext_s8(Dn, Dm, pos)                     ( __static_assert((pos) >= 0 && (pos) < 8, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8(pos), (Dn), (Dm)) )
#define vext_u16(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 4, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 2), (Dn), (Dm)) )
#define vext_u32(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 2, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 4), (Dn), (Dm)) )
#define vext_u64(Dn, Dm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 1, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8((pos) * 8), (Dn), (Dm)) )
#define vext_u8(Dn, Dm, pos)                     ( __static_assert((pos) >= 0 && (pos) < 8, "invalid position value"), __neon_DdDnDm( 0xf2b00000 | _NENC_11_8(pos), (Dn), (Dm)) )
#define vextq_f32(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 4, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 4), (Qn), (Qm)) )
#define vextq_p16(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 8, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 2), (Qn), (Qm)) )
#define vextq_p8(Qn, Qm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 16, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8(pos), (Qn), (Qm)) )
#define vextq_s16(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 8, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 2), (Qn), (Qm)) )
#define vextq_s32(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 4, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 4), (Qn), (Qm)) )
#define vextq_s64(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 2, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 8), (Qn), (Qm)) )
#define vextq_s8(Qn, Qm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 16, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8(pos), (Qn), (Qm)) )
#define vextq_u16(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 8, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 2), (Qn), (Qm)) )
#define vextq_u32(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 4, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 4), (Qn), (Qm)) )
#define vextq_u64(Qn, Qm, pos)                   ( __static_assert((pos) >= 0 && (pos) < 2, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8((pos) * 8), (Qn), (Qm)) )
#define vextq_u8(Qn, Qm, pos)                    ( __static_assert((pos) >= 0 && (pos) < 16, "invalid position value"), __neon_QdQnQm( 0xf2b00040 | _NENC_11_8(pos), (Qn), (Qm)) )

// VGET (access the 64bit high/low part of a 128bit register)
#define vget_high_f32(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_p16(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_p8(Qm)                         ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_s16(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_s32(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_s64(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_s8(Qm)                         ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_u16(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_u32(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_u64(Qm)                        ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_high_u8(Qm)                         ( __neon_DdQm_high( 0x00000000, (Qm)) )
#define vget_low_f32(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_p16(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_p8(Qm)                          ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_s16(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_s32(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_s64(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_s8(Qm)                          ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_u16(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_u32(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_u64(Qm)                         ( __neon_DdQm_low( 0x00000000, (Qm)) )
#define vget_low_u8(Qm)                          ( __neon_DdQm_low( 0x00000000, (Qm)) )

// VHADD, VRHADD, VHSUB
#define vhadd_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100000, (Dn), (Dm)) )
#define vhadd_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200000, (Dn), (Dm)) )
#define vhadd_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000000, (Dn), (Dm)) )
#define vhadd_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100000, (Dn), (Dm)) )
#define vhadd_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200000, (Dn), (Dm)) )
#define vhadd_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000000, (Dn), (Dm)) )
#define vhsub_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100200, (Dn), (Dm)) )
#define vhsub_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200200, (Dn), (Dm)) )
#define vhsub_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000200, (Dn), (Dm)) )
#define vhsub_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100200, (Dn), (Dm)) )
#define vhsub_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200200, (Dn), (Dm)) )
#define vhsub_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000200, (Dn), (Dm)) )
#define vrhadd_s16(Dn, Dm)                       ( __neon_DdDnDm( 0xf2100100, (Dn), (Dm)) )
#define vrhadd_s32(Dn, Dm)                       ( __neon_DdDnDm( 0xf2200100, (Dn), (Dm)) )
#define vrhadd_s8(Dn, Dm)                        ( __neon_DdDnDm( 0xf2000100, (Dn), (Dm)) )
#define vrhadd_u16(Dn, Dm)                       ( __neon_DdDnDm( 0xf3100100, (Dn), (Dm)) )
#define vrhadd_u32(Dn, Dm)                       ( __neon_DdDnDm( 0xf3200100, (Dn), (Dm)) )
#define vrhadd_u8(Dn, Dm)                        ( __neon_DdDnDm( 0xf3000100, (Dn), (Dm)) )
#define vhaddq_s16(Qn, Qm)                       ( __neon_QdQnQm( 0xf2100040, (Qn), (Qm)) )
#define vhaddq_s32(Qn, Qm)                       ( __neon_QdQnQm( 0xf2200040, (Qn), (Qm)) )
#define vhaddq_s8(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000040, (Qn), (Qm)) )
#define vhaddq_u16(Qn, Qm)                       ( __neon_QdQnQm( 0xf3100040, (Qn), (Qm)) )
#define vhaddq_u32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200040, (Qn), (Qm)) )
#define vhaddq_u8(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000040, (Qn), (Qm)) )
#define vhsubq_s16(Qn, Qm)                       ( __neon_QdQnQm( 0xf2100240, (Qn), (Qm)) )
#define vhsubq_s32(Qn, Qm)                       ( __neon_QdQnQm( 0xf2200240, (Qn), (Qm)) )
#define vhsubq_s8(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000240, (Qn), (Qm)) )
#define vhsubq_u16(Qn, Qm)                       ( __neon_QdQnQm( 0xf3100240, (Qn), (Qm)) )
#define vhsubq_u32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200240, (Qn), (Qm)) )
#define vhsubq_u8(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000240, (Qn), (Qm)) )
#define vrhaddq_s16(Qn, Qm)                      ( __neon_QdQnQm( 0xf2100140, (Qn), (Qm)) )
#define vrhaddq_s32(Qn, Qm)                      ( __neon_QdQnQm( 0xf2200140, (Qn), (Qm)) )
#define vrhaddq_s8(Qn, Qm)                       ( __neon_QdQnQm( 0xf2000140, (Qn), (Qm)) )
#define vrhaddq_u16(Qn, Qm)                      ( __neon_QdQnQm( 0xf3100140, (Qn), (Qm)) )
#define vrhaddq_u32(Qn, Qm)                      ( __neon_QdQnQm( 0xf3200140, (Qn), (Qm)) )
#define vrhaddq_u8(Qn, Qm)                       ( __neon_QdQnQm( 0xf3000140, (Qn), (Qm)) )

// VLD1 (multiple single elements)
#define vld1_f32(pcD)                            ( __neon_D1Adr( 0xf420078f, __float32ToN64_c(pcD)) )
#define vld1_p16(pcD)                            ( __neon_D1Adr( 0xf420074f, __poly16ToN64_c(pcD)) )
#define vld1_p8(pcD)                             ( __neon_D1Adr( 0xf420070f, __poly8ToN64_c(pcD)) )
#define vld1_s16(pcD)                            ( __neon_D1Adr( 0xf420074f, __int16ToN64_c(pcD)) )
#define vld1_s32(pcD)                            ( __neon_D1Adr( 0xf420078f, __int32ToN64_c(pcD)) )
#define vld1_s64(pcD)                            ( __neon_D1Adr( 0xf42007cf, __int64ToN64_c(pcD)) )
#define vld1_s8(pcD)                             ( __neon_D1Adr( 0xf420070f, __int8ToN64_c(pcD)) )
#define vld1_u16(pcD)                            ( __neon_D1Adr( 0xf420074f, __uint16ToN64_c(pcD)) )
#define vld1_u32(pcD)                            ( __neon_D1Adr( 0xf420078f, __uint32ToN64_c(pcD)) )
#define vld1_u64(pcD)                            ( __neon_D1Adr( 0xf42007cf, __uint64ToN64_c(pcD)) )
#define vld1_u8(pcD)                             ( __neon_D1Adr( 0xf420070f, __uint8ToN64_c(pcD)) )
#define vld1_f32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420078f | _NENC_5_4(_NEON_ALIGN64(align)), __float32ToN64_c(pcD)) )
#define vld1_p16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420074f | _NENC_5_4(_NEON_ALIGN64(align)), __poly16ToN64_c(pcD)) )
#define vld1_p8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420070f | _NENC_5_4(_NEON_ALIGN64(align)), __poly8ToN64_c(pcD)) )
#define vld1_s16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420074f | _NENC_5_4(_NEON_ALIGN64(align)), __int16ToN64_c(pcD)) )
#define vld1_s32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420078f | _NENC_5_4(_NEON_ALIGN64(align)), __int32ToN64_c(pcD)) )
#define vld1_s64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf42007cf | _NENC_5_4(_NEON_ALIGN64(align)), __int64ToN64_c(pcD)) )
#define vld1_s8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420070f | _NENC_5_4(_NEON_ALIGN64(align)), __int8ToN64_c(pcD)) )
#define vld1_u16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420074f | _NENC_5_4(_NEON_ALIGN64(align)), __uint16ToN64_c(pcD)) )
#define vld1_u32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420078f | _NENC_5_4(_NEON_ALIGN64(align)), __uint32ToN64_c(pcD)) )
#define vld1_u64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf42007cf | _NENC_5_4(_NEON_ALIGN64(align)), __uint64ToN64_c(pcD)) )
#define vld1_u8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_D1Adr( 0xf420070f | _NENC_5_4(_NEON_ALIGN64(align)), __uint8ToN64_c(pcD)) )
#define vld1q_f32(pcD)                           ( __neon_Q1Adr( 0xf4200a8f, __float32ToN64_c(pcD)) )
#define vld1q_p16(pcD)                           ( __neon_Q1Adr( 0xf4200a4f, __poly16ToN64_c(pcD)) )
#define vld1q_p8(pcD)                            ( __neon_Q1Adr( 0xf4200a0f, __poly8ToN64_c(pcD)) )
#define vld1q_s16(pcD)                           ( __neon_Q1Adr( 0xf4200a4f, __int16ToN64_c(pcD)) )
#define vld1q_s32(pcD)                           ( __neon_Q1Adr( 0xf4200a8f, __int32ToN64_c(pcD)) )
#define vld1q_s64(pcD)                           ( __neon_Q1Adr( 0xf4200acf, __int64ToN64_c(pcD)) )
#define vld1q_s8(pcD)                            ( __neon_Q1Adr( 0xf4200a0f, __int8ToN64_c(pcD)) )
#define vld1q_u16(pcD)                           ( __neon_Q1Adr( 0xf4200a4f, __uint16ToN64_c(pcD)) )
#define vld1q_u32(pcD)                           ( __neon_Q1Adr( 0xf4200a8f, __uint32ToN64_c(pcD)) )
#define vld1q_u64(pcD)                           ( __neon_Q1Adr( 0xf4200acf, __uint64ToN64_c(pcD)) )
#define vld1q_u8(pcD)                            ( __neon_Q1Adr( 0xf4200a0f, __uint8ToN64_c(pcD)) )
#define vld1q_f32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a8f | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64_c(pcD)) )
#define vld1q_p16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a4f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly16ToN64_c(pcD)) )
#define vld1q_p8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a0f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly8ToN64_c(pcD)) )
#define vld1q_s16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a4f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int16ToN64_c(pcD)) )
#define vld1q_s32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a8f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64_c(pcD)) )
#define vld1q_s64_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __int64ToN64_c(pcD)) )
#define vld1q_s8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a0f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int8ToN64_c(pcD)) )
#define vld1q_u16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a4f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint16ToN64_c(pcD)) )
#define vld1q_u32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a8f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64_c(pcD)) )
#define vld1q_u64_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint64ToN64_c(pcD)) )
#define vld1q_u8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4200a0f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint8ToN64_c(pcD)) )

// VLD1 (single element to all lanes)
#define vld1_dup_f32(pcD)                        ( __neon_D1Adr( 0xf4a00c8f, __float32ToN64_c(pcD)) )
#define vld1_dup_p16(pcD)                        ( __neon_D1Adr( 0xf4a00c4f, __poly16ToN64_c(pcD)) )
#define vld1_dup_p8(pcD)                         ( __neon_D1Adr( 0xf4a00c0f, __poly8ToN64_c(pcD)) )
#define vld1_dup_s16(pcD)                        ( __neon_D1Adr( 0xf4a00c4f, __int16ToN64_c(pcD)) )
#define vld1_dup_s32(pcD)                        ( __neon_D1Adr( 0xf4a00c8f, __int32ToN64_c(pcD)) )
#define vld1_dup_s8(pcD)                         ( __neon_D1Adr( 0xf4a00c0f, __int8ToN64_c(pcD)) )
#define vld1_dup_u16(pcD)                        ( __neon_D1Adr( 0xf4a00c4f, __uint16ToN64_c(pcD)) )
#define vld1_dup_u32(pcD)                        ( __neon_D1Adr( 0xf4a00c8f, __uint32ToN64_c(pcD)) )
#define vld1_dup_u8(pcD)                         ( __neon_D1Adr( 0xf4a00c0f, __uint8ToN64_c(pcD)) )
#define vld1q_dup_f32(pcD)                       ( __neon_Q1Adr( 0xf4a00caf, __float32ToN64_c(pcD)) )
#define vld1q_dup_p16(pcD)                       ( __neon_Q1Adr( 0xf4a00c6f, __poly16ToN64_c(pcD)) )
#define vld1q_dup_p8(pcD)                        ( __neon_Q1Adr( 0xf4a00c2f, __poly8ToN64_c(pcD)) )
#define vld1q_dup_s16(pcD)                       ( __neon_Q1Adr( 0xf4a00c6f, __int16ToN64_c(pcD)) )
#define vld1q_dup_s32(pcD)                       ( __neon_Q1Adr( 0xf4a00caf, __int32ToN64_c(pcD)) )
#define vld1q_dup_s8(pcD)                        ( __neon_Q1Adr( 0xf4a00c2f, __int8ToN64_c(pcD)) )
#define vld1q_dup_u16(pcD)                       ( __neon_Q1Adr( 0xf4a00c6f, __uint16ToN64_c(pcD)) )
#define vld1q_dup_u32(pcD)                       ( __neon_Q1Adr( 0xf4a00caf, __uint32ToN64_c(pcD)) )
#define vld1q_dup_u8(pcD)                        ( __neon_Q1Adr( 0xf4a00c2f, __uint8ToN64_c(pcD)) )

// VLD1 (single element to all lanes, aligned)
#define vld1_dup_f32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_D1Adr( 0xf4a00c8f | _NENC_4(_NEON_ALIGN32(align)), __float32ToN64_c(pcD)) )
#define vld1_dup_p16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_D1Adr( 0xf4a00c4f | _NENC_4(_NEON_ALIGN16(align)), __poly16ToN64_c(pcD)) )
#define vld1_dup_s16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_D1Adr( 0xf4a00c4f | _NENC_4(_NEON_ALIGN16(align)), __int16ToN64_c(pcD)) )
#define vld1_dup_s32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_D1Adr( 0xf4a00c8f | _NENC_4(_NEON_ALIGN32(align)), __int32ToN64_c(pcD)) )
#define vld1_dup_u16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_D1Adr( 0xf4a00c4f | _NENC_4(_NEON_ALIGN16(align)), __uint16ToN64_c(pcD)) )
#define vld1_dup_u32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_D1Adr( 0xf4a00c8f | _NENC_4(_NEON_ALIGN32(align)), __uint32ToN64_c(pcD)) )
#define vld1q_dup_f32_ex(pcD, align)             ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4a00caf | _NENC_4(_NEON_ALIGN32(align)), __float32ToN64_c(pcD)) )
#define vld1q_dup_p16_ex(pcD, align)             ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4a00c6f | _NENC_4(_NEON_ALIGN16(align)), __poly16ToN64_c(pcD)) )
#define vld1q_dup_s16_ex(pcD, align)             ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4a00c6f | _NENC_4(_NEON_ALIGN16(align)), __int16ToN64_c(pcD)) )
#define vld1q_dup_s32_ex(pcD, align)             ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4a00caf | _NENC_4(_NEON_ALIGN32(align)), __int32ToN64_c(pcD)) )
#define vld1q_dup_u16_ex(pcD, align)             ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4a00c6f | _NENC_4(_NEON_ALIGN16(align)), __uint16ToN64_c(pcD)) )
#define vld1q_dup_u32_ex(pcD, align)             ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Q1Adr( 0xf4a00caf | _NENC_4(_NEON_ALIGN32(align)), __uint32ToN64_c(pcD)) )

// VLD1 (single element to one lane)
#define vld1_lane_f32(pcD, Dd, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0080f | _NENC_7(lane), (Dd), __float32ToN64_c(pcD)) )
#define vld1_lane_p16(pcD, Dd, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0040f | _NENC_7_6(lane), (Dd), __poly16ToN64_c(pcD)) )
#define vld1_lane_p8(pcD, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0000f | _NENC_7_5(lane), (Dd), __poly8ToN64_c(pcD)) )
#define vld1_lane_s16(pcD, Dd, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0040f | _NENC_7_6(lane), (Dd), __int16ToN64_c(pcD)) )
#define vld1_lane_s32(pcD, Dd, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0080f | _NENC_7(lane), (Dd), __int32ToN64_c(pcD)) )
#define vld1_lane_s8(pcD, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0000f | _NENC_7_5(lane), (Dd), __int8ToN64_c(pcD)) )
#define vld1_lane_u16(pcD, Dd, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0040f | _NENC_7_6(lane), (Dd), __uint16ToN64_c(pcD)) )
#define vld1_lane_u32(pcD, Dd, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0080f | _NENC_7(lane), (Dd), __uint32ToN64_c(pcD)) )
#define vld1_lane_u8(pcD, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_D1Adr_acc( 0xf4a0000f | _NENC_7_5(lane), (Dd), __uint8ToN64_c(pcD)) )
#define vld1q_lane_f32(pcD, Qd, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Qd), __float32ToN64_c(pcD)) )
#define vld1q_lane_p16(pcD, Qd, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Qd), __poly16ToN64_c(pcD)) )
#define vld1q_lane_p8(pcD, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0000f | _NENC_7_5((lane) % 8) | _NENC_12((lane) >= 8 ? 1 : 0), (Qd), __poly8ToN64_c(pcD)) )
#define vld1q_lane_s16(pcD, Qd, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Qd), __int16ToN64_c(pcD)) )
#define vld1q_lane_s32(pcD, Qd, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Qd), __int32ToN64_c(pcD)) )
#define vld1q_lane_s8(pcD, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0000f | _NENC_7_5((lane) % 8) | _NENC_12((lane) >= 8 ? 1 : 0), (Qd), __int8ToN64_c(pcD)) )
#define vld1q_lane_u16(pcD, Qd, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Qd), __uint16ToN64_c(pcD)) )
#define vld1q_lane_u32(pcD, Qd, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Qd), __uint32ToN64_c(pcD)) )
#define vld1q_lane_u8(pcD, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_Q1Adr_acc( 0xf4a0000f | _NENC_7_5((lane) % 8) | _NENC_12((lane) >= 8 ? 1 : 0), (Qd), __uint8ToN64_c(pcD)) )

// VLD1 (single element to one lane, aligned)
#define vld1_lane_f32_ex(pcD, Dd, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_D1Adr_acc( 0xf4a0080f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), (Dd), __float32ToN64_c(pcD)) )
#define vld1_lane_p16_ex(pcD, Dd, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_D1Adr_acc( 0xf4a0040f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN16(align)), (Dd), __poly16ToN64_c(pcD)) )
#define vld1_lane_s16_ex(pcD, Dd, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_D1Adr_acc( 0xf4a0040f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN16(align)), (Dd), __int16ToN64_c(pcD)) )
#define vld1_lane_s32_ex(pcD, Dd, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_D1Adr_acc( 0xf4a0080f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), (Dd), __int32ToN64_c(pcD)) )
#define vld1_lane_u16_ex(pcD, Dd, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_D1Adr_acc( 0xf4a0040f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN16(align)), (Dd), __uint16ToN64_c(pcD)) )
#define vld1_lane_u32_ex(pcD, Dd, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_D1Adr_acc( 0xf4a0080f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), (Dd), __uint32ToN64_c(pcD)) )
#define vld1q_lane_f32_ex(pcD, Qd, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Q1Adr_acc( 0xf4a0080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), (Qd), __float32ToN64_c(pcD)) )
#define vld1q_lane_p16_ex(pcD, Qd, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Q1Adr_acc( 0xf4a0040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN16(align)), (Qd), __poly16ToN64_c(pcD)) )
#define vld1q_lane_s16_ex(pcD, Qd, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Q1Adr_acc( 0xf4a0040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN16(align)), (Qd), __int16ToN64_c(pcD)) )
#define vld1q_lane_s32_ex(pcD, Qd, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Q1Adr_acc( 0xf4a0080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), (Qd), __int32ToN64_c(pcD)) )
#define vld1q_lane_u16_ex(pcD, Qd, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Q1Adr_acc( 0xf4a0040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN16(align)), (Qd), __uint16ToN64_c(pcD)) )
#define vld1q_lane_u32_ex(pcD, Qd, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Q1Adr_acc( 0xf4a0080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), (Qd), __uint32ToN64_c(pcD)) )

// VLD2 (multiple 2-element structures)
#define vld2_f32(pcD)                            ( __neon_Dx2Adr( 0xf420088f, __float32ToN64_c(pcD)) )
#define vld2_p16(pcD)                            ( __neon_Dx2Adr( 0xf420084f, __poly16ToN64_c(pcD)) )
#define vld2_p8(pcD)                             ( __neon_Dx2Adr( 0xf420080f, __poly8ToN64_c(pcD)) )
#define vld2_s16(pcD)                            ( __neon_Dx2Adr( 0xf420084f, __int16ToN64_c(pcD)) )
#define vld2_s32(pcD)                            ( __neon_Dx2Adr( 0xf420088f, __int32ToN64_c(pcD)) )
#define vld2_s8(pcD)                             ( __neon_Dx2Adr( 0xf420080f, __int8ToN64_c(pcD)) )
#define vld2_u16(pcD)                            ( __neon_Dx2Adr( 0xf420084f, __uint16ToN64_c(pcD)) )
#define vld2_u32(pcD)                            ( __neon_Dx2Adr( 0xf420088f, __uint32ToN64_c(pcD)) )
#define vld2_u8(pcD)                             ( __neon_Dx2Adr( 0xf420080f, __uint8ToN64_c(pcD)) )
#define vld2_s64(pcD)                            ( __neon_Dx2Adr( 0xf4200acf, __int64ToN64_c(pcD)) )
#define vld2_u64(pcD)                            ( __neon_Dx2Adr( 0xf4200acf, __uint64ToN64_c(pcD)) )
#define vld2_s64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4200acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __int64ToN64_c(pcD)) )
#define vld2_u64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4200acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint64ToN64_c(pcD)) )
#define vld2_f32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420088f | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64_c(pcD)) )
#define vld2_p16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420084f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly16ToN64_c(pcD)) )
#define vld2_p8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420080f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly8ToN64_c(pcD)) )
#define vld2_s16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420084f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int16ToN64_c(pcD)) )
#define vld2_s32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420088f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64_c(pcD)) )
#define vld2_s8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420080f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int8ToN64_c(pcD)) )
#define vld2_u16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420084f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint16ToN64_c(pcD)) )
#define vld2_u32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420088f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64_c(pcD)) )
#define vld2_u8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf420080f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint8ToN64_c(pcD)) )
#define vld2q_f32(pcD)                           ( __neon_Qx2Adr( 0xf420098f, __float32ToN64_c(pcD)) )
#define vld2q_p16(pcD)                           ( __neon_Qx2Adr( 0xf420094f, __poly16ToN64_c(pcD)) )
#define vld2q_p8(pcD)                            ( __neon_Qx2Adr( 0xf420090f, __poly8ToN64_c(pcD)) )
#define vld2q_s16(pcD)                           ( __neon_Qx2Adr( 0xf420094f, __int16ToN64_c(pcD)) )
#define vld2q_s32(pcD)                           ( __neon_Qx2Adr( 0xf420098f, __int32ToN64_c(pcD)) )
#define vld2q_s8(pcD)                            ( __neon_Qx2Adr( 0xf420090f, __int8ToN64_c(pcD)) )
#define vld2q_u16(pcD)                           ( __neon_Qx2Adr( 0xf420094f, __uint16ToN64_c(pcD)) )
#define vld2q_u32(pcD)                           ( __neon_Qx2Adr( 0xf420098f, __uint32ToN64_c(pcD)) )
#define vld2q_u8(pcD)                            ( __neon_Qx2Adr( 0xf420090f, __uint8ToN64_c(pcD)) )
#define vld2q_f32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420098f | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64_c(pcD)) )
#define vld2q_p16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420094f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly16ToN64_c(pcD)) )
#define vld2q_p8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420090f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly8ToN64_c(pcD)) )
#define vld2q_s16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420094f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int16ToN64_c(pcD)) )
#define vld2q_s32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420098f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64_c(pcD)) )
#define vld2q_s8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420090f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int8ToN64_c(pcD)) )
#define vld2q_u16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420094f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint16ToN64_c(pcD)) )
#define vld2q_u32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420098f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64_c(pcD)) )
#define vld2q_u8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx2Adr( 0xf420090f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint8ToN64_c(pcD)) )

// VLD2 (single 2-element structure to all lanes)
#define vld2_dup_f32(pcD)                        ( __neon_Dx2Adr( 0xf4a00d8f, __float32ToN64_c(pcD)) )
#define vld2_dup_p16(pcD)                        ( __neon_Dx2Adr( 0xf4a00d4f, __poly16ToN64_c(pcD)) )
#define vld2_dup_p8(pcD)                         ( __neon_Dx2Adr( 0xf4a00d0f, __poly8ToN64_c(pcD)) )
#define vld2_dup_s16(pcD)                        ( __neon_Dx2Adr( 0xf4a00d4f, __int16ToN64_c(pcD)) )
#define vld2_dup_s32(pcD)                        ( __neon_Dx2Adr( 0xf4a00d8f, __int32ToN64_c(pcD)) )
#define vld2_dup_s8(pcD)                         ( __neon_Dx2Adr( 0xf4a00d0f, __int8ToN64_c(pcD)) )
#define vld2_dup_u16(pcD)                        ( __neon_Dx2Adr( 0xf4a00d4f, __uint16ToN64_c(pcD)) )
#define vld2_dup_u32(pcD)                        ( __neon_Dx2Adr( 0xf4a00d8f, __uint32ToN64_c(pcD)) )
#define vld2_dup_u8(pcD)                         ( __neon_Dx2Adr( 0xf4a00d0f, __uint8ToN64_c(pcD)) )
#define vld2_dup_s64(pcD)                        ( __neon_Dx2Adr( 0xf4200acf, __int64ToN64_c(pcD)) )
#define vld2_dup_u64(pcD)                        ( __neon_Dx2Adr( 0xf4200acf, __uint64ToN64_c(pcD)) )

// VLD2 (single 2-element structure to all lanes, aligned)
#define vld2_dup_s64_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4200acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __int64ToN64_c(pcD)) )
#define vld2_dup_u64_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4200acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint64ToN64_c(pcD)) )
#define vld2_dup_f32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d8f | _NENC_4(_NEON_ALIGN64(align)), __float32ToN64_c(pcD)) )
#define vld2_dup_p16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d4f | _NENC_4(_NEON_ALIGN32(align)), __poly16ToN64_c(pcD)) )
#define vld2_dup_p8_ex(pcD, align)               ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d0f | _NENC_4(_NEON_ALIGN16(align)), __poly8ToN64_c(pcD)) )
#define vld2_dup_s16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d4f | _NENC_4(_NEON_ALIGN32(align)), __int16ToN64_c(pcD)) )
#define vld2_dup_s32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d8f | _NENC_4(_NEON_ALIGN64(align)), __int32ToN64_c(pcD)) )
#define vld2_dup_s8_ex(pcD, align)               ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d0f | _NENC_4(_NEON_ALIGN16(align)), __int8ToN64_c(pcD)) )
#define vld2_dup_u16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d4f | _NENC_4(_NEON_ALIGN32(align)), __uint16ToN64_c(pcD)) )
#define vld2_dup_u32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d8f | _NENC_4(_NEON_ALIGN64(align)), __uint32ToN64_c(pcD)) )
#define vld2_dup_u8_ex(pcD, align)               ( __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Dx2Adr( 0xf4a00d0f | _NENC_4(_NEON_ALIGN16(align)), __uint8ToN64_c(pcD)) )

// VLD2 (single 2-element structure to one lane)
#define vld2_lane_f32(pcD, D2, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0090f | _NENC_7(lane), (D2), __float32ToN64_c(pcD)) )
#define vld2_lane_p16(pcD, D2, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0050f | _NENC_7_6(lane), (D2), __poly16ToN64_c(pcD)) )
#define vld2_lane_p8(pcD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0010f | _NENC_7_5(lane), (D2), __poly8ToN64_c(pcD)) )
#define vld2_lane_s16(pcD, D2, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0050f | _NENC_7_6(lane), (D2), __int16ToN64_c(pcD)) )
#define vld2_lane_s32(pcD, D2, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0090f | _NENC_7(lane), (D2), __int32ToN64_c(pcD)) )
#define vld2_lane_s8(pcD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0010f | _NENC_7_5(lane), (D2), __int8ToN64_c(pcD)) )
#define vld2_lane_u16(pcD, D2, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0050f | _NENC_7_6(lane), (D2), __uint16ToN64_c(pcD)) )
#define vld2_lane_u32(pcD, D2, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0090f | _NENC_7(lane), (D2), __uint32ToN64_c(pcD)) )
#define vld2_lane_u8(pcD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx2Adr_acc( 0xf4a0010f | _NENC_7_5(lane), (D2), __uint8ToN64_c(pcD)) )
#define vld2q_lane_f32(pcD, Q2, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx2Adr_acc( 0xf4a0094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q2), __float32ToN64_c(pcD)) )
#define vld2q_lane_p16(pcD, Q2, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx2Adr_acc( 0xf4a0052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q2), __poly16ToN64_c(pcD)) )
#define vld2q_lane_s16(pcD, Q2, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx2Adr_acc( 0xf4a0052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q2), __int16ToN64_c(pcD)) )
#define vld2q_lane_s32(pcD, Q2, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx2Adr_acc( 0xf4a0094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q2), __int32ToN64_c(pcD)) )
#define vld2q_lane_u16(pcD, Q2, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx2Adr_acc( 0xf4a0052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q2), __uint16ToN64_c(pcD)) )
#define vld2q_lane_u32(pcD, Q2, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx2Adr_acc( 0xf4a0094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q2), __uint32ToN64_c(pcD)) )

// VLD2 (single 2-element structure to one lane, aligned)
#define vld2_lane_f32_ex(pcD, D2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0090f | _NENC_7(lane) | _NENC_4(_NEON_ALIGN64(align)), (D2), __float32ToN64_c(pcD)) )
#define vld2_lane_p16_ex(pcD, D2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0050f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN32(align)), (D2), __poly16ToN64_c(pcD)) )
#define vld2_lane_p8_ex(pcD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0010f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN16(align)), (D2), __poly8ToN64_c(pcD)) )
#define vld2_lane_s16_ex(pcD, D2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0050f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN32(align)), (D2), __int16ToN64_c(pcD)) )
#define vld2_lane_s32_ex(pcD, D2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0090f | _NENC_7(lane) | _NENC_4(_NEON_ALIGN64(align)), (D2), __int32ToN64_c(pcD)) )
#define vld2_lane_s8_ex(pcD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0010f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN16(align)), (D2), __int8ToN64_c(pcD)) )
#define vld2_lane_u16_ex(pcD, D2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0050f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN32(align)), (D2), __uint16ToN64_c(pcD)) )
#define vld2_lane_u32_ex(pcD, D2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0090f | _NENC_7(lane) | _NENC_4(_NEON_ALIGN64(align)), (D2), __uint32ToN64_c(pcD)) )
#define vld2_lane_u8_ex(pcD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_Dx2Adr_acc( 0xf4a0010f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN16(align)), (D2), __uint8ToN64_c(pcD)) )
#define vld2q_lane_f32_ex(pcD, Q2, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx2Adr_acc( 0xf4a0094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), (Q2), __float32ToN64_c(pcD)) )
#define vld2q_lane_p16_ex(pcD, Q2, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Qx2Adr_acc( 0xf4a0052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN32(align)), (Q2), __poly16ToN64_c(pcD)) )
#define vld2q_lane_s16_ex(pcD, Q2, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Qx2Adr_acc( 0xf4a0052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN32(align)), (Q2), __int16ToN64_c(pcD)) )
#define vld2q_lane_s32_ex(pcD, Q2, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx2Adr_acc( 0xf4a0094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), (Q2), __int32ToN64_c(pcD)) )
#define vld2q_lane_u16_ex(pcD, Q2, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Qx2Adr_acc( 0xf4a0052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN32(align)), (Q2), __uint16ToN64_c(pcD)) )
#define vld2q_lane_u32_ex(pcD, Q2, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx2Adr_acc( 0xf4a0094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), (Q2), __uint32ToN64_c(pcD)) )

// VLD3 (multiple 3-element structures)
#define vld3_f32(pcD)                            ( __neon_Dx3Adr( 0xf420048f, __float32ToN64_c(pcD)) )
#define vld3_p16(pcD)                            ( __neon_Dx3Adr( 0xf420044f, __poly16ToN64_c(pcD)) )
#define vld3_p8(pcD)                             ( __neon_Dx3Adr( 0xf420040f, __poly8ToN64_c(pcD)) )
#define vld3_s16(pcD)                            ( __neon_Dx3Adr( 0xf420044f, __int16ToN64_c(pcD)) )
#define vld3_s32(pcD)                            ( __neon_Dx3Adr( 0xf420048f, __int32ToN64_c(pcD)) )
#define vld3_s8(pcD)                             ( __neon_Dx3Adr( 0xf420040f, __int8ToN64_c(pcD)) )
#define vld3_u16(pcD)                            ( __neon_Dx3Adr( 0xf420044f, __uint16ToN64_c(pcD)) )
#define vld3_u32(pcD)                            ( __neon_Dx3Adr( 0xf420048f, __uint32ToN64_c(pcD)) )
#define vld3_u8(pcD)                             ( __neon_Dx3Adr( 0xf420040f, __uint8ToN64_c(pcD)) )
#define vld3_s64(pcD)                            ( __neon_Dx3Adr( 0xf42006cf, __int64ToN64_c(pcD)) )
#define vld3_u64(pcD)                            ( __neon_Dx3Adr( 0xf42006cf, __uint64ToN64_c(pcD)) )
#define vld3_s64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf42006cf | _NENC_5_4(_NEON_ALIGN64(align)), __int64ToN64_c(pcD)) )
#define vld3_u64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf42006cf | _NENC_5_4(_NEON_ALIGN64(align)), __uint64ToN64_c(pcD)) )
#define vld3_f32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420048f | _NENC_5_4(_NEON_ALIGN64(align)), __float32ToN64_c(pcD)) )
#define vld3_p16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420044f | _NENC_5_4(_NEON_ALIGN64(align)), __poly16ToN64_c(pcD)) )
#define vld3_p8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420040f | _NENC_5_4(_NEON_ALIGN64(align)), __poly8ToN64_c(pcD)) )
#define vld3_s16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420044f | _NENC_5_4(_NEON_ALIGN64(align)), __int16ToN64_c(pcD)) )
#define vld3_s32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420048f | _NENC_5_4(_NEON_ALIGN64(align)), __int32ToN64_c(pcD)) )
#define vld3_s8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420040f | _NENC_5_4(_NEON_ALIGN64(align)), __int8ToN64_c(pcD)) )
#define vld3_u16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420044f | _NENC_5_4(_NEON_ALIGN64(align)), __uint16ToN64_c(pcD)) )
#define vld3_u32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420048f | _NENC_5_4(_NEON_ALIGN64(align)), __uint32ToN64_c(pcD)) )
#define vld3_u8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx3Adr( 0xf420040f | _NENC_5_4(_NEON_ALIGN64(align)), __uint8ToN64_c(pcD)) )
#define vld3q_f32(pcD)                           ( __neon_Qx3Adr( 0xf420058f, __float32ToN64_c(pcD)) )
#define vld3q_p16(pcD)                           ( __neon_Qx3Adr( 0xf420054f, __poly16ToN64_c(pcD)) )
#define vld3q_p8(pcD)                            ( __neon_Qx3Adr( 0xf420050f, __poly8ToN64_c(pcD)) )
#define vld3q_s16(pcD)                           ( __neon_Qx3Adr( 0xf420054f, __int16ToN64_c(pcD)) )
#define vld3q_s32(pcD)                           ( __neon_Qx3Adr( 0xf420058f, __int32ToN64_c(pcD)) )
#define vld3q_s8(pcD)                            ( __neon_Qx3Adr( 0xf420050f, __int8ToN64_c(pcD)) )
#define vld3q_u16(pcD)                           ( __neon_Qx3Adr( 0xf420054f, __uint16ToN64_c(pcD)) )
#define vld3q_u32(pcD)                           ( __neon_Qx3Adr( 0xf420058f, __uint32ToN64_c(pcD)) )
#define vld3q_u8(pcD)                            ( __neon_Qx3Adr( 0xf420050f, __uint8ToN64_c(pcD)) )
#define vld3q_f32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420058f | _NENC_5_4(_NEON_ALIGN64(align)), __float32ToN64_c(pcD)) )
#define vld3q_p16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420054f | _NENC_5_4(_NEON_ALIGN64(align)), __poly16ToN64_c(pcD)) )
#define vld3q_p8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420050f | _NENC_5_4(_NEON_ALIGN64(align)), __poly8ToN64_c(pcD)) )
#define vld3q_s16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420054f | _NENC_5_4(_NEON_ALIGN64(align)), __int16ToN64_c(pcD)) )
#define vld3q_s32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420058f | _NENC_5_4(_NEON_ALIGN64(align)), __int32ToN64_c(pcD)) )
#define vld3q_s8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420050f | _NENC_5_4(_NEON_ALIGN64(align)), __int8ToN64_c(pcD)) )
#define vld3q_u16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420054f | _NENC_5_4(_NEON_ALIGN64(align)), __uint16ToN64_c(pcD)) )
#define vld3q_u32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420058f | _NENC_5_4(_NEON_ALIGN64(align)), __uint32ToN64_c(pcD)) )
#define vld3q_u8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx3Adr( 0xf420050f | _NENC_5_4(_NEON_ALIGN64(align)), __uint8ToN64_c(pcD)) )

// VLD3 (single 3-element structure to all lanes)
#define vld3_dup_f32(pcD)                        ( __neon_Dx3Adr( 0xf4a00e8f, __float32ToN64_c(pcD)) )
#define vld3_dup_p16(pcD)                        ( __neon_Dx3Adr( 0xf4a00e4f, __poly16ToN64_c(pcD)) )
#define vld3_dup_p8(pcD)                         ( __neon_Dx3Adr( 0xf4a00e0f, __poly8ToN64_c(pcD)) )
#define vld3_dup_s16(pcD)                        ( __neon_Dx3Adr( 0xf4a00e4f, __int16ToN64_c(pcD)) )
#define vld3_dup_s32(pcD)                        ( __neon_Dx3Adr( 0xf4a00e8f, __int32ToN64_c(pcD)) )
#define vld3_dup_s8(pcD)                         ( __neon_Dx3Adr( 0xf4a00e0f, __int8ToN64_c(pcD)) )
#define vld3_dup_u16(pcD)                        ( __neon_Dx3Adr( 0xf4a00e4f, __uint16ToN64_c(pcD)) )
#define vld3_dup_u32(pcD)                        ( __neon_Dx3Adr( 0xf4a00e8f, __uint32ToN64_c(pcD)) )
#define vld3_dup_u8(pcD)                         ( __neon_Dx3Adr( 0xf4a00e0f, __uint8ToN64_c(pcD)) )
#define vld3_dup_s64(pcD)                        ( __neon_Dx3Adr( 0xf42006cf, __int64ToN64_c(pcD)) )
#define vld3_dup_u64(pcD)                        ( __neon_Dx3Adr( 0xf42006cf, __uint64ToN64_c(pcD)) )

// VLD3 (single 3-element structure to one lane)
#define vld3_lane_f32(pcD, D3, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a00a0f | _NENC_7(lane), (D3), __float32ToN64_c(pcD)) )
#define vld3_lane_p16(pcD, D3, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a0060f | _NENC_7_6(lane), (D3), __poly16ToN64_c(pcD)) )
#define vld3_lane_p8(pcD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a0020f | _NENC_7_5(lane), (D3), __poly8ToN64_c(pcD)) )
#define vld3_lane_s16(pcD, D3, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a0060f | _NENC_7_6(lane), (D3), __int16ToN64_c(pcD)) )
#define vld3_lane_s32(pcD, D3, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a00a0f | _NENC_7(lane), (D3), __int32ToN64_c(pcD)) )
#define vld3_lane_s8(pcD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a0020f | _NENC_7_5(lane), (D3), __int8ToN64_c(pcD)) )
#define vld3_lane_u16(pcD, D3, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a0060f | _NENC_7_6(lane), (D3), __uint16ToN64_c(pcD)) )
#define vld3_lane_u32(pcD, D3, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a00a0f | _NENC_7(lane), (D3), __uint32ToN64_c(pcD)) )
#define vld3_lane_u8(pcD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx3Adr_acc( 0xf4a0020f | _NENC_7_5(lane), (D3), __uint8ToN64_c(pcD)) )
#define vld3q_lane_f32(pcD, Q3, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx3Adr_acc( 0xf4a00a4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q3), __float32ToN64_c(pcD)) )
#define vld3q_lane_p16(pcD, Q3, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx3Adr_acc( 0xf4a0062f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q3), __poly16ToN64_c(pcD)) )
#define vld3q_lane_s16(pcD, Q3, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx3Adr_acc( 0xf4a0062f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q3), __int16ToN64_c(pcD)) )
#define vld3q_lane_s32(pcD, Q3, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx3Adr_acc( 0xf4a00a4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q3), __int32ToN64_c(pcD)) )
#define vld3q_lane_u16(pcD, Q3, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx3Adr_acc( 0xf4a0062f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q3), __uint16ToN64_c(pcD)) )
#define vld3q_lane_u32(pcD, Q3, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx3Adr_acc( 0xf4a00a4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q3), __uint32ToN64_c(pcD)) )

// VLD4 (multiple 4-element structures)
#define vld4_f32(pcD)                            ( __neon_Dx4Adr( 0xf420008f, __float32ToN64_c(pcD)) )
#define vld4_p16(pcD)                            ( __neon_Dx4Adr( 0xf420004f, __poly16ToN64_c(pcD)) )
#define vld4_p8(pcD)                             ( __neon_Dx4Adr( 0xf420000f, __poly8ToN64_c(pcD)) )
#define vld4_s16(pcD)                            ( __neon_Dx4Adr( 0xf420004f, __int16ToN64_c(pcD)) )
#define vld4_s32(pcD)                            ( __neon_Dx4Adr( 0xf420008f, __int32ToN64_c(pcD)) )
#define vld4_s8(pcD)                             ( __neon_Dx4Adr( 0xf420000f, __int8ToN64_c(pcD)) )
#define vld4_u16(pcD)                            ( __neon_Dx4Adr( 0xf420004f, __uint16ToN64_c(pcD)) )
#define vld4_u32(pcD)                            ( __neon_Dx4Adr( 0xf420008f, __uint32ToN64_c(pcD)) )
#define vld4_u8(pcD)                             ( __neon_Dx4Adr( 0xf420000f, __uint8ToN64_c(pcD)) )
#define vld4_s64(pcD)                            ( __neon_Dx4Adr( 0xf42002cf, __int64ToN64_c(pcD)) )
#define vld4_u64(pcD)                            ( __neon_Dx4Adr( 0xf42002cf, __uint64ToN64_c(pcD)) )
#define vld4_s64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf42002cf | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int64ToN64_c(pcD)) )
#define vld4_u64_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf42002cf | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint64ToN64_c(pcD)) )
#define vld4_f32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420008f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __float32ToN64_c(pcD)) )
#define vld4_p16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420004f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly16ToN64_c(pcD)) )
#define vld4_p8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420000f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly8ToN64_c(pcD)) )
#define vld4_s16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420004f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int16ToN64_c(pcD)) )
#define vld4_s32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420008f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int32ToN64_c(pcD)) )
#define vld4_s8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420000f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int8ToN64_c(pcD)) )
#define vld4_u16_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420004f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint16ToN64_c(pcD)) )
#define vld4_u32_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420008f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint32ToN64_c(pcD)) )
#define vld4_u8_ex(pcD, align)                   ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf420000f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint8ToN64_c(pcD)) )
#define vld4q_f32(pcD)                           ( __neon_Qx4Adr( 0xf420018f, __float32ToN64_c(pcD)) )
#define vld4q_p16(pcD)                           ( __neon_Qx4Adr( 0xf420014f, __poly16ToN64_c(pcD)) )
#define vld4q_p8(pcD)                            ( __neon_Qx4Adr( 0xf420010f, __poly8ToN64_c(pcD)) )
#define vld4q_s16(pcD)                           ( __neon_Qx4Adr( 0xf420014f, __int16ToN64_c(pcD)) )
#define vld4q_s32(pcD)                           ( __neon_Qx4Adr( 0xf420018f, __int32ToN64_c(pcD)) )
#define vld4q_s8(pcD)                            ( __neon_Qx4Adr( 0xf420010f, __int8ToN64_c(pcD)) )
#define vld4q_u16(pcD)                           ( __neon_Qx4Adr( 0xf420014f, __uint16ToN64_c(pcD)) )
#define vld4q_u32(pcD)                           ( __neon_Qx4Adr( 0xf420018f, __uint32ToN64_c(pcD)) )
#define vld4q_u8(pcD)                            ( __neon_Qx4Adr( 0xf420010f, __uint8ToN64_c(pcD)) )
#define vld4q_f32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420018f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __float32ToN64_c(pcD)) )
#define vld4q_p16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420014f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly16ToN64_c(pcD)) )
#define vld4q_p8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420010f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly8ToN64_c(pcD)) )
#define vld4q_s16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420014f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int16ToN64_c(pcD)) )
#define vld4q_s32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420018f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int32ToN64_c(pcD)) )
#define vld4q_s8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420010f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int8ToN64_c(pcD)) )
#define vld4q_u16_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420014f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint16ToN64_c(pcD)) )
#define vld4q_u32_ex(pcD, align)                 ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420018f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint32ToN64_c(pcD)) )
#define vld4q_u8_ex(pcD, align)                  ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_Qx4Adr( 0xf420010f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint8ToN64_c(pcD)) )

// VLD4 (single 4-element structure to all lanes)
#define vld4_dup_f32(pcD)                        ( __neon_Dx4Adr( 0xf4a00f8f, __float32ToN64_c(pcD)) )
#define vld4_dup_p16(pcD)                        ( __neon_Dx4Adr( 0xf4a00f4f, __poly16ToN64_c(pcD)) )
#define vld4_dup_p8(pcD)                         ( __neon_Dx4Adr( 0xf4a00f0f, __poly8ToN64_c(pcD)) )
#define vld4_dup_s16(pcD)                        ( __neon_Dx4Adr( 0xf4a00f4f, __int16ToN64_c(pcD)) )
#define vld4_dup_s32(pcD)                        ( __neon_Dx4Adr( 0xf4a00f8f, __int32ToN64_c(pcD)) )
#define vld4_dup_s8(pcD)                         ( __neon_Dx4Adr( 0xf4a00f0f, __int8ToN64_c(pcD)) )
#define vld4_dup_u16(pcD)                        ( __neon_Dx4Adr( 0xf4a00f4f, __uint16ToN64_c(pcD)) )
#define vld4_dup_u32(pcD)                        ( __neon_Dx4Adr( 0xf4a00f8f, __uint32ToN64_c(pcD)) )
#define vld4_dup_u8(pcD)                         ( __neon_Dx4Adr( 0xf4a00f0f, __uint8ToN64_c(pcD)) )
#define vld4_dup_s64(pcD)                        ( __neon_Dx4Adr( 0xf42002cf, __int64ToN64_c(pcD)) )
#define vld4_dup_u64(pcD)                        ( __neon_Dx4Adr( 0xf42002cf, __uint64ToN64_c(pcD)) )

// VLD4 (single 4-element structure to all lanes, aligned)
#define vld4_dup_f32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f0f | _NENC_7_6(_NEON_ALIGN64_128(align) > 1 ? 3 : 2) | _NENC_4(_NEON_ALIGN64_128(align) > 0 ? 1 : 0), __float32ToN64_c(pcD)) )
#define vld4_dup_p16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f4f | _NENC_4(_NEON_ALIGN64(align)), __poly16ToN64_c(pcD)) )
#define vld4_dup_p8_ex(pcD, align)               ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f0f | _NENC_4(_NEON_ALIGN32(align)), __poly8ToN64_c(pcD)) )
#define vld4_dup_s16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f4f | _NENC_4(_NEON_ALIGN64(align)), __int16ToN64_c(pcD)) )
#define vld4_dup_s32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f0f | _NENC_7_6(_NEON_ALIGN64_128(align) > 1 ? 3 : 2) | _NENC_4(_NEON_ALIGN64_128(align) > 0 ? 1 : 0), __int32ToN64_c(pcD)) )
#define vld4_dup_s8_ex(pcD, align)               ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f0f | _NENC_4(_NEON_ALIGN32(align)), __int8ToN64_c(pcD)) )
#define vld4_dup_u16_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f4f | _NENC_4(_NEON_ALIGN64(align)), __uint16ToN64_c(pcD)) )
#define vld4_dup_u32_ex(pcD, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f0f | _NENC_7_6(_NEON_ALIGN64_128(align) > 1 ? 3 : 2) | _NENC_4(_NEON_ALIGN64_128(align) > 0 ? 1 : 0), __uint32ToN64_c(pcD)) )
#define vld4_dup_u8_ex(pcD, align)               ( __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx4Adr( 0xf4a00f0f | _NENC_4(_NEON_ALIGN32(align)), __uint8ToN64_c(pcD)) )

// VLD4 (single 4-element structure to one lane)
#define vld4_lane_f32(pcD, D4, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a00b0f | _NENC_7(lane), (D4), __float32ToN64_c(pcD)) )
#define vld4_lane_p16(pcD, D4, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a0070f | _NENC_7_6(lane), (D4), __poly16ToN64_c(pcD)) )
#define vld4_lane_p8(pcD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a0030f | _NENC_7_5(lane), (D4), __poly8ToN64_c(pcD)) )
#define vld4_lane_s16(pcD, D4, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a0070f | _NENC_7_6(lane), (D4), __int16ToN64_c(pcD)) )
#define vld4_lane_s32(pcD, D4, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a00b0f | _NENC_7(lane), (D4), __int32ToN64_c(pcD)) )
#define vld4_lane_s8(pcD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a0030f | _NENC_7_5(lane), (D4), __int8ToN64_c(pcD)) )
#define vld4_lane_u16(pcD, D4, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a0070f | _NENC_7_6(lane), (D4), __uint16ToN64_c(pcD)) )
#define vld4_lane_u32(pcD, D4, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a00b0f | _NENC_7(lane), (D4), __uint32ToN64_c(pcD)) )
#define vld4_lane_u8(pcD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Dx4Adr_acc( 0xf4a0030f | _NENC_7_5(lane), (D4), __uint8ToN64_c(pcD)) )
#define vld4q_lane_f32(pcD, Q4, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx4Adr_acc( 0xf4a00b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q4), __float32ToN64_c(pcD)) )
#define vld4q_lane_p16(pcD, Q4, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx4Adr_acc( 0xf4a0072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q4), __poly16ToN64_c(pcD)) )
#define vld4q_lane_s16(pcD, Q4, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx4Adr_acc( 0xf4a0072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q4), __int16ToN64_c(pcD)) )
#define vld4q_lane_s32(pcD, Q4, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx4Adr_acc( 0xf4a00b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q4), __int32ToN64_c(pcD)) )
#define vld4q_lane_u16(pcD, Q4, lane)            ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_Qx4Adr_acc( 0xf4a0072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), (Q4), __uint16ToN64_c(pcD)) )
#define vld4q_lane_u32(pcD, Q4, lane)            ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_Qx4Adr_acc( 0xf4a00b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), (Q4), __uint32ToN64_c(pcD)) )

// VLD4 (single 4-element structure to one lane, aligned)
#define vld4_lane_f32_ex(pcD, D4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a00b0f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN64_128(align)), (D4), __float32ToN64_c(pcD)) )
#define vld4_lane_p16_ex(pcD, D4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a0070f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN64(align)), (D4), __poly16ToN64_c(pcD)) )
#define vld4_lane_p8_ex(pcD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a0030f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN32(align)), (D4), __poly8ToN64_c(pcD)) )
#define vld4_lane_s16_ex(pcD, D4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a0070f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN64(align)), (D4), __int16ToN64_c(pcD)) )
#define vld4_lane_s32_ex(pcD, D4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a00b0f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN64_128(align)), (D4), __int32ToN64_c(pcD)) )
#define vld4_lane_s8_ex(pcD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a0030f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN32(align)), (D4), __int8ToN64_c(pcD)) )
#define vld4_lane_u16_ex(pcD, D4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a0070f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN64(align)), (D4), __uint16ToN64_c(pcD)) )
#define vld4_lane_u32_ex(pcD, D4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a00b0f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN64_128(align)), (D4), __uint32ToN64_c(pcD)) )
#define vld4_lane_u8_ex(pcD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_Dx4Adr_acc( 0xf4a0030f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN32(align)), (D4), __uint8ToN64_c(pcD)) )
#define vld4q_lane_f32_ex(pcD, Q4, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx4Adr_acc( 0xf4a00b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN64_128(align)), (Q4), __float32ToN64_c(pcD)) )
#define vld4q_lane_p16_ex(pcD, Q4, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx4Adr_acc( 0xf4a0072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), (Q4), __poly16ToN64_c(pcD)) )
#define vld4q_lane_s16_ex(pcD, Q4, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx4Adr_acc( 0xf4a0072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), (Q4), __int16ToN64_c(pcD)) )
#define vld4q_lane_s32_ex(pcD, Q4, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx4Adr_acc( 0xf4a00b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN64_128(align)), (Q4), __int32ToN64_c(pcD)) )
#define vld4q_lane_u16_ex(pcD, Q4, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_Qx4Adr_acc( 0xf4a0072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), (Q4), __uint16ToN64_c(pcD)) )
#define vld4q_lane_u32_ex(pcD, Q4, lane, align)  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_Qx4Adr_acc( 0xf4a00b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN64_128(align)), (Q4), __uint32ToN64_c(pcD)) )

// VMAX, VMIN (floating point)
#define vmax_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000f00, (Dn), (Dm)) )
#define vmaxnm_f32(Dn, Dm)                       ( __neon_DdDnDm( 0xf3000f10, (Dn), (Dm)) )
#define vmin_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200f00, (Dn), (Dm)) )
#define vminnm_f32(Dn, Dm)                       ( __neon_DdDnDm( 0xf3200f10, (Dn), (Dm)) )
#define vmaxq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000f40, (Qn), (Qm)) )
#define vmaxnmq_f32(Qn, Qm)                      ( __neon_QdQnQm( 0xf3000f50, (Qn), (Qm)) )
#define vminq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200f40, (Qn), (Qm)) )
#define vminnmq_f32(Qn, Qm)                      ( __neon_QdQnQm( 0xf3200f50, (Qn), (Qm)) )

// VMAX, VMIN (integer)
#define vmax_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100600, (Dn), (Dm)) )
#define vmax_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200600, (Dn), (Dm)) )
#define vmax_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000600, (Dn), (Dm)) )
#define vmax_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100600, (Dn), (Dm)) )
#define vmax_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200600, (Dn), (Dm)) )
#define vmax_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000600, (Dn), (Dm)) )
#define vmin_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100610, (Dn), (Dm)) )
#define vmin_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200610, (Dn), (Dm)) )
#define vmin_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000610, (Dn), (Dm)) )
#define vmin_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100610, (Dn), (Dm)) )
#define vmin_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200610, (Dn), (Dm)) )
#define vmin_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000610, (Dn), (Dm)) )
#define vmaxq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100640, (Qn), (Qm)) )
#define vmaxq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200640, (Qn), (Qm)) )
#define vmaxq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000640, (Qn), (Qm)) )
#define vmaxq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100640, (Qn), (Qm)) )
#define vmaxq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200640, (Qn), (Qm)) )
#define vmaxq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000640, (Qn), (Qm)) )
#define vminq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100650, (Qn), (Qm)) )
#define vminq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200650, (Qn), (Qm)) )
#define vminq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000650, (Qn), (Qm)) )
#define vminq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100650, (Qn), (Qm)) )
#define vminq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200650, (Qn), (Qm)) )
#define vminq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000650, (Qn), (Qm)) )

// VMLA, VMLS (by scalar)
#define vmla_lane_f32(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2a00140 | _NENC_5(lane), (Dd), (Dn), (Dm)) )
#define vmla_lane_s16(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2900040 | _NENC_5x3(lane), (Dd), (Dn), (Dm)) )
#define vmla_lane_s32(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2a00040 | _NENC_5(lane), (Dd), (Dn), (Dm)) )
#define vmla_lane_u16(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2900040 | _NENC_5x3(lane), (Dd), (Dn), (Dm)) )
#define vmla_lane_u32(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2a00040 | _NENC_5(lane), (Dd), (Dn), (Dm)) )
#define vmls_lane_f32(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2a00540 | _NENC_5(lane), (Dd), (Dn), (Dm)) )
#define vmls_lane_s16(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2900440 | _NENC_5x3(lane), (Dd), (Dn), (Dm)) )
#define vmls_lane_s32(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2a00440 | _NENC_5(lane), (Dd), (Dn), (Dm)) )
#define vmls_lane_u16(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2900440 | _NENC_5x3(lane), (Dd), (Dn), (Dm)) )
#define vmls_lane_u32(Dd, Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx_acc( 0xf2a00440 | _NENC_5(lane), (Dd), (Dn), (Dm)) )
#define vmlaq_lane_f32(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3a00140 | _NENC_5(lane), (Qd), (Qn), (Dm)) )
#define vmlaq_lane_s16(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3900040 | _NENC_5x3(lane), (Qd), (Qn), (Dm)) )
#define vmlaq_lane_s32(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3a00040 | _NENC_5(lane), (Qd), (Qn), (Dm)) )
#define vmlaq_lane_u16(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3900040 | _NENC_5x3(lane), (Qd), (Qn), (Dm)) )
#define vmlaq_lane_u32(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3a00040 | _NENC_5(lane), (Qd), (Qn), (Dm)) )
#define vmlsq_lane_f32(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3a00540 | _NENC_5(lane), (Qd), (Qn), (Dm)) )
#define vmlsq_lane_s16(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3900440 | _NENC_5x3(lane), (Qd), (Qn), (Dm)) )
#define vmlsq_lane_s32(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3a00440 | _NENC_5(lane), (Qd), (Qn), (Dm)) )
#define vmlsq_lane_u16(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3900440 | _NENC_5x3(lane), (Qd), (Qn), (Dm)) )
#define vmlsq_lane_u32(Qd, Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx_acc( 0xf3a00440 | _NENC_5(lane), (Qd), (Qn), (Dm)) )

// VMLA, VMLS (float, by scalar)
#define vmla_n_f32(Dd, Dn, Ft)                   ( __neon_DdDnFt_acc( 0xf2a00140, (Dd), (Dn), (Ft)) )
#define vmls_n_f32(Dd, Dn, Ft)                   ( __neon_DdDnFt_acc( 0xf2a00540, (Dd), (Dn), (Ft)) )
#define vmlaq_n_f32(Qd, Qn, Ft)                  ( __neon_QdQnFt_acc( 0xf3a00140, (Qd), (Qn), (Ft)) )
#define vmlsq_n_f32(Qd, Qn, Ft)                  ( __neon_QdQnFt_acc( 0xf3a00540, (Qd), (Qn), (Ft)) )

// VMLA, VMLS (floating point)
#define vmla_f32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2000d10, (Dd), (Dn), (Dm)) )
#define vmls_f32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2200d10, (Dd), (Dn), (Dm)) )
#define vmlaq_f32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2000d50, (Qd), (Qn), (Qm)) )
#define vmlsq_f32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2200d50, (Qd), (Qn), (Qm)) )

// VMLA, VMLS (integer)
#define vmla_s16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2100900, (Dd), (Dn), (Dm)) )
#define vmla_s32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2200900, (Dd), (Dn), (Dm)) )
#define vmla_s8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf2000900, (Dd), (Dn), (Dm)) )
#define vmla_u16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2100900, (Dd), (Dn), (Dm)) )
#define vmla_u32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf2200900, (Dd), (Dn), (Dm)) )
#define vmla_u8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf2000900, (Dd), (Dn), (Dm)) )
#define vmls_s16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100900, (Dd), (Dn), (Dm)) )
#define vmls_s32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200900, (Dd), (Dn), (Dm)) )
#define vmls_s8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3000900, (Dd), (Dn), (Dm)) )
#define vmls_u16(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3100900, (Dd), (Dn), (Dm)) )
#define vmls_u32(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3200900, (Dd), (Dn), (Dm)) )
#define vmls_u8(Dd, Dn, Dm)                      ( __neon_DdDnDm_acc( 0xf3000900, (Dd), (Dn), (Dm)) )
#define vmlaq_s16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2100940, (Qd), (Qn), (Qm)) )
#define vmlaq_s32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2200940, (Qd), (Qn), (Qm)) )
#define vmlaq_s8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf2000940, (Qd), (Qn), (Qm)) )
#define vmlaq_u16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2100940, (Qd), (Qn), (Qm)) )
#define vmlaq_u32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf2200940, (Qd), (Qn), (Qm)) )
#define vmlaq_u8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf2000940, (Qd), (Qn), (Qm)) )
#define vmlsq_s16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100940, (Qd), (Qn), (Qm)) )
#define vmlsq_s32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200940, (Qd), (Qn), (Qm)) )
#define vmlsq_s8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3000940, (Qd), (Qn), (Qm)) )
#define vmlsq_u16(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3100940, (Qd), (Qn), (Qm)) )
#define vmlsq_u32(Qd, Qn, Qm)                    ( __neon_QdQnQm_acc( 0xf3200940, (Qd), (Qn), (Qm)) )
#define vmlsq_u8(Qd, Qn, Qm)                     ( __neon_QdQnQm_acc( 0xf3000940, (Qd), (Qn), (Qm)) )

// VMLAL, VMLSL
#define vmlal_s16(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf2900800, (Qd), (Dn), (Dm)) )
#define vmlal_s32(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf2a00800, (Qd), (Dn), (Dm)) )
#define vmlal_s8(Qd, Dn, Dm)                     ( __neon_QdDnDm_acc( 0xf2800800, (Qd), (Dn), (Dm)) )
#define vmlal_u16(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf3900800, (Qd), (Dn), (Dm)) )
#define vmlal_u32(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf3a00800, (Qd), (Dn), (Dm)) )
#define vmlal_u8(Qd, Dn, Dm)                     ( __neon_QdDnDm_acc( 0xf3800800, (Qd), (Dn), (Dm)) )
#define vmlsl_s16(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf2900a00, (Qd), (Dn), (Dm)) )
#define vmlsl_s32(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf2a00a00, (Qd), (Dn), (Dm)) )
#define vmlsl_s8(Qd, Dn, Dm)                     ( __neon_QdDnDm_acc( 0xf2800a00, (Qd), (Dn), (Dm)) )
#define vmlsl_u16(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf3900a00, (Qd), (Dn), (Dm)) )
#define vmlsl_u32(Qd, Dn, Dm)                    ( __neon_QdDnDm_acc( 0xf3a00a00, (Qd), (Dn), (Dm)) )
#define vmlsl_u8(Qd, Dn, Dm)                     ( __neon_QdDnDm_acc( 0xf3800a00, (Qd), (Dn), (Dm)) )

// VMLAL, VMLSL (by scalar)
#define vmlal_lane_s16(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2900240 | _NENC_5x3(lane), (Qd), (Dn), (Dm)) )
#define vmlal_lane_s32(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2a00240 | _NENC_5(lane), (Qd), (Dn), (Dm)) )
#define vmlal_lane_u16(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx_acc( 0xf3900240 | _NENC_5x3(lane), (Qd), (Dn), (Dm)) )
#define vmlal_lane_u32(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx_acc( 0xf3a00240 | _NENC_5(lane), (Qd), (Dn), (Dm)) )
#define vmlsl_lane_s16(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2900640 | _NENC_5x3(lane), (Qd), (Dn), (Dm)) )
#define vmlsl_lane_s32(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2a00640 | _NENC_5(lane), (Qd), (Dn), (Dm)) )
#define vmlsl_lane_u16(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx_acc( 0xf3900640 | _NENC_5x3(lane), (Qd), (Dn), (Dm)) )
#define vmlsl_lane_u32(Qd, Dn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx_acc( 0xf3a00640 | _NENC_5(lane), (Qd), (Dn), (Dm)) )

// VMOV (ARM core register to scalar)
#define vset_lane_f32(Ft, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdFt_acc( 0xee000b10 | _NENC_21(lane), (Dd), (Ft)) )
#define vset_lane_p16(Rt, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdRt_acc( 0xee000b30 | _NENC_21x6(lane), (Dd), __poly16ToInt32(Rt)) )
#define vset_lane_p8(Rt, Dd, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_DdRt_acc( 0xee400b10 | _NENC_21x6_5(lane), (Dd), __poly8ToInt32(Rt)) )
#define vset_lane_s16(Rt, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdRt_acc( 0xee000b30 | _NENC_21x6(lane), (Dd), __int16ToInt32(Rt)) )
#define vset_lane_s32(Rt, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdRt_acc( 0xee000b10 | _NENC_21(lane), (Dd), __int32ToInt32(Rt)) )
#define vset_lane_s8(Rt, Dd, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_DdRt_acc( 0xee400b10 | _NENC_21x6_5(lane), (Dd), __int8ToInt32(Rt)) )
#define vset_lane_u16(Rt, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdRt_acc( 0xee000b30 | _NENC_21x6(lane), (Dd), __uint16ToInt32(Rt)) )
#define vset_lane_u32(Rt, Dd, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdRt_acc( 0xee000b10 | _NENC_21(lane), (Dd), __uint32ToInt32(Rt)) )
#define vset_lane_u8(Rt, Dd, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_DdRt_acc( 0xee400b10 | _NENC_21x6_5(lane), (Dd), __uint8ToInt32(Rt)) )

// VMOV (scalar to ARM core register)
#define vget_lane_f32(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_FtDn( 0xee100b10 | _NENC_21(lane), (Dm)) )
#define vget_lane_p16(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), (poly16_t)__neon_RtDn( 0xee900b30 | _NENC_21x6(lane), (Dm)) )
#define vget_lane_p8(Dm, lane)                   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), (poly8_t)__neon_RtDn( 0xeed00b10 | _NENC_21x6_5(lane), (Dm)) )
#define vget_lane_s16(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), (int16_t)__neon_RtDn( 0xee100b30 | _NENC_21x6(lane), (Dm)) )
#define vget_lane_s8(Dm, lane)                   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), (int8_t)__neon_RtDn( 0xee500b10 | _NENC_21x6_5(lane), (Dm)) )
#define vget_lane_s32(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), (int32_t)__neon_RtDn( 0xee100b10 | _NENC_21(lane), (Dm)) )
#define vget_lane_u16(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), (uint16_t)__neon_RtDn( 0xee900b30 | _NENC_21x6(lane), (Dm)) )
#define vget_lane_u8(Dm, lane)                   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), (uint8_t)__neon_RtDn( 0xeed00b10 | _NENC_21x6_5(lane), (Dm)) )
#define vget_lane_u32(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), (uint32_t)__neon_RtDn( 0xee100b10 | _NENC_21(lane), (Dm)) )

// VMOV.64 (ARM core register pair to scalar)
#define vset_lane_s64(R64t, Dd, lane)            ( __static_assert((lane) >= 0 && (lane) < 1, "invalid lane index"), __neon_DdRtRt2_acc( 0xec400b10, (Dd), __int64ToInt64(R64t)) )
#define vset_lane_u64(R64t, Dd, lane)            ( __static_assert((lane) >= 0 && (lane) < 1, "invalid lane index"), __neon_DdRtRt2_acc( 0xec400b10, (Dd), __uint64ToInt64(R64t)) )
#define vsetq_lane_s64(R64t, Qd, lane)           ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdRtRt2_acc( 0xec400b10 | _NENC_0(lane), (Qd), __int64ToInt64(R64t)) )
#define vsetq_lane_u64(R64t, Qd, lane)           ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdRtRt2_acc( 0xec400b10 | _NENC_0(lane), (Qd), __uint64ToInt64(R64t)) )

// VMOV.64 (scalar to ARM core register pair)
#define vget_lane_s64(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 1, "invalid lane index"), (int64_t)__neon_RtRt2Dm( 0xec500b10, (Dm)) )
#define vget_lane_u64(Dm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 1, "invalid lane index"), (uint64_t)__neon_RtRt2Dm( 0xec500b10, (Dm)) )
#define vgetq_lane_s64(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), (int64_t)__neon_RtRt2Qm( 0xec500b10 | _NENC_0(lane), (Qm)) )
#define vgetq_lane_u64(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), (uint64_t)__neon_RtRt2Qm( 0xec500b10 | _NENC_0(lane), (Qm)) )

// VMOV.Q (ARM core register to scalar)
#define vsetq_lane_f32(Ft, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdFt_acc( 0xee000b10 | _NENC_16((lane) >= 2 ? 1 : 0) | _NENC_21((lane) % 2), (Qd), (Ft)) )
#define vsetq_lane_p16(Rt, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_QdRt_acc( 0xee000b30 | _NENC_16((lane) >= 4 ? 1 : 0) | _NENC_21x6((lane) % 4), (Qd), __poly16ToInt32(Rt)) )
#define vsetq_lane_p8(Rt, Qd, lane)              ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_QdRt_acc( 0xee400b10 | _NENC_16((lane) >= 8 ? 1 : 0) | _NENC_21x6_5((lane) % 8), (Qd), __poly8ToInt32(Rt)) )
#define vsetq_lane_s16(Rt, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_QdRt_acc( 0xee000b30 | _NENC_16((lane) >= 4 ? 1 : 0) | _NENC_21x6((lane) % 4), (Qd), __int16ToInt32(Rt)) )
#define vsetq_lane_s32(Rt, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdRt_acc( 0xee000b10 | _NENC_16((lane) >= 2 ? 1 : 0) | _NENC_21((lane) % 2), (Qd), __int32ToInt32(Rt)) )
#define vsetq_lane_s8(Rt, Qd, lane)              ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_QdRt_acc( 0xee400b10 | _NENC_16((lane) >= 8 ? 1 : 0) | _NENC_21x6_5((lane) % 8), (Qd), __int8ToInt32(Rt)) )
#define vsetq_lane_u16(Rt, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_QdRt_acc( 0xee000b30 | _NENC_16((lane) >= 4 ? 1 : 0) | _NENC_21x6((lane) % 4), (Qd), __uint16ToInt32(Rt)) )
#define vsetq_lane_u32(Rt, Qd, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdRt_acc( 0xee000b10 | _NENC_16((lane) >= 2 ? 1 : 0) | _NENC_21((lane) % 2), (Qd), __uint32ToInt32(Rt)) )
#define vsetq_lane_u8(Rt, Qd, lane)              ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_QdRt_acc( 0xee400b10 | _NENC_16((lane) >= 8 ? 1 : 0) | _NENC_21x6_5((lane) % 8), (Qd), __uint8ToInt32(Rt)) )

// VMOV.Q (scalar to ARM core register)
#define vgetq_lane_f32(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_FtQn( 0xee100b10 | _NENC_16((lane) >= 2 ? 1 : 0) | _NENC_21((lane) % 2), (Qm)) )
#define vgetq_lane_p16(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), (poly16_t)__neon_RtQn( 0xee900b30 | _NENC_16((lane) >= 4 ? 1 : 0) | _NENC_21x6((lane) % 4), (Qm)) )
#define vgetq_lane_p8(Qm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), (poly8_t)__neon_RtQn( 0xeed00b10 | _NENC_16((lane) >= 8 ? 1 : 0) | _NENC_21x6_5((lane) % 8), (Qm)) )
#define vgetq_lane_s16(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), (int16_t)__neon_RtQn( 0xee100b30 | _NENC_16((lane) >= 4 ? 1 : 0) | _NENC_21x6((lane) % 4), (Qm)) )
#define vgetq_lane_s8(Qm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), (int8_t)__neon_RtQn( 0xee500b10 | _NENC_16((lane) >= 8 ? 1 : 0) | _NENC_21x6_5((lane) % 8), (Qm)) )
#define vgetq_lane_s32(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), (int32_t)__neon_RtQn( 0xee100b10 | _NENC_16((lane) >= 2 ? 1 : 0) | _NENC_21((lane) % 2), (Qm)) )
#define vgetq_lane_u16(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), (uint16_t)__neon_RtQn( 0xee900b30 | _NENC_16((lane) >= 4 ? 1 : 0) | _NENC_21x6((lane) % 4), (Qm)) )
#define vgetq_lane_u8(Qm, lane)                  ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), (uint8_t)__neon_RtQn( 0xeed00b10 | _NENC_16((lane) >= 8 ? 1 : 0) | _NENC_21x6_5((lane) % 8), (Qm)) )
#define vgetq_lane_u32(Qm, lane)                 ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), (uint32_t)__neon_RtQn( 0xee100b10 | _NENC_16((lane) >= 2 ? 1 : 0) | _NENC_21((lane) % 2), (Qm)) )

// VMOVL
#define vmovl_s16(Dm)                            ( __neon_QdDm( 0xf2900a10, (Dm)) )
#define vmovl_s32(Dm)                            ( __neon_QdDm( 0xf2a00a10, (Dm)) )
#define vmovl_s8(Dm)                             ( __neon_QdDm( 0xf2880a10, (Dm)) )
#define vmovl_u16(Dm)                            ( __neon_QdDm( 0xf3900a10, (Dm)) )
#define vmovl_u32(Dm)                            ( __neon_QdDm( 0xf3a00a10, (Dm)) )
#define vmovl_u8(Dm)                             ( __neon_QdDm( 0xf3880a10, (Dm)) )

// VMOVN
#define vmovn_s16(Qm)                            ( __neon_DdQm( 0xf3b20200, (Qm)) )
#define vmovn_s32(Qm)                            ( __neon_DdQm( 0xf3b60200, (Qm)) )
#define vmovn_s64(Qm)                            ( __neon_DdQm( 0xf3ba0200, (Qm)) )
#define vmovn_u16(Qm)                            ( __neon_DdQm( 0xf3b20200, (Qm)) )
#define vmovn_u32(Qm)                            ( __neon_DdQm( 0xf3b60200, (Qm)) )
#define vmovn_u64(Qm)                            ( __neon_DdQm( 0xf3ba0200, (Qm)) )

// VMUL
#define vmul_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000d10, (Dn), (Dm)) )
#define vmul_p8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000910, (Dn), (Dm)) )
#define vmul_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100910, (Dn), (Dm)) )
#define vmul_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200910, (Dn), (Dm)) )
#define vmul_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000910, (Dn), (Dm)) )
#define vmul_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100910, (Dn), (Dm)) )
#define vmul_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200910, (Dn), (Dm)) )
#define vmul_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000910, (Dn), (Dm)) )
#define vmulq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000d50, (Qn), (Qm)) )
#define vmulq_p8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000950, (Qn), (Qm)) )
#define vmulq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100950, (Qn), (Qm)) )
#define vmulq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200950, (Qn), (Qm)) )
#define vmulq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000950, (Qn), (Qm)) )
#define vmulq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100950, (Qn), (Qm)) )
#define vmulq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200950, (Qn), (Qm)) )
#define vmulq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000950, (Qn), (Qm)) )

// VMUL (by scalar - float)
#define vmul_n_f32(Dn, Ft)                       ( __neon_DdDnFt( 0xf2a00940, (Dn), (Ft)) )
#define vmulq_n_f32(Qn, Ft)                      ( __neon_QdQnFt( 0xf3a00940, (Qn), (Ft)) )

// VMUL (by scalar)
#define vmul_lane_f32(Dn, Dm, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx( 0xf2a00940 | _NENC_5(lane), (Dn), (Dm)) )
#define vmul_lane_s16(Dn, Dm, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx( 0xf2900840 | _NENC_5x3(lane), (Dn), (Dm)) )
#define vmul_lane_s32(Dn, Dm, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx( 0xf2a00840 | _NENC_5(lane), (Dn), (Dm)) )
#define vmul_lane_u16(Dn, Dm, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx( 0xf2900840 | _NENC_5x3(lane), (Dn), (Dm)) )
#define vmul_lane_u32(Dn, Dm, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx( 0xf2a00840 | _NENC_5(lane), (Dn), (Dm)) )
#define vmulq_lane_f32(Qn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx( 0xf3a00940 | _NENC_5(lane), (Qn), (Dm)) )
#define vmulq_lane_s16(Qn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx( 0xf3900840 | _NENC_5x3(lane), (Qn), (Dm)) )
#define vmulq_lane_s32(Qn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx( 0xf3a00840 | _NENC_5(lane), (Qn), (Dm)) )
#define vmulq_lane_u16(Qn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx( 0xf3900840 | _NENC_5x3(lane), (Qn), (Dm)) )
#define vmulq_lane_u32(Qn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx( 0xf3a00840 | _NENC_5(lane), (Qn), (Dm)) )

// VMULL
#define vmull_p64(Dn, Dm)                        ( __neon_QdDnDm( 0xf2a00e00, (Dn), (Dm)) )
#define vmull_p8(Dn, Dm)                         ( __neon_QdDnDm( 0xf2800e00, (Dn), (Dm)) )
#define vmull_s16(Dn, Dm)                        ( __neon_QdDnDm( 0xf2900c00, (Dn), (Dm)) )
#define vmull_s32(Dn, Dm)                        ( __neon_QdDnDm( 0xf2a00c00, (Dn), (Dm)) )
#define vmull_s8(Dn, Dm)                         ( __neon_QdDnDm( 0xf2800c00, (Dn), (Dm)) )
#define vmull_u16(Dn, Dm)                        ( __neon_QdDnDm( 0xf3900c00, (Dn), (Dm)) )
#define vmull_u32(Dn, Dm)                        ( __neon_QdDnDm( 0xf3a00c00, (Dn), (Dm)) )
#define vmull_u8(Dn, Dm)                         ( __neon_QdDnDm( 0xf3800c00, (Dn), (Dm)) )

// VMULL (by scalar)
#define vmull_lane_s16(Dn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx( 0xf2900a40 | _NENC_5x3(lane), (Dn), (Dm)) )
#define vmull_lane_s32(Dn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx( 0xf2a00a40 | _NENC_5(lane), (Dn), (Dm)) )
#define vmull_lane_u16(Dn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx( 0xf3900a40 | _NENC_5x3(lane), (Dn), (Dm)) )
#define vmull_lane_u32(Dn, Dm, lane)             ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx( 0xf3a00a40 | _NENC_5(lane), (Dn), (Dm)) )

// VMVN
#define vmvn_p16(Dm)                             ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvn_p8(Dm)                              ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvn_s16(Dm)                             ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvn_s32(Dm)                             ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvn_s8(Dm)                              ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvn_u16(Dm)                             ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvn_u32(Dm)                             ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvn_u8(Dm)                              ( __neon_DdDm( 0xf3b00580, (Dm)) )
#define vmvnq_p16(Qm)                            ( __neon_QdQm( 0xf3b005c0, (Qm)) )
#define vmvnq_p8(Qm)                             ( __neon_QdQm( 0xf3b005c0, (Qm)) )
#define vmvnq_s16(Qm)                            ( __neon_QdQm( 0xf3b005c0, (Qm)) )
#define vmvnq_s32(Qm)                            ( __neon_QdQm( 0xf3b005c0, (Qm)) )
#define vmvnq_s8(Qm)                             ( __neon_QdQm( 0xf3b005c0, (Qm)) )
#define vmvnq_u16(Qm)                            ( __neon_QdQm( 0xf3b005c0, (Qm)) )
#define vmvnq_u32(Qm)                            ( __neon_QdQm( 0xf3b005c0, (Qm)) )
#define vmvnq_u8(Qm)                             ( __neon_QdQm( 0xf3b005c0, (Qm)) )

// VPADAL
#define vpadal_s16(Dd, Dm)                       ( __neon_DdDm_acc( 0xf3b40600, (Dd), (Dm)) )
#define vpadal_s32(Dd, Dm)                       ( __neon_DdDm_acc( 0xf3b80600, (Dd), (Dm)) )
#define vpadal_s8(Dd, Dm)                        ( __neon_DdDm_acc( 0xf3b00600, (Dd), (Dm)) )
#define vpadal_u16(Dd, Dm)                       ( __neon_DdDm_acc( 0xf3b40680, (Dd), (Dm)) )
#define vpadal_u32(Dd, Dm)                       ( __neon_DdDm_acc( 0xf3b80680, (Dd), (Dm)) )
#define vpadal_u8(Dd, Dm)                        ( __neon_DdDm_acc( 0xf3b00680, (Dd), (Dm)) )
#define vpadalq_s16(Qd, Qm)                      ( __neon_QdQm_acc( 0xf3b40640, (Qd), (Qm)) )
#define vpadalq_s32(Qd, Qm)                      ( __neon_QdQm_acc( 0xf3b80640, (Qd), (Qm)) )
#define vpadalq_s8(Qd, Qm)                       ( __neon_QdQm_acc( 0xf3b00640, (Qd), (Qm)) )
#define vpadalq_u16(Qd, Qm)                      ( __neon_QdQm_acc( 0xf3b406c0, (Qd), (Qm)) )
#define vpadalq_u32(Qd, Qm)                      ( __neon_QdQm_acc( 0xf3b806c0, (Qd), (Qm)) )
#define vpadalq_u8(Qd, Qm)                       ( __neon_QdQm_acc( 0xf3b006c0, (Qd), (Qm)) )

// VPADD (floating point)
#define vpadd_f32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3000d00, (Dn), (Dm)) )

// VPADD (integer)
#define vpadd_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100b10, (Dn), (Dm)) )
#define vpadd_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200b10, (Dn), (Dm)) )
#define vpadd_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000b10, (Dn), (Dm)) )
#define vpadd_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100b10, (Dn), (Dm)) )
#define vpadd_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200b10, (Dn), (Dm)) )
#define vpadd_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000b10, (Dn), (Dm)) )

// VPADDL
#define vpaddl_s16(Dm)                           ( __neon_DdDm( 0xf3b40200, (Dm)) )
#define vpaddl_s32(Dm)                           ( __neon_DdDm( 0xf3b80200, (Dm)) )
#define vpaddl_s8(Dm)                            ( __neon_DdDm( 0xf3b00200, (Dm)) )
#define vpaddl_u16(Dm)                           ( __neon_DdDm( 0xf3b40280, (Dm)) )
#define vpaddl_u32(Dm)                           ( __neon_DdDm( 0xf3b80280, (Dm)) )
#define vpaddl_u8(Dm)                            ( __neon_DdDm( 0xf3b00280, (Dm)) )
#define vpaddlq_s16(Qm)                          ( __neon_QdQm( 0xf3b40240, (Qm)) )
#define vpaddlq_s32(Qm)                          ( __neon_QdQm( 0xf3b80240, (Qm)) )
#define vpaddlq_s8(Qm)                           ( __neon_QdQm( 0xf3b00240, (Qm)) )
#define vpaddlq_u16(Qm)                          ( __neon_QdQm( 0xf3b402c0, (Qm)) )
#define vpaddlq_u32(Qm)                          ( __neon_QdQm( 0xf3b802c0, (Qm)) )
#define vpaddlq_u8(Qm)                           ( __neon_QdQm( 0xf3b002c0, (Qm)) )

// VPMAX, VPMIN (floating point)
#define vpmax_f32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3000f00, (Dn), (Dm)) )
#define vpmin_f32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200f00, (Dn), (Dm)) )

// VPMAX, VPMIN (integer)
#define vpmax_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100a00, (Dn), (Dm)) )
#define vpmax_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200a00, (Dn), (Dm)) )
#define vpmax_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000a00, (Dn), (Dm)) )
#define vpmax_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100a00, (Dn), (Dm)) )
#define vpmax_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200a00, (Dn), (Dm)) )
#define vpmax_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000a00, (Dn), (Dm)) )
#define vpmin_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100a10, (Dn), (Dm)) )
#define vpmin_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200a10, (Dn), (Dm)) )
#define vpmin_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000a10, (Dn), (Dm)) )
#define vpmin_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100a10, (Dn), (Dm)) )
#define vpmin_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200a10, (Dn), (Dm)) )
#define vpmin_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000a10, (Dn), (Dm)) )

// VQABS, VQNEG
#define vqabs_s16(Dm)                            ( __neon_DdDm( 0xf3b40700, (Dm)) )
#define vqabs_s32(Dm)                            ( __neon_DdDm( 0xf3b80700, (Dm)) )
#define vqabs_s8(Dm)                             ( __neon_DdDm( 0xf3b00700, (Dm)) )
#define vqneg_s16(Dm)                            ( __neon_DdDm( 0xf3b40780, (Dm)) )
#define vqneg_s32(Dm)                            ( __neon_DdDm( 0xf3b80780, (Dm)) )
#define vqneg_s8(Dm)                             ( __neon_DdDm( 0xf3b00780, (Dm)) )
#define vqabsq_s16(Qm)                           ( __neon_QdQm( 0xf3b40740, (Qm)) )
#define vqabsq_s32(Qm)                           ( __neon_QdQm( 0xf3b80740, (Qm)) )
#define vqabsq_s8(Qm)                            ( __neon_QdQm( 0xf3b00740, (Qm)) )
#define vqnegq_s16(Qm)                           ( __neon_QdQm( 0xf3b407c0, (Qm)) )
#define vqnegq_s32(Qm)                           ( __neon_QdQm( 0xf3b807c0, (Qm)) )
#define vqnegq_s8(Qm)                            ( __neon_QdQm( 0xf3b007c0, (Qm)) )

// VQADD
#define vqadd_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100010, (Dn), (Dm)) )
#define vqadd_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200010, (Dn), (Dm)) )
#define vqadd_s64(Dn, Dm)                        ( __neon_DdDnDm( 0xf2300010, (Dn), (Dm)) )
#define vqadd_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000010, (Dn), (Dm)) )
#define vqadd_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100010, (Dn), (Dm)) )
#define vqadd_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200010, (Dn), (Dm)) )
#define vqadd_u64(Dn, Dm)                        ( __neon_DdDnDm( 0xf3300010, (Dn), (Dm)) )
#define vqadd_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000010, (Dn), (Dm)) )
#define vqaddq_s16(Qn, Qm)                       ( __neon_QdQnQm( 0xf2100050, (Qn), (Qm)) )
#define vqaddq_s32(Qn, Qm)                       ( __neon_QdQnQm( 0xf2200050, (Qn), (Qm)) )
#define vqaddq_s64(Qn, Qm)                       ( __neon_QdQnQm( 0xf2300050, (Qn), (Qm)) )
#define vqaddq_s8(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000050, (Qn), (Qm)) )
#define vqaddq_u16(Qn, Qm)                       ( __neon_QdQnQm( 0xf3100050, (Qn), (Qm)) )
#define vqaddq_u32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200050, (Qn), (Qm)) )
#define vqaddq_u64(Qn, Qm)                       ( __neon_QdQnQm( 0xf3300050, (Qn), (Qm)) )
#define vqaddq_u8(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000050, (Qn), (Qm)) )

// VQDMLAL, VQDMLSL
#define vqdmlal_s16(Qd, Dn, Dm)                  ( __neon_QdDnDm_acc( 0xf2900900, (Qd), (Dn), (Dm)) )
#define vqdmlal_s32(Qd, Dn, Dm)                  ( __neon_QdDnDm_acc( 0xf2a00900, (Qd), (Dn), (Dm)) )
#define vqdmlsl_s16(Qd, Dn, Dm)                  ( __neon_QdDnDm_acc( 0xf2900b00, (Qd), (Dn), (Dm)) )
#define vqdmlsl_s32(Qd, Dn, Dm)                  ( __neon_QdDnDm_acc( 0xf2a00b00, (Qd), (Dn), (Dm)) )

// VQDMLAL, VQDMLSL (by scalar)
#define vqdmlal_lane_s16(Qd, Dn, Dm, lane)       ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2900340 | _NENC_5x3(lane), (Qd), (Dn), (Dm)) )
#define vqdmlal_lane_s32(Qd, Dn, Dm, lane)       ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2a00340 | _NENC_5(lane), (Qd), (Dn), (Dm)) )
#define vqdmlsl_lane_s16(Qd, Dn, Dm, lane)       ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2900740 | _NENC_5x3(lane), (Qd), (Dn), (Dm)) )
#define vqdmlsl_lane_s32(Qd, Dn, Dm, lane)       ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx_acc( 0xf2a00740 | _NENC_5(lane), (Qd), (Dn), (Dm)) )

// VQDMULH (by scalar)
#define vqdmulh_lane_s16(Dn, Dm, lane)           ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx( 0xf2900c40 | _NENC_5x3(lane), (Dn), (Dm)) )
#define vqdmulh_lane_s32(Dn, Dm, lane)           ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx( 0xf2a00c40 | _NENC_5(lane), (Dn), (Dm)) )
#define vqrdmulh_lane_s16(Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_DdDnDmx( 0xf2900d40 | _NENC_5x3(lane), (Dn), (Dm)) )
#define vqrdmulh_lane_s32(Dn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_DdDnDmx( 0xf2a00d40 | _NENC_5(lane), (Dn), (Dm)) )
#define vqdmulhq_lane_s16(Qn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx( 0xf3900c40 | _NENC_5x3(lane), (Qn), (Dm)) )
#define vqdmulhq_lane_s32(Qn, Dm, lane)          ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx( 0xf3a00c40 | _NENC_5(lane), (Qn), (Dm)) )
#define vqrdmulhq_lane_s16(Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdQnDmx( 0xf3900d40 | _NENC_5x3(lane), (Qn), (Dm)) )
#define vqrdmulhq_lane_s32(Qn, Dm, lane)         ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdQnDmx( 0xf3a00d40 | _NENC_5(lane), (Qn), (Dm)) )

// VQDMULH, VQRDMULH
#define vqdmulh_s16(Dn, Dm)                      ( __neon_DdDnDm( 0xf2100b00, (Dn), (Dm)) )
#define vqdmulh_s32(Dn, Dm)                      ( __neon_DdDnDm( 0xf2200b00, (Dn), (Dm)) )
#define vqrdmulh_s16(Dn, Dm)                     ( __neon_DdDnDm( 0xf3100b00, (Dn), (Dm)) )
#define vqrdmulh_s32(Dn, Dm)                     ( __neon_DdDnDm( 0xf3200b00, (Dn), (Dm)) )
#define vqdmulhq_s16(Qn, Qm)                     ( __neon_QdQnQm( 0xf2100b40, (Qn), (Qm)) )
#define vqdmulhq_s32(Qn, Qm)                     ( __neon_QdQnQm( 0xf2200b40, (Qn), (Qm)) )
#define vqrdmulhq_s16(Qn, Qm)                    ( __neon_QdQnQm( 0xf3100b40, (Qn), (Qm)) )
#define vqrdmulhq_s32(Qn, Qm)                    ( __neon_QdQnQm( 0xf3200b40, (Qn), (Qm)) )

// VQDMULL
#define vqdmull_s16(Dn, Dm)                      ( __neon_QdDnDm( 0xf2900d00, (Dn), (Dm)) )
#define vqdmull_s32(Dn, Dm)                      ( __neon_QdDnDm( 0xf2a00d00, (Dn), (Dm)) )

// VQDMULL (by scalar)
#define vqdmull_lane_s16(Dn, Dm, lane)           ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_QdDnDmx( 0xf2900b40 | _NENC_5x3(lane), (Dn), (Dm)) )
#define vqdmull_lane_s32(Dn, Dm, lane)           ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_QdDnDmx( 0xf2a00b40 | _NENC_5(lane), (Dn), (Dm)) )

// VQMOVN, VQMOVUN
#define vqmovn_s16(Qm)                           ( __neon_DdQm( 0xf3b20280, (Qm)) )
#define vqmovn_s32(Qm)                           ( __neon_DdQm( 0xf3b60280, (Qm)) )
#define vqmovn_s64(Qm)                           ( __neon_DdQm( 0xf3ba0280, (Qm)) )
#define vqmovn_u16(Qm)                           ( __neon_DdQm( 0xf3b202c0, (Qm)) )
#define vqmovn_u32(Qm)                           ( __neon_DdQm( 0xf3b602c0, (Qm)) )
#define vqmovn_u64(Qm)                           ( __neon_DdQm( 0xf3ba02c0, (Qm)) )
#define vqmovun_s16(Qm)                          ( __neon_DdQm( 0xf3b20240, (Qm)) )
#define vqmovun_s32(Qm)                          ( __neon_DdQm( 0xf3b60240, (Qm)) )
#define vqmovun_s64(Qm)                          ( __neon_DdQm( 0xf3ba0240, (Qm)) )

// VQSHL, VQSHLU (immediate)
#define vqshl_n_s16(Dm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm( 0xf2900710 | _NENC_19_16(shift_amount), (Dm)) )
#define vqshl_n_s32(Dm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_DdDm( 0xf2a00710 | _NENC_20_16(shift_amount), (Dm)) )
#define vqshl_n_s64(Dm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_DdDm( 0xf2800790 | _NENC_21_16(shift_amount), (Dm)) )
#define vqshl_n_s8(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm( 0xf2880710 | _NENC_18_16(shift_amount), (Dm)) )
#define vqshl_n_u16(Dm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm( 0xf3900710 | _NENC_19_16(shift_amount), (Dm)) )
#define vqshl_n_u32(Dm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_DdDm( 0xf3a00710 | _NENC_20_16(shift_amount), (Dm)) )
#define vqshl_n_u64(Dm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_DdDm( 0xf3800790 | _NENC_21_16(shift_amount), (Dm)) )
#define vqshl_n_u8(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm( 0xf3880710 | _NENC_18_16(shift_amount), (Dm)) )
#define vqshlu_n_s16(Dm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm( 0xf3900610 | _NENC_19_16(shift_amount), (Dm)) )
#define vqshlu_n_s32(Dm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_DdDm( 0xf3a00610 | _NENC_20_16(shift_amount), (Dm)) )
#define vqshlu_n_s64(Dm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_DdDm( 0xf3800690 | _NENC_21_16(shift_amount), (Dm)) )
#define vqshlu_n_s8(Dm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm( 0xf3880610 | _NENC_18_16(shift_amount), (Dm)) )
#define vqshlq_n_s16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm( 0xf2900750 | _NENC_19_16(shift_amount), (Qm)) )
#define vqshlq_n_s32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_QdQm( 0xf2a00750 | _NENC_20_16(shift_amount), (Qm)) )
#define vqshlq_n_s64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_QdQm( 0xf28007d0 | _NENC_21_16(shift_amount), (Qm)) )
#define vqshlq_n_s8(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm( 0xf2880750 | _NENC_18_16(shift_amount), (Qm)) )
#define vqshlq_n_u16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm( 0xf3900750 | _NENC_19_16(shift_amount), (Qm)) )
#define vqshlq_n_u32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_QdQm( 0xf3a00750 | _NENC_20_16(shift_amount), (Qm)) )
#define vqshlq_n_u64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_QdQm( 0xf38007d0 | _NENC_21_16(shift_amount), (Qm)) )
#define vqshlq_n_u8(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm( 0xf3880750 | _NENC_18_16(shift_amount), (Qm)) )
#define vqshluq_n_s16(Qm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm( 0xf3900650 | _NENC_19_16(shift_amount), (Qm)) )
#define vqshluq_n_s32(Qm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_QdQm( 0xf3a00650 | _NENC_20_16(shift_amount), (Qm)) )
#define vqshluq_n_s64(Qm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_QdQm( 0xf38006d0 | _NENC_21_16(shift_amount), (Qm)) )
#define vqshluq_n_s8(Qm, shift_amount)           ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm( 0xf3880650 | _NENC_18_16(shift_amount), (Qm)) )

// VQSHRN, VQSHRUN, VQRSHRN, VQRSHRUN (immediate)
#define vqrshrn_n_s16(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf2880950 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vqrshrn_n_s32(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf2900950 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vqrshrn_n_s64(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf2a00950 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vqrshrn_n_u16(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf3880950 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vqrshrn_n_u32(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf3900950 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vqrshrn_n_u64(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf3a00950 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vqrshrun_n_s16(Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf3880850 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vqrshrun_n_s32(Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf3900850 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vqrshrun_n_s64(Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf3a00850 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vqshrn_n_s16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf2880910 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vqshrn_n_s32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf2900910 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vqshrn_n_s64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf2a00910 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vqshrn_n_u16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf3880910 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vqshrn_n_u32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf3900910 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vqshrn_n_u64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf3a00910 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vqshrun_n_s16(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf3880810 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vqshrun_n_s32(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf3900810 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vqshrun_n_s64(Qm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf3a00810 | _NENC_20_16(32 - (shift_amount)), (Qm)) )

// VQSUB
#define vqsub_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100210, (Dn), (Dm)) )
#define vqsub_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200210, (Dn), (Dm)) )
#define vqsub_s64(Dn, Dm)                        ( __neon_DdDnDm( 0xf2300210, (Dn), (Dm)) )
#define vqsub_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000210, (Dn), (Dm)) )
#define vqsub_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100210, (Dn), (Dm)) )
#define vqsub_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200210, (Dn), (Dm)) )
#define vqsub_u64(Dn, Dm)                        ( __neon_DdDnDm( 0xf3300210, (Dn), (Dm)) )
#define vqsub_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000210, (Dn), (Dm)) )
#define vqsubq_s16(Qn, Qm)                       ( __neon_QdQnQm( 0xf2100250, (Qn), (Qm)) )
#define vqsubq_s32(Qn, Qm)                       ( __neon_QdQnQm( 0xf2200250, (Qn), (Qm)) )
#define vqsubq_s64(Qn, Qm)                       ( __neon_QdQnQm( 0xf2300250, (Qn), (Qm)) )
#define vqsubq_s8(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000250, (Qn), (Qm)) )
#define vqsubq_u16(Qn, Qm)                       ( __neon_QdQnQm( 0xf3100250, (Qn), (Qm)) )
#define vqsubq_u32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200250, (Qn), (Qm)) )
#define vqsubq_u64(Qn, Qm)                       ( __neon_QdQnQm( 0xf3300250, (Qn), (Qm)) )
#define vqsubq_u8(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000250, (Qn), (Qm)) )

// VRECPE, VRSQRTE
#define vrecpe_f32(Dm)                           ( __neon_DdDm( 0xf3bb0500, (Dm)) )
#define vrecpe_u32(Dm)                           ( __neon_DdDm( 0xf3bb0400, (Dm)) )
#define vrsqrte_f32(Dm)                          ( __neon_DdDm( 0xf3bb0580, (Dm)) )
#define vrsqrte_u32(Dm)                          ( __neon_DdDm( 0xf3bb0480, (Dm)) )
#define vrecpeq_f32(Qm)                          ( __neon_QdQm( 0xf3bb0540, (Qm)) )
#define vrecpeq_u32(Qm)                          ( __neon_QdQm( 0xf3bb0440, (Qm)) )
#define vrsqrteq_f32(Qm)                         ( __neon_QdQm( 0xf3bb05c0, (Qm)) )
#define vrsqrteq_u32(Qm)                         ( __neon_QdQm( 0xf3bb04c0, (Qm)) )

// VRECPS
#define vrecps_f32(Dn, Dm)                       ( __neon_DdDnDm( 0xf2000f10, (Dn), (Dm)) )
#define vrecpsq_f32(Qn, Qm)                      ( __neon_QdQnQm( 0xf2000f50, (Qn), (Qm)) )

// VREV
#define vrev16_p8(Dm)                            ( __neon_DdDm( 0xf3b00100, (Dm)) )
#define vrev16_s8(Dm)                            ( __neon_DdDm( 0xf3b00100, (Dm)) )
#define vrev16_u8(Dm)                            ( __neon_DdDm( 0xf3b00100, (Dm)) )
#define vrev32_p16(Dm)                           ( __neon_DdDm( 0xf3b40080, (Dm)) )
#define vrev32_p8(Dm)                            ( __neon_DdDm( 0xf3b00080, (Dm)) )
#define vrev32_s16(Dm)                           ( __neon_DdDm( 0xf3b40080, (Dm)) )
#define vrev32_s8(Dm)                            ( __neon_DdDm( 0xf3b00080, (Dm)) )
#define vrev32_u16(Dm)                           ( __neon_DdDm( 0xf3b40080, (Dm)) )
#define vrev32_u8(Dm)                            ( __neon_DdDm( 0xf3b00080, (Dm)) )
#define vrev64_f32(Dm)                           ( __neon_DdDm( 0xf3b80000, (Dm)) )
#define vrev64_p16(Dm)                           ( __neon_DdDm( 0xf3b40000, (Dm)) )
#define vrev64_p8(Dm)                            ( __neon_DdDm( 0xf3b00000, (Dm)) )
#define vrev64_s16(Dm)                           ( __neon_DdDm( 0xf3b40000, (Dm)) )
#define vrev64_s32(Dm)                           ( __neon_DdDm( 0xf3b80000, (Dm)) )
#define vrev64_s8(Dm)                            ( __neon_DdDm( 0xf3b00000, (Dm)) )
#define vrev64_u16(Dm)                           ( __neon_DdDm( 0xf3b40000, (Dm)) )
#define vrev64_u32(Dm)                           ( __neon_DdDm( 0xf3b80000, (Dm)) )
#define vrev64_u8(Dm)                            ( __neon_DdDm( 0xf3b00000, (Dm)) )
#define vrev16q_p8(Qm)                           ( __neon_QdQm( 0xf3b00140, (Qm)) )
#define vrev16q_s8(Qm)                           ( __neon_QdQm( 0xf3b00140, (Qm)) )
#define vrev16q_u8(Qm)                           ( __neon_QdQm( 0xf3b00140, (Qm)) )
#define vrev32q_p16(Qm)                          ( __neon_QdQm( 0xf3b400c0, (Qm)) )
#define vrev32q_p8(Qm)                           ( __neon_QdQm( 0xf3b000c0, (Qm)) )
#define vrev32q_s16(Qm)                          ( __neon_QdQm( 0xf3b400c0, (Qm)) )
#define vrev32q_s8(Qm)                           ( __neon_QdQm( 0xf3b000c0, (Qm)) )
#define vrev32q_u16(Qm)                          ( __neon_QdQm( 0xf3b400c0, (Qm)) )
#define vrev32q_u8(Qm)                           ( __neon_QdQm( 0xf3b000c0, (Qm)) )
#define vrev64q_f32(Qm)                          ( __neon_QdQm( 0xf3b80040, (Qm)) )
#define vrev64q_p16(Qm)                          ( __neon_QdQm( 0xf3b40040, (Qm)) )
#define vrev64q_p8(Qm)                           ( __neon_QdQm( 0xf3b00040, (Qm)) )
#define vrev64q_s16(Qm)                          ( __neon_QdQm( 0xf3b40040, (Qm)) )
#define vrev64q_s32(Qm)                          ( __neon_QdQm( 0xf3b80040, (Qm)) )
#define vrev64q_s8(Qm)                           ( __neon_QdQm( 0xf3b00040, (Qm)) )
#define vrev64q_u16(Qm)                          ( __neon_QdQm( 0xf3b40040, (Qm)) )
#define vrev64q_u32(Qm)                          ( __neon_QdQm( 0xf3b80040, (Qm)) )
#define vrev64q_u8(Qm)                           ( __neon_QdQm( 0xf3b00040, (Qm)) )

// VRINT
#define vrnd_f32(Dm)                             ( __neon_DdDm( 0xf3ba0580, (Dm)) )
#define vrnda_f32(Dm)                            ( __neon_DdDm( 0xf3ba0500, (Dm)) )
#define vrndm_f32(Dm)                            ( __neon_DdDm( 0xf3ba0680, (Dm)) )
#define vrndn_f32(Dm)                            ( __neon_DdDm( 0xf3ba0400, (Dm)) )
#define vrndp_f32(Dm)                            ( __neon_DdDm( 0xf3ba0780, (Dm)) )
#define vrndx_f32(Dm)                            ( __neon_DdDm( 0xf3ba0480, (Dm)) )
#define vrndq_f32(Qm)                            ( __neon_QdQm( 0xf3ba05c0, (Qm)) )
#define vrndaq_f32(Qm)                           ( __neon_QdQm( 0xf3ba0540, (Qm)) )
#define vrndmq_f32(Qm)                           ( __neon_QdQm( 0xf3ba06c0, (Qm)) )
#define vrndnq_f32(Qm)                           ( __neon_QdQm( 0xf3ba0440, (Qm)) )
#define vrndpq_f32(Qm)                           ( __neon_QdQm( 0xf3ba07c0, (Qm)) )
#define vrndxq_f32(Qm)                           ( __neon_QdQm( 0xf3ba04c0, (Qm)) )

// VRSQRTS
#define vrsqrts_f32(Dn, Dm)                      ( __neon_DdDnDm( 0xf2200f10, (Dn), (Dm)) )
#define vrsqrtsq_f32(Qn, Qm)                     ( __neon_QdQnQm( 0xf2200f50, (Qn), (Qm)) )

// VSHL (immediate)
#define vshl_n_s16(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm( 0xf2900510 | _NENC_19_16(shift_amount), (Dm)) )
#define vshl_n_s32(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_DdDm( 0xf2a00510 | _NENC_20_16(shift_amount), (Dm)) )
#define vshl_n_s64(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_DdDm( 0xf2800590 | _NENC_21_16(shift_amount), (Dm)) )
#define vshl_n_s8(Dm, shift_amount)              ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm( 0xf2880510 | _NENC_18_16(shift_amount), (Dm)) )
#define vshl_n_u16(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm( 0xf2900510 | _NENC_19_16(shift_amount), (Dm)) )
#define vshl_n_u32(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_DdDm( 0xf2a00510 | _NENC_20_16(shift_amount), (Dm)) )
#define vshl_n_u64(Dm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_DdDm( 0xf2800590 | _NENC_21_16(shift_amount), (Dm)) )
#define vshl_n_u8(Dm, shift_amount)              ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm( 0xf2880510 | _NENC_18_16(shift_amount), (Dm)) )
#define vshlq_n_s16(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm( 0xf2900550 | _NENC_19_16(shift_amount), (Qm)) )
#define vshlq_n_s32(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_QdQm( 0xf2a00550 | _NENC_20_16(shift_amount), (Qm)) )
#define vshlq_n_s64(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_QdQm( 0xf28005d0 | _NENC_21_16(shift_amount), (Qm)) )
#define vshlq_n_s8(Qm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm( 0xf2880550 | _NENC_18_16(shift_amount), (Qm)) )
#define vshlq_n_u16(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm( 0xf2900550 | _NENC_19_16(shift_amount), (Qm)) )
#define vshlq_n_u32(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_QdQm( 0xf2a00550 | _NENC_20_16(shift_amount), (Qm)) )
#define vshlq_n_u64(Qm, shift_amount)            ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_QdQm( 0xf28005d0 | _NENC_21_16(shift_amount), (Qm)) )
#define vshlq_n_u8(Qm, shift_amount)             ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm( 0xf2880550 | _NENC_18_16(shift_amount), (Qm)) )

// VSHL, VQSHL, VRSHL, VQRSHL (register)
#define vqrshl_s16(Dn, Dm)                       ( __neon_DdDnDm( 0xf2100510, (Dm), (Dn)) )
#define vqrshl_s32(Dn, Dm)                       ( __neon_DdDnDm( 0xf2200510, (Dm), (Dn)) )
#define vqrshl_s64(Dn, Dm)                       ( __neon_DdDnDm( 0xf2300510, (Dm), (Dn)) )
#define vqrshl_s8(Dn, Dm)                        ( __neon_DdDnDm( 0xf2000510, (Dm), (Dn)) )
#define vqrshl_u16(Dn, Dm)                       ( __neon_DdDnDm( 0xf3100510, (Dm), (Dn)) )
#define vqrshl_u32(Dn, Dm)                       ( __neon_DdDnDm( 0xf3200510, (Dm), (Dn)) )
#define vqrshl_u64(Dn, Dm)                       ( __neon_DdDnDm( 0xf3300510, (Dm), (Dn)) )
#define vqrshl_u8(Dn, Dm)                        ( __neon_DdDnDm( 0xf3000510, (Dm), (Dn)) )
#define vqshl_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100410, (Dm), (Dn)) )
#define vqshl_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200410, (Dm), (Dn)) )
#define vqshl_s64(Dn, Dm)                        ( __neon_DdDnDm( 0xf2300410, (Dm), (Dn)) )
#define vqshl_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000410, (Dm), (Dn)) )
#define vqshl_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100410, (Dm), (Dn)) )
#define vqshl_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200410, (Dm), (Dn)) )
#define vqshl_u64(Dn, Dm)                        ( __neon_DdDnDm( 0xf3300410, (Dm), (Dn)) )
#define vqshl_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000410, (Dm), (Dn)) )
#define vrshl_s16(Dn, Dm)                        ( __neon_DdDnDm( 0xf2100500, (Dm), (Dn)) )
#define vrshl_s32(Dn, Dm)                        ( __neon_DdDnDm( 0xf2200500, (Dm), (Dn)) )
#define vrshl_s64(Dn, Dm)                        ( __neon_DdDnDm( 0xf2300500, (Dm), (Dn)) )
#define vrshl_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf2000500, (Dm), (Dn)) )
#define vrshl_u16(Dn, Dm)                        ( __neon_DdDnDm( 0xf3100500, (Dm), (Dn)) )
#define vrshl_u32(Dn, Dm)                        ( __neon_DdDnDm( 0xf3200500, (Dm), (Dn)) )
#define vrshl_u64(Dn, Dm)                        ( __neon_DdDnDm( 0xf3300500, (Dm), (Dn)) )
#define vrshl_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3000500, (Dm), (Dn)) )
#define vshl_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100400, (Dm), (Dn)) )
#define vshl_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200400, (Dm), (Dn)) )
#define vshl_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf2300400, (Dm), (Dn)) )
#define vshl_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000400, (Dm), (Dn)) )
#define vshl_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100400, (Dm), (Dn)) )
#define vshl_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200400, (Dm), (Dn)) )
#define vshl_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf3300400, (Dm), (Dn)) )
#define vshl_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000400, (Dm), (Dn)) )
#define vqrshlq_s16(Qn, Qm)                      ( __neon_QdQnQm( 0xf2100550, (Qm), (Qn)) )
#define vqrshlq_s32(Qn, Qm)                      ( __neon_QdQnQm( 0xf2200550, (Qm), (Qn)) )
#define vqrshlq_s64(Qn, Qm)                      ( __neon_QdQnQm( 0xf2300550, (Qm), (Qn)) )
#define vqrshlq_s8(Qn, Qm)                       ( __neon_QdQnQm( 0xf2000550, (Qm), (Qn)) )
#define vqrshlq_u16(Qn, Qm)                      ( __neon_QdQnQm( 0xf3100550, (Qm), (Qn)) )
#define vqrshlq_u32(Qn, Qm)                      ( __neon_QdQnQm( 0xf3200550, (Qm), (Qn)) )
#define vqrshlq_u64(Qn, Qm)                      ( __neon_QdQnQm( 0xf3300550, (Qm), (Qn)) )
#define vqrshlq_u8(Qn, Qm)                       ( __neon_QdQnQm( 0xf3000550, (Qm), (Qn)) )
#define vqshlq_s16(Qn, Qm)                       ( __neon_QdQnQm( 0xf2100450, (Qm), (Qn)) )
#define vqshlq_s32(Qn, Qm)                       ( __neon_QdQnQm( 0xf2200450, (Qm), (Qn)) )
#define vqshlq_s64(Qn, Qm)                       ( __neon_QdQnQm( 0xf2300450, (Qm), (Qn)) )
#define vqshlq_s8(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000450, (Qm), (Qn)) )
#define vqshlq_u16(Qn, Qm)                       ( __neon_QdQnQm( 0xf3100450, (Qm), (Qn)) )
#define vqshlq_u32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200450, (Qm), (Qn)) )
#define vqshlq_u64(Qn, Qm)                       ( __neon_QdQnQm( 0xf3300450, (Qm), (Qn)) )
#define vqshlq_u8(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000450, (Qm), (Qn)) )
#define vrshlq_s16(Qn, Qm)                       ( __neon_QdQnQm( 0xf2100540, (Qm), (Qn)) )
#define vrshlq_s32(Qn, Qm)                       ( __neon_QdQnQm( 0xf2200540, (Qm), (Qn)) )
#define vrshlq_s64(Qn, Qm)                       ( __neon_QdQnQm( 0xf2300540, (Qm), (Qn)) )
#define vrshlq_s8(Qn, Qm)                        ( __neon_QdQnQm( 0xf2000540, (Qm), (Qn)) )
#define vrshlq_u16(Qn, Qm)                       ( __neon_QdQnQm( 0xf3100540, (Qm), (Qn)) )
#define vrshlq_u32(Qn, Qm)                       ( __neon_QdQnQm( 0xf3200540, (Qm), (Qn)) )
#define vrshlq_u64(Qn, Qm)                       ( __neon_QdQnQm( 0xf3300540, (Qm), (Qn)) )
#define vrshlq_u8(Qn, Qm)                        ( __neon_QdQnQm( 0xf3000540, (Qm), (Qn)) )
#define vshlq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100440, (Qm), (Qn)) )
#define vshlq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200440, (Qm), (Qn)) )
#define vshlq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf2300440, (Qm), (Qn)) )
#define vshlq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000440, (Qm), (Qn)) )
#define vshlq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100440, (Qm), (Qn)) )
#define vshlq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200440, (Qm), (Qn)) )
#define vshlq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf3300440, (Qm), (Qn)) )
#define vshlq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000440, (Qm), (Qn)) )

// VSHLL (shift_amount != size)
#define __internal_vshll_n_t1_s16(Dm, shift_amount)         ( __neon_QdDm( 0xf2900a10 | _NENC_19_16(shift_amount), (Dm)) )
#define __internal_vshll_n_t1_s32(Dm, shift_amount)         ( __neon_QdDm( 0xf2a00a10 | _NENC_20_16(shift_amount), (Dm)) )
#define __internal_vshll_n_t1_s8(Dm, shift_amount)          ( __neon_QdDm( 0xf2880a10 | _NENC_18_16(shift_amount), (Dm)) )
#define __internal_vshll_n_t1_u16(Dm, shift_amount)         ( __neon_QdDm( 0xf3900a10 | _NENC_19_16(shift_amount), (Dm)) )
#define __internal_vshll_n_t1_u32(Dm, shift_amount)         ( __neon_QdDm( 0xf3a00a10 | _NENC_20_16(shift_amount), (Dm)) )
#define __internal_vshll_n_t1_u8(Dm, shift_amount)          ( __neon_QdDm( 0xf3880a10 | _NENC_18_16(shift_amount), (Dm)) )

// VSHLL (shift_amount == size)
#define __internal_vshll_n_t2_s16(Dm)                       ( __neon_QdDm( 0xf3b60300, (Dm)) )
#define __internal_vshll_n_t2_s32(Dm)                       ( __neon_QdDm( 0xf3ba0300, (Dm)) )
#define __internal_vshll_n_t2_s8(Dm)                        ( __neon_QdDm( 0xf3b20300, (Dm)) )
#define __internal_vshll_n_t2_u16(Dm)                       ( __neon_QdDm( 0xf3b60300, (Dm)) )
#define __internal_vshll_n_t2_u32(Dm)                       ( __neon_QdDm( 0xf3ba0300, (Dm)) )
#define __internal_vshll_n_t2_u8(Dm)                        ( __neon_QdDm( 0xf3b20300, (Dm)) )

// VSHR, VRSHR (immediate)
#define vrshr_n_s16(Dm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm( 0xf2900210 | _NENC_19_16(16 - (shift_amount)), (Dm)) )
#define vrshr_n_s32(Dm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm( 0xf2a00210 | _NENC_20_16(32 - (shift_amount)), (Dm)) )
#define vrshr_n_s64(Dm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm( 0xf2800290 | _NENC_21_16(64 - (shift_amount)), (Dm)) )
#define vrshr_n_s8(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm( 0xf2880210 | _NENC_18_16(8 - (shift_amount)), (Dm)) )
#define vrshr_n_u16(Dm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm( 0xf3900210 | _NENC_19_16(16 - (shift_amount)), (Dm)) )
#define vrshr_n_u32(Dm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm( 0xf3a00210 | _NENC_20_16(32 - (shift_amount)), (Dm)) )
#define vrshr_n_u64(Dm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm( 0xf3800290 | _NENC_21_16(64 - (shift_amount)), (Dm)) )
#define vrshr_n_u8(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm( 0xf3880210 | _NENC_18_16(8 - (shift_amount)), (Dm)) )
#define vshr_n_s16(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm( 0xf2900010 | _NENC_19_16(16 - (shift_amount)), (Dm)) )
#define vshr_n_s32(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm( 0xf2a00010 | _NENC_20_16(32 - (shift_amount)), (Dm)) )
#define vshr_n_s64(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm( 0xf2800090 | _NENC_21_16(64 - (shift_amount)), (Dm)) )
#define vshr_n_s8(Dm, shift_amount)              ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm( 0xf2880010 | _NENC_18_16(8 - (shift_amount)), (Dm)) )
#define vshr_n_u16(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm( 0xf3900010 | _NENC_19_16(16 - (shift_amount)), (Dm)) )
#define vshr_n_u32(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm( 0xf3a00010 | _NENC_20_16(32 - (shift_amount)), (Dm)) )
#define vshr_n_u64(Dm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm( 0xf3800090 | _NENC_21_16(64 - (shift_amount)), (Dm)) )
#define vshr_n_u8(Dm, shift_amount)              ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm( 0xf3880010 | _NENC_18_16(8 - (shift_amount)), (Dm)) )
#define vrshrq_n_s16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm( 0xf2900250 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vrshrq_n_s32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm( 0xf2a00250 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vrshrq_n_s64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm( 0xf28002d0 | _NENC_21_16(64 - (shift_amount)), (Qm)) )
#define vrshrq_n_s8(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm( 0xf2880250 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vrshrq_n_u16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm( 0xf3900250 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vrshrq_n_u32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm( 0xf3a00250 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vrshrq_n_u64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm( 0xf38002d0 | _NENC_21_16(64 - (shift_amount)), (Qm)) )
#define vrshrq_n_u8(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm( 0xf3880250 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vshrq_n_s16(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm( 0xf2900050 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vshrq_n_s32(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm( 0xf2a00050 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vshrq_n_s64(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm( 0xf28000d0 | _NENC_21_16(64 - (shift_amount)), (Qm)) )
#define vshrq_n_s8(Qm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm( 0xf2880050 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vshrq_n_u16(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm( 0xf3900050 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vshrq_n_u32(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm( 0xf3a00050 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vshrq_n_u64(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm( 0xf38000d0 | _NENC_21_16(64 - (shift_amount)), (Qm)) )
#define vshrq_n_u8(Qm, shift_amount)             ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm( 0xf3880050 | _NENC_18_16(8 - (shift_amount)), (Qm)) )

// VSHRN, VRSHRN (immediate)
#define vrshrn_n_s16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf2880850 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vrshrn_n_s32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf2900850 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vrshrn_n_s64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf2a00850 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vrshrn_n_u16(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf2880850 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vrshrn_n_u32(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf2900850 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vrshrn_n_u64(Qm, shift_amount)           ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf2a00850 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vshrn_n_s16(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf2880810 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vshrn_n_s32(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf2900810 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vshrn_n_s64(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf2a00810 | _NENC_20_16(32 - (shift_amount)), (Qm)) )
#define vshrn_n_u16(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdQm( 0xf2880810 | _NENC_18_16(8 - (shift_amount)), (Qm)) )
#define vshrn_n_u32(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdQm( 0xf2900810 | _NENC_19_16(16 - (shift_amount)), (Qm)) )
#define vshrn_n_u64(Qm, shift_amount)            ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdQm( 0xf2a00810 | _NENC_20_16(32 - (shift_amount)), (Qm)) )

// VSLI (immediate)
#define vsli_n_p16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900510 | _NENC_19_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_p8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880510 | _NENC_18_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_s16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900510 | _NENC_19_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_s32(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_DdDm_acc( 0xf3a00510 | _NENC_20_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_s64(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_DdDm_acc( 0xf3800590 | _NENC_21_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_s8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880510 | _NENC_18_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_u16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900510 | _NENC_19_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_u32(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_DdDm_acc( 0xf3a00510 | _NENC_20_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_u64(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_DdDm_acc( 0xf3800590 | _NENC_21_16(shift_amount), (Dd), (Dm)) )
#define vsli_n_u8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880510 | _NENC_18_16(shift_amount), (Dd), (Dm)) )
#define vsliq_n_p16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900550 | _NENC_19_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_p8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880550 | _NENC_18_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_s16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900550 | _NENC_19_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_s32(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_QdQm_acc( 0xf3a00550 | _NENC_20_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_s64(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_QdQm_acc( 0xf38005d0 | _NENC_21_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_s8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880550 | _NENC_18_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_u16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 0 && (shift_amount) < 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900550 | _NENC_19_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_u32(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 0 && (shift_amount) < 32, "invalid shift amount"), __neon_QdQm_acc( 0xf3a00550 | _NENC_20_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_u64(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 0 && (shift_amount) < 64, "invalid shift amount"), __neon_QdQm_acc( 0xf38005d0 | _NENC_21_16(shift_amount), (Qd), (Qm)) )
#define vsliq_n_u8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 0 && (shift_amount) < 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880550 | _NENC_18_16(shift_amount), (Qd), (Qm)) )

// VSRA, VRSRA (immediate)
#define vrsra_n_s16(Dd, Dm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm_acc( 0xf2900310 | _NENC_19_16(16 - (shift_amount)), (Dd), (Dm)) )
#define vrsra_n_s32(Dd, Dm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm_acc( 0xf2a00310 | _NENC_20_16(32 - (shift_amount)), (Dd), (Dm)) )
#define vrsra_n_s64(Dd, Dm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm_acc( 0xf2800390 | _NENC_21_16(64 - (shift_amount)), (Dd), (Dm)) )
#define vrsra_n_s8(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm_acc( 0xf2880310 | _NENC_18_16(8 - (shift_amount)), (Dd), (Dm)) )
#define vrsra_n_u16(Dd, Dm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900310 | _NENC_19_16(16 - (shift_amount)), (Dd), (Dm)) )
#define vrsra_n_u32(Dd, Dm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm_acc( 0xf3a00310 | _NENC_20_16(32 - (shift_amount)), (Dd), (Dm)) )
#define vrsra_n_u64(Dd, Dm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm_acc( 0xf3800390 | _NENC_21_16(64 - (shift_amount)), (Dd), (Dm)) )
#define vrsra_n_u8(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880310 | _NENC_18_16(8 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_s16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm_acc( 0xf2900110 | _NENC_19_16(16 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_s32(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm_acc( 0xf2a00110 | _NENC_20_16(32 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_s64(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm_acc( 0xf2800190 | _NENC_21_16(64 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_s8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm_acc( 0xf2880110 | _NENC_18_16(8 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_u16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900110 | _NENC_19_16(16 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_u32(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm_acc( 0xf3a00110 | _NENC_20_16(32 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_u64(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm_acc( 0xf3800190 | _NENC_21_16(64 - (shift_amount)), (Dd), (Dm)) )
#define vsra_n_u8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880110 | _NENC_18_16(8 - (shift_amount)), (Dd), (Dm)) )
#define vrsraq_n_s16(Qd, Qm, shift_amount)       ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm_acc( 0xf2900350 | _NENC_19_16(16 - (shift_amount)), (Qd), (Qm)) )
#define vrsraq_n_s32(Qd, Qm, shift_amount)       ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm_acc( 0xf2a00350 | _NENC_20_16(32 - (shift_amount)), (Qd), (Qm)) )
#define vrsraq_n_s64(Qd, Qm, shift_amount)       ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm_acc( 0xf28003d0 | _NENC_21_16(64 - (shift_amount)), (Qd), (Qm)) )
#define vrsraq_n_s8(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm_acc( 0xf2880350 | _NENC_18_16(8 - (shift_amount)), (Qd), (Qm)) )
#define vrsraq_n_u16(Qd, Qm, shift_amount)       ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900350 | _NENC_19_16(16 - (shift_amount)), (Qd), (Qm)) )
#define vrsraq_n_u32(Qd, Qm, shift_amount)       ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm_acc( 0xf3a00350 | _NENC_20_16(32 - (shift_amount)), (Qd), (Qm)) )
#define vrsraq_n_u64(Qd, Qm, shift_amount)       ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm_acc( 0xf38003d0 | _NENC_21_16(64 - (shift_amount)), (Qd), (Qm)) )
#define vrsraq_n_u8(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880350 | _NENC_18_16(8 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_s16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm_acc( 0xf2900150 | _NENC_19_16(16 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_s32(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm_acc( 0xf2a00150 | _NENC_20_16(32 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_s64(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm_acc( 0xf28001d0 | _NENC_21_16(64 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_s8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm_acc( 0xf2880150 | _NENC_18_16(8 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_u16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900150 | _NENC_19_16(16 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_u32(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm_acc( 0xf3a00150 | _NENC_20_16(32 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_u64(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm_acc( 0xf38001d0 | _NENC_21_16(64 - (shift_amount)), (Qd), (Qm)) )
#define vsraq_n_u8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880150 | _NENC_18_16(8 - (shift_amount)), (Qd), (Qm)) )

// VSRI (immediate)
#define vsri_n_p16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900410 | _NENC_19_16(16 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_p8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880410 | _NENC_18_16(8 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_s16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900410 | _NENC_19_16(16 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_s32(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm_acc( 0xf3a00410 | _NENC_20_16(32 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_s64(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm_acc( 0xf3800490 | _NENC_21_16(64 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_s8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880410 | _NENC_18_16(8 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_u16(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_DdDm_acc( 0xf3900410 | _NENC_19_16(16 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_u32(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_DdDm_acc( 0xf3a00410 | _NENC_20_16(32 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_u64(Dd, Dm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_DdDm_acc( 0xf3800490 | _NENC_21_16(64 - (shift_amount)), (Dd), (Dm)) )
#define vsri_n_u8(Dd, Dm, shift_amount)          ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_DdDm_acc( 0xf3880410 | _NENC_18_16(8 - (shift_amount)), (Dd), (Dm)) )
#define vsriq_n_p16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900450 | _NENC_19_16(16 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_p8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880450 | _NENC_18_16(8 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_s16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900450 | _NENC_19_16(16 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_s32(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm_acc( 0xf3a00450 | _NENC_20_16(32 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_s64(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm_acc( 0xf38004d0 | _NENC_21_16(64 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_s8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880450 | _NENC_18_16(8 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_u16(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 16, "invalid shift amount"), __neon_QdQm_acc( 0xf3900450 | _NENC_19_16(16 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_u32(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 32, "invalid shift amount"), __neon_QdQm_acc( 0xf3a00450 | _NENC_20_16(32 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_u64(Qd, Qm, shift_amount)        ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 64, "invalid shift amount"), __neon_QdQm_acc( 0xf38004d0 | _NENC_21_16(64 - (shift_amount)), (Qd), (Qm)) )
#define vsriq_n_u8(Qd, Qm, shift_amount)         ( __static_assert((shift_amount) >= 1 && (shift_amount) <= 8, "invalid shift amount"), __neon_QdQm_acc( 0xf3880450 | _NENC_18_16(8 - (shift_amount)), (Qd), (Qm)) )

// VST1 (multiple single elements)
#define vst1_f32(pD, D)                          ( __neon_AdrD1( 0xf400078f, __float32ToN64(pD), (D)) )
#define vst1_p16(pD, D)                          ( __neon_AdrD1( 0xf400074f, __poly16ToN64(pD), (D)) )
#define vst1_p8(pD, D)                           ( __neon_AdrD1( 0xf400070f, __poly8ToN64(pD), (D)) )
#define vst1_s16(pD, D)                          ( __neon_AdrD1( 0xf400074f, __int16ToN64(pD), (D)) )
#define vst1_s32(pD, D)                          ( __neon_AdrD1( 0xf400078f, __int32ToN64(pD), (D)) )
#define vst1_s64(pD, D)                          ( __neon_AdrD1( 0xf40007cf, __int64ToN64(pD), (D)) )
#define vst1_s8(pD, D)                           ( __neon_AdrD1( 0xf400070f, __int8ToN64(pD), (D)) )
#define vst1_u16(pD, D)                          ( __neon_AdrD1( 0xf400074f, __uint16ToN64(pD), (D)) )
#define vst1_u32(pD, D)                          ( __neon_AdrD1( 0xf400078f, __uint32ToN64(pD), (D)) )
#define vst1_u64(pD, D)                          ( __neon_AdrD1( 0xf40007cf, __uint64ToN64(pD), (D)) )
#define vst1_u8(pD, D)                           ( __neon_AdrD1( 0xf400070f, __uint8ToN64(pD), (D)) )
#define vst1_f32_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400078f | _NENC_5_4(_NEON_ALIGN64(align)), __float32ToN64(pD), (D)) )
#define vst1_p16_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400074f | _NENC_5_4(_NEON_ALIGN64(align)), __poly16ToN64(pD), (D)) )
#define vst1_p8_ex(pD, D, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400070f | _NENC_5_4(_NEON_ALIGN64(align)), __poly8ToN64(pD), (D)) )
#define vst1_s16_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400074f | _NENC_5_4(_NEON_ALIGN64(align)), __int16ToN64(pD), (D)) )
#define vst1_s32_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400078f | _NENC_5_4(_NEON_ALIGN64(align)), __int32ToN64(pD), (D)) )
#define vst1_s64_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf40007cf | _NENC_5_4(_NEON_ALIGN64(align)), __int64ToN64(pD), (D)) )
#define vst1_s8_ex(pD, D, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400070f | _NENC_5_4(_NEON_ALIGN64(align)), __int8ToN64(pD), (D)) )
#define vst1_u16_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400074f | _NENC_5_4(_NEON_ALIGN64(align)), __uint16ToN64(pD), (D)) )
#define vst1_u32_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400078f | _NENC_5_4(_NEON_ALIGN64(align)), __uint32ToN64(pD), (D)) )
#define vst1_u64_ex(pD, D, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf40007cf | _NENC_5_4(_NEON_ALIGN64(align)), __uint64ToN64(pD), (D)) )
#define vst1_u8_ex(pD, D, align)                 ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrD1( 0xf400070f | _NENC_5_4(_NEON_ALIGN64(align)), __uint8ToN64(pD), (D)) )
#define vst1q_f32(pD, Q)                         ( __neon_AdrQ1( 0xf4000a8f, __float32ToN64(pD), (Q)) )
#define vst1q_p16(pD, Q)                         ( __neon_AdrQ1( 0xf4000a4f, __poly16ToN64(pD), (Q)) )
#define vst1q_p8(pD, Q)                          ( __neon_AdrQ1( 0xf4000a0f, __poly8ToN64(pD), (Q)) )
#define vst1q_s16(pD, Q)                         ( __neon_AdrQ1( 0xf4000a4f, __int16ToN64(pD), (Q)) )
#define vst1q_s32(pD, Q)                         ( __neon_AdrQ1( 0xf4000a8f, __int32ToN64(pD), (Q)) )
#define vst1q_s64(pD, Q)                         ( __neon_AdrQ1( 0xf4000acf, __int64ToN64(pD), (Q)) )
#define vst1q_s8(pD, Q)                          ( __neon_AdrQ1( 0xf4000a0f, __int8ToN64(pD), (Q)) )
#define vst1q_u16(pD, Q)                         ( __neon_AdrQ1( 0xf4000a4f, __uint16ToN64(pD), (Q)) )
#define vst1q_u32(pD, Q)                         ( __neon_AdrQ1( 0xf4000a8f, __uint32ToN64(pD), (Q)) )
#define vst1q_u64(pD, Q)                         ( __neon_AdrQ1( 0xf4000acf, __uint64ToN64(pD), (Q)) )
#define vst1q_u8(pD, Q)                          ( __neon_AdrQ1( 0xf4000a0f, __uint8ToN64(pD), (Q)) )
#define vst1q_f32_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a8f | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64(pD), (Q)) )
#define vst1q_p16_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a4f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly16ToN64(pD), (Q)) )
#define vst1q_p8_ex(pD, Q, align)                ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a0f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly8ToN64(pD), (Q)) )
#define vst1q_s16_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a4f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int16ToN64(pD), (Q)) )
#define vst1q_s32_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a8f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64(pD), (Q)) )
#define vst1q_s64_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __int64ToN64(pD), (Q)) )
#define vst1q_s8_ex(pD, Q, align)                ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a0f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int8ToN64(pD), (Q)) )
#define vst1q_u16_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a4f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint16ToN64(pD), (Q)) )
#define vst1q_u32_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a8f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64(pD), (Q)) )
#define vst1q_u64_ex(pD, Q, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint64ToN64(pD), (Q)) )
#define vst1q_u8_ex(pD, Q, align)                ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf4000a0f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint8ToN64(pD), (Q)) )

// VST1 (single element from one lane)
#define vst1_lane_f32(pD, D, lane)               ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrD1( 0xf480080f | _NENC_7(lane), __float32ToN64(pD), (D)) )
#define vst1_lane_p16(pD, D, lane)               ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrD1( 0xf480040f | _NENC_7_6(lane), __poly16ToN64(pD), (D)) )
#define vst1_lane_p8(pD, D, lane)                ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrD1( 0xf480000f | _NENC_7_5(lane), __poly8ToN64(pD), (D)) )
#define vst1_lane_s16(pD, D, lane)               ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrD1( 0xf480040f | _NENC_7_6(lane), __int16ToN64(pD), (D)) )
#define vst1_lane_s32(pD, D, lane)               ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrD1( 0xf480080f | _NENC_7(lane), __int32ToN64(pD), (D)) )
#define vst1_lane_s8(pD, D, lane)                ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrD1( 0xf480000f | _NENC_7_5(lane), __int8ToN64(pD), (D)) )
#define vst1_lane_u16(pD, D, lane)               ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrD1( 0xf480040f | _NENC_7_6(lane), __uint16ToN64(pD), (D)) )
#define vst1_lane_u32(pD, D, lane)               ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrD1( 0xf480080f | _NENC_7(lane), __uint32ToN64(pD), (D)) )
#define vst1_lane_u8(pD, D, lane)                ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrD1( 0xf480000f | _NENC_7_5(lane), __uint8ToN64(pD), (D)) )
#define vst1q_lane_f32(pD, Q, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQ1( 0xf480080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __float32ToN64(pD), (Q)) )
#define vst1q_lane_p16(pD, Q, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQ1( 0xf480040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __poly16ToN64(pD), (Q)) )
#define vst1q_lane_p8(pD, Q, lane)               ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_AdrQ1( 0xf480000f | _NENC_7_5((lane) % 8) | _NENC_12((lane) >= 8 ? 1 : 0), __poly8ToN64(pD), (Q)) )
#define vst1q_lane_s16(pD, Q, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQ1( 0xf480040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __int16ToN64(pD), (Q)) )
#define vst1q_lane_s32(pD, Q, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQ1( 0xf480080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __int32ToN64(pD), (Q)) )
#define vst1q_lane_s8(pD, Q, lane)               ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_AdrQ1( 0xf480000f | _NENC_7_5((lane) % 8) | _NENC_12((lane) >= 8 ? 1 : 0), __int8ToN64(pD), (Q)) )
#define vst1q_lane_u16(pD, Q, lane)              ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQ1( 0xf480040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __uint16ToN64(pD), (Q)) )
#define vst1q_lane_u32(pD, Q, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQ1( 0xf480080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __uint32ToN64(pD), (Q)) )
#define vst1q_lane_u8(pD, Q, lane)               ( __static_assert((lane) >= 0 && (lane) < 16, "invalid lane index"), __neon_AdrQ1( 0xf480000f | _NENC_7_5((lane) % 8) | _NENC_12((lane) >= 8 ? 1 : 0), __uint8ToN64(pD), (Q)) )

// VST1 (single element from one lane, aligned)
#define vst1_lane_f32_ex(pD, D, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrD1( 0xf480080f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), __float32ToN64(pD), (D)) )
#define vst1_lane_p16_ex(pD, D, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrD1( 0xf480040f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN16(align)), __poly16ToN64(pD), (D)) )
#define vst1_lane_s16_ex(pD, D, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrD1( 0xf480040f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN16(align)), __int16ToN64(pD), (D)) )
#define vst1_lane_s32_ex(pD, D, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrD1( 0xf480080f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), __int32ToN64(pD), (D)) )
#define vst1_lane_u16_ex(pD, D, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrD1( 0xf480040f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN16(align)), __uint16ToN64(pD), (D)) )
#define vst1_lane_u32_ex(pD, D, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrD1( 0xf480080f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), __uint32ToN64(pD), (D)) )
#define vst1q_lane_f32_ex(pD, Q, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf480080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), __float32ToN64(pD), (Q)) )
#define vst1q_lane_p16_ex(pD, Q, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf480040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN16(align)), __poly16ToN64(pD), (Q)) )
#define vst1q_lane_s16_ex(pD, Q, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf480040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN16(align)), __int16ToN64(pD), (Q)) )
#define vst1q_lane_s32_ex(pD, Q, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf480080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), __int32ToN64(pD), (Q)) )
#define vst1q_lane_u16_ex(pD, Q, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf480040f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN16(align)), __uint16ToN64(pD), (Q)) )
#define vst1q_lane_u32_ex(pD, Q, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrQ1( 0xf480080f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN32(align) > 0 ? 3 : 0), __uint32ToN64(pD), (Q)) )

// VST2 (multiple 2-element structures)
#define vst2_f32(pD, D2)                         ( __neon_AdrDx2( 0xf400088f, __float32ToN64(pD), (D2)) )
#define vst2_p16(pD, D2)                         ( __neon_AdrDx2( 0xf400084f, __poly16ToN64(pD), (D2)) )
#define vst2_p8(pD, D2)                          ( __neon_AdrDx2( 0xf400080f, __poly8ToN64(pD), (D2)) )
#define vst2_s16(pD, D2)                         ( __neon_AdrDx2( 0xf400084f, __int16ToN64(pD), (D2)) )
#define vst2_s32(pD, D2)                         ( __neon_AdrDx2( 0xf400088f, __int32ToN64(pD), (D2)) )
#define vst2_s8(pD, D2)                          ( __neon_AdrDx2( 0xf400080f, __int8ToN64(pD), (D2)) )
#define vst2_u16(pD, D2)                         ( __neon_AdrDx2( 0xf400084f, __uint16ToN64(pD), (D2)) )
#define vst2_u32(pD, D2)                         ( __neon_AdrDx2( 0xf400088f, __uint32ToN64(pD), (D2)) )
#define vst2_u8(pD, D2)                          ( __neon_AdrDx2( 0xf400080f, __uint8ToN64(pD), (D2)) )
#define vst2_s64(pD, D2)                         ( __neon_AdrDx2( 0xf4000acf, __int64ToN64(pD), (D2)) )
#define vst2_u64(pD, D2)                         ( __neon_AdrDx2( 0xf4000acf, __uint64ToN64(pD), (D2)) )
#define vst2_s64_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf4000acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __int64ToN64(pD), (D2)) )
#define vst2_u64_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf4000acf | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint64ToN64(pD), (D2)) )
#define vst2_f32_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400088f | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64(pD), (D2)) )
#define vst2_p16_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400084f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly16ToN64(pD), (D2)) )
#define vst2_p8_ex(pD, D2, align)                ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400080f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly8ToN64(pD), (D2)) )
#define vst2_s16_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400084f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int16ToN64(pD), (D2)) )
#define vst2_s32_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400088f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64(pD), (D2)) )
#define vst2_s8_ex(pD, D2, align)                ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400080f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int8ToN64(pD), (D2)) )
#define vst2_u16_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400084f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint16ToN64(pD), (D2)) )
#define vst2_u32_ex(pD, D2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400088f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64(pD), (D2)) )
#define vst2_u8_ex(pD, D2, align)                ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx2( 0xf400080f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint8ToN64(pD), (D2)) )
#define vst2q_f32(pD, Q2)                        ( __neon_AdrQx2( 0xf400098f, __float32ToN64(pD), (Q2)) )
#define vst2q_p16(pD, Q2)                        ( __neon_AdrQx2( 0xf400094f, __poly16ToN64(pD), (Q2)) )
#define vst2q_p8(pD, Q2)                         ( __neon_AdrQx2( 0xf400090f, __poly8ToN64(pD), (Q2)) )
#define vst2q_s16(pD, Q2)                        ( __neon_AdrQx2( 0xf400094f, __int16ToN64(pD), (Q2)) )
#define vst2q_s32(pD, Q2)                        ( __neon_AdrQx2( 0xf400098f, __int32ToN64(pD), (Q2)) )
#define vst2q_s8(pD, Q2)                         ( __neon_AdrQx2( 0xf400090f, __int8ToN64(pD), (Q2)) )
#define vst2q_u16(pD, Q2)                        ( __neon_AdrQx2( 0xf400094f, __uint16ToN64(pD), (Q2)) )
#define vst2q_u32(pD, Q2)                        ( __neon_AdrQx2( 0xf400098f, __uint32ToN64(pD), (Q2)) )
#define vst2q_u8(pD, Q2)                         ( __neon_AdrQx2( 0xf400090f, __uint8ToN64(pD), (Q2)) )
#define vst2q_f32_ex(pD, Q2, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400098f | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64(pD), (Q2)) )
#define vst2q_p16_ex(pD, Q2, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400094f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly16ToN64(pD), (Q2)) )
#define vst2q_p8_ex(pD, Q2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400090f | _NENC_5_4(_NEON_ALIGN64_128(align)), __poly8ToN64(pD), (Q2)) )
#define vst2q_s16_ex(pD, Q2, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400094f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int16ToN64(pD), (Q2)) )
#define vst2q_s32_ex(pD, Q2, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400098f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64(pD), (Q2)) )
#define vst2q_s8_ex(pD, Q2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400090f | _NENC_5_4(_NEON_ALIGN64_128(align)), __int8ToN64(pD), (Q2)) )
#define vst2q_u16_ex(pD, Q2, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400094f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint16ToN64(pD), (Q2)) )
#define vst2q_u32_ex(pD, Q2, align)              ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400098f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64(pD), (Q2)) )
#define vst2q_u8_ex(pD, Q2, align)               ( __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx2( 0xf400090f | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint8ToN64(pD), (Q2)) )

// VST2 (single 2-element structure from one lane)
#define vst2_lane_f32(pD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx2x( 0xf480090f | _NENC_7(lane), __float32ToN64(pD), (D2)) )
#define vst2_lane_p16(pD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx2x( 0xf480050f | _NENC_7_6(lane), __poly16ToN64(pD), (D2)) )
#define vst2_lane_p8(pD, D2, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx2x( 0xf480010f | _NENC_7_5(lane), __poly8ToN64(pD), (D2)) )
#define vst2_lane_s16(pD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx2x( 0xf480050f | _NENC_7_6(lane), __int16ToN64(pD), (D2)) )
#define vst2_lane_s32(pD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx2x( 0xf480090f | _NENC_7(lane), __int32ToN64(pD), (D2)) )
#define vst2_lane_s8(pD, D2, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx2x( 0xf480010f | _NENC_7_5(lane), __int8ToN64(pD), (D2)) )
#define vst2_lane_u16(pD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx2x( 0xf480050f | _NENC_7_6(lane), __uint16ToN64(pD), (D2)) )
#define vst2_lane_u32(pD, D2, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx2x( 0xf480090f | _NENC_7(lane), __uint32ToN64(pD), (D2)) )
#define vst2_lane_u8(pD, D2, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx2x( 0xf480010f | _NENC_7_5(lane), __uint8ToN64(pD), (D2)) )
#define vst2q_lane_f32(pD, Q2, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx2x( 0xf480094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __float32ToN64(pD), (Q2)) )
#define vst2q_lane_p16(pD, Q2, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx2x( 0xf480052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __poly16ToN64(pD), (Q2)) )
#define vst2q_lane_s16(pD, Q2, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx2x( 0xf480052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __int16ToN64(pD), (Q2)) )
#define vst2q_lane_s32(pD, Q2, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx2x( 0xf480094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __int32ToN64(pD), (Q2)) )
#define vst2q_lane_u16(pD, Q2, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx2x( 0xf480052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __uint16ToN64(pD), (Q2)) )
#define vst2q_lane_u32(pD, Q2, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx2x( 0xf480094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __uint32ToN64(pD), (Q2)) )

// VST2 (single 2-element structure from one lane, aligned)
#define vst2_lane_f32_ex(pD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480090f | _NENC_7(lane) | _NENC_4(_NEON_ALIGN64(align)), __float32ToN64(pD), (D2)) )
#define vst2_lane_p16_ex(pD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480050f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN32(align)), __poly16ToN64(pD), (D2)) )
#define vst2_lane_p8_ex(pD, D2, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480010f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN16(align)), __poly8ToN64(pD), (D2)) )
#define vst2_lane_s16_ex(pD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480050f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN32(align)), __int16ToN64(pD), (D2)) )
#define vst2_lane_s32_ex(pD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480090f | _NENC_7(lane) | _NENC_4(_NEON_ALIGN64(align)), __int32ToN64(pD), (D2)) )
#define vst2_lane_s8_ex(pD, D2, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480010f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN16(align)), __int8ToN64(pD), (D2)) )
#define vst2_lane_u16_ex(pD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480050f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN32(align)), __uint16ToN64(pD), (D2)) )
#define vst2_lane_u32_ex(pD, D2, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480090f | _NENC_7(lane) | _NENC_4(_NEON_ALIGN64(align)), __uint32ToN64(pD), (D2)) )
#define vst2_lane_u8_ex(pD, D2, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN16(align) >= 0, "invalid align"), __neon_AdrDx2x( 0xf480010f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN16(align)), __uint8ToN64(pD), (D2)) )
#define vst2q_lane_f32_ex(pD, Q2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx2x( 0xf480094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), __float32ToN64(pD), (Q2)) )
#define vst2q_lane_p16_ex(pD, Q2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrQx2x( 0xf480052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN32(align)), __poly16ToN64(pD), (Q2)) )
#define vst2q_lane_s16_ex(pD, Q2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrQx2x( 0xf480052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN32(align)), __int16ToN64(pD), (Q2)) )
#define vst2q_lane_s32_ex(pD, Q2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx2x( 0xf480094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), __int32ToN64(pD), (Q2)) )
#define vst2q_lane_u16_ex(pD, Q2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrQx2x( 0xf480052f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN32(align)), __uint16ToN64(pD), (Q2)) )
#define vst2q_lane_u32_ex(pD, Q2, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx2x( 0xf480094f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), __uint32ToN64(pD), (Q2)) )

// VST3 (multiple 3-element structures)
#define vst3_f32(pD, D3)                         ( __neon_AdrDx3( 0xf400048f, __float32ToN64(pD), (D3)) )
#define vst3_p16(pD, D3)                         ( __neon_AdrDx3( 0xf400044f, __poly16ToN64(pD), (D3)) )
#define vst3_p8(pD, D3)                          ( __neon_AdrDx3( 0xf400040f, __poly8ToN64(pD), (D3)) )
#define vst3_s16(pD, D3)                         ( __neon_AdrDx3( 0xf400044f, __int16ToN64(pD), (D3)) )
#define vst3_s32(pD, D3)                         ( __neon_AdrDx3( 0xf400048f, __int32ToN64(pD), (D3)) )
#define vst3_s8(pD, D3)                          ( __neon_AdrDx3( 0xf400040f, __int8ToN64(pD), (D3)) )
#define vst3_u16(pD, D3)                         ( __neon_AdrDx3( 0xf400044f, __uint16ToN64(pD), (D3)) )
#define vst3_u32(pD, D3)                         ( __neon_AdrDx3( 0xf400048f, __uint32ToN64(pD), (D3)) )
#define vst3_u8(pD, D3)                          ( __neon_AdrDx3( 0xf400040f, __uint8ToN64(pD), (D3)) )
#define vst3_s64(pD, D3)                         ( __neon_AdrDx3( 0xf40006cf, __int64ToN64(pD), (D3)) )
#define vst3_u64(pD, D3)                         ( __neon_AdrDx3( 0xf40006cf, __uint64ToN64(pD), (D3)) )
#define vst3_s64_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf40006cf | _NENC_4(_NEON_ALIGN64(align)), __int64ToN64(pD), (D3)) )
#define vst3_u64_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf40006cf | _NENC_4(_NEON_ALIGN64(align)), __uint64ToN64(pD), (D3)) )
#define vst3_f32_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400048f | _NENC_4(_NEON_ALIGN64(align)), __float32ToN64(pD), (D3)) )
#define vst3_p16_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400044f | _NENC_4(_NEON_ALIGN64(align)), __poly16ToN64(pD), (D3)) )
#define vst3_p8_ex(pD, D3, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400040f | _NENC_4(_NEON_ALIGN64(align)), __poly8ToN64(pD), (D3)) )
#define vst3_s16_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400044f | _NENC_4(_NEON_ALIGN64(align)), __int16ToN64(pD), (D3)) )
#define vst3_s32_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400048f | _NENC_4(_NEON_ALIGN64(align)), __int32ToN64(pD), (D3)) )
#define vst3_s8_ex(pD, D3, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400040f | _NENC_4(_NEON_ALIGN64(align)), __int8ToN64(pD), (D3)) )
#define vst3_u16_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400044f | _NENC_4(_NEON_ALIGN64(align)), __uint16ToN64(pD), (D3)) )
#define vst3_u32_ex(pD, D3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400048f | _NENC_4(_NEON_ALIGN64(align)), __uint32ToN64(pD), (D3)) )
#define vst3_u8_ex(pD, D3, align)                ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx3( 0xf400040f | _NENC_4(_NEON_ALIGN64(align)), __uint8ToN64(pD), (D3)) )
#define vst3q_f32(pD, Q3)                        ( __neon_AdrQx3( 0xf400058f, __float32ToN64(pD), (Q3)) )
#define vst3q_p16(pD, Q3)                        ( __neon_AdrQx3( 0xf400054f, __poly16ToN64(pD), (Q3)) )
#define vst3q_p8(pD, Q3)                         ( __neon_AdrQx3( 0xf400050f, __poly8ToN64(pD), (Q3)) )
#define vst3q_s16(pD, Q3)                        ( __neon_AdrQx3( 0xf400054f, __int16ToN64(pD), (Q3)) )
#define vst3q_s32(pD, Q3)                        ( __neon_AdrQx3( 0xf400058f, __int32ToN64(pD), (Q3)) )
#define vst3q_s8(pD, Q3)                         ( __neon_AdrQx3( 0xf400050f, __int8ToN64(pD), (Q3)) )
#define vst3q_u16(pD, Q3)                        ( __neon_AdrQx3( 0xf400054f, __uint16ToN64(pD), (Q3)) )
#define vst3q_u32(pD, Q3)                        ( __neon_AdrQx3( 0xf400058f, __uint32ToN64(pD), (Q3)) )
#define vst3q_u8(pD, Q3)                         ( __neon_AdrQx3( 0xf400050f, __uint8ToN64(pD), (Q3)) )
#define vst3q_f32_ex(pD, Q3, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400058f | _NENC_4(_NEON_ALIGN64(align)), __float32ToN64(pD), (Q3)) )
#define vst3q_p16_ex(pD, Q3, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400054f | _NENC_4(_NEON_ALIGN64(align)), __poly16ToN64(pD), (Q3)) )
#define vst3q_p8_ex(pD, Q3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400050f | _NENC_4(_NEON_ALIGN64(align)), __poly8ToN64(pD), (Q3)) )
#define vst3q_s16_ex(pD, Q3, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400054f | _NENC_4(_NEON_ALIGN64(align)), __int16ToN64(pD), (Q3)) )
#define vst3q_s32_ex(pD, Q3, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400058f | _NENC_4(_NEON_ALIGN64(align)), __int32ToN64(pD), (Q3)) )
#define vst3q_s8_ex(pD, Q3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400050f | _NENC_4(_NEON_ALIGN64(align)), __int8ToN64(pD), (Q3)) )
#define vst3q_u16_ex(pD, Q3, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400054f | _NENC_4(_NEON_ALIGN64(align)), __uint16ToN64(pD), (Q3)) )
#define vst3q_u32_ex(pD, Q3, align)              ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400058f | _NENC_4(_NEON_ALIGN64(align)), __uint32ToN64(pD), (Q3)) )
#define vst3q_u8_ex(pD, Q3, align)               ( __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx3( 0xf400050f | _NENC_4(_NEON_ALIGN64(align)), __uint8ToN64(pD), (Q3)) )

// VST3 (single 3-element structure from one lane)
#define vst3_lane_f32(pD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx3x( 0xf4800a0f | _NENC_7(lane), __float32ToN64(pD), (D3)) )
#define vst3_lane_p16(pD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx3x( 0xf480060f | _NENC_7_6(lane), __poly16ToN64(pD), (D3)) )
#define vst3_lane_p8(pD, D3, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx3x( 0xf480020f | _NENC_7_5(lane), __poly8ToN64(pD), (D3)) )
#define vst3_lane_s16(pD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx3x( 0xf480060f | _NENC_7_6(lane), __int16ToN64(pD), (D3)) )
#define vst3_lane_s32(pD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx3x( 0xf4800a0f | _NENC_7(lane), __int32ToN64(pD), (D3)) )
#define vst3_lane_s8(pD, D3, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx3x( 0xf480020f | _NENC_7_5(lane), __int8ToN64(pD), (D3)) )
#define vst3_lane_u16(pD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx3x( 0xf480060f | _NENC_7_6(lane), __uint16ToN64(pD), (D3)) )
#define vst3_lane_u32(pD, D3, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx3x( 0xf4800a0f | _NENC_7(lane), __uint32ToN64(pD), (D3)) )
#define vst3_lane_u8(pD, D3, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx3x( 0xf480020f | _NENC_7_5(lane), __uint8ToN64(pD), (D3)) )
#define vst3q_lane_f32(pD, Q3, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx3x( 0xf4800a4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __float32ToN64(pD), (Q3)) )
#define vst3q_lane_p16(pD, Q3, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx3x( 0xf480062f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __poly16ToN64(pD), (Q3)) )
#define vst3q_lane_s16(pD, Q3, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx3x( 0xf480062f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __int16ToN64(pD), (Q3)) )
#define vst3q_lane_s32(pD, Q3, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx3x( 0xf4800a4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __int32ToN64(pD), (Q3)) )
#define vst3q_lane_u16(pD, Q3, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx3x( 0xf480062f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __uint16ToN64(pD), (Q3)) )
#define vst3q_lane_u32(pD, Q3, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx3x( 0xf4800a4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __uint32ToN64(pD), (Q3)) )

// VST4 (multiple 4-element structures)
#define vst4_f32(pD, D4)                         ( __neon_AdrDx4( 0xf400008f, __float32ToN64(pD), (D4)) )
#define vst4_p16(pD, D4)                         ( __neon_AdrDx4( 0xf400004f, __poly16ToN64(pD), (D4)) )
#define vst4_p8(pD, D4)                          ( __neon_AdrDx4( 0xf400000f, __poly8ToN64(pD), (D4)) )
#define vst4_s16(pD, D4)                         ( __neon_AdrDx4( 0xf400004f, __int16ToN64(pD), (D4)) )
#define vst4_s32(pD, D4)                         ( __neon_AdrDx4( 0xf400008f, __int32ToN64(pD), (D4)) )
#define vst4_s8(pD, D4)                          ( __neon_AdrDx4( 0xf400000f, __int8ToN64(pD), (D4)) )
#define vst4_u16(pD, D4)                         ( __neon_AdrDx4( 0xf400004f, __uint16ToN64(pD), (D4)) )
#define vst4_u32(pD, D4)                         ( __neon_AdrDx4( 0xf400008f, __uint32ToN64(pD), (D4)) )
#define vst4_u8(pD, D4)                          ( __neon_AdrDx4( 0xf400000f, __uint8ToN64(pD), (D4)) )
#define vst4_s64(pD, D4)                         ( __neon_AdrDx4( 0xf40002cf, __int64ToN64(pD), (D4)) )
#define vst4_u64(pD, D4)                         ( __neon_AdrDx4( 0xf40002cf, __uint64ToN64(pD), (D4)) )
#define vst4_s64_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf40002cf | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int64ToN64(pD), (D4)) )
#define vst4_u64_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf40002cf | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint64ToN64(pD), (D4)) )
#define vst4_f32_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400008f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __float32ToN64(pD), (D4)) )
#define vst4_p16_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400004f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly16ToN64(pD), (D4)) )
#define vst4_p8_ex(pD, D4, align)                ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400000f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly8ToN64(pD), (D4)) )
#define vst4_s16_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400004f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int16ToN64(pD), (D4)) )
#define vst4_s32_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400008f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int32ToN64(pD), (D4)) )
#define vst4_s8_ex(pD, D4, align)                ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400000f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int8ToN64(pD), (D4)) )
#define vst4_u16_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400004f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint16ToN64(pD), (D4)) )
#define vst4_u32_ex(pD, D4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400008f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint32ToN64(pD), (D4)) )
#define vst4_u8_ex(pD, D4, align)                ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrDx4( 0xf400000f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint8ToN64(pD), (D4)) )
#define vst4q_f32(pD, Q4)                        ( __neon_AdrQx4( 0xf400018f, __float32ToN64(pD), (Q4)) )
#define vst4q_p16(pD, Q4)                        ( __neon_AdrQx4( 0xf400014f, __poly16ToN64(pD), (Q4)) )
#define vst4q_p8(pD, Q4)                         ( __neon_AdrQx4( 0xf400010f, __poly8ToN64(pD), (Q4)) )
#define vst4q_s16(pD, Q4)                        ( __neon_AdrQx4( 0xf400014f, __int16ToN64(pD), (Q4)) )
#define vst4q_s32(pD, Q4)                        ( __neon_AdrQx4( 0xf400018f, __int32ToN64(pD), (Q4)) )
#define vst4q_s8(pD, Q4)                         ( __neon_AdrQx4( 0xf400010f, __int8ToN64(pD), (Q4)) )
#define vst4q_u16(pD, Q4)                        ( __neon_AdrQx4( 0xf400014f, __uint16ToN64(pD), (Q4)) )
#define vst4q_u32(pD, Q4)                        ( __neon_AdrQx4( 0xf400018f, __uint32ToN64(pD), (Q4)) )
#define vst4q_u8(pD, Q4)                         ( __neon_AdrQx4( 0xf400010f, __uint8ToN64(pD), (Q4)) )
#define vst4q_f32_ex(pD, Q4, align)              ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400018f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __float32ToN64(pD), (Q4)) )
#define vst4q_p16_ex(pD, Q4, align)              ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400014f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly16ToN64(pD), (Q4)) )
#define vst4q_p8_ex(pD, Q4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400010f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __poly8ToN64(pD), (Q4)) )
#define vst4q_s16_ex(pD, Q4, align)              ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400014f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int16ToN64(pD), (Q4)) )
#define vst4q_s32_ex(pD, Q4, align)              ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400018f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int32ToN64(pD), (Q4)) )
#define vst4q_s8_ex(pD, Q4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400010f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __int8ToN64(pD), (Q4)) )
#define vst4q_u16_ex(pD, Q4, align)              ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400014f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint16ToN64(pD), (Q4)) )
#define vst4q_u32_ex(pD, Q4, align)              ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400018f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint32ToN64(pD), (Q4)) )
#define vst4q_u8_ex(pD, Q4, align)               ( __static_assert(_NEON_ALIGN64_128_256(align) >= 0, "invalid align"), __neon_AdrQx4( 0xf400010f | _NENC_5_4(_NEON_ALIGN64_128_256(align)), __uint8ToN64(pD), (Q4)) )

// VST4 (single 4-element structure from one lane)
#define vst4_lane_f32(pD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx4x( 0xf4800b0f | _NENC_7(lane), __float32ToN64(pD), (D4)) )
#define vst4_lane_p16(pD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx4x( 0xf480070f | _NENC_7_6(lane), __poly16ToN64(pD), (D4)) )
#define vst4_lane_p8(pD, D4, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx4x( 0xf480030f | _NENC_7_5(lane), __poly8ToN64(pD), (D4)) )
#define vst4_lane_s16(pD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx4x( 0xf480070f | _NENC_7_6(lane), __int16ToN64(pD), (D4)) )
#define vst4_lane_s32(pD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx4x( 0xf4800b0f | _NENC_7(lane), __int32ToN64(pD), (D4)) )
#define vst4_lane_s8(pD, D4, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx4x( 0xf480030f | _NENC_7_5(lane), __int8ToN64(pD), (D4)) )
#define vst4_lane_u16(pD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrDx4x( 0xf480070f | _NENC_7_6(lane), __uint16ToN64(pD), (D4)) )
#define vst4_lane_u32(pD, D4, lane)              ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __neon_AdrDx4x( 0xf4800b0f | _NENC_7(lane), __uint32ToN64(pD), (D4)) )
#define vst4_lane_u8(pD, D4, lane)               ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrDx4x( 0xf480030f | _NENC_7_5(lane), __uint8ToN64(pD), (D4)) )
#define vst4q_lane_f32(pD, Q4, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx4x( 0xf4800b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __float32ToN64(pD), (Q4)) )
#define vst4q_lane_p16(pD, Q4, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx4x( 0xf480072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __poly16ToN64(pD), (Q4)) )
#define vst4q_lane_s16(pD, Q4, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx4x( 0xf480072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __int16ToN64(pD), (Q4)) )
#define vst4q_lane_s32(pD, Q4, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx4x( 0xf4800b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __int32ToN64(pD), (Q4)) )
#define vst4q_lane_u16(pD, Q4, lane)             ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __neon_AdrQx4x( 0xf480072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0), __uint16ToN64(pD), (Q4)) )
#define vst4q_lane_u32(pD, Q4, lane)             ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __neon_AdrQx4x( 0xf4800b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0), __uint32ToN64(pD), (Q4)) )

// VST4 (single 4-element structure from one lane, aligned)
#define vst4_lane_f32_ex(pD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf4800b0f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64(pD), (D4)) )
#define vst4_lane_p16_ex(pD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf480070f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN64(align)), __poly16ToN64(pD), (D4)) )
#define vst4_lane_p8_ex(pD, D4, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf480030f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN32(align)), __poly8ToN64(pD), (D4)) )
#define vst4_lane_s16_ex(pD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf480070f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN64(align)), __int16ToN64(pD), (D4)) )
#define vst4_lane_s32_ex(pD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf4800b0f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64(pD), (D4)) )
#define vst4_lane_s8_ex(pD, D4, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf480030f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN32(align)), __int8ToN64(pD), (D4)) )
#define vst4_lane_u16_ex(pD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf480070f | _NENC_7_6(lane) | _NENC_4(_NEON_ALIGN64(align)), __uint16ToN64(pD), (D4)) )
#define vst4_lane_u32_ex(pD, D4, lane, align)    ( __static_assert((lane) >= 0 && (lane) < 2, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf4800b0f | _NENC_7(lane) | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64(pD), (D4)) )
#define vst4_lane_u8_ex(pD, D4, lane, align)     ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN32(align) >= 0, "invalid align"), __neon_AdrDx4x( 0xf480030f | _NENC_7_5(lane) | _NENC_4(_NEON_ALIGN32(align)), __uint8ToN64(pD), (D4)) )
#define vst4q_lane_f32_ex(pD, Q4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx4x( 0xf4800b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN64_128(align)), __float32ToN64(pD), (Q4)) )
#define vst4q_lane_p16_ex(pD, Q4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx4x( 0xf480072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), __poly16ToN64(pD), (Q4)) )
#define vst4q_lane_s16_ex(pD, Q4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx4x( 0xf480072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), __int16ToN64(pD), (Q4)) )
#define vst4q_lane_s32_ex(pD, Q4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx4x( 0xf4800b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN64_128(align)), __int32ToN64(pD), (Q4)) )
#define vst4q_lane_u16_ex(pD, Q4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 8, "invalid lane index"), __static_assert(_NEON_ALIGN64(align) >= 0, "invalid align"), __neon_AdrQx4x( 0xf480072f | _NENC_7_6((lane) % 4) | _NENC_12((lane) >= 4 ? 1 : 0) | _NENC_4(_NEON_ALIGN64(align)), __uint16ToN64(pD), (Q4)) )
#define vst4q_lane_u32_ex(pD, Q4, lane, align)   ( __static_assert((lane) >= 0 && (lane) < 4, "invalid lane index"), __static_assert(_NEON_ALIGN64_128(align) >= 0, "invalid align"), __neon_AdrQx4x( 0xf4800b4f | _NENC_7((lane) % 2) | _NENC_12((lane) >= 2 ? 1 : 0) | _NENC_5_4(_NEON_ALIGN64_128(align)), __uint32ToN64(pD), (Q4)) )

// VSUB
#define vsub_f32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200d00, (Dn), (Dm)) )
#define vsub_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100800, (Dn), (Dm)) )
#define vsub_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200800, (Dn), (Dm)) )
#define vsub_s64(Dn, Dm)                         ( __neon_DdDnDm( 0xf3300800, (Dn), (Dm)) )
#define vsub_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000800, (Dn), (Dm)) )
#define vsub_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf3100800, (Dn), (Dm)) )
#define vsub_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf3200800, (Dn), (Dm)) )
#define vsub_u64(Dn, Dm)                         ( __neon_DdDnDm( 0xf3300800, (Dn), (Dm)) )
#define vsub_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf3000800, (Dn), (Dm)) )
#define vsubq_f32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200d40, (Qn), (Qm)) )
#define vsubq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100840, (Qn), (Qm)) )
#define vsubq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200840, (Qn), (Qm)) )
#define vsubq_s64(Qn, Qm)                        ( __neon_QdQnQm( 0xf3300840, (Qn), (Qm)) )
#define vsubq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000840, (Qn), (Qm)) )
#define vsubq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf3100840, (Qn), (Qm)) )
#define vsubq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf3200840, (Qn), (Qm)) )
#define vsubq_u64(Qn, Qm)                        ( __neon_QdQnQm( 0xf3300840, (Qn), (Qm)) )
#define vsubq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf3000840, (Qn), (Qm)) )

// VSUBHN, VRSUBHN
#define vrsubhn_s16(Qn, Qm)                      ( __neon_DdQnQm( 0xf3800600, (Qn), (Qm)) )
#define vrsubhn_s32(Qn, Qm)                      ( __neon_DdQnQm( 0xf3900600, (Qn), (Qm)) )
#define vrsubhn_s64(Qn, Qm)                      ( __neon_DdQnQm( 0xf3a00600, (Qn), (Qm)) )
#define vrsubhn_u16(Qn, Qm)                      ( __neon_DdQnQm( 0xf3800600, (Qn), (Qm)) )
#define vrsubhn_u32(Qn, Qm)                      ( __neon_DdQnQm( 0xf3900600, (Qn), (Qm)) )
#define vrsubhn_u64(Qn, Qm)                      ( __neon_DdQnQm( 0xf3a00600, (Qn), (Qm)) )
#define vsubhn_s16(Qn, Qm)                       ( __neon_DdQnQm( 0xf2800600, (Qn), (Qm)) )
#define vsubhn_s32(Qn, Qm)                       ( __neon_DdQnQm( 0xf2900600, (Qn), (Qm)) )
#define vsubhn_s64(Qn, Qm)                       ( __neon_DdQnQm( 0xf2a00600, (Qn), (Qm)) )
#define vsubhn_u16(Qn, Qm)                       ( __neon_DdQnQm( 0xf2800600, (Qn), (Qm)) )
#define vsubhn_u32(Qn, Qm)                       ( __neon_DdQnQm( 0xf2900600, (Qn), (Qm)) )
#define vsubhn_u64(Qn, Qm)                       ( __neon_DdQnQm( 0xf2a00600, (Qn), (Qm)) )

// VSUBL, VSUBW
#define vsubl_s16(Dn, Dm)                        ( __neon_QdDnDm( 0xf2900200, (Dn), (Dm)) )
#define vsubl_s32(Dn, Dm)                        ( __neon_QdDnDm( 0xf2a00200, (Dn), (Dm)) )
#define vsubl_s8(Dn, Dm)                         ( __neon_QdDnDm( 0xf2800200, (Dn), (Dm)) )
#define vsubl_u16(Dn, Dm)                        ( __neon_QdDnDm( 0xf3900200, (Dn), (Dm)) )
#define vsubl_u32(Dn, Dm)                        ( __neon_QdDnDm( 0xf3a00200, (Dn), (Dm)) )
#define vsubl_u8(Dn, Dm)                         ( __neon_QdDnDm( 0xf3800200, (Dn), (Dm)) )
#define vsubw_s16(Qn, Dm)                        ( __neon_QdQnDm( 0xf2900300, (Qn), (Dm)) )
#define vsubw_s32(Qn, Dm)                        ( __neon_QdQnDm( 0xf2a00300, (Qn), (Dm)) )
#define vsubw_s8(Qn, Dm)                         ( __neon_QdQnDm( 0xf2800300, (Qn), (Dm)) )
#define vsubw_u16(Qn, Dm)                        ( __neon_QdQnDm( 0xf3900300, (Qn), (Dm)) )
#define vsubw_u32(Qn, Dm)                        ( __neon_QdQnDm( 0xf3a00300, (Qn), (Dm)) )
#define vsubw_u8(Qn, Dm)                         ( __neon_QdQnDm( 0xf3800300, (Qn), (Dm)) )

// VTBL, VTBX
#define vtbl2_p8(D2, Dm)                         ( __neon_DdDx2Dm( 0xf3b00900, (D2), (Dm)) )
#define vtbl2_s8(D2, Dm)                         ( __neon_DdDx2Dm( 0xf3b00900, (D2), (Dm)) )
#define vtbl2_u8(D2, Dm)                         ( __neon_DdDx2Dm( 0xf3b00900, (D2), (Dm)) )
#define vtbx2_p8(Dd, D2, Dm)                     ( __neon_DdDx2Dm_acc( 0xf3b00940, (Dd), (D2), (Dm)) )
#define vtbx2_s8(Dd, D2, Dm)                     ( __neon_DdDx2Dm_acc( 0xf3b00940, (Dd), (D2), (Dm)) )
#define vtbx2_u8(Dd, D2, Dm)                     ( __neon_DdDx2Dm_acc( 0xf3b00940, (Dd), (D2), (Dm)) )
#define vtbl3_p8(D3, Dm)                         ( __neon_DdDx3Dm( 0xf3b00a00, (D3), (Dm)) )
#define vtbl3_s8(D3, Dm)                         ( __neon_DdDx3Dm( 0xf3b00a00, (D3), (Dm)) )
#define vtbl3_u8(D3, Dm)                         ( __neon_DdDx3Dm( 0xf3b00a00, (D3), (Dm)) )
#define vtbx3_p8(Dd, D3, Dm)                     ( __neon_DdDx3Dm_acc( 0xf3b00a40, (Dd), (D3), (Dm)) )
#define vtbx3_s8(Dd, D3, Dm)                     ( __neon_DdDx3Dm_acc( 0xf3b00a40, (Dd), (D3), (Dm)) )
#define vtbx3_u8(Dd, D3, Dm)                     ( __neon_DdDx3Dm_acc( 0xf3b00a40, (Dd), (D3), (Dm)) )
#define vtbl4_p8(D4, Dm)                         ( __neon_DdDx4Dm( 0xf3b00b00, (D4), (Dm)) )
#define vtbl4_s8(D4, Dm)                         ( __neon_DdDx4Dm( 0xf3b00b00, (D4), (Dm)) )
#define vtbl4_u8(D4, Dm)                         ( __neon_DdDx4Dm( 0xf3b00b00, (D4), (Dm)) )
#define vtbx4_p8(Dd, D4, Dm)                     ( __neon_DdDx4Dm_acc( 0xf3b00b40, (Dd), (D4), (Dm)) )
#define vtbx4_s8(Dd, D4, Dm)                     ( __neon_DdDx4Dm_acc( 0xf3b00b40, (Dd), (D4), (Dm)) )
#define vtbx4_u8(Dd, D4, Dm)                     ( __neon_DdDx4Dm_acc( 0xf3b00b40, (Dd), (D4), (Dm)) )
#define vtbl1_p8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3b00800, (Dn), (Dm)) )
#define vtbl1_s8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3b00800, (Dn), (Dm)) )
#define vtbl1_u8(Dn, Dm)                         ( __neon_DdDnDm( 0xf3b00800, (Dn), (Dm)) )
#define vtbx1_p8(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3b00840, (Dd), (Dn), (Dm)) )
#define vtbx1_s8(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3b00840, (Dd), (Dn), (Dm)) )
#define vtbx1_u8(Dd, Dn, Dm)                     ( __neon_DdDnDm_acc( 0xf3b00840, (Dd), (Dn), (Dm)) )

// VTRN
#define vtrn_f32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vtrn_p16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60080, (Dd), (Dm)) )
#define vtrn_p8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20080, (Dd), (Dm)) )
#define vtrn_s16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60080, (Dd), (Dm)) )
#define vtrn_s32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vtrn_s8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20080, (Dd), (Dm)) )
#define vtrn_u16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60080, (Dd), (Dm)) )
#define vtrn_u32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vtrn_u8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20080, (Dd), (Dm)) )
#define vtrnq_f32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba00c0, (Qd), (Qm)) )
#define vtrnq_p16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b600c0, (Qd), (Qm)) )
#define vtrnq_p8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b200c0, (Qd), (Qm)) )
#define vtrnq_s16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b600c0, (Qd), (Qm)) )
#define vtrnq_s32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba00c0, (Qd), (Qm)) )
#define vtrnq_s8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b200c0, (Qd), (Qm)) )
#define vtrnq_u16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b600c0, (Qd), (Qm)) )
#define vtrnq_u32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba00c0, (Qd), (Qm)) )
#define vtrnq_u8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b200c0, (Qd), (Qm)) )

// VTRNQ64
#define vtrnq_s64(Qd, Qm)                        ( __neon_QdQm_acc3( 0x00000000, (Qd), (Qm)) )
#define vtrnq_u64(Qd, Qm)                        ( __neon_QdQm_acc3( 0x00000000, (Qd), (Qm)) )

// VTST
#define vtst_p8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000810, (Dn), (Dm)) )
#define vtst_s16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100810, (Dn), (Dm)) )
#define vtst_s32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200810, (Dn), (Dm)) )
#define vtst_s8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000810, (Dn), (Dm)) )
#define vtst_u16(Dn, Dm)                         ( __neon_DdDnDm( 0xf2100810, (Dn), (Dm)) )
#define vtst_u32(Dn, Dm)                         ( __neon_DdDnDm( 0xf2200810, (Dn), (Dm)) )
#define vtst_u8(Dn, Dm)                          ( __neon_DdDnDm( 0xf2000810, (Dn), (Dm)) )
#define vtstq_p8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000850, (Qn), (Qm)) )
#define vtstq_s16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100850, (Qn), (Qm)) )
#define vtstq_s32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200850, (Qn), (Qm)) )
#define vtstq_s8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000850, (Qn), (Qm)) )
#define vtstq_u16(Qn, Qm)                        ( __neon_QdQnQm( 0xf2100850, (Qn), (Qm)) )
#define vtstq_u32(Qn, Qm)                        ( __neon_QdQnQm( 0xf2200850, (Qn), (Qm)) )
#define vtstq_u8(Qn, Qm)                         ( __neon_QdQnQm( 0xf2000850, (Qn), (Qm)) )

// VUZP
#define vuzp_p16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60100, (Dd), (Dm)) )
#define vuzp_p8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20100, (Dd), (Dm)) )
#define vuzp_s16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60100, (Dd), (Dm)) )
#define vuzp_s8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20100, (Dd), (Dm)) )
#define vuzp_u16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60100, (Dd), (Dm)) )
#define vuzp_u8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20100, (Dd), (Dm)) )
#define vuzp_f32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vuzp_s32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vuzp_u32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vuzpq_f32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba0140, (Qd), (Qm)) )
#define vuzpq_p16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b60140, (Qd), (Qm)) )
#define vuzpq_p8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b20140, (Qd), (Qm)) )
#define vuzpq_s16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b60140, (Qd), (Qm)) )
#define vuzpq_s32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba0140, (Qd), (Qm)) )
#define vuzpq_s8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b20140, (Qd), (Qm)) )
#define vuzpq_u16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b60140, (Qd), (Qm)) )
#define vuzpq_u32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba0140, (Qd), (Qm)) )
#define vuzpq_u8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b20140, (Qd), (Qm)) )

// VZIP
#define vzip_p16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60180, (Dd), (Dm)) )
#define vzip_p8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20180, (Dd), (Dm)) )
#define vzip_s16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60180, (Dd), (Dm)) )
#define vzip_s8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20180, (Dd), (Dm)) )
#define vzip_u16(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3b60180, (Dd), (Dm)) )
#define vzip_u8(Dd, Dm)                          ( __neon_DdDm_acc2( 0xf3b20180, (Dd), (Dm)) )
#define vzip_f32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vzip_s32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vzip_u32(Dd, Dm)                         ( __neon_DdDm_acc2( 0xf3ba0080, (Dd), (Dm)) )
#define vzipq_f32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba01c0, (Qd), (Qm)) )
#define vzipq_p16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b601c0, (Qd), (Qm)) )
#define vzipq_p8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b201c0, (Qd), (Qm)) )
#define vzipq_s16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b601c0, (Qd), (Qm)) )
#define vzipq_s32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba01c0, (Qd), (Qm)) )
#define vzipq_s8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b201c0, (Qd), (Qm)) )
#define vzipq_u16(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3b601c0, (Qd), (Qm)) )
#define vzipq_u32(Qd, Qm)                        ( __neon_QdQm_acc2( 0xf3ba01c0, (Qd), (Qm)) )
#define vzipq_u8(Qd, Qm)                         ( __neon_QdQm_acc2( 0xf3b201c0, (Qd), (Qm)) )

// } +++ auto-generated code ends (Neon macros)



///////////////////////////////////////////////////////////////////////////////
//
// { +++ auto-generated code begins (vreinterpret macros)

#define vreinterpret_f32_s8(a)         (a)
#define vreinterpret_f32_s16(a)        (a)
#define vreinterpret_f32_s32(a)        (a)
#define vreinterpret_f32_s64(a)        (a)
#define vreinterpret_f32_p8(a)         (a)
#define vreinterpret_f32_p16(a)        (a)
#define vreinterpret_f32_u8(a)         (a)
#define vreinterpret_f32_u16(a)        (a)
#define vreinterpret_f32_u32(a)        (a)
#define vreinterpret_f32_u64(a)        (a)
#define vreinterpret_s8_f32(a)         (a)
#define vreinterpret_s8_s16(a)         (a)
#define vreinterpret_s8_s32(a)         (a)
#define vreinterpret_s8_s64(a)         (a)
#define vreinterpret_s8_p8(a)          (a)
#define vreinterpret_s8_p16(a)         (a)
#define vreinterpret_s8_u8(a)          (a)
#define vreinterpret_s8_u16(a)         (a)
#define vreinterpret_s8_u32(a)         (a)
#define vreinterpret_s8_u64(a)         (a)
#define vreinterpret_s16_f32(a)        (a)
#define vreinterpret_s16_s8(a)         (a)
#define vreinterpret_s16_s32(a)        (a)
#define vreinterpret_s16_s64(a)        (a)
#define vreinterpret_s16_p8(a)         (a)
#define vreinterpret_s16_p16(a)        (a)
#define vreinterpret_s16_u8(a)         (a)
#define vreinterpret_s16_u16(a)        (a)
#define vreinterpret_s16_u32(a)        (a)
#define vreinterpret_s16_u64(a)        (a)
#define vreinterpret_s32_f32(a)        (a)
#define vreinterpret_s32_s8(a)         (a)
#define vreinterpret_s32_s16(a)        (a)
#define vreinterpret_s32_s64(a)        (a)
#define vreinterpret_s32_p8(a)         (a)
#define vreinterpret_s32_p16(a)        (a)
#define vreinterpret_s32_u8(a)         (a)
#define vreinterpret_s32_u16(a)        (a)
#define vreinterpret_s32_u32(a)        (a)
#define vreinterpret_s32_u64(a)        (a)
#define vreinterpret_s64_f32(a)        (a)
#define vreinterpret_s64_s8(a)         (a)
#define vreinterpret_s64_s16(a)        (a)
#define vreinterpret_s64_s32(a)        (a)
#define vreinterpret_s64_p8(a)         (a)
#define vreinterpret_s64_p16(a)        (a)
#define vreinterpret_s64_u8(a)         (a)
#define vreinterpret_s64_u16(a)        (a)
#define vreinterpret_s64_u32(a)        (a)
#define vreinterpret_s64_u64(a)        (a)
#define vreinterpret_p8_f32(a)         (a)
#define vreinterpret_p8_s8(a)          (a)
#define vreinterpret_p8_s16(a)         (a)
#define vreinterpret_p8_s32(a)         (a)
#define vreinterpret_p8_s64(a)         (a)
#define vreinterpret_p8_p16(a)         (a)
#define vreinterpret_p8_u8(a)          (a)
#define vreinterpret_p8_u16(a)         (a)
#define vreinterpret_p8_u32(a)         (a)
#define vreinterpret_p8_u64(a)         (a)
#define vreinterpret_p16_f32(a)        (a)
#define vreinterpret_p16_s8(a)         (a)
#define vreinterpret_p16_s16(a)        (a)
#define vreinterpret_p16_s32(a)        (a)
#define vreinterpret_p16_s64(a)        (a)
#define vreinterpret_p16_p8(a)         (a)
#define vreinterpret_p16_u8(a)         (a)
#define vreinterpret_p16_u16(a)        (a)
#define vreinterpret_p16_u32(a)        (a)
#define vreinterpret_p16_u64(a)        (a)
#define vreinterpret_u8_f32(a)         (a)
#define vreinterpret_u8_s8(a)          (a)
#define vreinterpret_u8_s16(a)         (a)
#define vreinterpret_u8_s32(a)         (a)
#define vreinterpret_u8_s64(a)         (a)
#define vreinterpret_u8_p8(a)          (a)
#define vreinterpret_u8_p16(a)         (a)
#define vreinterpret_u8_u16(a)         (a)
#define vreinterpret_u8_u32(a)         (a)
#define vreinterpret_u8_u64(a)         (a)
#define vreinterpret_u16_f32(a)        (a)
#define vreinterpret_u16_s8(a)         (a)
#define vreinterpret_u16_s16(a)        (a)
#define vreinterpret_u16_s32(a)        (a)
#define vreinterpret_u16_s64(a)        (a)
#define vreinterpret_u16_p8(a)         (a)
#define vreinterpret_u16_p16(a)        (a)
#define vreinterpret_u16_u8(a)         (a)
#define vreinterpret_u16_u32(a)        (a)
#define vreinterpret_u16_u64(a)        (a)
#define vreinterpret_u32_f32(a)        (a)
#define vreinterpret_u32_s8(a)         (a)
#define vreinterpret_u32_s16(a)        (a)
#define vreinterpret_u32_s32(a)        (a)
#define vreinterpret_u32_s64(a)        (a)
#define vreinterpret_u32_p8(a)         (a)
#define vreinterpret_u32_p16(a)        (a)
#define vreinterpret_u32_u8(a)         (a)
#define vreinterpret_u32_u16(a)        (a)
#define vreinterpret_u32_u64(a)        (a)
#define vreinterpret_u64_f32(a)        (a)
#define vreinterpret_u64_s8(a)         (a)
#define vreinterpret_u64_s16(a)        (a)
#define vreinterpret_u64_s32(a)        (a)
#define vreinterpret_u64_s64(a)        (a)
#define vreinterpret_u64_p8(a)         (a)
#define vreinterpret_u64_p16(a)        (a)
#define vreinterpret_u64_u8(a)         (a)
#define vreinterpret_u64_u16(a)        (a)
#define vreinterpret_u64_u32(a)        (a)
#define vreinterpretq_f32_s8(a)        (a)
#define vreinterpretq_f32_s16(a)       (a)
#define vreinterpretq_f32_s32(a)       (a)
#define vreinterpretq_f32_s64(a)       (a)
#define vreinterpretq_f32_p8(a)        (a)
#define vreinterpretq_f32_p16(a)       (a)
#define vreinterpretq_f32_u8(a)        (a)
#define vreinterpretq_f32_u16(a)       (a)
#define vreinterpretq_f32_u32(a)       (a)
#define vreinterpretq_f32_u64(a)       (a)
#define vreinterpretq_s8_f32(a)        (a)
#define vreinterpretq_s8_s16(a)        (a)
#define vreinterpretq_s8_s32(a)        (a)
#define vreinterpretq_s8_s64(a)        (a)
#define vreinterpretq_s8_p8(a)         (a)
#define vreinterpretq_s8_p16(a)        (a)
#define vreinterpretq_s8_u8(a)         (a)
#define vreinterpretq_s8_u16(a)        (a)
#define vreinterpretq_s8_u32(a)        (a)
#define vreinterpretq_s8_u64(a)        (a)
#define vreinterpretq_s16_f32(a)       (a)
#define vreinterpretq_s16_s8(a)        (a)
#define vreinterpretq_s16_s32(a)       (a)
#define vreinterpretq_s16_s64(a)       (a)
#define vreinterpretq_s16_p8(a)        (a)
#define vreinterpretq_s16_p16(a)       (a)
#define vreinterpretq_s16_u8(a)        (a)
#define vreinterpretq_s16_u16(a)       (a)
#define vreinterpretq_s16_u32(a)       (a)
#define vreinterpretq_s16_u64(a)       (a)
#define vreinterpretq_s32_f32(a)       (a)
#define vreinterpretq_s32_s8(a)        (a)
#define vreinterpretq_s32_s16(a)       (a)
#define vreinterpretq_s32_s64(a)       (a)
#define vreinterpretq_s32_p8(a)        (a)
#define vreinterpretq_s32_p16(a)       (a)
#define vreinterpretq_s32_u8(a)        (a)
#define vreinterpretq_s32_u16(a)       (a)
#define vreinterpretq_s32_u32(a)       (a)
#define vreinterpretq_s32_u64(a)       (a)
#define vreinterpretq_s64_f32(a)       (a)
#define vreinterpretq_s64_s8(a)        (a)
#define vreinterpretq_s64_s16(a)       (a)
#define vreinterpretq_s64_s32(a)       (a)
#define vreinterpretq_s64_p8(a)        (a)
#define vreinterpretq_s64_p16(a)       (a)
#define vreinterpretq_s64_u8(a)        (a)
#define vreinterpretq_s64_u16(a)       (a)
#define vreinterpretq_s64_u32(a)       (a)
#define vreinterpretq_s64_u64(a)       (a)
#define vreinterpretq_p8_f32(a)        (a)
#define vreinterpretq_p8_s8(a)         (a)
#define vreinterpretq_p8_s16(a)        (a)
#define vreinterpretq_p8_s32(a)        (a)
#define vreinterpretq_p8_s64(a)        (a)
#define vreinterpretq_p8_p16(a)        (a)
#define vreinterpretq_p8_u8(a)         (a)
#define vreinterpretq_p8_u16(a)        (a)
#define vreinterpretq_p8_u32(a)        (a)
#define vreinterpretq_p8_u64(a)        (a)
#define vreinterpretq_p16_f32(a)       (a)
#define vreinterpretq_p16_s8(a)        (a)
#define vreinterpretq_p16_s16(a)       (a)
#define vreinterpretq_p16_s32(a)       (a)
#define vreinterpretq_p16_s64(a)       (a)
#define vreinterpretq_p16_p8(a)        (a)
#define vreinterpretq_p16_u8(a)        (a)
#define vreinterpretq_p16_u16(a)       (a)
#define vreinterpretq_p16_u32(a)       (a)
#define vreinterpretq_p16_u64(a)       (a)
#define vreinterpretq_u8_f32(a)        (a)
#define vreinterpretq_u8_s8(a)         (a)
#define vreinterpretq_u8_s16(a)        (a)
#define vreinterpretq_u8_s32(a)        (a)
#define vreinterpretq_u8_s64(a)        (a)
#define vreinterpretq_u8_p8(a)         (a)
#define vreinterpretq_u8_p16(a)        (a)
#define vreinterpretq_u8_u16(a)        (a)
#define vreinterpretq_u8_u32(a)        (a)
#define vreinterpretq_u8_u64(a)        (a)
#define vreinterpretq_u16_f32(a)       (a)
#define vreinterpretq_u16_s8(a)        (a)
#define vreinterpretq_u16_s16(a)       (a)
#define vreinterpretq_u16_s32(a)       (a)
#define vreinterpretq_u16_s64(a)       (a)
#define vreinterpretq_u16_p8(a)        (a)
#define vreinterpretq_u16_p16(a)       (a)
#define vreinterpretq_u16_u8(a)        (a)
#define vreinterpretq_u16_u32(a)       (a)
#define vreinterpretq_u16_u64(a)       (a)
#define vreinterpretq_u32_f32(a)       (a)
#define vreinterpretq_u32_s8(a)        (a)
#define vreinterpretq_u32_s16(a)       (a)
#define vreinterpretq_u32_s32(a)       (a)
#define vreinterpretq_u32_s64(a)       (a)
#define vreinterpretq_u32_p8(a)        (a)
#define vreinterpretq_u32_p16(a)       (a)
#define vreinterpretq_u32_u8(a)        (a)
#define vreinterpretq_u32_u16(a)       (a)
#define vreinterpretq_u32_u64(a)       (a)
#define vreinterpretq_u64_f32(a)       (a)
#define vreinterpretq_u64_s8(a)        (a)
#define vreinterpretq_u64_s16(a)       (a)
#define vreinterpretq_u64_s32(a)       (a)
#define vreinterpretq_u64_s64(a)       (a)
#define vreinterpretq_u64_p8(a)        (a)
#define vreinterpretq_u64_p16(a)       (a)
#define vreinterpretq_u64_u8(a)        (a)
#define vreinterpretq_u64_u16(a)       (a)
#define vreinterpretq_u64_u32(a)       (a)

// } +++ auto-generated code ends (vreinterpret macros)

// { +++ auto-generated code begins (Pseudo intrinsics)

//  Multiply by scalar
#define vmul_n_s16(Vd, Rt)             vmul_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vmul_n_s32(Vd, Rt)             vmul_lane_s32((Vd), vmov_n_s32(Rt), 0)
#define vmul_n_u16(Vd, Rt)             vmul_lane_u16((Vd), vmov_n_u16(Rt), 0)
#define vmul_n_u32(Vd, Rt)             vmul_lane_u32((Vd), vmov_n_u32(Rt), 0)
#define vmulq_n_s16(Vd, Rt)            vmulq_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vmulq_n_s32(Vd, Rt)            vmulq_lane_s32((Vd), vmov_n_s32(Rt), 0)
#define vmulq_n_u16(Vd, Rt)            vmulq_lane_u16((Vd), vmov_n_u16(Rt), 0)
#define vmulq_n_u32(Vd, Rt)            vmulq_lane_u32((Vd), vmov_n_u32(Rt), 0)
#define vmull_n_s16(Vd, Rt)            vmull_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vmull_n_s32(Vd, Rt)            vmull_lane_s32((Vd), vmov_n_s32(Rt), 0)
#define vmull_n_u16(Vd, Rt)            vmull_lane_u16((Vd), vmov_n_u16(Rt), 0)
#define vmull_n_u32(Vd, Rt)            vmull_lane_u32((Vd), vmov_n_u32(Rt), 0)
#define vqdmulh_n_s16(Vd, Rt)          vqdmulh_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vqdmulh_n_s32(Vd, Rt)          vqdmulh_lane_s32((Vd), vmov_n_s32(Rt), 0)
#define vqdmulhq_n_s16(Vd, Rt)         vqdmulhq_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vqdmulhq_n_s32(Vd, Rt)         vqdmulhq_lane_s32((Vd), vmov_n_s32(Rt), 0)
#define vqdmull_n_s16(Vd, Rt)          vqdmull_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vqdmull_n_s32(Vd, Rt)          vqdmull_lane_s32((Vd), vmov_n_s32(Rt), 0)
#define vqrdmulh_n_s16(Vd, Rt)         vqrdmulh_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vqrdmulh_n_s32(Vd, Rt)         vqrdmulh_lane_s32((Vd), vmov_n_s32(Rt), 0)
#define vqrdmulhq_n_s16(Vd, Rt)        vqrdmulhq_lane_s16((Vd), vmov_n_s16(Rt), 0)
#define vqrdmulhq_n_s32(Vd, Rt)        vqrdmulhq_lane_s32((Vd), vmov_n_s32(Rt), 0)

//  Multiply by scalar with accumulate
#define vmla_n_s16(Vd, Vn, Rt)         vmla_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vmla_n_s32(Vd, Vn, Rt)         vmla_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)
#define vmla_n_u16(Vd, Vn, Rt)         vmla_lane_u16((Vd), (Vn), vmov_n_u16(Rt), 0)
#define vmla_n_u32(Vd, Vn, Rt)         vmla_lane_u32((Vd), (Vn), vmov_n_u32(Rt), 0)
#define vmlaq_n_s16(Vd, Vn, Rt)        vmlaq_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vmlaq_n_s32(Vd, Vn, Rt)        vmlaq_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)
#define vmlaq_n_u16(Vd, Vn, Rt)        vmlaq_lane_u16((Vd), (Vn), vmov_n_u16(Rt), 0)
#define vmlaq_n_u32(Vd, Vn, Rt)        vmlaq_lane_u32((Vd), (Vn), vmov_n_u32(Rt), 0)
#define vmlal_n_s16(Vd, Vn, Rt)        vmlal_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vmlal_n_s32(Vd, Vn, Rt)        vmlal_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)
#define vmlal_n_u16(Vd, Vn, Rt)        vmlal_lane_u16((Vd), (Vn), vmov_n_u16(Rt), 0)
#define vmlal_n_u32(Vd, Vn, Rt)        vmlal_lane_u32((Vd), (Vn), vmov_n_u32(Rt), 0)
#define vmls_n_s16(Vd, Vn, Rt)         vmls_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vmls_n_s32(Vd, Vn, Rt)         vmls_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)
#define vmls_n_u16(Vd, Vn, Rt)         vmls_lane_u16((Vd), (Vn), vmov_n_u16(Rt), 0)
#define vmls_n_u32(Vd, Vn, Rt)         vmls_lane_u32((Vd), (Vn), vmov_n_u32(Rt), 0)
#define vmlsq_n_s16(Vd, Vn, Rt)        vmlsq_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vmlsq_n_s32(Vd, Vn, Rt)        vmlsq_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)
#define vmlsq_n_u16(Vd, Vn, Rt)        vmlsq_lane_u16((Vd), (Vn), vmov_n_u16(Rt), 0)
#define vmlsq_n_u32(Vd, Vn, Rt)        vmlsq_lane_u32((Vd), (Vn), vmov_n_u32(Rt), 0)
#define vmlsl_n_s16(Vd, Vn, Rt)        vmlsl_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vmlsl_n_s32(Vd, Vn, Rt)        vmlsl_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)
#define vmlsl_n_u16(Vd, Vn, Rt)        vmlsl_lane_u16((Vd), (Vn), vmov_n_u16(Rt), 0)
#define vmlsl_n_u32(Vd, Vn, Rt)        vmlsl_lane_u32((Vd), (Vn), vmov_n_u32(Rt), 0)
#define vqdmlal_n_s16(Vd, Vn, Rt)      vqdmlal_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vqdmlal_n_s32(Vd, Vn, Rt)      vqdmlal_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)
#define vqdmlsl_n_s16(Vd, Vn, Rt)      vqdmlsl_lane_s16((Vd), (Vn), vmov_n_s16(Rt), 0)
#define vqdmlsl_n_s32(Vd, Vn, Rt)      vqdmlsl_lane_s32((Vd), (Vn), vmov_n_s32(Rt), 0)

//  VDUP.64 (scalar)
#define vdup_lane_s64(Dn, lane)        (__static_assert((lane) == 0, "invalid lane index"), (Dn))
#define vdup_lane_u64(Dn, lane)        (__static_assert((lane) == 0, "invalid lane index"), (Dn))

//  VDUP.W.64 (scalar)
#define vdupq_lane_s64(Dn, lane)       (__static_assert((lane) == 0, "invalid lane index"), vcombine_s64((Dn), (Dn)))
#define vdupq_lane_u64(Dn, lane)       (__static_assert((lane) == 0, "invalid lane index"), vcombine_u64((Dn), (Dn)))

// } +++ auto-generated code ends (Pseudo intrinsics)

#else // defined(_ARM_USE_DEPRECATED_NEON_INTRINSICS)

#if defined (__cplusplus)
extern "C" {
#endif  /* defined (__cplusplus) */

__n128 __aesd_p8(__n128 _Qd, __n128 _Qm);
__n128 __aesd_s8(__n128 _Qd, __n128 _Qm);
__n128 __aesd_u8(__n128 _Qd, __n128 _Qm);
__n128 __aese_p8(__n128 _Qd, __n128 _Qm);
__n128 __aese_s8(__n128 _Qd, __n128 _Qm);
__n128 __aese_u8(__n128 _Qd, __n128 _Qm);
__n128 __aesimc_p8(__n128 _Qm);
__n128 __aesimc_s8(__n128 _Qm);
__n128 __aesimc_u8(__n128 _Qm);
__n128 __aesmc_p8(__n128 _Qm);
__n128 __aesmc_s8(__n128 _Qm);
__n128 __aesmc_u8(__n128 _Qm);
__n128 __sha1h_f32(__n128 _Qm);
__n128 __sha1h_s32(__n128 _Qm);
__n128 __sha1h_u32(__n128 _Qm);
__n128 __sha1su1_f32(__n128 _Qd, __n128 _Qm);
__n128 __sha1su1_s32(__n128 _Qd, __n128 _Qm);
__n128 __sha1su1_u32(__n128 _Qd, __n128 _Qm);
__n128 __sha256su0_f32(__n128 _Qd, __n128 _Qm);
__n128 __sha256su0_s32(__n128 _Qd, __n128 _Qm);
__n128 __sha256su0_u32(__n128 _Qd, __n128 _Qm);
__n128 __sha1c_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1c_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1c_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1m_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1m_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1m_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1p_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1p_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1p_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1su0_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1su0_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha1su0_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256h_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256h_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256h_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256h2_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256h2_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256h2_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256su1_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256su1_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __sha256su1_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n64 __vaba_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vaba_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vaba_s8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vaba_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vaba_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vaba_u8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n128 __vabal_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vabal_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vabal_s8(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vabal_u16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vabal_u32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vabal_u8(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vabaq_s16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vabaq_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vabaq_s8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vabaq_u16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vabaq_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vabaq_u8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n64 __vabd_f32(__n64 _Dn, __n64 _Dm);
__n128 __vabdq_f32(__n128 _Qn, __n128 _Qm);
__n64 __vabd_s16(__n64 _Dn, __n64 _Dm);
__n64 __vabd_s32(__n64 _Dn, __n64 _Dm);
__n64 __vabd_s8(__n64 _Dn, __n64 _Dm);
__n64 __vabd_u16(__n64 _Dn, __n64 _Dm);
__n64 __vabd_u32(__n64 _Dn, __n64 _Dm);
__n64 __vabd_u8(__n64 _Dn, __n64 _Dm);
__n128 __vabdl_s16(__n64 _Dn, __n64 _Dm);
__n128 __vabdl_s32(__n64 _Dn, __n64 _Dm);
__n128 __vabdl_s8(__n64 _Dn, __n64 _Dm);
__n128 __vabdl_u16(__n64 _Dn, __n64 _Dm);
__n128 __vabdl_u32(__n64 _Dn, __n64 _Dm);
__n128 __vabdl_u8(__n64 _Dn, __n64 _Dm);
__n128 __vabdq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vabdq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vabdq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vabdq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vabdq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vabdq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vabs_f32(__n64 _Dm);
__n64 __vabs_s16(__n64 _Dm);
__n64 __vabs_s32(__n64 _Dm);
__n64 __vabs_s8(__n64 _Dm);
__n64 __vneg_f32(__n64 _Dm);
__n64 __vneg_s16(__n64 _Dm);
__n64 __vneg_s32(__n64 _Dm);
__n64 __vneg_s8(__n64 _Dm);
__n128 __vabsq_f32(__n128 _Qm);
__n128 __vabsq_s16(__n128 _Qm);
__n128 __vabsq_s32(__n128 _Qm);
__n128 __vabsq_s8(__n128 _Qm);
__n128 __vnegq_f32(__n128 _Qm);
__n128 __vnegq_s16(__n128 _Qm);
__n128 __vnegq_s32(__n128 _Qm);
__n128 __vnegq_s8(__n128 _Qm);
__n64 __vacge_f32(__n64 _Dn, __n64 _Dm);
__n64 __vacgt_f32(__n64 _Dn, __n64 _Dm);
__n64 __vacle_f32(__n64 _Dn, __n64 _Dm);
__n64 __vaclt_f32(__n64 _Dn, __n64 _Dm);
__n128 __vacgeq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vacgtq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vacleq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vacltq_f32(__n128 _Qn, __n128 _Qm);
__n64 __vadd_f32(__n64 _Dn, __n64 _Dm);
__n64 __vadd_s16(__n64 _Dn, __n64 _Dm);
__n64 __vadd_s32(__n64 _Dn, __n64 _Dm);
__n64 __vadd_s64(__n64 _Dn, __n64 _Dm);
__n64 __vadd_s8(__n64 _Dn, __n64 _Dm);
__n64 __vadd_u16(__n64 _Dn, __n64 _Dm);
__n64 __vadd_u32(__n64 _Dn, __n64 _Dm);
__n64 __vadd_u64(__n64 _Dn, __n64 _Dm);
__n64 __vadd_u8(__n64 _Dn, __n64 _Dm);
__n128 __vaddq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vaddq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vaddhn_s16(__n128 _Qn, __n128 _Qm);
__n64 __vaddhn_s32(__n128 _Qn, __n128 _Qm);
__n64 __vaddhn_s64(__n128 _Qn, __n128 _Qm);
__n64 __vaddhn_u16(__n128 _Qn, __n128 _Qm);
__n64 __vaddhn_u32(__n128 _Qn, __n128 _Qm);
__n64 __vaddhn_u64(__n128 _Qn, __n128 _Qm);
__n64 __vraddhn_s16(__n128 _Qn, __n128 _Qm);
__n64 __vraddhn_s32(__n128 _Qn, __n128 _Qm);
__n64 __vraddhn_s64(__n128 _Qn, __n128 _Qm);
__n64 __vraddhn_u16(__n128 _Qn, __n128 _Qm);
__n64 __vraddhn_u32(__n128 _Qn, __n128 _Qm);
__n64 __vraddhn_u64(__n128 _Qn, __n128 _Qm);
__n128 __vaddl_s16(__n64 _Dn, __n64 _Dm);
__n128 __vaddl_s32(__n64 _Dn, __n64 _Dm);
__n128 __vaddl_s8(__n64 _Dn, __n64 _Dm);
__n128 __vaddl_u16(__n64 _Dn, __n64 _Dm);
__n128 __vaddl_u32(__n64 _Dn, __n64 _Dm);
__n128 __vaddl_u8(__n64 _Dn, __n64 _Dm);
__n128 __vaddw_s16(__n128 _Qn, __n64 _Dm);
__n128 __vaddw_s32(__n128 _Qn, __n64 _Dm);
__n128 __vaddw_s8(__n128 _Qn, __n64 _Dm);
__n128 __vaddw_u16(__n128 _Qn, __n64 _Dm);
__n128 __vaddw_u32(__n128 _Qn, __n64 _Dm);
__n128 __vaddw_u8(__n128 _Qn, __n64 _Dm);
__n64 __vand_s16(__n64 _Dn, __n64 _Dm);
__n64 __vand_s32(__n64 _Dn, __n64 _Dm);
__n64 __vand_s64(__n64 _Dn, __n64 _Dm);
__n64 __vand_s8(__n64 _Dn, __n64 _Dm);
__n64 __vand_u16(__n64 _Dn, __n64 _Dm);
__n64 __vand_u32(__n64 _Dn, __n64 _Dm);
__n64 __vand_u64(__n64 _Dn, __n64 _Dm);
__n64 __vand_u8(__n64 _Dn, __n64 _Dm);
__n64 __vorr_s16(__n64 _Dn, __n64 _Dm);
__n64 __vorr_s32(__n64 _Dn, __n64 _Dm);
__n64 __vorr_s64(__n64 _Dn, __n64 _Dm);
__n64 __vorr_s8(__n64 _Dn, __n64 _Dm);
__n64 __vorr_u16(__n64 _Dn, __n64 _Dm);
__n64 __vorr_u32(__n64 _Dn, __n64 _Dm);
__n64 __vorr_u64(__n64 _Dn, __n64 _Dm);
__n64 __vorr_u8(__n64 _Dn, __n64 _Dm);
__n128 __vandq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vandq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vandq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vandq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vandq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vandq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vandq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vandq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vorrq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vbif_f32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_p16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_p8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_s64(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_s8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_u64(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbif_u8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_f32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_p16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_p8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_s64(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_s8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_u64(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbit_u8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_f32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_p16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_p8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_s64(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_s8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_u64(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vbsl_u8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n128 __vbifq_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_p16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_p8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_s16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_s64(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_s8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_u16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_u64(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbifq_u8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_p16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_p8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_s16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_s64(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_s8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_u16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_u64(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbitq_u8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_p16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_p8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_s16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_s64(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_s8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_u16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_u64(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vbslq_u8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n64 __vceq_z_f32_ex(__n64 _Dm);
__n64 __vceq_z_s16_ex(__n64 _Dm);
__n64 __vceq_z_s32_ex(__n64 _Dm);
__n64 __vceq_z_s8_ex(__n64 _Dm);
__n64 __vceq_z_u16_ex(__n64 _Dm);
__n64 __vceq_z_u32_ex(__n64 _Dm);
__n64 __vceq_z_u8_ex(__n64 _Dm);
__n128 __vceqq_z_f32_ex(__n128 _Qm);
__n128 __vceqq_z_s16_ex(__n128 _Qm);
__n128 __vceqq_z_s32_ex(__n128 _Qm);
__n128 __vceqq_z_s8_ex(__n128 _Qm);
__n128 __vceqq_z_u16_ex(__n128 _Qm);
__n128 __vceqq_z_u32_ex(__n128 _Qm);
__n128 __vceqq_z_u8_ex(__n128 _Qm);
__n64 __vceq_f32(__n64 _Dn, __n64 _Dm);
__n64 __vceq_p8(__n64 _Dn, __n64 _Dm);
__n64 __vceq_s16(__n64 _Dn, __n64 _Dm);
__n64 __vceq_s32(__n64 _Dn, __n64 _Dm);
__n64 __vceq_s8(__n64 _Dn, __n64 _Dm);
__n64 __vceq_u16(__n64 _Dn, __n64 _Dm);
__n64 __vceq_u32(__n64 _Dn, __n64 _Dm);
__n64 __vceq_u8(__n64 _Dn, __n64 _Dm);
__n128 __vceqq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vceqq_p8(__n128 _Qn, __n128 _Qm);
__n128 __vceqq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vceqq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vceqq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vceqq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vceqq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vceqq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vcge_z_f32_ex(__n64 _Dm);
__n64 __vcge_z_s16_ex(__n64 _Dm);
__n64 __vcge_z_s32_ex(__n64 _Dm);
__n64 __vcge_z_s8_ex(__n64 _Dm);
__n128 __vcgeq_z_f32_ex(__n128 _Qm);
__n128 __vcgeq_z_s16_ex(__n128 _Qm);
__n128 __vcgeq_z_s32_ex(__n128 _Qm);
__n128 __vcgeq_z_s8_ex(__n128 _Qm);
__n64 __vcge_f32(__n64 _Dn, __n64 _Dm);
__n64 __vcge_s16(__n64 _Dn, __n64 _Dm);
__n64 __vcge_s32(__n64 _Dn, __n64 _Dm);
__n64 __vcge_s8(__n64 _Dn, __n64 _Dm);
__n64 __vcge_u16(__n64 _Dn, __n64 _Dm);
__n64 __vcge_u32(__n64 _Dn, __n64 _Dm);
__n64 __vcge_u8(__n64 _Dn, __n64 _Dm);
__n64 __vcle_f32(__n64 _Dn, __n64 _Dm);
__n64 __vcle_s16(__n64 _Dn, __n64 _Dm);
__n64 __vcle_s32(__n64 _Dn, __n64 _Dm);
__n64 __vcle_s8(__n64 _Dn, __n64 _Dm);
__n64 __vcle_u16(__n64 _Dn, __n64 _Dm);
__n64 __vcle_u32(__n64 _Dn, __n64 _Dm);
__n64 __vcle_u8(__n64 _Dn, __n64 _Dm);
__n128 __vcgeq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vcgeq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vcgeq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vcgeq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vcgeq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vcgeq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vcgeq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vcleq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vcleq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vcleq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vcleq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vcleq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vcleq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vcleq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vcgt_z_f32_ex(__n64 _Dm);
__n64 __vcgt_z_s16_ex(__n64 _Dm);
__n64 __vcgt_z_s32_ex(__n64 _Dm);
__n64 __vcgt_z_s8_ex(__n64 _Dm);
__n128 __vcgtq_z_f32_ex(__n128 _Qm);
__n128 __vcgtq_z_s16_ex(__n128 _Qm);
__n128 __vcgtq_z_s32_ex(__n128 _Qm);
__n128 __vcgtq_z_s8_ex(__n128 _Qm);
__n64 __vcgt_f32(__n64 _Dn, __n64 _Dm);
__n64 __vcgt_s16(__n64 _Dn, __n64 _Dm);
__n64 __vcgt_s32(__n64 _Dn, __n64 _Dm);
__n64 __vcgt_s8(__n64 _Dn, __n64 _Dm);
__n64 __vcgt_u16(__n64 _Dn, __n64 _Dm);
__n64 __vcgt_u32(__n64 _Dn, __n64 _Dm);
__n64 __vcgt_u8(__n64 _Dn, __n64 _Dm);
__n64 __vclt_f32(__n64 _Dn, __n64 _Dm);
__n64 __vclt_s16(__n64 _Dn, __n64 _Dm);
__n64 __vclt_s32(__n64 _Dn, __n64 _Dm);
__n64 __vclt_s8(__n64 _Dn, __n64 _Dm);
__n64 __vclt_u16(__n64 _Dn, __n64 _Dm);
__n64 __vclt_u32(__n64 _Dn, __n64 _Dm);
__n64 __vclt_u8(__n64 _Dn, __n64 _Dm);
__n128 __vcgtq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vcgtq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vcgtq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vcgtq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vcgtq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vcgtq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vcgtq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vcltq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vcltq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vcltq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vcltq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vcltq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vcltq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vcltq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vcle_z_f32_ex(__n64 _Dm);
__n64 __vcle_z_s16_ex(__n64 _Dm);
__n64 __vcle_z_s32_ex(__n64 _Dm);
__n64 __vcle_z_s8_ex(__n64 _Dm);
__n128 __vcleq_z_f32_ex(__n128 _Qm);
__n128 __vcleq_z_s16_ex(__n128 _Qm);
__n128 __vcleq_z_s32_ex(__n128 _Qm);
__n128 __vcleq_z_s8_ex(__n128 _Qm);
__n64 __vcls_s16(__n64 _Dm);
__n64 __vcls_s32(__n64 _Dm);
__n64 __vcls_s8(__n64 _Dm);
__n64 __vclz_s16(__n64 _Dm);
__n64 __vclz_s32(__n64 _Dm);
__n64 __vclz_s8(__n64 _Dm);
__n64 __vclz_u16(__n64 _Dm);
__n64 __vclz_u32(__n64 _Dm);
__n64 __vclz_u8(__n64 _Dm);
__n128 __vclsq_s16(__n128 _Qm);
__n128 __vclsq_s32(__n128 _Qm);
__n128 __vclsq_s8(__n128 _Qm);
__n128 __vclzq_s16(__n128 _Qm);
__n128 __vclzq_s32(__n128 _Qm);
__n128 __vclzq_s8(__n128 _Qm);
__n128 __vclzq_u16(__n128 _Qm);
__n128 __vclzq_u32(__n128 _Qm);
__n128 __vclzq_u8(__n128 _Qm);
__n64 __vclt_z_f32_ex(__n64 _Dm);
__n64 __vclt_z_s16_ex(__n64 _Dm);
__n64 __vclt_z_s32_ex(__n64 _Dm);
__n64 __vclt_z_s8_ex(__n64 _Dm);
__n128 __vcltq_z_f32_ex(__n128 _Qm);
__n128 __vcltq_z_s16_ex(__n128 _Qm);
__n128 __vcltq_z_s32_ex(__n128 _Qm);
__n128 __vcltq_z_s8_ex(__n128 _Qm);
__n64 __vcnt_p8(__n64 _Dm);
__n64 __vcnt_s8(__n64 _Dm);
__n64 __vcnt_u8(__n64 _Dm);
__n128 __vcntq_p8(__n128 _Qm);
__n128 __vcntq_s8(__n128 _Qm);
__n128 __vcntq_u8(__n128 _Qm);
__n128 __vcombine_f32(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_p16(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_p8(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_s16(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_s32(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_s64(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_s8(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_u16(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_u32(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_u64(__n64 _Dn, __n64 _Dm);
__n128 __vcombine_u8(__n64 _Dn, __n64 _Dm);
__n64 __vcreate_f32(uint64_t _R64t);
__n64 __vcreate_p16(uint64_t _R64t);
__n64 __vcreate_p8(uint64_t _R64t);
__n64 __vcreate_s16(uint64_t _R64t);
__n64 __vcreate_s32(uint64_t _R64t);
__n64 __vcreate_s64(uint64_t _R64t);
__n64 __vcreate_s8(uint64_t _R64t);
__n64 __vcreate_u16(uint64_t _R64t);
__n64 __vcreate_u32(uint64_t _R64t);
__n64 __vcreate_u64(uint64_t _R64t);
__n64 __vcreate_u8(uint64_t _R64t);
__n64 __vcvt_n_f32_s32(__n64 _Dm, const int _Fbits);
__n64 __vcvt_n_f32_u32(__n64 _Dm, const int _Fbits);
__n64 __vcvt_n_s32_f32(__n64 _Dm, const int _Fbits);
__n64 __vcvt_n_u32_f32(__n64 _Dm, const int _Fbits);
__n128 __vcvtq_n_f32_s32(__n128 _Qm, const int _Fbits);
__n128 __vcvtq_n_f32_u32(__n128 _Qm, const int _Fbits);
__n128 __vcvtq_n_s32_f32(__n128 _Qm, const int _Fbits);
__n128 __vcvtq_n_u32_f32(__n128 _Qm, const int _Fbits);
__n64 __vcvta_s32_f32(__n64 _Dm);
__n64 __vcvta_u32_f32(__n64 _Dm);
__n64 __vcvtm_s32_f32(__n64 _Dm);
__n64 __vcvtm_u32_f32(__n64 _Dm);
__n64 __vcvtn_s32_f32(__n64 _Dm);
__n64 __vcvtn_u32_f32(__n64 _Dm);
__n64 __vcvtp_s32_f32(__n64 _Dm);
__n64 __vcvtp_u32_f32(__n64 _Dm);
__n128 __vcvtaq_s32_f32(__n128 _Qm);
__n128 __vcvtaq_u32_f32(__n128 _Qm);
__n128 __vcvtmq_s32_f32(__n128 _Qm);
__n128 __vcvtmq_u32_f32(__n128 _Qm);
__n128 __vcvtnq_s32_f32(__n128 _Qm);
__n128 __vcvtnq_u32_f32(__n128 _Qm);
__n128 __vcvtpq_s32_f32(__n128 _Qm);
__n128 __vcvtpq_u32_f32(__n128 _Qm);
__n64 __vcvt_f32_s32(__n64 _Dm);
__n64 __vcvt_f32_u32(__n64 _Dm);
__n64 __vcvt_s32_f32(__n64 _Dm);
__n64 __vcvt_u32_f32(__n64 _Dm);
__n128 __vcvtq_f32_s32(__n128 _Qm);
__n128 __vcvtq_f32_u32(__n128 _Qm);
__n128 __vcvtq_s32_f32(__n128 _Qm);
__n128 __vcvtq_u32_f32(__n128 _Qm);
__n64 __vdup_lane_f32(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_p16(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_p8(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_s16(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_s32(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_s8(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_u16(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_u32(__n64 _Dm, const int _Lane);
__n64 __vdup_lane_u8(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_f32(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_p16(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_p8(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_s16(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_s32(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_s8(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_u16(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_u32(__n64 _Dm, const int _Lane);
__n128 __vdupq_lane_u8(__n64 _Dm, const int _Lane);
__n64 __vdup_n_f32(float32_t _Ft);
__n64 __vmov_n_f32(float32_t _Ft);
__n64 __vdup_n_p16(poly16_t _Rt);
__n64 __vdup_n_p8(poly8_t _Rt);
__n64 __vdup_n_s16(int16_t _Rt);
__n64 __vdup_n_s32(int32_t _Rt);
__n64 __vdup_n_s8(int8_t _Rt);
__n64 __vdup_n_u16(uint16_t _Rt);
__n64 __vdup_n_u32(uint32_t _Rt);
__n64 __vdup_n_u8(uint8_t _Rt);
__n64 __vmov_n_p16(poly16_t _Rt);
__n64 __vmov_n_p8(poly8_t _Rt);
__n64 __vmov_n_s16(int16_t _Rt);
__n64 __vmov_n_s32(int32_t _Rt);
__n64 __vmov_n_s8(int8_t _Rt);
__n64 __vmov_n_u16(uint16_t _Rt);
__n64 __vmov_n_u32(uint32_t _Rt);
__n64 __vmov_n_u8(uint8_t _Rt);
__n128 __vdupq_n_f32(float32_t _Ft);
__n128 __vmovq_n_f32(float32_t _Ft);
__n128 __vdupq_n_p16(poly16_t _Rt);
__n128 __vdupq_n_p8(poly8_t _Rt);
__n128 __vdupq_n_s16(int16_t _Rt);
__n128 __vdupq_n_s32(int32_t _Rt);
__n128 __vdupq_n_s8(int8_t _Rt);
__n128 __vdupq_n_u16(uint16_t _Rt);
__n128 __vdupq_n_u32(uint32_t _Rt);
__n128 __vdupq_n_u8(uint8_t _Rt);
__n128 __vmovq_n_p16(poly16_t _Rt);
__n128 __vmovq_n_p8(poly8_t _Rt);
__n128 __vmovq_n_s16(int16_t _Rt);
__n128 __vmovq_n_s32(int32_t _Rt);
__n128 __vmovq_n_s8(int8_t _Rt);
__n128 __vmovq_n_u16(uint16_t _Rt);
__n128 __vmovq_n_u32(uint32_t _Rt);
__n128 __vmovq_n_u8(uint8_t _Rt);
__n64 __vdup_n_s64(int64_t _R64t);
__n64 __vdup_n_u64(uint64_t _R64t);
__n64 __vmov_n_s64(int64_t _R64t);
__n64 __vmov_n_u64(uint64_t _R64t);
__n128 __vdupq_n_s64(int64_t _R64t);
__n128 __vdupq_n_u64(uint64_t _R64t);
__n128 __vmovq_n_s64(int64_t _R64t);
__n128 __vmovq_n_u64(uint64_t _R64t);
__n64 __vbic_s16(__n64 _Dn, __n64 _Dm);
__n64 __vbic_s32(__n64 _Dn, __n64 _Dm);
__n64 __vbic_s64(__n64 _Dn, __n64 _Dm);
__n64 __vbic_s8(__n64 _Dn, __n64 _Dm);
__n64 __vbic_u16(__n64 _Dn, __n64 _Dm);
__n64 __vbic_u32(__n64 _Dn, __n64 _Dm);
__n64 __vbic_u64(__n64 _Dn, __n64 _Dm);
__n64 __vbic_u8(__n64 _Dn, __n64 _Dm);
__n64 __veor_s16(__n64 _Dn, __n64 _Dm);
__n64 __veor_s32(__n64 _Dn, __n64 _Dm);
__n64 __veor_s64(__n64 _Dn, __n64 _Dm);
__n64 __veor_s8(__n64 _Dn, __n64 _Dm);
__n64 __veor_u16(__n64 _Dn, __n64 _Dm);
__n64 __veor_u32(__n64 _Dn, __n64 _Dm);
__n64 __veor_u64(__n64 _Dn, __n64 _Dm);
__n64 __veor_u8(__n64 _Dn, __n64 _Dm);
__n64 __vorn_s16(__n64 _Dn, __n64 _Dm);
__n64 __vorn_s32(__n64 _Dn, __n64 _Dm);
__n64 __vorn_s64(__n64 _Dn, __n64 _Dm);
__n64 __vorn_s8(__n64 _Dn, __n64 _Dm);
__n64 __vorn_u16(__n64 _Dn, __n64 _Dm);
__n64 __vorn_u32(__n64 _Dn, __n64 _Dm);
__n64 __vorn_u64(__n64 _Dn, __n64 _Dm);
__n64 __vorn_u8(__n64 _Dn, __n64 _Dm);
__n128 __vbicq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vbicq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vbicq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vbicq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vbicq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vbicq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vbicq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vbicq_u8(__n128 _Qn, __n128 _Qm);
__n128 __veorq_s16(__n128 _Qn, __n128 _Qm);
__n128 __veorq_s32(__n128 _Qn, __n128 _Qm);
__n128 __veorq_s64(__n128 _Qn, __n128 _Qm);
__n128 __veorq_s8(__n128 _Qn, __n128 _Qm);
__n128 __veorq_u16(__n128 _Qn, __n128 _Qm);
__n128 __veorq_u32(__n128 _Qn, __n128 _Qm);
__n128 __veorq_u64(__n128 _Qn, __n128 _Qm);
__n128 __veorq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vornq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vornq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vornq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vornq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vornq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vornq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vornq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vornq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vext_f32(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_p16(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_p8(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_s16(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_s32(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_s64(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_s8(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_u16(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_u32(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_u64(__n64 _Dn, __n64 _Dm, const int _Pos);
__n64 __vext_u8(__n64 _Dn, __n64 _Dm, const int _Pos);
__n128 __vextq_f32(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_p16(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_p8(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_s16(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_s32(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_s64(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_s8(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_u16(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_u32(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_u64(__n128 _Qn, __n128 _Qm, const int _Pos);
__n128 __vextq_u8(__n128 _Qn, __n128 _Qm, const int _Pos);
__n64 __vget_high_f32(__n128 _Qm);
__n64 __vget_high_p16(__n128 _Qm);
__n64 __vget_high_p8(__n128 _Qm);
__n64 __vget_high_s16(__n128 _Qm);
__n64 __vget_high_s32(__n128 _Qm);
__n64 __vget_high_s64(__n128 _Qm);
__n64 __vget_high_s8(__n128 _Qm);
__n64 __vget_high_u16(__n128 _Qm);
__n64 __vget_high_u32(__n128 _Qm);
__n64 __vget_high_u64(__n128 _Qm);
__n64 __vget_high_u8(__n128 _Qm);
__n64 __vget_low_f32(__n128 _Qm);
__n64 __vget_low_p16(__n128 _Qm);
__n64 __vget_low_p8(__n128 _Qm);
__n64 __vget_low_s16(__n128 _Qm);
__n64 __vget_low_s32(__n128 _Qm);
__n64 __vget_low_s64(__n128 _Qm);
__n64 __vget_low_s8(__n128 _Qm);
__n64 __vget_low_u16(__n128 _Qm);
__n64 __vget_low_u32(__n128 _Qm);
__n64 __vget_low_u64(__n128 _Qm);
__n64 __vget_low_u8(__n128 _Qm);
__n64 __vhadd_s16(__n64 _Dn, __n64 _Dm);
__n64 __vhadd_s32(__n64 _Dn, __n64 _Dm);
__n64 __vhadd_s8(__n64 _Dn, __n64 _Dm);
__n64 __vhadd_u16(__n64 _Dn, __n64 _Dm);
__n64 __vhadd_u32(__n64 _Dn, __n64 _Dm);
__n64 __vhadd_u8(__n64 _Dn, __n64 _Dm);
__n64 __vhsub_s16(__n64 _Dn, __n64 _Dm);
__n64 __vhsub_s32(__n64 _Dn, __n64 _Dm);
__n64 __vhsub_s8(__n64 _Dn, __n64 _Dm);
__n64 __vhsub_u16(__n64 _Dn, __n64 _Dm);
__n64 __vhsub_u32(__n64 _Dn, __n64 _Dm);
__n64 __vhsub_u8(__n64 _Dn, __n64 _Dm);
__n64 __vrhadd_s16(__n64 _Dn, __n64 _Dm);
__n64 __vrhadd_s32(__n64 _Dn, __n64 _Dm);
__n64 __vrhadd_s8(__n64 _Dn, __n64 _Dm);
__n64 __vrhadd_u16(__n64 _Dn, __n64 _Dm);
__n64 __vrhadd_u32(__n64 _Dn, __n64 _Dm);
__n64 __vrhadd_u8(__n64 _Dn, __n64 _Dm);
__n128 __vhaddq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vhaddq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vhaddq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vhaddq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vhaddq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vhaddq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vhsubq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vhsubq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vhsubq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vhsubq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vhsubq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vhsubq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vrhaddq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vrhaddq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vrhaddq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vrhaddq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vrhaddq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vrhaddq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vld1_f32(_In_reads_(2) const float32_t * _PcD);
__n64 __vld1_p16(_In_reads_(4) const poly16_t * _PcD);
__n64 __vld1_p8(_In_reads_(8) const poly8_t * _PcD);
__n64 __vld1_s16(_In_reads_(4) const int16_t * _PcD);
__n64 __vld1_s32(_In_reads_(2) const int32_t * _PcD);
__n64 __vld1_s64(_In_reads_(1) const int64_t * _PcD);
__n64 __vld1_s8(_In_reads_(8) const int8_t * _PcD);
__n64 __vld1_u16(_In_reads_(4) const uint16_t * _PcD);
__n64 __vld1_u32(_In_reads_(2) const uint32_t * _PcD);
__n64 __vld1_u64(_In_reads_(1) const uint64_t * _PcD);
__n64 __vld1_u8(_In_reads_(8) const uint8_t * _PcD);
__n64 __vld1_f32_ex(_In_reads_(2) const float32_t * _PcD, const int _Align);
__n64 __vld1_p16_ex(_In_reads_(4) const poly16_t * _PcD, const int _Align);
__n64 __vld1_p8_ex(_In_reads_(8) const poly8_t * _PcD, const int _Align);
__n64 __vld1_s16_ex(_In_reads_(4) const int16_t * _PcD, const int _Align);
__n64 __vld1_s32_ex(_In_reads_(2) const int32_t * _PcD, const int _Align);
__n64 __vld1_s64_ex(_In_reads_(1) const int64_t * _PcD, const int _Align);
__n64 __vld1_s8_ex(_In_reads_(8) const int8_t * _PcD, const int _Align);
__n64 __vld1_u16_ex(_In_reads_(4) const uint16_t * _PcD, const int _Align);
__n64 __vld1_u32_ex(_In_reads_(2) const uint32_t * _PcD, const int _Align);
__n64 __vld1_u64_ex(_In_reads_(1) const uint64_t * _PcD, const int _Align);
__n64 __vld1_u8_ex(_In_reads_(8) const uint8_t * _PcD, const int _Align);
__n128 __vld1q_f32(_In_reads_(4) const float32_t * _PcD);
__n128 __vld1q_p16(_In_reads_(8) const poly16_t * _PcD);
__n128 __vld1q_p8(_In_reads_(16) const poly8_t * _PcD);
__n128 __vld1q_s16(_In_reads_(8) const int16_t * _PcD);
__n128 __vld1q_s32(_In_reads_(4) const int32_t * _PcD);
__n128 __vld1q_s64(_In_reads_(2) const int64_t * _PcD);
__n128 __vld1q_s8(_In_reads_(16) const int8_t * _PcD);
__n128 __vld1q_u16(_In_reads_(8) const uint16_t * _PcD);
__n128 __vld1q_u32(_In_reads_(4) const uint32_t * _PcD);
__n128 __vld1q_u64(_In_reads_(2) const uint64_t * _PcD);
__n128 __vld1q_u8(_In_reads_(16) const uint8_t * _PcD);
__n128 __vld1q_f32_ex(_In_reads_(4) const float32_t * _PcD, const int _Align);
__n128 __vld1q_p16_ex(_In_reads_(8) const poly16_t * _PcD, const int _Align);
__n128 __vld1q_p8_ex(_In_reads_(16) const poly8_t * _PcD, const int _Align);
__n128 __vld1q_s16_ex(_In_reads_(8) const int16_t * _PcD, const int _Align);
__n128 __vld1q_s32_ex(_In_reads_(4) const int32_t * _PcD, const int _Align);
__n128 __vld1q_s64_ex(_In_reads_(2) const int64_t * _PcD, const int _Align);
__n128 __vld1q_s8_ex(_In_reads_(16) const int8_t * _PcD, const int _Align);
__n128 __vld1q_u16_ex(_In_reads_(8) const uint16_t * _PcD, const int _Align);
__n128 __vld1q_u32_ex(_In_reads_(4) const uint32_t * _PcD, const int _Align);
__n128 __vld1q_u64_ex(_In_reads_(2) const uint64_t * _PcD, const int _Align);
__n128 __vld1q_u8_ex(_In_reads_(16) const uint8_t * _PcD, const int _Align);
__n64 __vld1_dup_f32(_In_reads_(1) const float32_t * _PcD);
__n64 __vld1_dup_p16(_In_reads_(1) const poly16_t * _PcD);
__n64 __vld1_dup_p8(_In_reads_(1) const poly8_t * _PcD);
__n64 __vld1_dup_s16(_In_reads_(1) const int16_t * _PcD);
__n64 __vld1_dup_s32(_In_reads_(1) const int32_t * _PcD);
__n64 __vld1_dup_s8(_In_reads_(1) const int8_t * _PcD);
__n64 __vld1_dup_u16(_In_reads_(1) const uint16_t * _PcD);
__n64 __vld1_dup_u32(_In_reads_(1) const uint32_t * _PcD);
__n64 __vld1_dup_u8(_In_reads_(1) const uint8_t * _PcD);
__n128 __vld1q_dup_f32(_In_reads_(1) const float32_t * _PcD);
__n128 __vld1q_dup_p16(_In_reads_(1) const poly16_t * _PcD);
__n128 __vld1q_dup_p8(_In_reads_(1) const poly8_t * _PcD);
__n128 __vld1q_dup_s16(_In_reads_(1) const int16_t * _PcD);
__n128 __vld1q_dup_s32(_In_reads_(1) const int32_t * _PcD);
__n128 __vld1q_dup_s8(_In_reads_(1) const int8_t * _PcD);
__n128 __vld1q_dup_u16(_In_reads_(1) const uint16_t * _PcD);
__n128 __vld1q_dup_u32(_In_reads_(1) const uint32_t * _PcD);
__n128 __vld1q_dup_u8(_In_reads_(1) const uint8_t * _PcD);
__n64 __vld1_dup_f32_ex(_In_reads_(1) const float32_t * _PcD, const int _Align);
__n64 __vld1_dup_p16_ex(_In_reads_(1) const poly16_t * _PcD, const int _Align);
__n64 __vld1_dup_s16_ex(_In_reads_(1) const int16_t * _PcD, const int _Align);
__n64 __vld1_dup_s32_ex(_In_reads_(1) const int32_t * _PcD, const int _Align);
__n64 __vld1_dup_u16_ex(_In_reads_(1) const uint16_t * _PcD, const int _Align);
__n64 __vld1_dup_u32_ex(_In_reads_(1) const uint32_t * _PcD, const int _Align);
__n128 __vld1q_dup_f32_ex(_In_reads_(1) const float32_t * _PcD, const int _Align);
__n128 __vld1q_dup_p16_ex(_In_reads_(1) const poly16_t * _PcD, const int _Align);
__n128 __vld1q_dup_s16_ex(_In_reads_(1) const int16_t * _PcD, const int _Align);
__n128 __vld1q_dup_s32_ex(_In_reads_(1) const int32_t * _PcD, const int _Align);
__n128 __vld1q_dup_u16_ex(_In_reads_(1) const uint16_t * _PcD, const int _Align);
__n128 __vld1q_dup_u32_ex(_In_reads_(1) const uint32_t * _PcD, const int _Align);
__n64 __vld1_lane_f32(_In_reads_(1) const float32_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_p16(_In_reads_(1) const poly16_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_p8(_In_reads_(1) const poly8_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_s16(_In_reads_(1) const int16_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_s32(_In_reads_(1) const int32_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_s8(_In_reads_(1) const int8_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_u16(_In_reads_(1) const uint16_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_u32(_In_reads_(1) const uint32_t * _PcD, __n64 _Dd, const int _Lane);
__n64 __vld1_lane_u8(_In_reads_(1) const uint8_t * _PcD, __n64 _Dd, const int _Lane);
__n128 __vld1q_lane_f32(_In_reads_(1) const float32_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_p16(_In_reads_(1) const poly16_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_p8(_In_reads_(1) const poly8_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_s16(_In_reads_(1) const int16_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_s32(_In_reads_(1) const int32_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_s8(_In_reads_(1) const int8_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_u16(_In_reads_(1) const uint16_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_u32(_In_reads_(1) const uint32_t * _PcD, __n128 _Qd, const int _Lane);
__n128 __vld1q_lane_u8(_In_reads_(1) const uint8_t * _PcD, __n128 _Qd, const int _Lane);
__n64 __vld1_lane_f32_ex(_In_reads_(1) const float32_t * _PcD, __n64 _Dd, const int _Lane, const int _Align);
__n64 __vld1_lane_p16_ex(_In_reads_(1) const poly16_t * _PcD, __n64 _Dd, const int _Lane, const int _Align);
__n64 __vld1_lane_s16_ex(_In_reads_(1) const int16_t * _PcD, __n64 _Dd, const int _Lane, const int _Align);
__n64 __vld1_lane_s32_ex(_In_reads_(1) const int32_t * _PcD, __n64 _Dd, const int _Lane, const int _Align);
__n64 __vld1_lane_u16_ex(_In_reads_(1) const uint16_t * _PcD, __n64 _Dd, const int _Lane, const int _Align);
__n64 __vld1_lane_u32_ex(_In_reads_(1) const uint32_t * _PcD, __n64 _Dd, const int _Lane, const int _Align);
__n128 __vld1q_lane_f32_ex(_In_reads_(1) const float32_t * _PcD, __n128 _Qd, const int _Lane, const int _Align);
__n128 __vld1q_lane_p16_ex(_In_reads_(1) const poly16_t * _PcD, __n128 _Qd, const int _Lane, const int _Align);
__n128 __vld1q_lane_s16_ex(_In_reads_(1) const int16_t * _PcD, __n128 _Qd, const int _Lane, const int _Align);
__n128 __vld1q_lane_s32_ex(_In_reads_(1) const int32_t * _PcD, __n128 _Qd, const int _Lane, const int _Align);
__n128 __vld1q_lane_u16_ex(_In_reads_(1) const uint16_t * _PcD, __n128 _Qd, const int _Lane, const int _Align);
__n128 __vld1q_lane_u32_ex(_In_reads_(1) const uint32_t * _PcD, __n128 _Qd, const int _Lane, const int _Align);
__n64x2 __vld2_f32(_In_reads_(4) const float32_t * _PcD);
__n64x2 __vld2_p16(_In_reads_(8) const poly16_t * _PcD);
__n64x2 __vld2_p8(_In_reads_(16) const poly8_t * _PcD);
__n64x2 __vld2_s16(_In_reads_(8) const int16_t * _PcD);
__n64x2 __vld2_s32(_In_reads_(4) const int32_t * _PcD);
__n64x2 __vld2_s8(_In_reads_(16) const int8_t * _PcD);
__n64x2 __vld2_u16(_In_reads_(8) const uint16_t * _PcD);
__n64x2 __vld2_u32(_In_reads_(4) const uint32_t * _PcD);
__n64x2 __vld2_u8(_In_reads_(16) const uint8_t * _PcD);
__n64x2 __vld2_s64(_In_reads_(2) const int64_t * _PcD);
__n64x2 __vld2_u64(_In_reads_(2) const uint64_t * _PcD);
__n64x2 __vld2_s64_ex(_In_reads_(2) const int64_t * _PcD, const int _Align);
__n64x2 __vld2_u64_ex(_In_reads_(2) const uint64_t * _PcD, const int _Align);
__n64x2 __vld2_f32_ex(_In_reads_(4) const float32_t * _PcD, const int _Align);
__n64x2 __vld2_p16_ex(_In_reads_(8) const poly16_t * _PcD, const int _Align);
__n64x2 __vld2_p8_ex(_In_reads_(16) const poly8_t * _PcD, const int _Align);
__n64x2 __vld2_s16_ex(_In_reads_(8) const int16_t * _PcD, const int _Align);
__n64x2 __vld2_s32_ex(_In_reads_(4) const int32_t * _PcD, const int _Align);
__n64x2 __vld2_s8_ex(_In_reads_(16) const int8_t * _PcD, const int _Align);
__n64x2 __vld2_u16_ex(_In_reads_(8) const uint16_t * _PcD, const int _Align);
__n64x2 __vld2_u32_ex(_In_reads_(4) const uint32_t * _PcD, const int _Align);
__n64x2 __vld2_u8_ex(_In_reads_(16) const uint8_t * _PcD, const int _Align);
__n128x2 __vld2q_f32(_In_reads_(8) const float32_t * _PcD);
__n128x2 __vld2q_p16(_In_reads_(16) const poly16_t * _PcD);
__n128x2 __vld2q_p8(_In_reads_(32) const poly8_t * _PcD);
__n128x2 __vld2q_s16(_In_reads_(16) const int16_t * _PcD);
__n128x2 __vld2q_s32(_In_reads_(8) const int32_t * _PcD);
__n128x2 __vld2q_s8(_In_reads_(32) const int8_t * _PcD);
__n128x2 __vld2q_u16(_In_reads_(16) const uint16_t * _PcD);
__n128x2 __vld2q_u32(_In_reads_(8) const uint32_t * _PcD);
__n128x2 __vld2q_u8(_In_reads_(32) const uint8_t * _PcD);
__n128x2 __vld2q_f32_ex(_In_reads_(8) const float32_t * _PcD, const int _Align);
__n128x2 __vld2q_p16_ex(_In_reads_(16) const poly16_t * _PcD, const int _Align);
__n128x2 __vld2q_p8_ex(_In_reads_(32) const poly8_t * _PcD, const int _Align);
__n128x2 __vld2q_s16_ex(_In_reads_(16) const int16_t * _PcD, const int _Align);
__n128x2 __vld2q_s32_ex(_In_reads_(8) const int32_t * _PcD, const int _Align);
__n128x2 __vld2q_s8_ex(_In_reads_(32) const int8_t * _PcD, const int _Align);
__n128x2 __vld2q_u16_ex(_In_reads_(16) const uint16_t * _PcD, const int _Align);
__n128x2 __vld2q_u32_ex(_In_reads_(8) const uint32_t * _PcD, const int _Align);
__n128x2 __vld2q_u8_ex(_In_reads_(32) const uint8_t * _PcD, const int _Align);
__n64x2 __vld2_dup_f32(_In_reads_(2) const float32_t * _PcD);
__n64x2 __vld2_dup_p16(_In_reads_(2) const poly16_t * _PcD);
__n64x2 __vld2_dup_p8(_In_reads_(2) const poly8_t * _PcD);
__n64x2 __vld2_dup_s16(_In_reads_(2) const int16_t * _PcD);
__n64x2 __vld2_dup_s32(_In_reads_(2) const int32_t * _PcD);
__n64x2 __vld2_dup_s8(_In_reads_(2) const int8_t * _PcD);
__n64x2 __vld2_dup_u16(_In_reads_(2) const uint16_t * _PcD);
__n64x2 __vld2_dup_u32(_In_reads_(2) const uint32_t * _PcD);
__n64x2 __vld2_dup_u8(_In_reads_(2) const uint8_t * _PcD);
__n64x2 __vld2_dup_s64(_In_reads_(2) const int64_t * _PcD);
__n64x2 __vld2_dup_u64(_In_reads_(2) const uint64_t * _PcD);
__n64x2 __vld2_dup_s64_ex(_In_reads_(2) const int64_t * _PcD, const int _Align);
__n64x2 __vld2_dup_u64_ex(_In_reads_(2) const uint64_t * _PcD, const int _Align);
__n64x2 __vld2_dup_f32_ex(_In_reads_(2) const float32_t * _PcD, const int _Align);
__n64x2 __vld2_dup_p16_ex(_In_reads_(2) const poly16_t * _PcD, const int _Align);
__n64x2 __vld2_dup_p8_ex(_In_reads_(2) const poly8_t * _PcD, const int _Align);
__n64x2 __vld2_dup_s16_ex(_In_reads_(2) const int16_t * _PcD, const int _Align);
__n64x2 __vld2_dup_s32_ex(_In_reads_(2) const int32_t * _PcD, const int _Align);
__n64x2 __vld2_dup_s8_ex(_In_reads_(2) const int8_t * _PcD, const int _Align);
__n64x2 __vld2_dup_u16_ex(_In_reads_(2) const uint16_t * _PcD, const int _Align);
__n64x2 __vld2_dup_u32_ex(_In_reads_(2) const uint32_t * _PcD, const int _Align);
__n64x2 __vld2_dup_u8_ex(_In_reads_(2) const uint8_t * _PcD, const int _Align);
__n64x2 __vld2_lane_f32(_In_reads_(2) const float32_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_p16(_In_reads_(2) const poly16_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_p8(_In_reads_(2) const poly8_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_s16(_In_reads_(2) const int16_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_s32(_In_reads_(2) const int32_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_s8(_In_reads_(2) const int8_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_u16(_In_reads_(2) const uint16_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_u32(_In_reads_(2) const uint32_t * _PcD, __n64x2 _D2, const int _Lane);
__n64x2 __vld2_lane_u8(_In_reads_(2) const uint8_t * _PcD, __n64x2 _D2, const int _Lane);
__n128x2 __vld2q_lane_f32(_In_reads_(2) const float32_t * _PcD, __n128x2 _Q2, const int _Lane);
__n128x2 __vld2q_lane_p16(_In_reads_(2) const poly16_t * _PcD, __n128x2 _Q2, const int _Lane);
__n128x2 __vld2q_lane_s16(_In_reads_(2) const int16_t * _PcD, __n128x2 _Q2, const int _Lane);
__n128x2 __vld2q_lane_s32(_In_reads_(2) const int32_t * _PcD, __n128x2 _Q2, const int _Lane);
__n128x2 __vld2q_lane_u16(_In_reads_(2) const uint16_t * _PcD, __n128x2 _Q2, const int _Lane);
__n128x2 __vld2q_lane_u32(_In_reads_(2) const uint32_t * _PcD, __n128x2 _Q2, const int _Lane);
__n64x2 __vld2_lane_f32_ex(_In_reads_(2) const float32_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_p16_ex(_In_reads_(2) const poly16_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_p8_ex(_In_reads_(2) const poly8_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_s16_ex(_In_reads_(2) const int16_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_s32_ex(_In_reads_(2) const int32_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_s8_ex(_In_reads_(2) const int8_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_u16_ex(_In_reads_(2) const uint16_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_u32_ex(_In_reads_(2) const uint32_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n64x2 __vld2_lane_u8_ex(_In_reads_(2) const uint8_t * _PcD, __n64x2 _D2, const int _Lane, const int _Align);
__n128x2 __vld2q_lane_f32_ex(_In_reads_(2) const float32_t * _PcD, __n128x2 _Q2, const int _Lane, const int _Align);
__n128x2 __vld2q_lane_p16_ex(_In_reads_(2) const poly16_t * _PcD, __n128x2 _Q2, const int _Lane, const int _Align);
__n128x2 __vld2q_lane_s16_ex(_In_reads_(2) const int16_t * _PcD, __n128x2 _Q2, const int _Lane, const int _Align);
__n128x2 __vld2q_lane_s32_ex(_In_reads_(2) const int32_t * _PcD, __n128x2 _Q2, const int _Lane, const int _Align);
__n128x2 __vld2q_lane_u16_ex(_In_reads_(2) const uint16_t * _PcD, __n128x2 _Q2, const int _Lane, const int _Align);
__n128x2 __vld2q_lane_u32_ex(_In_reads_(2) const uint32_t * _PcD, __n128x2 _Q2, const int _Lane, const int _Align);
__n64x3 __vld3_f32(_In_reads_(6) const float32_t * _PcD);
__n64x3 __vld3_p16(_In_reads_(12) const poly16_t * _PcD);
__n64x3 __vld3_p8(_In_reads_(24) const poly8_t * _PcD);
__n64x3 __vld3_s16(_In_reads_(12) const int16_t * _PcD);
__n64x3 __vld3_s32(_In_reads_(6) const int32_t * _PcD);
__n64x3 __vld3_s8(_In_reads_(24) const int8_t * _PcD);
__n64x3 __vld3_u16(_In_reads_(12) const uint16_t * _PcD);
__n64x3 __vld3_u32(_In_reads_(6) const uint32_t * _PcD);
__n64x3 __vld3_u8(_In_reads_(24) const uint8_t * _PcD);
__n64x3 __vld3_s64(_In_reads_(3) const int64_t * _PcD);
__n64x3 __vld3_u64(_In_reads_(3) const uint64_t * _PcD);
__n64x3 __vld3_s64_ex(_In_reads_(3) const int64_t * _PcD, const int _Align);
__n64x3 __vld3_u64_ex(_In_reads_(3) const uint64_t * _PcD, const int _Align);
__n64x3 __vld3_f32_ex(_In_reads_(6) const float32_t * _PcD, const int _Align);
__n64x3 __vld3_p16_ex(_In_reads_(12) const poly16_t * _PcD, const int _Align);
__n64x3 __vld3_p8_ex(_In_reads_(24) const poly8_t * _PcD, const int _Align);
__n64x3 __vld3_s16_ex(_In_reads_(12) const int16_t * _PcD, const int _Align);
__n64x3 __vld3_s32_ex(_In_reads_(6) const int32_t * _PcD, const int _Align);
__n64x3 __vld3_s8_ex(_In_reads_(24) const int8_t * _PcD, const int _Align);
__n64x3 __vld3_u16_ex(_In_reads_(12) const uint16_t * _PcD, const int _Align);
__n64x3 __vld3_u32_ex(_In_reads_(6) const uint32_t * _PcD, const int _Align);
__n64x3 __vld3_u8_ex(_In_reads_(24) const uint8_t * _PcD, const int _Align);
__n128x3 __vld3q_f32(_In_reads_(12) const float32_t * _PcD);
__n128x3 __vld3q_p16(_In_reads_(24) const poly16_t * _PcD);
__n128x3 __vld3q_p8(_In_reads_(48) const poly8_t * _PcD);
__n128x3 __vld3q_s16(_In_reads_(24) const int16_t * _PcD);
__n128x3 __vld3q_s32(_In_reads_(12) const int32_t * _PcD);
__n128x3 __vld3q_s8(_In_reads_(48) const int8_t * _PcD);
__n128x3 __vld3q_u16(_In_reads_(24) const uint16_t * _PcD);
__n128x3 __vld3q_u32(_In_reads_(12) const uint32_t * _PcD);
__n128x3 __vld3q_u8(_In_reads_(48) const uint8_t * _PcD);
__n128x3 __vld3q_f32_ex(_In_reads_(12) const float32_t * _PcD, const int _Align);
__n128x3 __vld3q_p16_ex(_In_reads_(24) const poly16_t * _PcD, const int _Align);
__n128x3 __vld3q_p8_ex(_In_reads_(48) const poly8_t * _PcD, const int _Align);
__n128x3 __vld3q_s16_ex(_In_reads_(24) const int16_t * _PcD, const int _Align);
__n128x3 __vld3q_s32_ex(_In_reads_(12) const int32_t * _PcD, const int _Align);
__n128x3 __vld3q_s8_ex(_In_reads_(48) const int8_t * _PcD, const int _Align);
__n128x3 __vld3q_u16_ex(_In_reads_(24) const uint16_t * _PcD, const int _Align);
__n128x3 __vld3q_u32_ex(_In_reads_(12) const uint32_t * _PcD, const int _Align);
__n128x3 __vld3q_u8_ex(_In_reads_(48) const uint8_t * _PcD, const int _Align);
__n64x3 __vld3_dup_f32(_In_reads_(3) const float32_t * _PcD);
__n64x3 __vld3_dup_p16(_In_reads_(3) const poly16_t * _PcD);
__n64x3 __vld3_dup_p8(_In_reads_(3) const poly8_t * _PcD);
__n64x3 __vld3_dup_s16(_In_reads_(3) const int16_t * _PcD);
__n64x3 __vld3_dup_s32(_In_reads_(3) const int32_t * _PcD);
__n64x3 __vld3_dup_s8(_In_reads_(3) const int8_t * _PcD);
__n64x3 __vld3_dup_u16(_In_reads_(3) const uint16_t * _PcD);
__n64x3 __vld3_dup_u32(_In_reads_(3) const uint32_t * _PcD);
__n64x3 __vld3_dup_u8(_In_reads_(3) const uint8_t * _PcD);
__n64x3 __vld3_dup_s64(_In_reads_(3) const int64_t * _PcD);
__n64x3 __vld3_dup_u64(_In_reads_(3) const uint64_t * _PcD);
__n64x3 __vld3_lane_f32(_In_reads_(3) const float32_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_p16(_In_reads_(3) const poly16_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_p8(_In_reads_(3) const poly8_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_s16(_In_reads_(3) const int16_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_s32(_In_reads_(3) const int32_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_s8(_In_reads_(3) const int8_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_u16(_In_reads_(3) const uint16_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_u32(_In_reads_(3) const uint32_t * _PcD, __n64x3 _D3, const int _Lane);
__n64x3 __vld3_lane_u8(_In_reads_(3) const uint8_t * _PcD, __n64x3 _D3, const int _Lane);
__n128x3 __vld3q_lane_f32(_In_reads_(3) const float32_t * _PcD, __n128x3 _Q3, const int _Lane);
__n128x3 __vld3q_lane_p16(_In_reads_(3) const poly16_t * _PcD, __n128x3 _Q3, const int _Lane);
__n128x3 __vld3q_lane_s16(_In_reads_(3) const int16_t * _PcD, __n128x3 _Q3, const int _Lane);
__n128x3 __vld3q_lane_s32(_In_reads_(3) const int32_t * _PcD, __n128x3 _Q3, const int _Lane);
__n128x3 __vld3q_lane_u16(_In_reads_(3) const uint16_t * _PcD, __n128x3 _Q3, const int _Lane);
__n128x3 __vld3q_lane_u32(_In_reads_(3) const uint32_t * _PcD, __n128x3 _Q3, const int _Lane);
__n64x4 __vld4_f32(_In_reads_(8) const float32_t * _PcD);
__n64x4 __vld4_p16(_In_reads_(16) const poly16_t * _PcD);
__n64x4 __vld4_p8(_In_reads_(32) const poly8_t * _PcD);
__n64x4 __vld4_s16(_In_reads_(16) const int16_t * _PcD);
__n64x4 __vld4_s32(_In_reads_(8) const int32_t * _PcD);
__n64x4 __vld4_s8(_In_reads_(32) const int8_t * _PcD);
__n64x4 __vld4_u16(_In_reads_(16) const uint16_t * _PcD);
__n64x4 __vld4_u32(_In_reads_(8) const uint32_t * _PcD);
__n64x4 __vld4_u8(_In_reads_(32) const uint8_t * _PcD);
__n64x4 __vld4_s64(_In_reads_(4) const int64_t * _PcD);
__n64x4 __vld4_u64(_In_reads_(4) const uint64_t * _PcD);
__n64x4 __vld4_s64_ex(_In_reads_(4) const int64_t * _PcD, const int _Align);
__n64x4 __vld4_u64_ex(_In_reads_(4) const uint64_t * _PcD, const int _Align);
__n64x4 __vld4_f32_ex(_In_reads_(8) const float32_t * _PcD, const int _Align);
__n64x4 __vld4_p16_ex(_In_reads_(16) const poly16_t * _PcD, const int _Align);
__n64x4 __vld4_p8_ex(_In_reads_(32) const poly8_t * _PcD, const int _Align);
__n64x4 __vld4_s16_ex(_In_reads_(16) const int16_t * _PcD, const int _Align);
__n64x4 __vld4_s32_ex(_In_reads_(8) const int32_t * _PcD, const int _Align);
__n64x4 __vld4_s8_ex(_In_reads_(32) const int8_t * _PcD, const int _Align);
__n64x4 __vld4_u16_ex(_In_reads_(16) const uint16_t * _PcD, const int _Align);
__n64x4 __vld4_u32_ex(_In_reads_(8) const uint32_t * _PcD, const int _Align);
__n64x4 __vld4_u8_ex(_In_reads_(32) const uint8_t * _PcD, const int _Align);
__n128x4 __vld4q_f32(_In_reads_(16) const float32_t * _PcD);
__n128x4 __vld4q_p16(_In_reads_(32) const poly16_t * _PcD);
__n128x4 __vld4q_p8(_In_reads_(64) const poly8_t * _PcD);
__n128x4 __vld4q_s16(_In_reads_(32) const int16_t * _PcD);
__n128x4 __vld4q_s32(_In_reads_(16) const int32_t * _PcD);
__n128x4 __vld4q_s8(_In_reads_(64) const int8_t * _PcD);
__n128x4 __vld4q_u16(_In_reads_(32) const uint16_t * _PcD);
__n128x4 __vld4q_u32(_In_reads_(16) const uint32_t * _PcD);
__n128x4 __vld4q_u8(_In_reads_(64) const uint8_t * _PcD);
__n128x4 __vld4q_f32_ex(_In_reads_(16) const float32_t * _PcD, const int _Align);
__n128x4 __vld4q_p16_ex(_In_reads_(32) const poly16_t * _PcD, const int _Align);
__n128x4 __vld4q_p8_ex(_In_reads_(64) const poly8_t * _PcD, const int _Align);
__n128x4 __vld4q_s16_ex(_In_reads_(32) const int16_t * _PcD, const int _Align);
__n128x4 __vld4q_s32_ex(_In_reads_(16) const int32_t * _PcD, const int _Align);
__n128x4 __vld4q_s8_ex(_In_reads_(64) const int8_t * _PcD, const int _Align);
__n128x4 __vld4q_u16_ex(_In_reads_(32) const uint16_t * _PcD, const int _Align);
__n128x4 __vld4q_u32_ex(_In_reads_(16) const uint32_t * _PcD, const int _Align);
__n128x4 __vld4q_u8_ex(_In_reads_(64) const uint8_t * _PcD, const int _Align);
__n64x4 __vld4_dup_f32(_In_reads_(4) const float32_t * _PcD);
__n64x4 __vld4_dup_p16(_In_reads_(4) const poly16_t * _PcD);
__n64x4 __vld4_dup_p8(_In_reads_(4) const poly8_t * _PcD);
__n64x4 __vld4_dup_s16(_In_reads_(4) const int16_t * _PcD);
__n64x4 __vld4_dup_s32(_In_reads_(4) const int32_t * _PcD);
__n64x4 __vld4_dup_s8(_In_reads_(4) const int8_t * _PcD);
__n64x4 __vld4_dup_u16(_In_reads_(4) const uint16_t * _PcD);
__n64x4 __vld4_dup_u32(_In_reads_(4) const uint32_t * _PcD);
__n64x4 __vld4_dup_u8(_In_reads_(4) const uint8_t * _PcD);
__n64x4 __vld4_dup_s64(_In_reads_(4) const int64_t * _PcD);
__n64x4 __vld4_dup_u64(_In_reads_(4) const uint64_t * _PcD);
__n64x4 __vld4_dup_f32_ex(_In_reads_(4) const float32_t * _PcD, const int _Align);
__n64x4 __vld4_dup_p16_ex(_In_reads_(4) const poly16_t * _PcD, const int _Align);
__n64x4 __vld4_dup_p8_ex(_In_reads_(4) const poly8_t * _PcD, const int _Align);
__n64x4 __vld4_dup_s16_ex(_In_reads_(4) const int16_t * _PcD, const int _Align);
__n64x4 __vld4_dup_s32_ex(_In_reads_(4) const int32_t * _PcD, const int _Align);
__n64x4 __vld4_dup_s8_ex(_In_reads_(4) const int8_t * _PcD, const int _Align);
__n64x4 __vld4_dup_u16_ex(_In_reads_(4) const uint16_t * _PcD, const int _Align);
__n64x4 __vld4_dup_u32_ex(_In_reads_(4) const uint32_t * _PcD, const int _Align);
__n64x4 __vld4_dup_u8_ex(_In_reads_(4) const uint8_t * _PcD, const int _Align);
__n64x4 __vld4_lane_f32(_In_reads_(4) const float32_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_p16(_In_reads_(4) const poly16_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_p8(_In_reads_(4) const poly8_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_s16(_In_reads_(4) const int16_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_s32(_In_reads_(4) const int32_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_s8(_In_reads_(4) const int8_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_u16(_In_reads_(4) const uint16_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_u32(_In_reads_(4) const uint32_t * _PcD, __n64x4 _D4, const int _Lane);
__n64x4 __vld4_lane_u8(_In_reads_(4) const uint8_t * _PcD, __n64x4 _D4, const int _Lane);
__n128x4 __vld4q_lane_f32(_In_reads_(4) const float32_t * _PcD, __n128x4 _Q4, const int _Lane);
__n128x4 __vld4q_lane_p16(_In_reads_(4) const poly16_t * _PcD, __n128x4 _Q4, const int _Lane);
__n128x4 __vld4q_lane_s16(_In_reads_(4) const int16_t * _PcD, __n128x4 _Q4, const int _Lane);
__n128x4 __vld4q_lane_s32(_In_reads_(4) const int32_t * _PcD, __n128x4 _Q4, const int _Lane);
__n128x4 __vld4q_lane_u16(_In_reads_(4) const uint16_t * _PcD, __n128x4 _Q4, const int _Lane);
__n128x4 __vld4q_lane_u32(_In_reads_(4) const uint32_t * _PcD, __n128x4 _Q4, const int _Lane);
__n64x4 __vld4_lane_f32_ex(_In_reads_(4) const float32_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_p16_ex(_In_reads_(4) const poly16_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_p8_ex(_In_reads_(4) const poly8_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_s16_ex(_In_reads_(4) const int16_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_s32_ex(_In_reads_(4) const int32_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_s8_ex(_In_reads_(4) const int8_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_u16_ex(_In_reads_(4) const uint16_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_u32_ex(_In_reads_(4) const uint32_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n64x4 __vld4_lane_u8_ex(_In_reads_(4) const uint8_t * _PcD, __n64x4 _D4, const int _Lane, const int _Align);
__n128x4 __vld4q_lane_f32_ex(_In_reads_(4) const float32_t * _PcD, __n128x4 _Q4, const int _Lane, const int _Align);
__n128x4 __vld4q_lane_p16_ex(_In_reads_(4) const poly16_t * _PcD, __n128x4 _Q4, const int _Lane, const int _Align);
__n128x4 __vld4q_lane_s16_ex(_In_reads_(4) const int16_t * _PcD, __n128x4 _Q4, const int _Lane, const int _Align);
__n128x4 __vld4q_lane_s32_ex(_In_reads_(4) const int32_t * _PcD, __n128x4 _Q4, const int _Lane, const int _Align);
__n128x4 __vld4q_lane_u16_ex(_In_reads_(4) const uint16_t * _PcD, __n128x4 _Q4, const int _Lane, const int _Align);
__n128x4 __vld4q_lane_u32_ex(_In_reads_(4) const uint32_t * _PcD, __n128x4 _Q4, const int _Lane, const int _Align);
__n64 __vmax_f32(__n64 _Dn, __n64 _Dm);
__n64 __vmaxnm_f32(__n64 _Dn, __n64 _Dm);
__n64 __vmin_f32(__n64 _Dn, __n64 _Dm);
__n64 __vminnm_f32(__n64 _Dn, __n64 _Dm);
__n128 __vmaxq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vmaxnmq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vminq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vminnmq_f32(__n128 _Qn, __n128 _Qm);
__n64 __vmax_s16(__n64 _Dn, __n64 _Dm);
__n64 __vmax_s32(__n64 _Dn, __n64 _Dm);
__n64 __vmax_s8(__n64 _Dn, __n64 _Dm);
__n64 __vmax_u16(__n64 _Dn, __n64 _Dm);
__n64 __vmax_u32(__n64 _Dn, __n64 _Dm);
__n64 __vmax_u8(__n64 _Dn, __n64 _Dm);
__n64 __vmin_s16(__n64 _Dn, __n64 _Dm);
__n64 __vmin_s32(__n64 _Dn, __n64 _Dm);
__n64 __vmin_s8(__n64 _Dn, __n64 _Dm);
__n64 __vmin_u16(__n64 _Dn, __n64 _Dm);
__n64 __vmin_u32(__n64 _Dn, __n64 _Dm);
__n64 __vmin_u8(__n64 _Dn, __n64 _Dm);
__n128 __vmaxq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vmaxq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vmaxq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vmaxq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vmaxq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vmaxq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vminq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vminq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vminq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vminq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vminq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vminq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vmla_lane_f32(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmla_lane_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmla_lane_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmla_lane_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmla_lane_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmls_lane_f32(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmls_lane_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmls_lane_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmls_lane_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmls_lane_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlaq_lane_f32(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlaq_lane_s16(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlaq_lane_s32(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlaq_lane_u16(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlaq_lane_u32(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlsq_lane_f32(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlsq_lane_s16(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlsq_lane_s32(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlsq_lane_u16(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmlsq_lane_u32(__n128 _Qd, __n128 _Qn, __n64 _Dm, const int _Lane);
__n64 __vmla_n_f32(__n64 _Dd, __n64 _Dn, float32_t _Ft);
__n64 __vmls_n_f32(__n64 _Dd, __n64 _Dn, float32_t _Ft);
__n128 __vmlaq_n_f32(__n128 _Qd, __n128 _Qn, float32_t _Ft);
__n128 __vmlsq_n_f32(__n128 _Qd, __n128 _Qn, float32_t _Ft);
__n64 __vmla_f32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmls_f32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n128 __vmlaq_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlsq_f32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n64 __vmla_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmla_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmla_s8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmla_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmla_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmla_u8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmls_s16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmls_s32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmls_s8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmls_u16(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmls_u32(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vmls_u8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n128 __vmlaq_s16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlaq_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlaq_s8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlaq_u16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlaq_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlaq_u8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlsq_s16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlsq_s32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlsq_s8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlsq_u16(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlsq_u32(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlsq_u8(__n128 _Qd, __n128 _Qn, __n128 _Qm);
__n128 __vmlal_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlal_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlal_s8(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlal_u16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlal_u32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlal_u8(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlsl_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlsl_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlsl_s8(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlsl_u16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlsl_u32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlsl_u8(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vmlal_lane_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlal_lane_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlal_lane_u16(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlal_lane_u32(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlsl_lane_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlsl_lane_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlsl_lane_u16(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmlsl_lane_u32(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vset_lane_f32(float32_t _Ft, __n64 _Dd, const int _Lane);
__n64 __vset_lane_p16(poly16_t _Rt, __n64 _Dd, const int _Lane);
__n64 __vset_lane_p8(poly8_t _Rt, __n64 _Dd, const int _Lane);
__n64 __vset_lane_s16(int16_t _Rt, __n64 _Dd, const int _Lane);
__n64 __vset_lane_s32(int32_t _Rt, __n64 _Dd, const int _Lane);
__n64 __vset_lane_s8(int8_t _Rt, __n64 _Dd, const int _Lane);
__n64 __vset_lane_u16(uint16_t _Rt, __n64 _Dd, const int _Lane);
__n64 __vset_lane_u32(uint32_t _Rt, __n64 _Dd, const int _Lane);
__n64 __vset_lane_u8(uint8_t _Rt, __n64 _Dd, const int _Lane);
float32_t __vget_lane_f32(__n64 _Dm, const int _Lane);
poly16_t __vget_lane_p16(__n64 _Dm, const int _Lane);
poly8_t __vget_lane_p8(__n64 _Dm, const int _Lane);
int16_t __vget_lane_s16(__n64 _Dm, const int _Lane);
int8_t __vget_lane_s8(__n64 _Dm, const int _Lane);
int32_t __vget_lane_s32(__n64 _Dm, const int _Lane);
uint16_t __vget_lane_u16(__n64 _Dm, const int _Lane);
uint8_t __vget_lane_u8(__n64 _Dm, const int _Lane);
uint32_t __vget_lane_u32(__n64 _Dm, const int _Lane);
__n64 __vset_lane_s64(int64_t _R64t, __n64 _Dd, const int _Lane);
__n64 __vset_lane_u64(uint64_t _R64t, __n64 _Dd, const int _Lane);
__n128 __vsetq_lane_s64(int64_t _R64t, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_u64(uint64_t _R64t, __n128 _Qd, const int _Lane);
int64_t __vget_lane_s64(__n64 _Dm, const int _Lane);
uint64_t __vget_lane_u64(__n64 _Dm, const int _Lane);
int64_t __vgetq_lane_s64(__n128 _Qm, const int _Lane);
uint64_t __vgetq_lane_u64(__n128 _Qm, const int _Lane);
__n128 __vsetq_lane_f32(float32_t _Ft, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_p16(poly16_t _Rt, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_p8(poly8_t _Rt, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_s16(int16_t _Rt, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_s32(int32_t _Rt, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_s8(int8_t _Rt, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_u16(uint16_t _Rt, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_u32(uint32_t _Rt, __n128 _Qd, const int _Lane);
__n128 __vsetq_lane_u8(uint8_t _Rt, __n128 _Qd, const int _Lane);
float32_t __vgetq_lane_f32(__n128 _Qm, const int _Lane);
poly16_t __vgetq_lane_p16(__n128 _Qm, const int _Lane);
poly8_t __vgetq_lane_p8(__n128 _Qm, const int _Lane);
int16_t __vgetq_lane_s16(__n128 _Qm, const int _Lane);
int8_t __vgetq_lane_s8(__n128 _Qm, const int _Lane);
int32_t __vgetq_lane_s32(__n128 _Qm, const int _Lane);
uint16_t __vgetq_lane_u16(__n128 _Qm, const int _Lane);
uint8_t __vgetq_lane_u8(__n128 _Qm, const int _Lane);
uint32_t __vgetq_lane_u32(__n128 _Qm, const int _Lane);
__n128 __vmovl_s16(__n64 _Dm);
__n128 __vmovl_s32(__n64 _Dm);
__n128 __vmovl_s8(__n64 _Dm);
__n128 __vmovl_u16(__n64 _Dm);
__n128 __vmovl_u32(__n64 _Dm);
__n128 __vmovl_u8(__n64 _Dm);
__n64 __vmovn_s16(__n128 _Qm);
__n64 __vmovn_s32(__n128 _Qm);
__n64 __vmovn_s64(__n128 _Qm);
__n64 __vmovn_u16(__n128 _Qm);
__n64 __vmovn_u32(__n128 _Qm);
__n64 __vmovn_u64(__n128 _Qm);
__n64 __vmul_f32(__n64 _Dn, __n64 _Dm);
__n64 __vmul_p8(__n64 _Dn, __n64 _Dm);
__n64 __vmul_s16(__n64 _Dn, __n64 _Dm);
__n64 __vmul_s32(__n64 _Dn, __n64 _Dm);
__n64 __vmul_s8(__n64 _Dn, __n64 _Dm);
__n64 __vmul_u16(__n64 _Dn, __n64 _Dm);
__n64 __vmul_u32(__n64 _Dn, __n64 _Dm);
__n64 __vmul_u8(__n64 _Dn, __n64 _Dm);
__n128 __vmulq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vmulq_p8(__n128 _Qn, __n128 _Qm);
__n128 __vmulq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vmulq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vmulq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vmulq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vmulq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vmulq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vmul_n_f32(__n64 _Dn, float32_t _Ft);
__n128 __vmulq_n_f32(__n128 _Qn, float32_t _Ft);
__n64 __vmul_lane_f32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmul_lane_s16(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmul_lane_s32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmul_lane_u16(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmul_lane_u32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmulq_lane_f32(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmulq_lane_s16(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmulq_lane_s32(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmulq_lane_u16(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmulq_lane_u32(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vmull_p64(__n64 _Dn, __n64 _Dm);
__n128 __vmull_p8(__n64 _Dn, __n64 _Dm);
__n128 __vmull_s16(__n64 _Dn, __n64 _Dm);
__n128 __vmull_s32(__n64 _Dn, __n64 _Dm);
__n128 __vmull_s8(__n64 _Dn, __n64 _Dm);
__n128 __vmull_u16(__n64 _Dn, __n64 _Dm);
__n128 __vmull_u32(__n64 _Dn, __n64 _Dm);
__n128 __vmull_u8(__n64 _Dn, __n64 _Dm);
__n128 __vmull_lane_s16(__n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmull_lane_s32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmull_lane_u16(__n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vmull_lane_u32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vmvn_p16(__n64 _Dm);
__n64 __vmvn_p8(__n64 _Dm);
__n64 __vmvn_s16(__n64 _Dm);
__n64 __vmvn_s32(__n64 _Dm);
__n64 __vmvn_s8(__n64 _Dm);
__n64 __vmvn_u16(__n64 _Dm);
__n64 __vmvn_u32(__n64 _Dm);
__n64 __vmvn_u8(__n64 _Dm);
__n128 __vmvnq_p16(__n128 _Qm);
__n128 __vmvnq_p8(__n128 _Qm);
__n128 __vmvnq_s16(__n128 _Qm);
__n128 __vmvnq_s32(__n128 _Qm);
__n128 __vmvnq_s8(__n128 _Qm);
__n128 __vmvnq_u16(__n128 _Qm);
__n128 __vmvnq_u32(__n128 _Qm);
__n128 __vmvnq_u8(__n128 _Qm);
__n64 __vpadal_s16(__n64 _Dd, __n64 _Dm);
__n64 __vpadal_s32(__n64 _Dd, __n64 _Dm);
__n64 __vpadal_s8(__n64 _Dd, __n64 _Dm);
__n64 __vpadal_u16(__n64 _Dd, __n64 _Dm);
__n64 __vpadal_u32(__n64 _Dd, __n64 _Dm);
__n64 __vpadal_u8(__n64 _Dd, __n64 _Dm);
__n128 __vpadalq_s16(__n128 _Qd, __n128 _Qm);
__n128 __vpadalq_s32(__n128 _Qd, __n128 _Qm);
__n128 __vpadalq_s8(__n128 _Qd, __n128 _Qm);
__n128 __vpadalq_u16(__n128 _Qd, __n128 _Qm);
__n128 __vpadalq_u32(__n128 _Qd, __n128 _Qm);
__n128 __vpadalq_u8(__n128 _Qd, __n128 _Qm);
__n64 __vpadd_f32(__n64 _Dn, __n64 _Dm);
__n64 __vpadd_s16(__n64 _Dn, __n64 _Dm);
__n64 __vpadd_s32(__n64 _Dn, __n64 _Dm);
__n64 __vpadd_s8(__n64 _Dn, __n64 _Dm);
__n64 __vpadd_u16(__n64 _Dn, __n64 _Dm);
__n64 __vpadd_u32(__n64 _Dn, __n64 _Dm);
__n64 __vpadd_u8(__n64 _Dn, __n64 _Dm);
__n64 __vpaddl_s16(__n64 _Dm);
__n64 __vpaddl_s32(__n64 _Dm);
__n64 __vpaddl_s8(__n64 _Dm);
__n64 __vpaddl_u16(__n64 _Dm);
__n64 __vpaddl_u32(__n64 _Dm);
__n64 __vpaddl_u8(__n64 _Dm);
__n128 __vpaddlq_s16(__n128 _Qm);
__n128 __vpaddlq_s32(__n128 _Qm);
__n128 __vpaddlq_s8(__n128 _Qm);
__n128 __vpaddlq_u16(__n128 _Qm);
__n128 __vpaddlq_u32(__n128 _Qm);
__n128 __vpaddlq_u8(__n128 _Qm);
__n64 __vpmax_f32(__n64 _Dn, __n64 _Dm);
__n64 __vpmin_f32(__n64 _Dn, __n64 _Dm);
__n64 __vpmax_s16(__n64 _Dn, __n64 _Dm);
__n64 __vpmax_s32(__n64 _Dn, __n64 _Dm);
__n64 __vpmax_s8(__n64 _Dn, __n64 _Dm);
__n64 __vpmax_u16(__n64 _Dn, __n64 _Dm);
__n64 __vpmax_u32(__n64 _Dn, __n64 _Dm);
__n64 __vpmax_u8(__n64 _Dn, __n64 _Dm);
__n64 __vpmin_s16(__n64 _Dn, __n64 _Dm);
__n64 __vpmin_s32(__n64 _Dn, __n64 _Dm);
__n64 __vpmin_s8(__n64 _Dn, __n64 _Dm);
__n64 __vpmin_u16(__n64 _Dn, __n64 _Dm);
__n64 __vpmin_u32(__n64 _Dn, __n64 _Dm);
__n64 __vpmin_u8(__n64 _Dn, __n64 _Dm);
__n64 __vqabs_s16(__n64 _Dm);
__n64 __vqabs_s32(__n64 _Dm);
__n64 __vqabs_s8(__n64 _Dm);
__n64 __vqneg_s16(__n64 _Dm);
__n64 __vqneg_s32(__n64 _Dm);
__n64 __vqneg_s8(__n64 _Dm);
__n128 __vqabsq_s16(__n128 _Qm);
__n128 __vqabsq_s32(__n128 _Qm);
__n128 __vqabsq_s8(__n128 _Qm);
__n128 __vqnegq_s16(__n128 _Qm);
__n128 __vqnegq_s32(__n128 _Qm);
__n128 __vqnegq_s8(__n128 _Qm);
__n64 __vqadd_s16(__n64 _Dn, __n64 _Dm);
__n64 __vqadd_s32(__n64 _Dn, __n64 _Dm);
__n64 __vqadd_s64(__n64 _Dn, __n64 _Dm);
__n64 __vqadd_s8(__n64 _Dn, __n64 _Dm);
__n64 __vqadd_u16(__n64 _Dn, __n64 _Dm);
__n64 __vqadd_u32(__n64 _Dn, __n64 _Dm);
__n64 __vqadd_u64(__n64 _Dn, __n64 _Dm);
__n64 __vqadd_u8(__n64 _Dn, __n64 _Dm);
__n128 __vqaddq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vqaddq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vqaddq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vqaddq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vqaddq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vqaddq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vqaddq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vqaddq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vqdmlal_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vqdmlal_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vqdmlsl_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vqdmlsl_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm);
__n128 __vqdmlal_lane_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vqdmlal_lane_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vqdmlsl_lane_s16(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vqdmlsl_lane_s32(__n128 _Qd, __n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vqdmulh_lane_s16(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vqdmulh_lane_s32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vqrdmulh_lane_s16(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vqrdmulh_lane_s32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vqdmulhq_lane_s16(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vqdmulhq_lane_s32(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vqrdmulhq_lane_s16(__n128 _Qn, __n64 _Dm, const int _Lane);
__n128 __vqrdmulhq_lane_s32(__n128 _Qn, __n64 _Dm, const int _Lane);
__n64 __vqdmulh_s16(__n64 _Dn, __n64 _Dm);
__n64 __vqdmulh_s32(__n64 _Dn, __n64 _Dm);
__n64 __vqrdmulh_s16(__n64 _Dn, __n64 _Dm);
__n64 __vqrdmulh_s32(__n64 _Dn, __n64 _Dm);
__n128 __vqdmulhq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vqdmulhq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vqrdmulhq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vqrdmulhq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vqdmull_s16(__n64 _Dn, __n64 _Dm);
__n128 __vqdmull_s32(__n64 _Dn, __n64 _Dm);
__n128 __vqdmull_lane_s16(__n64 _Dn, __n64 _Dm, const int _Lane);
__n128 __vqdmull_lane_s32(__n64 _Dn, __n64 _Dm, const int _Lane);
__n64 __vqmovn_s16(__n128 _Qm);
__n64 __vqmovn_s32(__n128 _Qm);
__n64 __vqmovn_s64(__n128 _Qm);
__n64 __vqmovn_u16(__n128 _Qm);
__n64 __vqmovn_u32(__n128 _Qm);
__n64 __vqmovn_u64(__n128 _Qm);
__n64 __vqmovun_s16(__n128 _Qm);
__n64 __vqmovun_s32(__n128 _Qm);
__n64 __vqmovun_s64(__n128 _Qm);
__n64 __vqshl_n_s16(__n64 _Dm, const int _Shift_amount);
__n64 __vqshl_n_s32(__n64 _Dm, const int _Shift_amount);
__n64 __vqshl_n_s64(__n64 _Dm, const int _Shift_amount);
__n64 __vqshl_n_s8(__n64 _Dm, const int _Shift_amount);
__n64 __vqshl_n_u16(__n64 _Dm, const int _Shift_amount);
__n64 __vqshl_n_u32(__n64 _Dm, const int _Shift_amount);
__n64 __vqshl_n_u64(__n64 _Dm, const int _Shift_amount);
__n64 __vqshl_n_u8(__n64 _Dm, const int _Shift_amount);
__n64 __vqshlu_n_s16(__n64 _Dm, const int _Shift_amount);
__n64 __vqshlu_n_s32(__n64 _Dm, const int _Shift_amount);
__n64 __vqshlu_n_s64(__n64 _Dm, const int _Shift_amount);
__n64 __vqshlu_n_s8(__n64 _Dm, const int _Shift_amount);
__n128 __vqshlq_n_s16(__n128 _Qm, const int _Shift_amount);
__n128 __vqshlq_n_s32(__n128 _Qm, const int _Shift_amount);
__n128 __vqshlq_n_s64(__n128 _Qm, const int _Shift_amount);
__n128 __vqshlq_n_s8(__n128 _Qm, const int _Shift_amount);
__n128 __vqshlq_n_u16(__n128 _Qm, const int _Shift_amount);
__n128 __vqshlq_n_u32(__n128 _Qm, const int _Shift_amount);
__n128 __vqshlq_n_u64(__n128 _Qm, const int _Shift_amount);
__n128 __vqshlq_n_u8(__n128 _Qm, const int _Shift_amount);
__n128 __vqshluq_n_s16(__n128 _Qm, const int _Shift_amount);
__n128 __vqshluq_n_s32(__n128 _Qm, const int _Shift_amount);
__n128 __vqshluq_n_s64(__n128 _Qm, const int _Shift_amount);
__n128 __vqshluq_n_s8(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrn_n_s16(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrn_n_s32(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrn_n_s64(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrn_n_u16(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrn_n_u32(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrn_n_u64(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrun_n_s16(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrun_n_s32(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshrun_n_s64(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrn_n_s16(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrn_n_s32(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrn_n_s64(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrn_n_u16(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrn_n_u32(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrn_n_u64(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrun_n_s16(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrun_n_s32(__n128 _Qm, const int _Shift_amount);
__n64 __vqshrun_n_s64(__n128 _Qm, const int _Shift_amount);
__n64 __vqsub_s16(__n64 _Dn, __n64 _Dm);
__n64 __vqsub_s32(__n64 _Dn, __n64 _Dm);
__n64 __vqsub_s64(__n64 _Dn, __n64 _Dm);
__n64 __vqsub_s8(__n64 _Dn, __n64 _Dm);
__n64 __vqsub_u16(__n64 _Dn, __n64 _Dm);
__n64 __vqsub_u32(__n64 _Dn, __n64 _Dm);
__n64 __vqsub_u64(__n64 _Dn, __n64 _Dm);
__n64 __vqsub_u8(__n64 _Dn, __n64 _Dm);
__n128 __vqsubq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vqsubq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vqsubq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vqsubq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vqsubq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vqsubq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vqsubq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vqsubq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vrecpe_f32(__n64 _Dm);
__n64 __vrecpe_u32(__n64 _Dm);
__n64 __vrsqrte_f32(__n64 _Dm);
__n64 __vrsqrte_u32(__n64 _Dm);
__n128 __vrecpeq_f32(__n128 _Qm);
__n128 __vrecpeq_u32(__n128 _Qm);
__n128 __vrsqrteq_f32(__n128 _Qm);
__n128 __vrsqrteq_u32(__n128 _Qm);
__n64 __vrecps_f32(__n64 _Dn, __n64 _Dm);
__n128 __vrecpsq_f32(__n128 _Qn, __n128 _Qm);
__n64 __vrev16_p8(__n64 _Dm);
__n64 __vrev16_s8(__n64 _Dm);
__n64 __vrev16_u8(__n64 _Dm);
__n64 __vrev32_p16(__n64 _Dm);
__n64 __vrev32_p8(__n64 _Dm);
__n64 __vrev32_s16(__n64 _Dm);
__n64 __vrev32_s8(__n64 _Dm);
__n64 __vrev32_u16(__n64 _Dm);
__n64 __vrev32_u8(__n64 _Dm);
__n64 __vrev64_f32(__n64 _Dm);
__n64 __vrev64_p16(__n64 _Dm);
__n64 __vrev64_p8(__n64 _Dm);
__n64 __vrev64_s16(__n64 _Dm);
__n64 __vrev64_s32(__n64 _Dm);
__n64 __vrev64_s8(__n64 _Dm);
__n64 __vrev64_u16(__n64 _Dm);
__n64 __vrev64_u32(__n64 _Dm);
__n64 __vrev64_u8(__n64 _Dm);
__n128 __vrev16q_p8(__n128 _Qm);
__n128 __vrev16q_s8(__n128 _Qm);
__n128 __vrev16q_u8(__n128 _Qm);
__n128 __vrev32q_p16(__n128 _Qm);
__n128 __vrev32q_p8(__n128 _Qm);
__n128 __vrev32q_s16(__n128 _Qm);
__n128 __vrev32q_s8(__n128 _Qm);
__n128 __vrev32q_u16(__n128 _Qm);
__n128 __vrev32q_u8(__n128 _Qm);
__n128 __vrev64q_f32(__n128 _Qm);
__n128 __vrev64q_p16(__n128 _Qm);
__n128 __vrev64q_p8(__n128 _Qm);
__n128 __vrev64q_s16(__n128 _Qm);
__n128 __vrev64q_s32(__n128 _Qm);
__n128 __vrev64q_s8(__n128 _Qm);
__n128 __vrev64q_u16(__n128 _Qm);
__n128 __vrev64q_u32(__n128 _Qm);
__n128 __vrev64q_u8(__n128 _Qm);
__n64 __vrnd_f32(__n64 _Dm);
__n64 __vrnda_f32(__n64 _Dm);
__n64 __vrndm_f32(__n64 _Dm);
__n64 __vrndn_f32(__n64 _Dm);
__n64 __vrndp_f32(__n64 _Dm);
__n64 __vrndx_f32(__n64 _Dm);
__n128 __vrndq_f32(__n128 _Qm);
__n128 __vrndaq_f32(__n128 _Qm);
__n128 __vrndmq_f32(__n128 _Qm);
__n128 __vrndnq_f32(__n128 _Qm);
__n128 __vrndpq_f32(__n128 _Qm);
__n128 __vrndxq_f32(__n128 _Qm);
__n64 __vrsqrts_f32(__n64 _Dn, __n64 _Dm);
__n128 __vrsqrtsq_f32(__n128 _Qn, __n128 _Qm);
__n64 __vshl_n_s16(__n64 _Dm, const int _Shift_amount);
__n64 __vshl_n_s32(__n64 _Dm, const int _Shift_amount);
__n64 __vshl_n_s64(__n64 _Dm, const int _Shift_amount);
__n64 __vshl_n_s8(__n64 _Dm, const int _Shift_amount);
__n64 __vshl_n_u16(__n64 _Dm, const int _Shift_amount);
__n64 __vshl_n_u32(__n64 _Dm, const int _Shift_amount);
__n64 __vshl_n_u64(__n64 _Dm, const int _Shift_amount);
__n64 __vshl_n_u8(__n64 _Dm, const int _Shift_amount);
__n128 __vshlq_n_s16(__n128 _Qm, const int _Shift_amount);
__n128 __vshlq_n_s32(__n128 _Qm, const int _Shift_amount);
__n128 __vshlq_n_s64(__n128 _Qm, const int _Shift_amount);
__n128 __vshlq_n_s8(__n128 _Qm, const int _Shift_amount);
__n128 __vshlq_n_u16(__n128 _Qm, const int _Shift_amount);
__n128 __vshlq_n_u32(__n128 _Qm, const int _Shift_amount);
__n128 __vshlq_n_u64(__n128 _Qm, const int _Shift_amount);
__n128 __vshlq_n_u8(__n128 _Qm, const int _Shift_amount);
__n64 __vqrshl_s16(__n64 _Dn, __n64 _Dm);
__n64 __vqrshl_s32(__n64 _Dn, __n64 _Dm);
__n64 __vqrshl_s64(__n64 _Dn, __n64 _Dm);
__n64 __vqrshl_s8(__n64 _Dn, __n64 _Dm);
__n64 __vqrshl_u16(__n64 _Dn, __n64 _Dm);
__n64 __vqrshl_u32(__n64 _Dn, __n64 _Dm);
__n64 __vqrshl_u64(__n64 _Dn, __n64 _Dm);
__n64 __vqrshl_u8(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_s16(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_s32(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_s64(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_s8(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_u16(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_u32(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_u64(__n64 _Dn, __n64 _Dm);
__n64 __vqshl_u8(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_s16(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_s32(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_s64(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_s8(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_u16(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_u32(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_u64(__n64 _Dn, __n64 _Dm);
__n64 __vrshl_u8(__n64 _Dn, __n64 _Dm);
__n64 __vshl_s16(__n64 _Dn, __n64 _Dm);
__n64 __vshl_s32(__n64 _Dn, __n64 _Dm);
__n64 __vshl_s64(__n64 _Dn, __n64 _Dm);
__n64 __vshl_s8(__n64 _Dn, __n64 _Dm);
__n64 __vshl_u16(__n64 _Dn, __n64 _Dm);
__n64 __vshl_u32(__n64 _Dn, __n64 _Dm);
__n64 __vshl_u64(__n64 _Dn, __n64 _Dm);
__n64 __vshl_u8(__n64 _Dn, __n64 _Dm);
__n128 __vqrshlq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vqrshlq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vqrshlq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vqrshlq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vqrshlq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vqrshlq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vqrshlq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vqrshlq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vqshlq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vrshlq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vshlq_u8(__n128 _Qn, __n128 _Qm);
__n128 __vshll_n_s16(__n64 _Dm, const int _Shift_amount);
__n128 __vshll_n_s32(__n64 _Dm, const int _Shift_amount);
__n128 __vshll_n_s8(__n64 _Dm, const int _Shift_amount);
__n128 __vshll_n_u16(__n64 _Dm, const int _Shift_amount);
__n128 __vshll_n_u32(__n64 _Dm, const int _Shift_amount);
__n128 __vshll_n_u8(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_s16(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_s32(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_s64(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_s8(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_u16(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_u32(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_u64(__n64 _Dm, const int _Shift_amount);
__n64 __vrshr_n_u8(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_s16(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_s32(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_s64(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_s8(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_u16(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_u32(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_u64(__n64 _Dm, const int _Shift_amount);
__n64 __vshr_n_u8(__n64 _Dm, const int _Shift_amount);
__n128 __vrshrq_n_s16(__n128 _Qm, const int _Shift_amount);
__n128 __vrshrq_n_s32(__n128 _Qm, const int _Shift_amount);
__n128 __vrshrq_n_s64(__n128 _Qm, const int _Shift_amount);
__n128 __vrshrq_n_s8(__n128 _Qm, const int _Shift_amount);
__n128 __vrshrq_n_u16(__n128 _Qm, const int _Shift_amount);
__n128 __vrshrq_n_u32(__n128 _Qm, const int _Shift_amount);
__n128 __vrshrq_n_u64(__n128 _Qm, const int _Shift_amount);
__n128 __vrshrq_n_u8(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_s16(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_s32(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_s64(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_s8(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_u16(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_u32(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_u64(__n128 _Qm, const int _Shift_amount);
__n128 __vshrq_n_u8(__n128 _Qm, const int _Shift_amount);
__n64 __vrshrn_n_s16(__n128 _Qm, const int _Shift_amount);
__n64 __vrshrn_n_s32(__n128 _Qm, const int _Shift_amount);
__n64 __vrshrn_n_s64(__n128 _Qm, const int _Shift_amount);
__n64 __vrshrn_n_u16(__n128 _Qm, const int _Shift_amount);
__n64 __vrshrn_n_u32(__n128 _Qm, const int _Shift_amount);
__n64 __vrshrn_n_u64(__n128 _Qm, const int _Shift_amount);
__n64 __vshrn_n_s16(__n128 _Qm, const int _Shift_amount);
__n64 __vshrn_n_s32(__n128 _Qm, const int _Shift_amount);
__n64 __vshrn_n_s64(__n128 _Qm, const int _Shift_amount);
__n64 __vshrn_n_u16(__n128 _Qm, const int _Shift_amount);
__n64 __vshrn_n_u32(__n128 _Qm, const int _Shift_amount);
__n64 __vshrn_n_u64(__n128 _Qm, const int _Shift_amount);
__n64 __vsli_n_p16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_p8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_s16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_s32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_s64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_s8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_u16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_u32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_u64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsli_n_u8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n128 __vsliq_n_p16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_p8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_s16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_s32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_s64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_s8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_u16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_u32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_u64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsliq_n_u8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n64 __vrsra_n_s16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vrsra_n_s32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vrsra_n_s64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vrsra_n_s8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vrsra_n_u16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vrsra_n_u32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vrsra_n_u64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vrsra_n_u8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_s16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_s32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_s64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_s8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_u16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_u32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_u64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsra_n_u8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n128 __vrsraq_n_s16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vrsraq_n_s32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vrsraq_n_s64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vrsraq_n_s8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vrsraq_n_u16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vrsraq_n_u32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vrsraq_n_u64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vrsraq_n_u8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_s16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_s32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_s64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_s8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_u16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_u32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_u64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsraq_n_u8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n64 __vsri_n_p16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_p8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_s16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_s32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_s64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_s8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_u16(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_u32(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_u64(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n64 __vsri_n_u8(__n64 _Dd, __n64 _Dm, const int _Shift_amount);
__n128 __vsriq_n_p16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_p8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_s16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_s32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_s64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_s8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_u16(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_u32(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_u64(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
__n128 __vsriq_n_u8(__n128 _Qd, __n128 _Qm, const int _Shift_amount);
void __vst1_f32(_Out_writes_(2) float32_t * _PD, __n64 _D);
void __vst1_p16(_Out_writes_(4) poly16_t * _PD, __n64 _D);
void __vst1_p8(_Out_writes_(8) poly8_t * _PD, __n64 _D);
void __vst1_s16(_Out_writes_(4) int16_t * _PD, __n64 _D);
void __vst1_s32(_Out_writes_(2) int32_t * _PD, __n64 _D);
void __vst1_s64(_Out_writes_(1) int64_t * _PD, __n64 _D);
void __vst1_s8(_Out_writes_(8) int8_t * _PD, __n64 _D);
void __vst1_u16(_Out_writes_(4) uint16_t * _PD, __n64 _D);
void __vst1_u32(_Out_writes_(2) uint32_t * _PD, __n64 _D);
void __vst1_u64(_Out_writes_(1) uint64_t * _PD, __n64 _D);
void __vst1_u8(_Out_writes_(8) uint8_t * _PD, __n64 _D);
void __vst1_f32_ex(_Out_writes_(2) float32_t * _PD, __n64 _D, const int _Align);
void __vst1_p16_ex(_Out_writes_(4) poly16_t * _PD, __n64 _D, const int _Align);
void __vst1_p8_ex(_Out_writes_(8) poly8_t * _PD, __n64 _D, const int _Align);
void __vst1_s16_ex(_Out_writes_(4) int16_t * _PD, __n64 _D, const int _Align);
void __vst1_s32_ex(_Out_writes_(2) int32_t * _PD, __n64 _D, const int _Align);
void __vst1_s64_ex(_Out_writes_(1) int64_t * _PD, __n64 _D, const int _Align);
void __vst1_s8_ex(_Out_writes_(8) int8_t * _PD, __n64 _D, const int _Align);
void __vst1_u16_ex(_Out_writes_(4) uint16_t * _PD, __n64 _D, const int _Align);
void __vst1_u32_ex(_Out_writes_(2) uint32_t * _PD, __n64 _D, const int _Align);
void __vst1_u64_ex(_Out_writes_(1) uint64_t * _PD, __n64 _D, const int _Align);
void __vst1_u8_ex(_Out_writes_(8) uint8_t * _PD, __n64 _D, const int _Align);
void __vst1q_f32(_Out_writes_(4) float32_t * _PD, __n128 _Q);
void __vst1q_p16(_Out_writes_(8) poly16_t * _PD, __n128 _Q);
void __vst1q_p8(_Out_writes_(16) poly8_t * _PD, __n128 _Q);
void __vst1q_s16(_Out_writes_(8) int16_t * _PD, __n128 _Q);
void __vst1q_s32(_Out_writes_(4) int32_t * _PD, __n128 _Q);
void __vst1q_s64(_Out_writes_(2) int64_t * _PD, __n128 _Q);
void __vst1q_s8(_Out_writes_(16) int8_t * _PD, __n128 _Q);
void __vst1q_u16(_Out_writes_(8) uint16_t * _PD, __n128 _Q);
void __vst1q_u32(_Out_writes_(4) uint32_t * _PD, __n128 _Q);
void __vst1q_u64(_Out_writes_(2) uint64_t * _PD, __n128 _Q);
void __vst1q_u8(_Out_writes_(16) uint8_t * _PD, __n128 _Q);
void __vst1q_f32_ex(_Out_writes_(4) float32_t * _PD, __n128 _Q, const int _Align);
void __vst1q_p16_ex(_Out_writes_(8) poly16_t * _PD, __n128 _Q, const int _Align);
void __vst1q_p8_ex(_Out_writes_(16) poly8_t * _PD, __n128 _Q, const int _Align);
void __vst1q_s16_ex(_Out_writes_(8) int16_t * _PD, __n128 _Q, const int _Align);
void __vst1q_s32_ex(_Out_writes_(4) int32_t * _PD, __n128 _Q, const int _Align);
void __vst1q_s64_ex(_Out_writes_(2) int64_t * _PD, __n128 _Q, const int _Align);
void __vst1q_s8_ex(_Out_writes_(16) int8_t * _PD, __n128 _Q, const int _Align);
void __vst1q_u16_ex(_Out_writes_(8) uint16_t * _PD, __n128 _Q, const int _Align);
void __vst1q_u32_ex(_Out_writes_(4) uint32_t * _PD, __n128 _Q, const int _Align);
void __vst1q_u64_ex(_Out_writes_(2) uint64_t * _PD, __n128 _Q, const int _Align);
void __vst1q_u8_ex(_Out_writes_(16) uint8_t * _PD, __n128 _Q, const int _Align);
void __vst1_lane_f32(_Out_writes_(1) float32_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_p16(_Out_writes_(1) poly16_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_p8(_Out_writes_(1) poly8_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_s16(_Out_writes_(1) int16_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_s32(_Out_writes_(1) int32_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_s8(_Out_writes_(1) int8_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_u16(_Out_writes_(1) uint16_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_u32(_Out_writes_(1) uint32_t * _PD, __n64 _D, const int _Lane);
void __vst1_lane_u8(_Out_writes_(1) uint8_t * _PD, __n64 _D, const int _Lane);
void __vst1q_lane_f32(_Out_writes_(1) float32_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_p16(_Out_writes_(1) poly16_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_p8(_Out_writes_(1) poly8_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_s16(_Out_writes_(1) int16_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_s32(_Out_writes_(1) int32_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_s8(_Out_writes_(1) int8_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_u16(_Out_writes_(1) uint16_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_u32(_Out_writes_(1) uint32_t * _PD, __n128 _Q, const int _Lane);
void __vst1q_lane_u8(_Out_writes_(1) uint8_t * _PD, __n128 _Q, const int _Lane);
void __vst1_lane_f32_ex(_Out_writes_(1) float32_t * _PD, __n64 _D, const int _Lane, const int _Align);
void __vst1_lane_p16_ex(_Out_writes_(1) poly16_t * _PD, __n64 _D, const int _Lane, const int _Align);
void __vst1_lane_s16_ex(_Out_writes_(1) int16_t * _PD, __n64 _D, const int _Lane, const int _Align);
void __vst1_lane_s32_ex(_Out_writes_(1) int32_t * _PD, __n64 _D, const int _Lane, const int _Align);
void __vst1_lane_u16_ex(_Out_writes_(1) uint16_t * _PD, __n64 _D, const int _Lane, const int _Align);
void __vst1_lane_u32_ex(_Out_writes_(1) uint32_t * _PD, __n64 _D, const int _Lane, const int _Align);
void __vst1q_lane_f32_ex(_Out_writes_(1) float32_t * _PD, __n128 _Q, const int _Lane, const int _Align);
void __vst1q_lane_p16_ex(_Out_writes_(1) poly16_t * _PD, __n128 _Q, const int _Lane, const int _Align);
void __vst1q_lane_s16_ex(_Out_writes_(1) int16_t * _PD, __n128 _Q, const int _Lane, const int _Align);
void __vst1q_lane_s32_ex(_Out_writes_(1) int32_t * _PD, __n128 _Q, const int _Lane, const int _Align);
void __vst1q_lane_u16_ex(_Out_writes_(1) uint16_t * _PD, __n128 _Q, const int _Lane, const int _Align);
void __vst1q_lane_u32_ex(_Out_writes_(1) uint32_t * _PD, __n128 _Q, const int _Lane, const int _Align);
void __vst2_f32(_Out_writes_(4) float32_t * _PD, __n64x2 _D2);
void __vst2_p16(_Out_writes_(8) poly16_t * _PD, __n64x2 _D2);
void __vst2_p8(_Out_writes_(16) poly8_t * _PD, __n64x2 _D2);
void __vst2_s16(_Out_writes_(8) int16_t * _PD, __n64x2 _D2);
void __vst2_s32(_Out_writes_(4) int32_t * _PD, __n64x2 _D2);
void __vst2_s8(_Out_writes_(16) int8_t * _PD, __n64x2 _D2);
void __vst2_u16(_Out_writes_(8) uint16_t * _PD, __n64x2 _D2);
void __vst2_u32(_Out_writes_(4) uint32_t * _PD, __n64x2 _D2);
void __vst2_u8(_Out_writes_(16) uint8_t * _PD, __n64x2 _D2);
void __vst2_s64(_Out_writes_(2) int64_t * _PD, __n64x2 _D2);
void __vst2_u64(_Out_writes_(2) uint64_t * _PD, __n64x2 _D2);
void __vst2_s64_ex(_Out_writes_(2) int64_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_u64_ex(_Out_writes_(2) uint64_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_f32_ex(_Out_writes_(4) float32_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_p16_ex(_Out_writes_(8) poly16_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_p8_ex(_Out_writes_(16) poly8_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_s16_ex(_Out_writes_(8) int16_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_s32_ex(_Out_writes_(4) int32_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_s8_ex(_Out_writes_(16) int8_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_u16_ex(_Out_writes_(8) uint16_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_u32_ex(_Out_writes_(4) uint32_t * _PD, __n64x2 _D2, const int _Align);
void __vst2_u8_ex(_Out_writes_(16) uint8_t * _PD, __n64x2 _D2, const int _Align);
void __vst2q_f32(_Out_writes_(8) float32_t * _PD, __n128x2 _Q2);
void __vst2q_p16(_Out_writes_(16) poly16_t * _PD, __n128x2 _Q2);
void __vst2q_p8(_Out_writes_(32) poly8_t * _PD, __n128x2 _Q2);
void __vst2q_s16(_Out_writes_(16) int16_t * _PD, __n128x2 _Q2);
void __vst2q_s32(_Out_writes_(8) int32_t * _PD, __n128x2 _Q2);
void __vst2q_s8(_Out_writes_(32) int8_t * _PD, __n128x2 _Q2);
void __vst2q_u16(_Out_writes_(16) uint16_t * _PD, __n128x2 _Q2);
void __vst2q_u32(_Out_writes_(8) uint32_t * _PD, __n128x2 _Q2);
void __vst2q_u8(_Out_writes_(32) uint8_t * _PD, __n128x2 _Q2);
void __vst2q_f32_ex(_Out_writes_(8) float32_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_p16_ex(_Out_writes_(16) poly16_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_p8_ex(_Out_writes_(32) poly8_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_s16_ex(_Out_writes_(16) int16_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_s32_ex(_Out_writes_(8) int32_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_s8_ex(_Out_writes_(32) int8_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_u16_ex(_Out_writes_(16) uint16_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_u32_ex(_Out_writes_(8) uint32_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2q_u8_ex(_Out_writes_(32) uint8_t * _PD, __n128x2 _Q2, const int _Align);
void __vst2_lane_f32(_Out_writes_(2) float32_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_p16(_Out_writes_(2) poly16_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_p8(_Out_writes_(2) poly8_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_s16(_Out_writes_(2) int16_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_s32(_Out_writes_(2) int32_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_s8(_Out_writes_(2) int8_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_u16(_Out_writes_(2) uint16_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_u32(_Out_writes_(2) uint32_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2_lane_u8(_Out_writes_(2) uint8_t * _PD, __n64x2 _D2, const int _Lane);
void __vst2q_lane_f32(_Out_writes_(2) float32_t * _PD, __n128x2 _Q2, const int _Lane);
void __vst2q_lane_p16(_Out_writes_(2) poly16_t * _PD, __n128x2 _Q2, const int _Lane);
void __vst2q_lane_s16(_Out_writes_(2) int16_t * _PD, __n128x2 _Q2, const int _Lane);
void __vst2q_lane_s32(_Out_writes_(2) int32_t * _PD, __n128x2 _Q2, const int _Lane);
void __vst2q_lane_u16(_Out_writes_(2) uint16_t * _PD, __n128x2 _Q2, const int _Lane);
void __vst2q_lane_u32(_Out_writes_(2) uint32_t * _PD, __n128x2 _Q2, const int _Lane);
void __vst2_lane_f32_ex(_Out_writes_(2) float32_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_p16_ex(_Out_writes_(2) poly16_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_p8_ex(_Out_writes_(2) poly8_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_s16_ex(_Out_writes_(2) int16_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_s32_ex(_Out_writes_(2) int32_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_s8_ex(_Out_writes_(2) int8_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_u16_ex(_Out_writes_(2) uint16_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_u32_ex(_Out_writes_(2) uint32_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2_lane_u8_ex(_Out_writes_(2) uint8_t * _PD, __n64x2 _D2, const int _Lane, const int _Align);
void __vst2q_lane_f32_ex(_Out_writes_(2) float32_t * _PD, __n128x2 _Q2, const int _Lane, const int _Align);
void __vst2q_lane_p16_ex(_Out_writes_(2) poly16_t * _PD, __n128x2 _Q2, const int _Lane, const int _Align);
void __vst2q_lane_s16_ex(_Out_writes_(2) int16_t * _PD, __n128x2 _Q2, const int _Lane, const int _Align);
void __vst2q_lane_s32_ex(_Out_writes_(2) int32_t * _PD, __n128x2 _Q2, const int _Lane, const int _Align);
void __vst2q_lane_u16_ex(_Out_writes_(2) uint16_t * _PD, __n128x2 _Q2, const int _Lane, const int _Align);
void __vst2q_lane_u32_ex(_Out_writes_(2) uint32_t * _PD, __n128x2 _Q2, const int _Lane, const int _Align);
void __vst3_f32(_Out_writes_(6) float32_t * _PD, __n64x3 _D3);
void __vst3_p16(_Out_writes_(12) poly16_t * _PD, __n64x3 _D3);
void __vst3_p8(_Out_writes_(24) poly8_t * _PD, __n64x3 _D3);
void __vst3_s16(_Out_writes_(12) int16_t * _PD, __n64x3 _D3);
void __vst3_s32(_Out_writes_(6) int32_t * _PD, __n64x3 _D3);
void __vst3_s8(_Out_writes_(24) int8_t * _PD, __n64x3 _D3);
void __vst3_u16(_Out_writes_(12) uint16_t * _PD, __n64x3 _D3);
void __vst3_u32(_Out_writes_(6) uint32_t * _PD, __n64x3 _D3);
void __vst3_u8(_Out_writes_(24) uint8_t * _PD, __n64x3 _D3);
void __vst3_s64(_Out_writes_(3) int64_t * _PD, __n64x3 _D3);
void __vst3_u64(_Out_writes_(3) uint64_t * _PD, __n64x3 _D3);
void __vst3_s64_ex(_Out_writes_(3) int64_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_u64_ex(_Out_writes_(3) uint64_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_f32_ex(_Out_writes_(6) float32_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_p16_ex(_Out_writes_(12) poly16_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_p8_ex(_Out_writes_(24) poly8_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_s16_ex(_Out_writes_(12) int16_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_s32_ex(_Out_writes_(6) int32_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_s8_ex(_Out_writes_(24) int8_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_u16_ex(_Out_writes_(12) uint16_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_u32_ex(_Out_writes_(6) uint32_t * _PD, __n64x3 _D3, const int _Align);
void __vst3_u8_ex(_Out_writes_(24) uint8_t * _PD, __n64x3 _D3, const int _Align);
void __vst3q_f32(_Out_writes_(12) float32_t * _PD, __n128x3 _Q3);
void __vst3q_p16(_Out_writes_(24) poly16_t * _PD, __n128x3 _Q3);
void __vst3q_p8(_Out_writes_(48) poly8_t * _PD, __n128x3 _Q3);
void __vst3q_s16(_Out_writes_(24) int16_t * _PD, __n128x3 _Q3);
void __vst3q_s32(_Out_writes_(12) int32_t * _PD, __n128x3 _Q3);
void __vst3q_s8(_Out_writes_(48) int8_t * _PD, __n128x3 _Q3);
void __vst3q_u16(_Out_writes_(24) uint16_t * _PD, __n128x3 _Q3);
void __vst3q_u32(_Out_writes_(12) uint32_t * _PD, __n128x3 _Q3);
void __vst3q_u8(_Out_writes_(48) uint8_t * _PD, __n128x3 _Q3);
void __vst3q_f32_ex(_Out_writes_(12) float32_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_p16_ex(_Out_writes_(24) poly16_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_p8_ex(_Out_writes_(48) poly8_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_s16_ex(_Out_writes_(24) int16_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_s32_ex(_Out_writes_(12) int32_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_s8_ex(_Out_writes_(48) int8_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_u16_ex(_Out_writes_(24) uint16_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_u32_ex(_Out_writes_(12) uint32_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3q_u8_ex(_Out_writes_(48) uint8_t * _PD, __n128x3 _Q3, const int _Align);
void __vst3_lane_f32(_Out_writes_(3) float32_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_p16(_Out_writes_(3) poly16_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_p8(_Out_writes_(3) poly8_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_s16(_Out_writes_(3) int16_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_s32(_Out_writes_(3) int32_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_s8(_Out_writes_(3) int8_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_u16(_Out_writes_(3) uint16_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_u32(_Out_writes_(3) uint32_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3_lane_u8(_Out_writes_(3) uint8_t * _PD, __n64x3 _D3, const int _Lane);
void __vst3q_lane_f32(_Out_writes_(3) float32_t * _PD, __n128x3 _Q3, const int _Lane);
void __vst3q_lane_p16(_Out_writes_(3) poly16_t * _PD, __n128x3 _Q3, const int _Lane);
void __vst3q_lane_s16(_Out_writes_(3) int16_t * _PD, __n128x3 _Q3, const int _Lane);
void __vst3q_lane_s32(_Out_writes_(3) int32_t * _PD, __n128x3 _Q3, const int _Lane);
void __vst3q_lane_u16(_Out_writes_(3) uint16_t * _PD, __n128x3 _Q3, const int _Lane);
void __vst3q_lane_u32(_Out_writes_(3) uint32_t * _PD, __n128x3 _Q3, const int _Lane);
void __vst4_f32(_Out_writes_(8) float32_t * _PD, __n64x4 _D4);
void __vst4_p16(_Out_writes_(16) poly16_t * _PD, __n64x4 _D4);
void __vst4_p8(_Out_writes_(32) poly8_t * _PD, __n64x4 _D4);
void __vst4_s16(_Out_writes_(16) int16_t * _PD, __n64x4 _D4);
void __vst4_s32(_Out_writes_(8) int32_t * _PD, __n64x4 _D4);
void __vst4_s8(_Out_writes_(32) int8_t * _PD, __n64x4 _D4);
void __vst4_u16(_Out_writes_(16) uint16_t * _PD, __n64x4 _D4);
void __vst4_u32(_Out_writes_(8) uint32_t * _PD, __n64x4 _D4);
void __vst4_u8(_Out_writes_(32) uint8_t * _PD, __n64x4 _D4);
void __vst4_s64(_Out_writes_(4) int64_t * _PD, __n64x4 _D4);
void __vst4_u64(_Out_writes_(4) uint64_t * _PD, __n64x4 _D4);
void __vst4_s64_ex(_Out_writes_(4) int64_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_u64_ex(_Out_writes_(4) uint64_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_f32_ex(_Out_writes_(8) float32_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_p16_ex(_Out_writes_(16) poly16_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_p8_ex(_Out_writes_(32) poly8_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_s16_ex(_Out_writes_(16) int16_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_s32_ex(_Out_writes_(8) int32_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_s8_ex(_Out_writes_(32) int8_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_u16_ex(_Out_writes_(16) uint16_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_u32_ex(_Out_writes_(8) uint32_t * _PD, __n64x4 _D4, const int _Align);
void __vst4_u8_ex(_Out_writes_(32) uint8_t * _PD, __n64x4 _D4, const int _Align);
void __vst4q_f32(_Out_writes_(16) float32_t * _PD, __n128x4 _Q4);
void __vst4q_p16(_Out_writes_(32) poly16_t * _PD, __n128x4 _Q4);
void __vst4q_p8(_Out_writes_(64) poly8_t * _PD, __n128x4 _Q4);
void __vst4q_s16(_Out_writes_(32) int16_t * _PD, __n128x4 _Q4);
void __vst4q_s32(_Out_writes_(16) int32_t * _PD, __n128x4 _Q4);
void __vst4q_s8(_Out_writes_(64) int8_t * _PD, __n128x4 _Q4);
void __vst4q_u16(_Out_writes_(32) uint16_t * _PD, __n128x4 _Q4);
void __vst4q_u32(_Out_writes_(16) uint32_t * _PD, __n128x4 _Q4);
void __vst4q_u8(_Out_writes_(64) uint8_t * _PD, __n128x4 _Q4);
void __vst4q_f32_ex(_Out_writes_(16) float32_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_p16_ex(_Out_writes_(32) poly16_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_p8_ex(_Out_writes_(64) poly8_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_s16_ex(_Out_writes_(32) int16_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_s32_ex(_Out_writes_(16) int32_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_s8_ex(_Out_writes_(64) int8_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_u16_ex(_Out_writes_(32) uint16_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_u32_ex(_Out_writes_(16) uint32_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4q_u8_ex(_Out_writes_(64) uint8_t * _PD, __n128x4 _Q4, const int _Align);
void __vst4_lane_f32(_Out_writes_(4) float32_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_p16(_Out_writes_(4) poly16_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_p8(_Out_writes_(4) poly8_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_s16(_Out_writes_(4) int16_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_s32(_Out_writes_(4) int32_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_s8(_Out_writes_(4) int8_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_u16(_Out_writes_(4) uint16_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_u32(_Out_writes_(4) uint32_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4_lane_u8(_Out_writes_(4) uint8_t * _PD, __n64x4 _D4, const int _Lane);
void __vst4q_lane_f32(_Out_writes_(4) float32_t * _PD, __n128x4 _Q4, const int _Lane);
void __vst4q_lane_p16(_Out_writes_(4) poly16_t * _PD, __n128x4 _Q4, const int _Lane);
void __vst4q_lane_s16(_Out_writes_(4) int16_t * _PD, __n128x4 _Q4, const int _Lane);
void __vst4q_lane_s32(_Out_writes_(4) int32_t * _PD, __n128x4 _Q4, const int _Lane);
void __vst4q_lane_u16(_Out_writes_(4) uint16_t * _PD, __n128x4 _Q4, const int _Lane);
void __vst4q_lane_u32(_Out_writes_(4) uint32_t * _PD, __n128x4 _Q4, const int _Lane);
void __vst4_lane_f32_ex(_Out_writes_(4) float32_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_p16_ex(_Out_writes_(4) poly16_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_p8_ex(_Out_writes_(4) poly8_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_s16_ex(_Out_writes_(4) int16_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_s32_ex(_Out_writes_(4) int32_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_s8_ex(_Out_writes_(4) int8_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_u16_ex(_Out_writes_(4) uint16_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_u32_ex(_Out_writes_(4) uint32_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4_lane_u8_ex(_Out_writes_(4) uint8_t * _PD, __n64x4 _D4, const int _Lane, const int _Align);
void __vst4q_lane_f32_ex(_Out_writes_(4) float32_t * _PD, __n128x4 _Q4, const int _Lane, const int _Align);
void __vst4q_lane_p16_ex(_Out_writes_(4) poly16_t * _PD, __n128x4 _Q4, const int _Lane, const int _Align);
void __vst4q_lane_s16_ex(_Out_writes_(4) int16_t * _PD, __n128x4 _Q4, const int _Lane, const int _Align);
void __vst4q_lane_s32_ex(_Out_writes_(4) int32_t * _PD, __n128x4 _Q4, const int _Lane, const int _Align);
void __vst4q_lane_u16_ex(_Out_writes_(4) uint16_t * _PD, __n128x4 _Q4, const int _Lane, const int _Align);
void __vst4q_lane_u32_ex(_Out_writes_(4) uint32_t * _PD, __n128x4 _Q4, const int _Lane, const int _Align);
__n64 __vsub_f32(__n64 _Dn, __n64 _Dm);
__n64 __vsub_s16(__n64 _Dn, __n64 _Dm);
__n64 __vsub_s32(__n64 _Dn, __n64 _Dm);
__n64 __vsub_s64(__n64 _Dn, __n64 _Dm);
__n64 __vsub_s8(__n64 _Dn, __n64 _Dm);
__n64 __vsub_u16(__n64 _Dn, __n64 _Dm);
__n64 __vsub_u32(__n64 _Dn, __n64 _Dm);
__n64 __vsub_u64(__n64 _Dn, __n64 _Dm);
__n64 __vsub_u8(__n64 _Dn, __n64 _Dm);
__n128 __vsubq_f32(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_s64(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_u64(__n128 _Qn, __n128 _Qm);
__n128 __vsubq_u8(__n128 _Qn, __n128 _Qm);
__n64 __vrsubhn_s16(__n128 _Qn, __n128 _Qm);
__n64 __vrsubhn_s32(__n128 _Qn, __n128 _Qm);
__n64 __vrsubhn_s64(__n128 _Qn, __n128 _Qm);
__n64 __vrsubhn_u16(__n128 _Qn, __n128 _Qm);
__n64 __vrsubhn_u32(__n128 _Qn, __n128 _Qm);
__n64 __vrsubhn_u64(__n128 _Qn, __n128 _Qm);
__n64 __vsubhn_s16(__n128 _Qn, __n128 _Qm);
__n64 __vsubhn_s32(__n128 _Qn, __n128 _Qm);
__n64 __vsubhn_s64(__n128 _Qn, __n128 _Qm);
__n64 __vsubhn_u16(__n128 _Qn, __n128 _Qm);
__n64 __vsubhn_u32(__n128 _Qn, __n128 _Qm);
__n64 __vsubhn_u64(__n128 _Qn, __n128 _Qm);
__n128 __vsubl_s16(__n64 _Dn, __n64 _Dm);
__n128 __vsubl_s32(__n64 _Dn, __n64 _Dm);
__n128 __vsubl_s8(__n64 _Dn, __n64 _Dm);
__n128 __vsubl_u16(__n64 _Dn, __n64 _Dm);
__n128 __vsubl_u32(__n64 _Dn, __n64 _Dm);
__n128 __vsubl_u8(__n64 _Dn, __n64 _Dm);
__n128 __vsubw_s16(__n128 _Qn, __n64 _Dm);
__n128 __vsubw_s32(__n128 _Qn, __n64 _Dm);
__n128 __vsubw_s8(__n128 _Qn, __n64 _Dm);
__n128 __vsubw_u16(__n128 _Qn, __n64 _Dm);
__n128 __vsubw_u32(__n128 _Qn, __n64 _Dm);
__n128 __vsubw_u8(__n128 _Qn, __n64 _Dm);
__n64 __vtbl2_p8(__n64x2 _D2, __n64 _Dm);
__n64 __vtbl2_s8(__n64x2 _D2, __n64 _Dm);
__n64 __vtbl2_u8(__n64x2 _D2, __n64 _Dm);
__n64 __vtbx2_p8(__n64 _Dd, __n64x2 _D2, __n64 _Dm);
__n64 __vtbx2_s8(__n64 _Dd, __n64x2 _D2, __n64 _Dm);
__n64 __vtbx2_u8(__n64 _Dd, __n64x2 _D2, __n64 _Dm);
__n64 __vtbl3_p8(__n64x3 _D3, __n64 _Dm);
__n64 __vtbl3_s8(__n64x3 _D3, __n64 _Dm);
__n64 __vtbl3_u8(__n64x3 _D3, __n64 _Dm);
__n64 __vtbx3_p8(__n64 _Dd, __n64x3 _D3, __n64 _Dm);
__n64 __vtbx3_s8(__n64 _Dd, __n64x3 _D3, __n64 _Dm);
__n64 __vtbx3_u8(__n64 _Dd, __n64x3 _D3, __n64 _Dm);
__n64 __vtbl4_p8(__n64x4 _D4, __n64 _Dm);
__n64 __vtbl4_s8(__n64x4 _D4, __n64 _Dm);
__n64 __vtbl4_u8(__n64x4 _D4, __n64 _Dm);
__n64 __vtbx4_p8(__n64 _Dd, __n64x4 _D4, __n64 _Dm);
__n64 __vtbx4_s8(__n64 _Dd, __n64x4 _D4, __n64 _Dm);
__n64 __vtbx4_u8(__n64 _Dd, __n64x4 _D4, __n64 _Dm);
__n64 __vtbl1_p8(__n64 _Dn, __n64 _Dm);
__n64 __vtbl1_s8(__n64 _Dn, __n64 _Dm);
__n64 __vtbl1_u8(__n64 _Dn, __n64 _Dm);
__n64 __vtbx1_p8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vtbx1_s8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64 __vtbx1_u8(__n64 _Dd, __n64 _Dn, __n64 _Dm);
__n64x2 __vtrn_f32(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_p16(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_p8(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_s16(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_s32(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_s8(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_u16(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_u32(__n64 _Dd, __n64 _Dm);
__n64x2 __vtrn_u8(__n64 _Dd, __n64 _Dm);
__n128x2 __vtrnq_f32(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_p16(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_p8(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_s16(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_s32(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_s8(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_u16(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_u32(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_u8(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_s64(__n128 _Qd, __n128 _Qm);
__n128x2 __vtrnq_u64(__n128 _Qd, __n128 _Qm);
__n64 __vtst_p8(__n64 _Dn, __n64 _Dm);
__n64 __vtst_s16(__n64 _Dn, __n64 _Dm);
__n64 __vtst_s32(__n64 _Dn, __n64 _Dm);
__n64 __vtst_s8(__n64 _Dn, __n64 _Dm);
__n64 __vtst_u16(__n64 _Dn, __n64 _Dm);
__n64 __vtst_u32(__n64 _Dn, __n64 _Dm);
__n64 __vtst_u8(__n64 _Dn, __n64 _Dm);
__n128 __vtstq_p8(__n128 _Qn, __n128 _Qm);
__n128 __vtstq_s16(__n128 _Qn, __n128 _Qm);
__n128 __vtstq_s32(__n128 _Qn, __n128 _Qm);
__n128 __vtstq_s8(__n128 _Qn, __n128 _Qm);
__n128 __vtstq_u16(__n128 _Qn, __n128 _Qm);
__n128 __vtstq_u32(__n128 _Qn, __n128 _Qm);
__n128 __vtstq_u8(__n128 _Qn, __n128 _Qm);
__n64x2 __vuzp_p16(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_p8(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_s16(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_s8(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_u16(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_u8(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_f32(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_s32(__n64 _Dd, __n64 _Dm);
__n64x2 __vuzp_u32(__n64 _Dd, __n64 _Dm);
__n128x2 __vuzpq_f32(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_p16(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_p8(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_s16(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_s32(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_s8(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_u16(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_u32(__n128 _Qd, __n128 _Qm);
__n128x2 __vuzpq_u8(__n128 _Qd, __n128 _Qm);
__n64x2 __vzip_p16(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_p8(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_s16(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_s8(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_u16(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_u8(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_f32(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_s32(__n64 _Dd, __n64 _Dm);
__n64x2 __vzip_u32(__n64 _Dd, __n64 _Dm);
__n128x2 __vzipq_f32(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_p16(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_p8(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_s16(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_s32(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_s8(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_u16(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_u32(__n128 _Qd, __n128 _Qm);
__n128x2 __vzipq_u8(__n128 _Qd, __n128 _Qm);

// AESD/AESE/AESIMC/AESMC
#define vaesdq_u8(src1, src2)   __aesd_u8(src1, src2)
#define vaeseq_u8(src1, src2)   __aese_u8(src1, src2)
#define vaesimcq_u8(src)        __aesimc_u8(src)
#define vaesmcq_u8(src)         __aesmc_u8(src)

// SHA1C/SHA1H/SHA1M/SHA1P/SHA1SU0/SHA1SU1/SHA256H/SHA256H2/SHA256SU0/SHA256SU1/
#define vsha1cq_u32(src1, src2, src3)       __sha1c_u32(src1, __vmovq_n_u32(src2), src3)
#define vsha1h_u32(src)                     __vgetq_lane_u32(__sha1h_u32(__vmovq_n_u32(src)), 0)
#define vsha1mq_u32(src1, src2, src3)       __sha1m_u32(src1, __vmovq_n_u32(src2), src3)
#define vsha1pq_u32(src1, src2, src3)       __sha1p_u32(src1, __vmovq_n_u32(src2), src3)
#define vsha1su0q_u32(src1, src2, src3)     __sha1su0_u32(src1, src2, src3)
#define vsha1su1q_u32(src1, src2)           __sha1su1_u32(src1, src2)
#define vsha256hq_u32(src1, src2, src3)     __sha256h_u32(src1, src2, src3)
#define vsha256h2q_u32(src1, src2, src3)    __sha256h2_u32(src1, src2, src3)
#define vsha256su0q_u32(src1, src2)         __sha256su0_u32(src1, src2)
#define vsha256su1q_u32(src1, src2, src3)   __sha256su1_u32(src1, src2, src3)

// Type reinterpretation no-ops
#define __vreinterpret_f32_s8(a)    (a)
#define __vreinterpret_f32_s16(a)   (a)
#define __vreinterpret_f32_s32(a)   (a)
#define __vreinterpret_f32_s64(a)   (a)
#define __vreinterpret_f32_p8(a)    (a)
#define __vreinterpret_f32_p16(a)   (a)
#define __vreinterpret_f32_u8(a)    (a)
#define __vreinterpret_f32_u16(a)   (a)
#define __vreinterpret_f32_u32(a)   (a)
#define __vreinterpret_f32_u64(a)   (a)
#define __vreinterpret_s8_f32(a)    (a)
#define __vreinterpret_s8_s16(a)    (a)
#define __vreinterpret_s8_s32(a)    (a)
#define __vreinterpret_s8_s64(a)    (a)
#define __vreinterpret_s8_p8(a)     (a)
#define __vreinterpret_s8_p16(a)    (a)
#define __vreinterpret_s8_u8(a)     (a)
#define __vreinterpret_s8_u16(a)    (a)
#define __vreinterpret_s8_u32(a)    (a)
#define __vreinterpret_s8_u64(a)    (a)
#define __vreinterpret_s16_f32(a)   (a)
#define __vreinterpret_s16_s8(a)    (a)
#define __vreinterpret_s16_s32(a)   (a)
#define __vreinterpret_s16_s64(a)   (a)
#define __vreinterpret_s16_p8(a)    (a)
#define __vreinterpret_s16_p16(a)   (a)
#define __vreinterpret_s16_u8(a)    (a)
#define __vreinterpret_s16_u16(a)   (a)
#define __vreinterpret_s16_u32(a)   (a)
#define __vreinterpret_s16_u64(a)   (a)
#define __vreinterpret_s32_f32(a)   (a)
#define __vreinterpret_s32_s8(a)    (a)
#define __vreinterpret_s32_s16(a)   (a)
#define __vreinterpret_s32_s64(a)   (a)
#define __vreinterpret_s32_p8(a)    (a)
#define __vreinterpret_s32_p16(a)   (a)
#define __vreinterpret_s32_u8(a)    (a)
#define __vreinterpret_s32_u16(a)   (a)
#define __vreinterpret_s32_u32(a)   (a)
#define __vreinterpret_s32_u64(a)   (a)
#define __vreinterpret_s64_f32(a)   (a)
#define __vreinterpret_s64_s8(a)    (a)
#define __vreinterpret_s64_s16(a)   (a)
#define __vreinterpret_s64_s32(a)   (a)
#define __vreinterpret_s64_p8(a)    (a)
#define __vreinterpret_s64_p16(a)   (a)
#define __vreinterpret_s64_u8(a)    (a)
#define __vreinterpret_s64_u16(a)   (a)
#define __vreinterpret_s64_u32(a)   (a)
#define __vreinterpret_s64_u64(a)   (a)
#define __vreinterpret_p8_f32(a)    (a)
#define __vreinterpret_p8_s8(a)     (a)
#define __vreinterpret_p8_s16(a)    (a)
#define __vreinterpret_p8_s32(a)    (a)
#define __vreinterpret_p8_s64(a)    (a)
#define __vreinterpret_p8_p16(a)    (a)
#define __vreinterpret_p8_u8(a)     (a)
#define __vreinterpret_p8_u16(a)    (a)
#define __vreinterpret_p8_u32(a)    (a)
#define __vreinterpret_p8_u64(a)    (a)
#define __vreinterpret_p16_f32(a)   (a)
#define __vreinterpret_p16_s8(a)    (a)
#define __vreinterpret_p16_s16(a)   (a)
#define __vreinterpret_p16_s32(a)   (a)
#define __vreinterpret_p16_s64(a)   (a)
#define __vreinterpret_p16_p8(a)    (a)
#define __vreinterpret_p16_u8(a)    (a)
#define __vreinterpret_p16_u16(a)   (a)
#define __vreinterpret_p16_u32(a)   (a)
#define __vreinterpret_p16_u64(a)   (a)
#define __vreinterpret_u8_f32(a)    (a)
#define __vreinterpret_u8_s8(a)     (a)
#define __vreinterpret_u8_s16(a)    (a)
#define __vreinterpret_u8_s32(a)    (a)
#define __vreinterpret_u8_s64(a)    (a)
#define __vreinterpret_u8_p8(a)     (a)
#define __vreinterpret_u8_p16(a)    (a)
#define __vreinterpret_u8_u16(a)    (a)
#define __vreinterpret_u8_u32(a)    (a)
#define __vreinterpret_u8_u64(a)    (a)
#define __vreinterpret_u16_f32(a)   (a)
#define __vreinterpret_u16_s8(a)    (a)
#define __vreinterpret_u16_s16(a)   (a)
#define __vreinterpret_u16_s32(a)   (a)
#define __vreinterpret_u16_s64(a)   (a)
#define __vreinterpret_u16_p8(a)    (a)
#define __vreinterpret_u16_p16(a)   (a)
#define __vreinterpret_u16_u8(a)    (a)
#define __vreinterpret_u16_u32(a)   (a)
#define __vreinterpret_u16_u64(a)   (a)
#define __vreinterpret_u32_f32(a)   (a)
#define __vreinterpret_u32_s8(a)    (a)
#define __vreinterpret_u32_s16(a)   (a)
#define __vreinterpret_u32_s32(a)   (a)
#define __vreinterpret_u32_s64(a)   (a)
#define __vreinterpret_u32_p8(a)    (a)
#define __vreinterpret_u32_p16(a)   (a)
#define __vreinterpret_u32_u8(a)    (a)
#define __vreinterpret_u32_u16(a)   (a)
#define __vreinterpret_u32_u64(a)   (a)
#define __vreinterpret_u64_f32(a)   (a)
#define __vreinterpret_u64_s8(a)    (a)
#define __vreinterpret_u64_s16(a)   (a)
#define __vreinterpret_u64_s32(a)   (a)
#define __vreinterpret_u64_s64(a)   (a)
#define __vreinterpret_u64_p8(a)    (a)
#define __vreinterpret_u64_p16(a)   (a)
#define __vreinterpret_u64_u8(a)    (a)
#define __vreinterpret_u64_u16(a)   (a)
#define __vreinterpret_u64_u32(a)   (a)
#define __vreinterpretq_f32_s8(a)   (a)
#define __vreinterpretq_f32_s16(a)  (a)
#define __vreinterpretq_f32_s32(a)  (a)
#define __vreinterpretq_f32_s64(a)  (a)
#define __vreinterpretq_f32_p8(a)   (a)
#define __vreinterpretq_f32_p16(a)  (a)
#define __vreinterpretq_f32_u8(a)   (a)
#define __vreinterpretq_f32_u16(a)  (a)
#define __vreinterpretq_f32_u32(a)  (a)
#define __vreinterpretq_f32_u64(a)  (a)
#define __vreinterpretq_s8_f32(a)   (a)
#define __vreinterpretq_s8_s16(a)   (a)
#define __vreinterpretq_s8_s32(a)   (a)
#define __vreinterpretq_s8_s64(a)   (a)
#define __vreinterpretq_s8_p8(a)    (a)
#define __vreinterpretq_s8_p16(a)   (a)
#define __vreinterpretq_s8_u8(a)    (a)
#define __vreinterpretq_s8_u16(a)   (a)
#define __vreinterpretq_s8_u32(a)   (a)
#define __vreinterpretq_s8_u64(a)   (a)
#define __vreinterpretq_s16_f32(a)  (a)
#define __vreinterpretq_s16_s8(a)   (a)
#define __vreinterpretq_s16_s32(a)  (a)
#define __vreinterpretq_s16_s64(a)  (a)
#define __vreinterpretq_s16_p8(a)   (a)
#define __vreinterpretq_s16_p16(a)  (a)
#define __vreinterpretq_s16_u8(a)   (a)
#define __vreinterpretq_s16_u16(a)  (a)
#define __vreinterpretq_s16_u32(a)  (a)
#define __vreinterpretq_s16_u64(a)  (a)
#define __vreinterpretq_s32_f32(a)  (a)
#define __vreinterpretq_s32_s8(a)   (a)
#define __vreinterpretq_s32_s16(a)  (a)
#define __vreinterpretq_s32_s64(a)  (a)
#define __vreinterpretq_s32_p8(a)   (a)
#define __vreinterpretq_s32_p16(a)  (a)
#define __vreinterpretq_s32_u8(a)   (a)
#define __vreinterpretq_s32_u16(a)  (a)
#define __vreinterpretq_s32_u32(a)  (a)
#define __vreinterpretq_s32_u64(a)  (a)
#define __vreinterpretq_s64_f32(a)  (a)
#define __vreinterpretq_s64_s8(a)   (a)
#define __vreinterpretq_s64_s16(a)  (a)
#define __vreinterpretq_s64_s32(a)  (a)
#define __vreinterpretq_s64_p8(a)   (a)
#define __vreinterpretq_s64_p16(a)  (a)
#define __vreinterpretq_s64_u8(a)   (a)
#define __vreinterpretq_s64_u16(a)  (a)
#define __vreinterpretq_s64_u32(a)  (a)
#define __vreinterpretq_s64_u64(a)  (a)
#define __vreinterpretq_p8_f32(a)   (a)
#define __vreinterpretq_p8_s8(a)    (a)
#define __vreinterpretq_p8_s16(a)   (a)
#define __vreinterpretq_p8_s32(a)   (a)
#define __vreinterpretq_p8_s64(a)   (a)
#define __vreinterpretq_p8_p16(a)   (a)
#define __vreinterpretq_p8_u8(a)    (a)
#define __vreinterpretq_p8_u16(a)   (a)
#define __vreinterpretq_p8_u32(a)   (a)
#define __vreinterpretq_p8_u64(a)   (a)
#define __vreinterpretq_p16_f32(a)  (a)
#define __vreinterpretq_p16_s8(a)   (a)
#define __vreinterpretq_p16_s16(a)  (a)
#define __vreinterpretq_p16_s32(a)  (a)
#define __vreinterpretq_p16_s64(a)  (a)
#define __vreinterpretq_p16_p8(a)   (a)
#define __vreinterpretq_p16_u8(a)   (a)
#define __vreinterpretq_p16_u16(a)  (a)
#define __vreinterpretq_p16_u32(a)  (a)
#define __vreinterpretq_p16_u64(a)  (a)
#define __vreinterpretq_u8_f32(a)   (a)
#define __vreinterpretq_u8_s8(a)    (a)
#define __vreinterpretq_u8_s16(a)   (a)
#define __vreinterpretq_u8_s32(a)   (a)
#define __vreinterpretq_u8_s64(a)   (a)
#define __vreinterpretq_u8_p8(a)    (a)
#define __vreinterpretq_u8_p16(a)   (a)
#define __vreinterpretq_u8_u16(a)   (a)
#define __vreinterpretq_u8_u32(a)   (a)
#define __vreinterpretq_u8_u64(a)   (a)
#define __vreinterpretq_u16_f32(a)  (a)
#define __vreinterpretq_u16_s8(a)   (a)
#define __vreinterpretq_u16_s16(a)  (a)
#define __vreinterpretq_u16_s32(a)  (a)
#define __vreinterpretq_u16_s64(a)  (a)
#define __vreinterpretq_u16_p8(a)   (a)
#define __vreinterpretq_u16_p16(a)  (a)
#define __vreinterpretq_u16_u8(a)   (a)
#define __vreinterpretq_u16_u32(a)  (a)
#define __vreinterpretq_u16_u64(a)  (a)
#define __vreinterpretq_u32_f32(a)  (a)
#define __vreinterpretq_u32_s8(a)   (a)
#define __vreinterpretq_u32_s16(a)  (a)
#define __vreinterpretq_u32_s32(a)  (a)
#define __vreinterpretq_u32_s64(a)  (a)
#define __vreinterpretq_u32_p8(a)   (a)
#define __vreinterpretq_u32_p16(a)  (a)
#define __vreinterpretq_u32_u8(a)   (a)
#define __vreinterpretq_u32_u16(a)  (a)
#define __vreinterpretq_u32_u64(a)  (a)
#define __vreinterpretq_u64_f32(a)  (a)
#define __vreinterpretq_u64_s8(a)   (a)
#define __vreinterpretq_u64_s16(a)  (a)
#define __vreinterpretq_u64_s32(a)  (a)
#define __vreinterpretq_u64_s64(a)  (a)
#define __vreinterpretq_u64_p8(a)   (a)
#define __vreinterpretq_u64_p16(a)  (a)
#define __vreinterpretq_u64_u8(a)   (a)
#define __vreinterpretq_u64_u16(a)  (a)
#define __vreinterpretq_u64_u32(a)  (a)

//  Multiply by scalar
#define __vmul_n_s16(Vd, Rt)        __vmul_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vmul_n_s32(Vd, Rt)        __vmul_lane_s32((Vd), __vmov_n_s32(Rt), 0)
#define __vmul_n_u16(Vd, Rt)        __vmul_lane_u16((Vd), __vmov_n_u16(Rt), 0)
#define __vmul_n_u32(Vd, Rt)        __vmul_lane_u32((Vd), __vmov_n_u32(Rt), 0)
#define __vmulq_n_s16(Vd, Rt)       __vmulq_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vmulq_n_s32(Vd, Rt)       __vmulq_lane_s32((Vd), __vmov_n_s32(Rt), 0)
#define __vmulq_n_u16(Vd, Rt)       __vmulq_lane_u16((Vd), __vmov_n_u16(Rt), 0)
#define __vmulq_n_u32(Vd, Rt)       __vmulq_lane_u32((Vd), __vmov_n_u32(Rt), 0)
#define __vmull_n_s16(Vd, Rt)       __vmull_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vmull_n_s32(Vd, Rt)       __vmull_lane_s32((Vd), __vmov_n_s32(Rt), 0)
#define __vmull_n_u16(Vd, Rt)       __vmull_lane_u16((Vd), __vmov_n_u16(Rt), 0)
#define __vmull_n_u32(Vd, Rt)       __vmull_lane_u32((Vd), __vmov_n_u32(Rt), 0)
#define __vqdmulh_n_s16(Vd, Rt)     __vqdmulh_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vqdmulh_n_s32(Vd, Rt)     __vqdmulh_lane_s32((Vd), __vmov_n_s32(Rt), 0)
#define __vqdmulhq_n_s16(Vd, Rt)    __vqdmulhq_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vqdmulhq_n_s32(Vd, Rt)    __vqdmulhq_lane_s32((Vd), __vmov_n_s32(Rt), 0)
#define __vqdmull_n_s16(Vd, Rt)     __vqdmull_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vqdmull_n_s32(Vd, Rt)     __vqdmull_lane_s32((Vd), __vmov_n_s32(Rt), 0)
#define __vqrdmulh_n_s16(Vd, Rt)    __vqrdmulh_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vqrdmulh_n_s32(Vd, Rt)    __vqrdmulh_lane_s32((Vd), __vmov_n_s32(Rt), 0)
#define __vqrdmulhq_n_s16(Vd, Rt)   __vqrdmulhq_lane_s16((Vd), __vmov_n_s16(Rt), 0)
#define __vqrdmulhq_n_s32(Vd, Rt)   __vqrdmulhq_lane_s32((Vd), __vmov_n_s32(Rt), 0)

//  Multiply by scalar with accumulate
#define __vmla_n_s16(Vd, Vn, Rt)    __vmla_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vmla_n_s32(Vd, Vn, Rt)    __vmla_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)
#define __vmla_n_u16(Vd, Vn, Rt)    __vmla_lane_u16((Vd), (Vn), __vmov_n_u16(Rt), 0)
#define __vmla_n_u32(Vd, Vn, Rt)    __vmla_lane_u32((Vd), (Vn), __vmov_n_u32(Rt), 0)
#define __vmlaq_n_s16(Vd, Vn, Rt)   __vmlaq_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vmlaq_n_s32(Vd, Vn, Rt)   __vmlaq_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)
#define __vmlaq_n_u16(Vd, Vn, Rt)   __vmlaq_lane_u16((Vd), (Vn), __vmov_n_u16(Rt), 0)
#define __vmlaq_n_u32(Vd, Vn, Rt)   __vmlaq_lane_u32((Vd), (Vn), __vmov_n_u32(Rt), 0)
#define __vmlal_n_s16(Vd, Vn, Rt)   __vmlal_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vmlal_n_s32(Vd, Vn, Rt)   __vmlal_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)
#define __vmlal_n_u16(Vd, Vn, Rt)   __vmlal_lane_u16((Vd), (Vn), __vmov_n_u16(Rt), 0)
#define __vmlal_n_u32(Vd, Vn, Rt)   __vmlal_lane_u32((Vd), (Vn), __vmov_n_u32(Rt), 0)
#define __vmls_n_s16(Vd, Vn, Rt)    __vmls_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vmls_n_s32(Vd, Vn, Rt)    __vmls_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)
#define __vmls_n_u16(Vd, Vn, Rt)    __vmls_lane_u16((Vd), (Vn), __vmov_n_u16(Rt), 0)
#define __vmls_n_u32(Vd, Vn, Rt)    __vmls_lane_u32((Vd), (Vn), __vmov_n_u32(Rt), 0)
#define __vmlsq_n_s16(Vd, Vn, Rt)   __vmlsq_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vmlsq_n_s32(Vd, Vn, Rt)   __vmlsq_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)
#define __vmlsq_n_u16(Vd, Vn, Rt)   __vmlsq_lane_u16((Vd), (Vn), __vmov_n_u16(Rt), 0)
#define __vmlsq_n_u32(Vd, Vn, Rt)   __vmlsq_lane_u32((Vd), (Vn), __vmov_n_u32(Rt), 0)
#define __vmlsl_n_s16(Vd, Vn, Rt)   __vmlsl_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vmlsl_n_s32(Vd, Vn, Rt)   __vmlsl_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)
#define __vmlsl_n_u16(Vd, Vn, Rt)   __vmlsl_lane_u16((Vd), (Vn), __vmov_n_u16(Rt), 0)
#define __vmlsl_n_u32(Vd, Vn, Rt)   __vmlsl_lane_u32((Vd), (Vn), __vmov_n_u32(Rt), 0)
#define __vqdmlal_n_s16(Vd, Vn, Rt) __vqdmlal_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vqdmlal_n_s32(Vd, Vn, Rt) __vqdmlal_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)
#define __vqdmlsl_n_s16(Vd, Vn, Rt) __vqdmlsl_lane_s16((Vd), (Vn), __vmov_n_s16(Rt), 0)
#define __vqdmlsl_n_s32(Vd, Vn, Rt) __vqdmlsl_lane_s32((Vd), (Vn), __vmov_n_s32(Rt), 0)

//  VDUP.64 (scalar)
#define __vdup_lane_s64(Dn, lane)   (__static_assert((lane) == 0, "invalid lane index"), (Dn))
#define __vdup_lane_u64(Dn, lane)   (__static_assert((lane) == 0, "invalid lane index"), (Dn))

//  VDUP.W.64 (scalar)
#define __vdupq_lane_s64(Dn, lane)  (__static_assert((lane) == 0, "invalid lane index"), vcombine_s64((Dn), (Dn)))
#define __vdupq_lane_u64(Dn, lane)  (__static_assert((lane) == 0, "invalid lane index"), vcombine_u64((Dn), (Dn)))

#if !defined(_ARM_ISO_COMPATIBLE_INTRINSIC_NAMES)

#define aesd_p8                     __aesd_p8
#define aesd_s8                     __aesd_s8
#define aesd_u8                     __aesd_u8
#define aese_p8                     __aese_p8
#define aese_s8                     __aese_s8
#define aese_u8                     __aese_u8
#define aesimc_p8                   __aesimc_p8
#define aesimc_s8                   __aesimc_s8
#define aesimc_u8                   __aesimc_u8
#define aesmc_p8                    __aesmc_p8
#define aesmc_s8                    __aesmc_s8
#define aesmc_u8                    __aesmc_u8
#define sha1h_f32                   __sha1h_f32
#define sha1h_s32                   __sha1h_s32
#define sha1h_u32                   __sha1h_u32
#define sha1su1_f32                 __sha1su1_f32
#define sha1su1_s32                 __sha1su1_s32
#define sha1su1_u32                 __sha1su1_u32
#define sha256su0_f32               __sha256su0_f32
#define sha256su0_s32               __sha256su0_s32
#define sha256su0_u32               __sha256su0_u32
#define sha1c_f32                   __sha1c_f32
#define sha1c_s32                   __sha1c_s32
#define sha1c_u32                   __sha1c_u32
#define sha1m_f32                   __sha1m_f32
#define sha1m_s32                   __sha1m_s32
#define sha1m_u32                   __sha1m_u32
#define sha1p_f32                   __sha1p_f32
#define sha1p_s32                   __sha1p_s32
#define sha1p_u32                   __sha1p_u32
#define sha1su0_f32                 __sha1su0_f32
#define sha1su0_s32                 __sha1su0_s32
#define sha1su0_u32                 __sha1su0_u32
#define sha256h_f32                 __sha256h_f32
#define sha256h_s32                 __sha256h_s32
#define sha256h_u32                 __sha256h_u32
#define sha256h2_f32                __sha256h2_f32
#define sha256h2_s32                __sha256h2_s32
#define sha256h2_u32                __sha256h2_u32
#define sha256su1_f32               __sha256su1_f32
#define sha256su1_s32               __sha256su1_s32
#define sha256su1_u32               __sha256su1_u32
#define vaba_s16                    __vaba_s16
#define vaba_s32                    __vaba_s32
#define vaba_s8                     __vaba_s8
#define vaba_u16                    __vaba_u16
#define vaba_u32                    __vaba_u32
#define vaba_u8                     __vaba_u8
#define vabal_s16                   __vabal_s16
#define vabal_s32                   __vabal_s32
#define vabal_s8                    __vabal_s8
#define vabal_u16                   __vabal_u16
#define vabal_u32                   __vabal_u32
#define vabal_u8                    __vabal_u8
#define vabaq_s16                   __vabaq_s16
#define vabaq_s32                   __vabaq_s32
#define vabaq_s8                    __vabaq_s8
#define vabaq_u16                   __vabaq_u16
#define vabaq_u32                   __vabaq_u32
#define vabaq_u8                    __vabaq_u8
#define vabd_f32                    __vabd_f32
#define vabdq_f32                   __vabdq_f32
#define vabd_s16                    __vabd_s16
#define vabd_s32                    __vabd_s32
#define vabd_s8                     __vabd_s8
#define vabd_u16                    __vabd_u16
#define vabd_u32                    __vabd_u32
#define vabd_u8                     __vabd_u8
#define vabdl_s16                   __vabdl_s16
#define vabdl_s32                   __vabdl_s32
#define vabdl_s8                    __vabdl_s8
#define vabdl_u16                   __vabdl_u16
#define vabdl_u32                   __vabdl_u32
#define vabdl_u8                    __vabdl_u8
#define vabdq_s16                   __vabdq_s16
#define vabdq_s32                   __vabdq_s32
#define vabdq_s8                    __vabdq_s8
#define vabdq_u16                   __vabdq_u16
#define vabdq_u32                   __vabdq_u32
#define vabdq_u8                    __vabdq_u8
#define vabs_f32                    __vabs_f32
#define vabs_s16                    __vabs_s16
#define vabs_s32                    __vabs_s32
#define vabs_s8                     __vabs_s8
#define vneg_f32                    __vneg_f32
#define vneg_s16                    __vneg_s16
#define vneg_s32                    __vneg_s32
#define vneg_s8                     __vneg_s8
#define vabsq_f32                   __vabsq_f32
#define vabsq_s16                   __vabsq_s16
#define vabsq_s32                   __vabsq_s32
#define vabsq_s8                    __vabsq_s8
#define vnegq_f32                   __vnegq_f32
#define vnegq_s16                   __vnegq_s16
#define vnegq_s32                   __vnegq_s32
#define vnegq_s8                    __vnegq_s8
#define vacge_f32                   __vacge_f32
#define vacgt_f32                   __vacgt_f32
#define vacle_f32                   __vacle_f32
#define vaclt_f32                   __vaclt_f32
#define vacgeq_f32                  __vacgeq_f32
#define vacgtq_f32                  __vacgtq_f32
#define vacleq_f32                  __vacleq_f32
#define vacltq_f32                  __vacltq_f32
#define vadd_f32                    __vadd_f32
#define vadd_s16                    __vadd_s16
#define vadd_s32                    __vadd_s32
#define vadd_s64                    __vadd_s64
#define vadd_s8                     __vadd_s8
#define vadd_u16                    __vadd_u16
#define vadd_u32                    __vadd_u32
#define vadd_u64                    __vadd_u64
#define vadd_u8                     __vadd_u8
#define vaddq_f32                   __vaddq_f32
#define vaddq_s16                   __vaddq_s16
#define vaddq_s32                   __vaddq_s32
#define vaddq_s64                   __vaddq_s64
#define vaddq_s8                    __vaddq_s8
#define vaddq_u16                   __vaddq_u16
#define vaddq_u32                   __vaddq_u32
#define vaddq_u64                   __vaddq_u64
#define vaddq_u8                    __vaddq_u8
#define vaddhn_s16                  __vaddhn_s16
#define vaddhn_s32                  __vaddhn_s32
#define vaddhn_s64                  __vaddhn_s64
#define vaddhn_u16                  __vaddhn_u16
#define vaddhn_u32                  __vaddhn_u32
#define vaddhn_u64                  __vaddhn_u64
#define vraddhn_s16                 __vraddhn_s16
#define vraddhn_s32                 __vraddhn_s32
#define vraddhn_s64                 __vraddhn_s64
#define vraddhn_u16                 __vraddhn_u16
#define vraddhn_u32                 __vraddhn_u32
#define vraddhn_u64                 __vraddhn_u64
#define vaddl_s16                   __vaddl_s16
#define vaddl_s32                   __vaddl_s32
#define vaddl_s8                    __vaddl_s8
#define vaddl_u16                   __vaddl_u16
#define vaddl_u32                   __vaddl_u32
#define vaddl_u8                    __vaddl_u8
#define vaddw_s16                   __vaddw_s16
#define vaddw_s32                   __vaddw_s32
#define vaddw_s8                    __vaddw_s8
#define vaddw_u16                   __vaddw_u16
#define vaddw_u32                   __vaddw_u32
#define vaddw_u8                    __vaddw_u8
#define vand_s16                    __vand_s16
#define vand_s32                    __vand_s32
#define vand_s64                    __vand_s64
#define vand_s8                     __vand_s8
#define vand_u16                    __vand_u16
#define vand_u32                    __vand_u32
#define vand_u64                    __vand_u64
#define vand_u8                     __vand_u8
#define vorr_s16                    __vorr_s16
#define vorr_s32                    __vorr_s32
#define vorr_s64                    __vorr_s64
#define vorr_s8                     __vorr_s8
#define vorr_u16                    __vorr_u16
#define vorr_u32                    __vorr_u32
#define vorr_u64                    __vorr_u64
#define vorr_u8                     __vorr_u8
#define vandq_s16                   __vandq_s16
#define vandq_s32                   __vandq_s32
#define vandq_s64                   __vandq_s64
#define vandq_s8                    __vandq_s8
#define vandq_u16                   __vandq_u16
#define vandq_u32                   __vandq_u32
#define vandq_u64                   __vandq_u64
#define vandq_u8                    __vandq_u8
#define vorrq_s16                   __vorrq_s16
#define vorrq_s32                   __vorrq_s32
#define vorrq_s64                   __vorrq_s64
#define vorrq_s8                    __vorrq_s8
#define vorrq_u16                   __vorrq_u16
#define vorrq_u32                   __vorrq_u32
#define vorrq_u64                   __vorrq_u64
#define vorrq_u8                    __vorrq_u8
#define vbif_f32                    __vbif_f32
#define vbif_p16                    __vbif_p16
#define vbif_p8                     __vbif_p8
#define vbif_s16                    __vbif_s16
#define vbif_s32                    __vbif_s32
#define vbif_s64                    __vbif_s64
#define vbif_s8                     __vbif_s8
#define vbif_u16                    __vbif_u16
#define vbif_u32                    __vbif_u32
#define vbif_u64                    __vbif_u64
#define vbif_u8                     __vbif_u8
#define vbit_f32                    __vbit_f32
#define vbit_p16                    __vbit_p16
#define vbit_p8                     __vbit_p8
#define vbit_s16                    __vbit_s16
#define vbit_s32                    __vbit_s32
#define vbit_s64                    __vbit_s64
#define vbit_s8                     __vbit_s8
#define vbit_u16                    __vbit_u16
#define vbit_u32                    __vbit_u32
#define vbit_u64                    __vbit_u64
#define vbit_u8                     __vbit_u8
#define vbsl_f32                    __vbsl_f32
#define vbsl_p16                    __vbsl_p16
#define vbsl_p8                     __vbsl_p8
#define vbsl_s16                    __vbsl_s16
#define vbsl_s32                    __vbsl_s32
#define vbsl_s64                    __vbsl_s64
#define vbsl_s8                     __vbsl_s8
#define vbsl_u16                    __vbsl_u16
#define vbsl_u32                    __vbsl_u32
#define vbsl_u64                    __vbsl_u64
#define vbsl_u8                     __vbsl_u8
#define vbifq_f32                   __vbifq_f32
#define vbifq_p16                   __vbifq_p16
#define vbifq_p8                    __vbifq_p8
#define vbifq_s16                   __vbifq_s16
#define vbifq_s32                   __vbifq_s32
#define vbifq_s64                   __vbifq_s64
#define vbifq_s8                    __vbifq_s8
#define vbifq_u16                   __vbifq_u16
#define vbifq_u32                   __vbifq_u32
#define vbifq_u64                   __vbifq_u64
#define vbifq_u8                    __vbifq_u8
#define vbitq_f32                   __vbitq_f32
#define vbitq_p16                   __vbitq_p16
#define vbitq_p8                    __vbitq_p8
#define vbitq_s16                   __vbitq_s16
#define vbitq_s32                   __vbitq_s32
#define vbitq_s64                   __vbitq_s64
#define vbitq_s8                    __vbitq_s8
#define vbitq_u16                   __vbitq_u16
#define vbitq_u32                   __vbitq_u32
#define vbitq_u64                   __vbitq_u64
#define vbitq_u8                    __vbitq_u8
#define vbslq_f32                   __vbslq_f32
#define vbslq_p16                   __vbslq_p16
#define vbslq_p8                    __vbslq_p8
#define vbslq_s16                   __vbslq_s16
#define vbslq_s32                   __vbslq_s32
#define vbslq_s64                   __vbslq_s64
#define vbslq_s8                    __vbslq_s8
#define vbslq_u16                   __vbslq_u16
#define vbslq_u32                   __vbslq_u32
#define vbslq_u64                   __vbslq_u64
#define vbslq_u8                    __vbslq_u8
#define vceq_z_f32_ex               __vceq_z_f32_ex
#define vceq_z_s16_ex               __vceq_z_s16_ex
#define vceq_z_s32_ex               __vceq_z_s32_ex
#define vceq_z_s8_ex                __vceq_z_s8_ex
#define vceq_z_u16_ex               __vceq_z_u16_ex
#define vceq_z_u32_ex               __vceq_z_u32_ex
#define vceq_z_u8_ex                __vceq_z_u8_ex
#define vceqq_z_f32_ex              __vceqq_z_f32_ex
#define vceqq_z_s16_ex              __vceqq_z_s16_ex
#define vceqq_z_s32_ex              __vceqq_z_s32_ex
#define vceqq_z_s8_ex               __vceqq_z_s8_ex
#define vceqq_z_u16_ex              __vceqq_z_u16_ex
#define vceqq_z_u32_ex              __vceqq_z_u32_ex
#define vceqq_z_u8_ex               __vceqq_z_u8_ex
#define vceq_f32                    __vceq_f32
#define vceq_p8                     __vceq_p8
#define vceq_s16                    __vceq_s16
#define vceq_s32                    __vceq_s32
#define vceq_s8                     __vceq_s8
#define vceq_u16                    __vceq_u16
#define vceq_u32                    __vceq_u32
#define vceq_u8                     __vceq_u8
#define vceqq_f32                   __vceqq_f32
#define vceqq_p8                    __vceqq_p8
#define vceqq_s16                   __vceqq_s16
#define vceqq_s32                   __vceqq_s32
#define vceqq_s8                    __vceqq_s8
#define vceqq_u16                   __vceqq_u16
#define vceqq_u32                   __vceqq_u32
#define vceqq_u8                    __vceqq_u8
#define vcge_z_f32_ex               __vcge_z_f32_ex
#define vcge_z_s16_ex               __vcge_z_s16_ex
#define vcge_z_s32_ex               __vcge_z_s32_ex
#define vcge_z_s8_ex                __vcge_z_s8_ex
#define vcgeq_z_f32_ex              __vcgeq_z_f32_ex
#define vcgeq_z_s16_ex              __vcgeq_z_s16_ex
#define vcgeq_z_s32_ex              __vcgeq_z_s32_ex
#define vcgeq_z_s8_ex               __vcgeq_z_s8_ex
#define vcge_f32                    __vcge_f32
#define vcge_s16                    __vcge_s16
#define vcge_s32                    __vcge_s32
#define vcge_s8                     __vcge_s8
#define vcge_u16                    __vcge_u16
#define vcge_u32                    __vcge_u32
#define vcge_u8                     __vcge_u8
#define vcle_f32                    __vcle_f32
#define vcle_s16                    __vcle_s16
#define vcle_s32                    __vcle_s32
#define vcle_s8                     __vcle_s8
#define vcle_u16                    __vcle_u16
#define vcle_u32                    __vcle_u32
#define vcle_u8                     __vcle_u8
#define vcgeq_f32                   __vcgeq_f32
#define vcgeq_s16                   __vcgeq_s16
#define vcgeq_s32                   __vcgeq_s32
#define vcgeq_s8                    __vcgeq_s8
#define vcgeq_u16                   __vcgeq_u16
#define vcgeq_u32                   __vcgeq_u32
#define vcgeq_u8                    __vcgeq_u8
#define vcleq_f32                   __vcleq_f32
#define vcleq_s16                   __vcleq_s16
#define vcleq_s32                   __vcleq_s32
#define vcleq_s8                    __vcleq_s8
#define vcleq_u16                   __vcleq_u16
#define vcleq_u32                   __vcleq_u32
#define vcleq_u8                    __vcleq_u8
#define vcgt_z_f32_ex               __vcgt_z_f32_ex
#define vcgt_z_s16_ex               __vcgt_z_s16_ex
#define vcgt_z_s32_ex               __vcgt_z_s32_ex
#define vcgt_z_s8_ex                __vcgt_z_s8_ex
#define vcgtq_z_f32_ex              __vcgtq_z_f32_ex
#define vcgtq_z_s16_ex              __vcgtq_z_s16_ex
#define vcgtq_z_s32_ex              __vcgtq_z_s32_ex
#define vcgtq_z_s8_ex               __vcgtq_z_s8_ex
#define vcgt_f32                    __vcgt_f32
#define vcgt_s16                    __vcgt_s16
#define vcgt_s32                    __vcgt_s32
#define vcgt_s8                     __vcgt_s8
#define vcgt_u16                    __vcgt_u16
#define vcgt_u32                    __vcgt_u32
#define vcgt_u8                     __vcgt_u8
#define vclt_f32                    __vclt_f32
#define vclt_s16                    __vclt_s16
#define vclt_s32                    __vclt_s32
#define vclt_s8                     __vclt_s8
#define vclt_u16                    __vclt_u16
#define vclt_u32                    __vclt_u32
#define vclt_u8                     __vclt_u8
#define vcgtq_f32                   __vcgtq_f32
#define vcgtq_s16                   __vcgtq_s16
#define vcgtq_s32                   __vcgtq_s32
#define vcgtq_s8                    __vcgtq_s8
#define vcgtq_u16                   __vcgtq_u16
#define vcgtq_u32                   __vcgtq_u32
#define vcgtq_u8                    __vcgtq_u8
#define vcltq_f32                   __vcltq_f32
#define vcltq_s16                   __vcltq_s16
#define vcltq_s32                   __vcltq_s32
#define vcltq_s8                    __vcltq_s8
#define vcltq_u16                   __vcltq_u16
#define vcltq_u32                   __vcltq_u32
#define vcltq_u8                    __vcltq_u8
#define vcle_z_f32_ex               __vcle_z_f32_ex
#define vcle_z_s16_ex               __vcle_z_s16_ex
#define vcle_z_s32_ex               __vcle_z_s32_ex
#define vcle_z_s8_ex                __vcle_z_s8_ex
#define vcleq_z_f32_ex              __vcleq_z_f32_ex
#define vcleq_z_s16_ex              __vcleq_z_s16_ex
#define vcleq_z_s32_ex              __vcleq_z_s32_ex
#define vcleq_z_s8_ex               __vcleq_z_s8_ex
#define vcls_s16                    __vcls_s16
#define vcls_s32                    __vcls_s32
#define vcls_s8                     __vcls_s8
#define vclz_s16                    __vclz_s16
#define vclz_s32                    __vclz_s32
#define vclz_s8                     __vclz_s8
#define vclz_u16                    __vclz_u16
#define vclz_u32                    __vclz_u32
#define vclz_u8                     __vclz_u8
#define vclsq_s16                   __vclsq_s16
#define vclsq_s32                   __vclsq_s32
#define vclsq_s8                    __vclsq_s8
#define vclzq_s16                   __vclzq_s16
#define vclzq_s32                   __vclzq_s32
#define vclzq_s8                    __vclzq_s8
#define vclzq_u16                   __vclzq_u16
#define vclzq_u32                   __vclzq_u32
#define vclzq_u8                    __vclzq_u8
#define vclt_z_f32_ex               __vclt_z_f32_ex
#define vclt_z_s16_ex               __vclt_z_s16_ex
#define vclt_z_s32_ex               __vclt_z_s32_ex
#define vclt_z_s8_ex                __vclt_z_s8_ex
#define vcltq_z_f32_ex              __vcltq_z_f32_ex
#define vcltq_z_s16_ex              __vcltq_z_s16_ex
#define vcltq_z_s32_ex              __vcltq_z_s32_ex
#define vcltq_z_s8_ex               __vcltq_z_s8_ex
#define vcnt_p8                     __vcnt_p8
#define vcnt_s8                     __vcnt_s8
#define vcnt_u8                     __vcnt_u8
#define vcntq_p8                    __vcntq_p8
#define vcntq_s8                    __vcntq_s8
#define vcntq_u8                    __vcntq_u8
#define vcombine_f32                __vcombine_f32
#define vcombine_p16                __vcombine_p16
#define vcombine_p8                 __vcombine_p8
#define vcombine_s16                __vcombine_s16
#define vcombine_s32                __vcombine_s32
#define vcombine_s64                __vcombine_s64
#define vcombine_s8                 __vcombine_s8
#define vcombine_u16                __vcombine_u16
#define vcombine_u32                __vcombine_u32
#define vcombine_u64                __vcombine_u64
#define vcombine_u8                 __vcombine_u8
#define vcreate_f32                 __vcreate_f32
#define vcreate_p16                 __vcreate_p16
#define vcreate_p8                  __vcreate_p8
#define vcreate_s16                 __vcreate_s16
#define vcreate_s32                 __vcreate_s32
#define vcreate_s64                 __vcreate_s64
#define vcreate_s8                  __vcreate_s8
#define vcreate_u16                 __vcreate_u16
#define vcreate_u32                 __vcreate_u32
#define vcreate_u64                 __vcreate_u64
#define vcreate_u8                  __vcreate_u8
#define vcvt_n_f32_s32              __vcvt_n_f32_s32
#define vcvt_n_f32_u32              __vcvt_n_f32_u32
#define vcvt_n_s32_f32              __vcvt_n_s32_f32
#define vcvt_n_u32_f32              __vcvt_n_u32_f32
#define vcvtq_n_f32_s32             __vcvtq_n_f32_s32
#define vcvtq_n_f32_u32             __vcvtq_n_f32_u32
#define vcvtq_n_s32_f32             __vcvtq_n_s32_f32
#define vcvtq_n_u32_f32             __vcvtq_n_u32_f32
#define vcvta_s32_f32               __vcvta_s32_f32
#define vcvta_u32_f32               __vcvta_u32_f32
#define vcvtm_s32_f32               __vcvtm_s32_f32
#define vcvtm_u32_f32               __vcvtm_u32_f32
#define vcvtn_s32_f32               __vcvtn_s32_f32
#define vcvtn_u32_f32               __vcvtn_u32_f32
#define vcvtp_s32_f32               __vcvtp_s32_f32
#define vcvtp_u32_f32               __vcvtp_u32_f32
#define vcvtaq_s32_f32              __vcvtaq_s32_f32
#define vcvtaq_u32_f32              __vcvtaq_u32_f32
#define vcvtmq_s32_f32              __vcvtmq_s32_f32
#define vcvtmq_u32_f32              __vcvtmq_u32_f32
#define vcvtnq_s32_f32              __vcvtnq_s32_f32
#define vcvtnq_u32_f32              __vcvtnq_u32_f32
#define vcvtpq_s32_f32              __vcvtpq_s32_f32
#define vcvtpq_u32_f32              __vcvtpq_u32_f32
#define vcvt_f32_s32                __vcvt_f32_s32
#define vcvt_f32_u32                __vcvt_f32_u32
#define vcvt_s32_f32                __vcvt_s32_f32
#define vcvt_u32_f32                __vcvt_u32_f32
#define vcvtq_f32_s32               __vcvtq_f32_s32
#define vcvtq_f32_u32               __vcvtq_f32_u32
#define vcvtq_s32_f32               __vcvtq_s32_f32
#define vcvtq_u32_f32               __vcvtq_u32_f32
#define vdup_lane_f32               __vdup_lane_f32
#define vdup_lane_p16               __vdup_lane_p16
#define vdup_lane_p8                __vdup_lane_p8
#define vdup_lane_s16               __vdup_lane_s16
#define vdup_lane_s32               __vdup_lane_s32
#define vdup_lane_s8                __vdup_lane_s8
#define vdup_lane_u16               __vdup_lane_u16
#define vdup_lane_u32               __vdup_lane_u32
#define vdup_lane_u8                __vdup_lane_u8
#define vdupq_lane_f32              __vdupq_lane_f32
#define vdupq_lane_p16              __vdupq_lane_p16
#define vdupq_lane_p8               __vdupq_lane_p8
#define vdupq_lane_s16              __vdupq_lane_s16
#define vdupq_lane_s32              __vdupq_lane_s32
#define vdupq_lane_s8               __vdupq_lane_s8
#define vdupq_lane_u16              __vdupq_lane_u16
#define vdupq_lane_u32              __vdupq_lane_u32
#define vdupq_lane_u8               __vdupq_lane_u8
#define vdup_n_f32                  __vdup_n_f32
#define vmov_n_f32                  __vmov_n_f32
#define vdup_n_p16                  __vdup_n_p16
#define vdup_n_p8                   __vdup_n_p8
#define vdup_n_s16                  __vdup_n_s16
#define vdup_n_s32                  __vdup_n_s32
#define vdup_n_s8                   __vdup_n_s8
#define vdup_n_u16                  __vdup_n_u16
#define vdup_n_u32                  __vdup_n_u32
#define vdup_n_u8                   __vdup_n_u8
#define vmov_n_p16                  __vmov_n_p16
#define vmov_n_p8                   __vmov_n_p8
#define vmov_n_s16                  __vmov_n_s16
#define vmov_n_s32                  __vmov_n_s32
#define vmov_n_s8                   __vmov_n_s8
#define vmov_n_u16                  __vmov_n_u16
#define vmov_n_u32                  __vmov_n_u32
#define vmov_n_u8                   __vmov_n_u8
#define vdupq_n_f32                 __vdupq_n_f32
#define vmovq_n_f32                 __vmovq_n_f32
#define vdupq_n_p16                 __vdupq_n_p16
#define vdupq_n_p8                  __vdupq_n_p8
#define vdupq_n_s16                 __vdupq_n_s16
#define vdupq_n_s32                 __vdupq_n_s32
#define vdupq_n_s8                  __vdupq_n_s8
#define vdupq_n_u16                 __vdupq_n_u16
#define vdupq_n_u32                 __vdupq_n_u32
#define vdupq_n_u8                  __vdupq_n_u8
#define vmovq_n_p16                 __vmovq_n_p16
#define vmovq_n_p8                  __vmovq_n_p8
#define vmovq_n_s16                 __vmovq_n_s16
#define vmovq_n_s32                 __vmovq_n_s32
#define vmovq_n_s8                  __vmovq_n_s8
#define vmovq_n_u16                 __vmovq_n_u16
#define vmovq_n_u32                 __vmovq_n_u32
#define vmovq_n_u8                  __vmovq_n_u8
#define vdup_n_s64                  __vdup_n_s64
#define vdup_n_u64                  __vdup_n_u64
#define vmov_n_s64                  __vmov_n_s64
#define vmov_n_u64                  __vmov_n_u64
#define vdupq_n_s64                 __vdupq_n_s64
#define vdupq_n_u64                 __vdupq_n_u64
#define vmovq_n_s64                 __vmovq_n_s64
#define vmovq_n_u64                 __vmovq_n_u64
#define vbic_s16                    __vbic_s16
#define vbic_s32                    __vbic_s32
#define vbic_s64                    __vbic_s64
#define vbic_s8                     __vbic_s8
#define vbic_u16                    __vbic_u16
#define vbic_u32                    __vbic_u32
#define vbic_u64                    __vbic_u64
#define vbic_u8                     __vbic_u8
#define veor_s16                    __veor_s16
#define veor_s32                    __veor_s32
#define veor_s64                    __veor_s64
#define veor_s8                     __veor_s8
#define veor_u16                    __veor_u16
#define veor_u32                    __veor_u32
#define veor_u64                    __veor_u64
#define veor_u8                     __veor_u8
#define vorn_s16                    __vorn_s16
#define vorn_s32                    __vorn_s32
#define vorn_s64                    __vorn_s64
#define vorn_s8                     __vorn_s8
#define vorn_u16                    __vorn_u16
#define vorn_u32                    __vorn_u32
#define vorn_u64                    __vorn_u64
#define vorn_u8                     __vorn_u8
#define vbicq_s16                   __vbicq_s16
#define vbicq_s32                   __vbicq_s32
#define vbicq_s64                   __vbicq_s64
#define vbicq_s8                    __vbicq_s8
#define vbicq_u16                   __vbicq_u16
#define vbicq_u32                   __vbicq_u32
#define vbicq_u64                   __vbicq_u64
#define vbicq_u8                    __vbicq_u8
#define veorq_s16                   __veorq_s16
#define veorq_s32                   __veorq_s32
#define veorq_s64                   __veorq_s64
#define veorq_s8                    __veorq_s8
#define veorq_u16                   __veorq_u16
#define veorq_u32                   __veorq_u32
#define veorq_u64                   __veorq_u64
#define veorq_u8                    __veorq_u8
#define vornq_s16                   __vornq_s16
#define vornq_s32                   __vornq_s32
#define vornq_s64                   __vornq_s64
#define vornq_s8                    __vornq_s8
#define vornq_u16                   __vornq_u16
#define vornq_u32                   __vornq_u32
#define vornq_u64                   __vornq_u64
#define vornq_u8                    __vornq_u8
#define vext_f32                    __vext_f32
#define vext_p16                    __vext_p16
#define vext_p8                     __vext_p8
#define vext_s16                    __vext_s16
#define vext_s32                    __vext_s32
#define vext_s64                    __vext_s64
#define vext_s8                     __vext_s8
#define vext_u16                    __vext_u16
#define vext_u32                    __vext_u32
#define vext_u64                    __vext_u64
#define vext_u8                     __vext_u8
#define vextq_f32                   __vextq_f32
#define vextq_p16                   __vextq_p16
#define vextq_p8                    __vextq_p8
#define vextq_s16                   __vextq_s16
#define vextq_s32                   __vextq_s32
#define vextq_s64                   __vextq_s64
#define vextq_s8                    __vextq_s8
#define vextq_u16                   __vextq_u16
#define vextq_u32                   __vextq_u32
#define vextq_u64                   __vextq_u64
#define vextq_u8                    __vextq_u8
#define vget_high_f32               __vget_high_f32
#define vget_high_p16               __vget_high_p16
#define vget_high_p8                __vget_high_p8
#define vget_high_s16               __vget_high_s16
#define vget_high_s32               __vget_high_s32
#define vget_high_s64               __vget_high_s64
#define vget_high_s8                __vget_high_s8
#define vget_high_u16               __vget_high_u16
#define vget_high_u32               __vget_high_u32
#define vget_high_u64               __vget_high_u64
#define vget_high_u8                __vget_high_u8
#define vget_low_f32                __vget_low_f32
#define vget_low_p16                __vget_low_p16
#define vget_low_p8                 __vget_low_p8
#define vget_low_s16                __vget_low_s16
#define vget_low_s32                __vget_low_s32
#define vget_low_s64                __vget_low_s64
#define vget_low_s8                 __vget_low_s8
#define vget_low_u16                __vget_low_u16
#define vget_low_u32                __vget_low_u32
#define vget_low_u64                __vget_low_u64
#define vget_low_u8                 __vget_low_u8
#define vhadd_s16                   __vhadd_s16
#define vhadd_s32                   __vhadd_s32
#define vhadd_s8                    __vhadd_s8
#define vhadd_u16                   __vhadd_u16
#define vhadd_u32                   __vhadd_u32
#define vhadd_u8                    __vhadd_u8
#define vhsub_s16                   __vhsub_s16
#define vhsub_s32                   __vhsub_s32
#define vhsub_s8                    __vhsub_s8
#define vhsub_u16                   __vhsub_u16
#define vhsub_u32                   __vhsub_u32
#define vhsub_u8                    __vhsub_u8
#define vrhadd_s16                  __vrhadd_s16
#define vrhadd_s32                  __vrhadd_s32
#define vrhadd_s8                   __vrhadd_s8
#define vrhadd_u16                  __vrhadd_u16
#define vrhadd_u32                  __vrhadd_u32
#define vrhadd_u8                   __vrhadd_u8
#define vhaddq_s16                  __vhaddq_s16
#define vhaddq_s32                  __vhaddq_s32
#define vhaddq_s8                   __vhaddq_s8
#define vhaddq_u16                  __vhaddq_u16
#define vhaddq_u32                  __vhaddq_u32
#define vhaddq_u8                   __vhaddq_u8
#define vhsubq_s16                  __vhsubq_s16
#define vhsubq_s32                  __vhsubq_s32
#define vhsubq_s8                   __vhsubq_s8
#define vhsubq_u16                  __vhsubq_u16
#define vhsubq_u32                  __vhsubq_u32
#define vhsubq_u8                   __vhsubq_u8
#define vrhaddq_s16                 __vrhaddq_s16
#define vrhaddq_s32                 __vrhaddq_s32
#define vrhaddq_s8                  __vrhaddq_s8
#define vrhaddq_u16                 __vrhaddq_u16
#define vrhaddq_u32                 __vrhaddq_u32
#define vrhaddq_u8                  __vrhaddq_u8
#define vld1_f32                    __vld1_f32
#define vld1_p16                    __vld1_p16
#define vld1_p8                     __vld1_p8
#define vld1_s16                    __vld1_s16
#define vld1_s32                    __vld1_s32
#define vld1_s64                    __vld1_s64
#define vld1_s8                     __vld1_s8
#define vld1_u16                    __vld1_u16
#define vld1_u32                    __vld1_u32
#define vld1_u64                    __vld1_u64
#define vld1_u8                     __vld1_u8
#define vld1_f32_ex                 __vld1_f32_ex
#define vld1_p16_ex                 __vld1_p16_ex
#define vld1_p8_ex                  __vld1_p8_ex
#define vld1_s16_ex                 __vld1_s16_ex
#define vld1_s32_ex                 __vld1_s32_ex
#define vld1_s64_ex                 __vld1_s64_ex
#define vld1_s8_ex                  __vld1_s8_ex
#define vld1_u16_ex                 __vld1_u16_ex
#define vld1_u32_ex                 __vld1_u32_ex
#define vld1_u64_ex                 __vld1_u64_ex
#define vld1_u8_ex                  __vld1_u8_ex
#define vld1q_f32                   __vld1q_f32
#define vld1q_p16                   __vld1q_p16
#define vld1q_p8                    __vld1q_p8
#define vld1q_s16                   __vld1q_s16
#define vld1q_s32                   __vld1q_s32
#define vld1q_s64                   __vld1q_s64
#define vld1q_s8                    __vld1q_s8
#define vld1q_u16                   __vld1q_u16
#define vld1q_u32                   __vld1q_u32
#define vld1q_u64                   __vld1q_u64
#define vld1q_u8                    __vld1q_u8
#define vld1q_f32_ex                __vld1q_f32_ex
#define vld1q_p16_ex                __vld1q_p16_ex
#define vld1q_p8_ex                 __vld1q_p8_ex
#define vld1q_s16_ex                __vld1q_s16_ex
#define vld1q_s32_ex                __vld1q_s32_ex
#define vld1q_s64_ex                __vld1q_s64_ex
#define vld1q_s8_ex                 __vld1q_s8_ex
#define vld1q_u16_ex                __vld1q_u16_ex
#define vld1q_u32_ex                __vld1q_u32_ex
#define vld1q_u64_ex                __vld1q_u64_ex
#define vld1q_u8_ex                 __vld1q_u8_ex
#define vld1_dup_f32                __vld1_dup_f32
#define vld1_dup_p16                __vld1_dup_p16
#define vld1_dup_p8                 __vld1_dup_p8
#define vld1_dup_s16                __vld1_dup_s16
#define vld1_dup_s32                __vld1_dup_s32
#define vld1_dup_s8                 __vld1_dup_s8
#define vld1_dup_u16                __vld1_dup_u16
#define vld1_dup_u32                __vld1_dup_u32
#define vld1_dup_u8                 __vld1_dup_u8
#define vld1q_dup_f32               __vld1q_dup_f32
#define vld1q_dup_p16               __vld1q_dup_p16
#define vld1q_dup_p8                __vld1q_dup_p8
#define vld1q_dup_s16               __vld1q_dup_s16
#define vld1q_dup_s32               __vld1q_dup_s32
#define vld1q_dup_s8                __vld1q_dup_s8
#define vld1q_dup_u16               __vld1q_dup_u16
#define vld1q_dup_u32               __vld1q_dup_u32
#define vld1q_dup_u8                __vld1q_dup_u8
#define vld1_dup_f32_ex             __vld1_dup_f32_ex
#define vld1_dup_p16_ex             __vld1_dup_p16_ex
#define vld1_dup_s16_ex             __vld1_dup_s16_ex
#define vld1_dup_s32_ex             __vld1_dup_s32_ex
#define vld1_dup_u16_ex             __vld1_dup_u16_ex
#define vld1_dup_u32_ex             __vld1_dup_u32_ex
#define vld1q_dup_f32_ex            __vld1q_dup_f32_ex
#define vld1q_dup_p16_ex            __vld1q_dup_p16_ex
#define vld1q_dup_s16_ex            __vld1q_dup_s16_ex
#define vld1q_dup_s32_ex            __vld1q_dup_s32_ex
#define vld1q_dup_u16_ex            __vld1q_dup_u16_ex
#define vld1q_dup_u32_ex            __vld1q_dup_u32_ex
#define vld1_lane_f32               __vld1_lane_f32
#define vld1_lane_p16               __vld1_lane_p16
#define vld1_lane_p8                __vld1_lane_p8
#define vld1_lane_s16               __vld1_lane_s16
#define vld1_lane_s32               __vld1_lane_s32
#define vld1_lane_s8                __vld1_lane_s8
#define vld1_lane_u16               __vld1_lane_u16
#define vld1_lane_u32               __vld1_lane_u32
#define vld1_lane_u8                __vld1_lane_u8
#define vld1q_lane_f32              __vld1q_lane_f32
#define vld1q_lane_p16              __vld1q_lane_p16
#define vld1q_lane_p8               __vld1q_lane_p8
#define vld1q_lane_s16              __vld1q_lane_s16
#define vld1q_lane_s32              __vld1q_lane_s32
#define vld1q_lane_s8               __vld1q_lane_s8
#define vld1q_lane_u16              __vld1q_lane_u16
#define vld1q_lane_u32              __vld1q_lane_u32
#define vld1q_lane_u8               __vld1q_lane_u8
#define vld1_lane_f32_ex            __vld1_lane_f32_ex
#define vld1_lane_p16_ex            __vld1_lane_p16_ex
#define vld1_lane_s16_ex            __vld1_lane_s16_ex
#define vld1_lane_s32_ex            __vld1_lane_s32_ex
#define vld1_lane_u16_ex            __vld1_lane_u16_ex
#define vld1_lane_u32_ex            __vld1_lane_u32_ex
#define vld1q_lane_f32_ex           __vld1q_lane_f32_ex
#define vld1q_lane_p16_ex           __vld1q_lane_p16_ex
#define vld1q_lane_s16_ex           __vld1q_lane_s16_ex
#define vld1q_lane_s32_ex           __vld1q_lane_s32_ex
#define vld1q_lane_u16_ex           __vld1q_lane_u16_ex
#define vld1q_lane_u32_ex           __vld1q_lane_u32_ex
#define vld2_f32                    __vld2_f32
#define vld2_p16                    __vld2_p16
#define vld2_p8                     __vld2_p8
#define vld2_s16                    __vld2_s16
#define vld2_s32                    __vld2_s32
#define vld2_s8                     __vld2_s8
#define vld2_u16                    __vld2_u16
#define vld2_u32                    __vld2_u32
#define vld2_u8                     __vld2_u8
#define vld2_s64                    __vld2_s64
#define vld2_u64                    __vld2_u64
#define vld2_s64_ex                 __vld2_s64_ex
#define vld2_u64_ex                 __vld2_u64_ex
#define vld2_f32_ex                 __vld2_f32_ex
#define vld2_p16_ex                 __vld2_p16_ex
#define vld2_p8_ex                  __vld2_p8_ex
#define vld2_s16_ex                 __vld2_s16_ex
#define vld2_s32_ex                 __vld2_s32_ex
#define vld2_s8_ex                  __vld2_s8_ex
#define vld2_u16_ex                 __vld2_u16_ex
#define vld2_u32_ex                 __vld2_u32_ex
#define vld2_u8_ex                  __vld2_u8_ex
#define vld2q_f32                   __vld2q_f32
#define vld2q_p16                   __vld2q_p16
#define vld2q_p8                    __vld2q_p8
#define vld2q_s16                   __vld2q_s16
#define vld2q_s32                   __vld2q_s32
#define vld2q_s8                    __vld2q_s8
#define vld2q_u16                   __vld2q_u16
#define vld2q_u32                   __vld2q_u32
#define vld2q_u8                    __vld2q_u8
#define vld2q_f32_ex                __vld2q_f32_ex
#define vld2q_p16_ex                __vld2q_p16_ex
#define vld2q_p8_ex                 __vld2q_p8_ex
#define vld2q_s16_ex                __vld2q_s16_ex
#define vld2q_s32_ex                __vld2q_s32_ex
#define vld2q_s8_ex                 __vld2q_s8_ex
#define vld2q_u16_ex                __vld2q_u16_ex
#define vld2q_u32_ex                __vld2q_u32_ex
#define vld2q_u8_ex                 __vld2q_u8_ex
#define vld2_dup_f32                __vld2_dup_f32
#define vld2_dup_p16                __vld2_dup_p16
#define vld2_dup_p8                 __vld2_dup_p8
#define vld2_dup_s16                __vld2_dup_s16
#define vld2_dup_s32                __vld2_dup_s32
#define vld2_dup_s8                 __vld2_dup_s8
#define vld2_dup_u16                __vld2_dup_u16
#define vld2_dup_u32                __vld2_dup_u32
#define vld2_dup_u8                 __vld2_dup_u8
#define vld2_dup_s64                __vld2_dup_s64
#define vld2_dup_u64                __vld2_dup_u64
#define vld2_dup_s64_ex             __vld2_dup_s64_ex
#define vld2_dup_u64_ex             __vld2_dup_u64_ex
#define vld2_dup_f32_ex             __vld2_dup_f32_ex
#define vld2_dup_p16_ex             __vld2_dup_p16_ex
#define vld2_dup_p8_ex              __vld2_dup_p8_ex
#define vld2_dup_s16_ex             __vld2_dup_s16_ex
#define vld2_dup_s32_ex             __vld2_dup_s32_ex
#define vld2_dup_s8_ex              __vld2_dup_s8_ex
#define vld2_dup_u16_ex             __vld2_dup_u16_ex
#define vld2_dup_u32_ex             __vld2_dup_u32_ex
#define vld2_dup_u8_ex              __vld2_dup_u8_ex
#define vld2_lane_f32               __vld2_lane_f32
#define vld2_lane_p16               __vld2_lane_p16
#define vld2_lane_p8                __vld2_lane_p8
#define vld2_lane_s16               __vld2_lane_s16
#define vld2_lane_s32               __vld2_lane_s32
#define vld2_lane_s8                __vld2_lane_s8
#define vld2_lane_u16               __vld2_lane_u16
#define vld2_lane_u32               __vld2_lane_u32
#define vld2_lane_u8                __vld2_lane_u8
#define vld2q_lane_f32              __vld2q_lane_f32
#define vld2q_lane_p16              __vld2q_lane_p16
#define vld2q_lane_s16              __vld2q_lane_s16
#define vld2q_lane_s32              __vld2q_lane_s32
#define vld2q_lane_u16              __vld2q_lane_u16
#define vld2q_lane_u32              __vld2q_lane_u32
#define vld2_lane_f32_ex            __vld2_lane_f32_ex
#define vld2_lane_p16_ex            __vld2_lane_p16_ex
#define vld2_lane_p8_ex             __vld2_lane_p8_ex
#define vld2_lane_s16_ex            __vld2_lane_s16_ex
#define vld2_lane_s32_ex            __vld2_lane_s32_ex
#define vld2_lane_s8_ex             __vld2_lane_s8_ex
#define vld2_lane_u16_ex            __vld2_lane_u16_ex
#define vld2_lane_u32_ex            __vld2_lane_u32_ex
#define vld2_lane_u8_ex             __vld2_lane_u8_ex
#define vld2q_lane_f32_ex           __vld2q_lane_f32_ex
#define vld2q_lane_p16_ex           __vld2q_lane_p16_ex
#define vld2q_lane_s16_ex           __vld2q_lane_s16_ex
#define vld2q_lane_s32_ex           __vld2q_lane_s32_ex
#define vld2q_lane_u16_ex           __vld2q_lane_u16_ex
#define vld2q_lane_u32_ex           __vld2q_lane_u32_ex
#define vld3_f32                    __vld3_f32
#define vld3_p16                    __vld3_p16
#define vld3_p8                     __vld3_p8
#define vld3_s16                    __vld3_s16
#define vld3_s32                    __vld3_s32
#define vld3_s8                     __vld3_s8
#define vld3_u16                    __vld3_u16
#define vld3_u32                    __vld3_u32
#define vld3_u8                     __vld3_u8
#define vld3_s64                    __vld3_s64
#define vld3_u64                    __vld3_u64
#define vld3_s64_ex                 __vld3_s64_ex
#define vld3_u64_ex                 __vld3_u64_ex
#define vld3_f32_ex                 __vld3_f32_ex
#define vld3_p16_ex                 __vld3_p16_ex
#define vld3_p8_ex                  __vld3_p8_ex
#define vld3_s16_ex                 __vld3_s16_ex
#define vld3_s32_ex                 __vld3_s32_ex
#define vld3_s8_ex                  __vld3_s8_ex
#define vld3_u16_ex                 __vld3_u16_ex
#define vld3_u32_ex                 __vld3_u32_ex
#define vld3_u8_ex                  __vld3_u8_ex
#define vld3q_f32                   __vld3q_f32
#define vld3q_p16                   __vld3q_p16
#define vld3q_p8                    __vld3q_p8
#define vld3q_s16                   __vld3q_s16
#define vld3q_s32                   __vld3q_s32
#define vld3q_s8                    __vld3q_s8
#define vld3q_u16                   __vld3q_u16
#define vld3q_u32                   __vld3q_u32
#define vld3q_u8                    __vld3q_u8
#define vld3q_f32_ex                __vld3q_f32_ex
#define vld3q_p16_ex                __vld3q_p16_ex
#define vld3q_p8_ex                 __vld3q_p8_ex
#define vld3q_s16_ex                __vld3q_s16_ex
#define vld3q_s32_ex                __vld3q_s32_ex
#define vld3q_s8_ex                 __vld3q_s8_ex
#define vld3q_u16_ex                __vld3q_u16_ex
#define vld3q_u32_ex                __vld3q_u32_ex
#define vld3q_u8_ex                 __vld3q_u8_ex
#define vld3_dup_f32                __vld3_dup_f32
#define vld3_dup_p16                __vld3_dup_p16
#define vld3_dup_p8                 __vld3_dup_p8
#define vld3_dup_s16                __vld3_dup_s16
#define vld3_dup_s32                __vld3_dup_s32
#define vld3_dup_s8                 __vld3_dup_s8
#define vld3_dup_u16                __vld3_dup_u16
#define vld3_dup_u32                __vld3_dup_u32
#define vld3_dup_u8                 __vld3_dup_u8
#define vld3_dup_s64                __vld3_dup_s64
#define vld3_dup_u64                __vld3_dup_u64
#define vld3_lane_f32               __vld3_lane_f32
#define vld3_lane_p16               __vld3_lane_p16
#define vld3_lane_p8                __vld3_lane_p8
#define vld3_lane_s16               __vld3_lane_s16
#define vld3_lane_s32               __vld3_lane_s32
#define vld3_lane_s8                __vld3_lane_s8
#define vld3_lane_u16               __vld3_lane_u16
#define vld3_lane_u32               __vld3_lane_u32
#define vld3_lane_u8                __vld3_lane_u8
#define vld3q_lane_f32              __vld3q_lane_f32
#define vld3q_lane_p16              __vld3q_lane_p16
#define vld3q_lane_s16              __vld3q_lane_s16
#define vld3q_lane_s32              __vld3q_lane_s32
#define vld3q_lane_u16              __vld3q_lane_u16
#define vld3q_lane_u32              __vld3q_lane_u32
#define vld4_f32                    __vld4_f32
#define vld4_p16                    __vld4_p16
#define vld4_p8                     __vld4_p8
#define vld4_s16                    __vld4_s16
#define vld4_s32                    __vld4_s32
#define vld4_s8                     __vld4_s8
#define vld4_u16                    __vld4_u16
#define vld4_u32                    __vld4_u32
#define vld4_u8                     __vld4_u8
#define vld4_s64                    __vld4_s64
#define vld4_u64                    __vld4_u64
#define vld4_s64_ex                 __vld4_s64_ex
#define vld4_u64_ex                 __vld4_u64_ex
#define vld4_f32_ex                 __vld4_f32_ex
#define vld4_p16_ex                 __vld4_p16_ex
#define vld4_p8_ex                  __vld4_p8_ex
#define vld4_s16_ex                 __vld4_s16_ex
#define vld4_s32_ex                 __vld4_s32_ex
#define vld4_s8_ex                  __vld4_s8_ex
#define vld4_u16_ex                 __vld4_u16_ex
#define vld4_u32_ex                 __vld4_u32_ex
#define vld4_u8_ex                  __vld4_u8_ex
#define vld4q_f32                   __vld4q_f32
#define vld4q_p16                   __vld4q_p16
#define vld4q_p8                    __vld4q_p8
#define vld4q_s16                   __vld4q_s16
#define vld4q_s32                   __vld4q_s32
#define vld4q_s8                    __vld4q_s8
#define vld4q_u16                   __vld4q_u16
#define vld4q_u32                   __vld4q_u32
#define vld4q_u8                    __vld4q_u8
#define vld4q_f32_ex                __vld4q_f32_ex
#define vld4q_p16_ex                __vld4q_p16_ex
#define vld4q_p8_ex                 __vld4q_p8_ex
#define vld4q_s16_ex                __vld4q_s16_ex
#define vld4q_s32_ex                __vld4q_s32_ex
#define vld4q_s8_ex                 __vld4q_s8_ex
#define vld4q_u16_ex                __vld4q_u16_ex
#define vld4q_u32_ex                __vld4q_u32_ex
#define vld4q_u8_ex                 __vld4q_u8_ex
#define vld4_dup_f32                __vld4_dup_f32
#define vld4_dup_p16                __vld4_dup_p16
#define vld4_dup_p8                 __vld4_dup_p8
#define vld4_dup_s16                __vld4_dup_s16
#define vld4_dup_s32                __vld4_dup_s32
#define vld4_dup_s8                 __vld4_dup_s8
#define vld4_dup_u16                __vld4_dup_u16
#define vld4_dup_u32                __vld4_dup_u32
#define vld4_dup_u8                 __vld4_dup_u8
#define vld4_dup_s64                __vld4_dup_s64
#define vld4_dup_u64                __vld4_dup_u64
#define vld4_dup_f32_ex             __vld4_dup_f32_ex
#define vld4_dup_p16_ex             __vld4_dup_p16_ex
#define vld4_dup_p8_ex              __vld4_dup_p8_ex
#define vld4_dup_s16_ex             __vld4_dup_s16_ex
#define vld4_dup_s32_ex             __vld4_dup_s32_ex
#define vld4_dup_s8_ex              __vld4_dup_s8_ex
#define vld4_dup_u16_ex             __vld4_dup_u16_ex
#define vld4_dup_u32_ex             __vld4_dup_u32_ex
#define vld4_dup_u8_ex              __vld4_dup_u8_ex
#define vld4_lane_f32               __vld4_lane_f32
#define vld4_lane_p16               __vld4_lane_p16
#define vld4_lane_p8                __vld4_lane_p8
#define vld4_lane_s16               __vld4_lane_s16
#define vld4_lane_s32               __vld4_lane_s32
#define vld4_lane_s8                __vld4_lane_s8
#define vld4_lane_u16               __vld4_lane_u16
#define vld4_lane_u32               __vld4_lane_u32
#define vld4_lane_u8                __vld4_lane_u8
#define vld4q_lane_f32              __vld4q_lane_f32
#define vld4q_lane_p16              __vld4q_lane_p16
#define vld4q_lane_s16              __vld4q_lane_s16
#define vld4q_lane_s32              __vld4q_lane_s32
#define vld4q_lane_u16              __vld4q_lane_u16
#define vld4q_lane_u32              __vld4q_lane_u32
#define vld4_lane_f32_ex            __vld4_lane_f32_ex
#define vld4_lane_p16_ex            __vld4_lane_p16_ex
#define vld4_lane_p8_ex             __vld4_lane_p8_ex
#define vld4_lane_s16_ex            __vld4_lane_s16_ex
#define vld4_lane_s32_ex            __vld4_lane_s32_ex
#define vld4_lane_s8_ex             __vld4_lane_s8_ex
#define vld4_lane_u16_ex            __vld4_lane_u16_ex
#define vld4_lane_u32_ex            __vld4_lane_u32_ex
#define vld4_lane_u8_ex             __vld4_lane_u8_ex
#define vld4q_lane_f32_ex           __vld4q_lane_f32_ex
#define vld4q_lane_p16_ex           __vld4q_lane_p16_ex
#define vld4q_lane_s16_ex           __vld4q_lane_s16_ex
#define vld4q_lane_s32_ex           __vld4q_lane_s32_ex
#define vld4q_lane_u16_ex           __vld4q_lane_u16_ex
#define vld4q_lane_u32_ex           __vld4q_lane_u32_ex
#define vmax_f32                    __vmax_f32
#define vmaxnm_f32                  __vmaxnm_f32
#define vmin_f32                    __vmin_f32
#define vminnm_f32                  __vminnm_f32
#define vmaxq_f32                   __vmaxq_f32
#define vmaxnmq_f32                 __vmaxnmq_f32
#define vminq_f32                   __vminq_f32
#define vminnmq_f32                 __vminnmq_f32
#define vmax_s16                    __vmax_s16
#define vmax_s32                    __vmax_s32
#define vmax_s8                     __vmax_s8
#define vmax_u16                    __vmax_u16
#define vmax_u32                    __vmax_u32
#define vmax_u8                     __vmax_u8
#define vmin_s16                    __vmin_s16
#define vmin_s32                    __vmin_s32
#define vmin_s8                     __vmin_s8
#define vmin_u16                    __vmin_u16
#define vmin_u32                    __vmin_u32
#define vmin_u8                     __vmin_u8
#define vmaxq_s16                   __vmaxq_s16
#define vmaxq_s32                   __vmaxq_s32
#define vmaxq_s8                    __vmaxq_s8
#define vmaxq_u16                   __vmaxq_u16
#define vmaxq_u32                   __vmaxq_u32
#define vmaxq_u8                    __vmaxq_u8
#define vminq_s16                   __vminq_s16
#define vminq_s32                   __vminq_s32
#define vminq_s8                    __vminq_s8
#define vminq_u16                   __vminq_u16
#define vminq_u32                   __vminq_u32
#define vminq_u8                    __vminq_u8
#define vmla_lane_f32               __vmla_lane_f32
#define vmla_lane_s16               __vmla_lane_s16
#define vmla_lane_s32               __vmla_lane_s32
#define vmla_lane_u16               __vmla_lane_u16
#define vmla_lane_u32               __vmla_lane_u32
#define vmls_lane_f32               __vmls_lane_f32
#define vmls_lane_s16               __vmls_lane_s16
#define vmls_lane_s32               __vmls_lane_s32
#define vmls_lane_u16               __vmls_lane_u16
#define vmls_lane_u32               __vmls_lane_u32
#define vmlaq_lane_f32              __vmlaq_lane_f32
#define vmlaq_lane_s16              __vmlaq_lane_s16
#define vmlaq_lane_s32              __vmlaq_lane_s32
#define vmlaq_lane_u16              __vmlaq_lane_u16
#define vmlaq_lane_u32              __vmlaq_lane_u32
#define vmlsq_lane_f32              __vmlsq_lane_f32
#define vmlsq_lane_s16              __vmlsq_lane_s16
#define vmlsq_lane_s32              __vmlsq_lane_s32
#define vmlsq_lane_u16              __vmlsq_lane_u16
#define vmlsq_lane_u32              __vmlsq_lane_u32
#define vmla_n_f32                  __vmla_n_f32
#define vmls_n_f32                  __vmls_n_f32
#define vmlaq_n_f32                 __vmlaq_n_f32
#define vmlsq_n_f32                 __vmlsq_n_f32
#define vmla_f32                    __vmla_f32
#define vfma_f32                    __vmla_f32
#define vfmaq_f32                   __vmlaq_f32
#define vmls_f32                    __vmls_f32
#define vfms_f32                    __vmls_f32
#define vfmsq_f32                   __vmlsq_f32
#define vmlaq_f32                   __vmlaq_f32
#define vmlsq_f32                   __vmlsq_f32
#define vmla_s16                    __vmla_s16
#define vmla_s32                    __vmla_s32
#define vmla_s8                     __vmla_s8
#define vmla_u16                    __vmla_u16
#define vmla_u32                    __vmla_u32
#define vmla_u8                     __vmla_u8
#define vmls_s16                    __vmls_s16
#define vmls_s32                    __vmls_s32
#define vmls_s8                     __vmls_s8
#define vmls_u16                    __vmls_u16
#define vmls_u32                    __vmls_u32
#define vmls_u8                     __vmls_u8
#define vmlaq_s16                   __vmlaq_s16
#define vmlaq_s32                   __vmlaq_s32
#define vmlaq_s8                    __vmlaq_s8
#define vmlaq_u16                   __vmlaq_u16
#define vmlaq_u32                   __vmlaq_u32
#define vmlaq_u8                    __vmlaq_u8
#define vmlsq_s16                   __vmlsq_s16
#define vmlsq_s32                   __vmlsq_s32
#define vmlsq_s8                    __vmlsq_s8
#define vmlsq_u16                   __vmlsq_u16
#define vmlsq_u32                   __vmlsq_u32
#define vmlsq_u8                    __vmlsq_u8
#define vmlal_s16                   __vmlal_s16
#define vmlal_s32                   __vmlal_s32
#define vmlal_s8                    __vmlal_s8
#define vmlal_u16                   __vmlal_u16
#define vmlal_u32                   __vmlal_u32
#define vmlal_u8                    __vmlal_u8
#define vmlsl_s16                   __vmlsl_s16
#define vmlsl_s32                   __vmlsl_s32
#define vmlsl_s8                    __vmlsl_s8
#define vmlsl_u16                   __vmlsl_u16
#define vmlsl_u32                   __vmlsl_u32
#define vmlsl_u8                    __vmlsl_u8
#define vmlal_lane_s16              __vmlal_lane_s16
#define vmlal_lane_s32              __vmlal_lane_s32
#define vmlal_lane_u16              __vmlal_lane_u16
#define vmlal_lane_u32              __vmlal_lane_u32
#define vmlsl_lane_s16              __vmlsl_lane_s16
#define vmlsl_lane_s32              __vmlsl_lane_s32
#define vmlsl_lane_u16              __vmlsl_lane_u16
#define vmlsl_lane_u32              __vmlsl_lane_u32
#define vset_lane_f32               __vset_lane_f32
#define vset_lane_p16               __vset_lane_p16
#define vset_lane_p8                __vset_lane_p8
#define vset_lane_s16               __vset_lane_s16
#define vset_lane_s32               __vset_lane_s32
#define vset_lane_s8                __vset_lane_s8
#define vset_lane_u16               __vset_lane_u16
#define vset_lane_u32               __vset_lane_u32
#define vset_lane_u8                __vset_lane_u8
#define vget_lane_f32               __vget_lane_f32
#define vget_lane_p16               __vget_lane_p16
#define vget_lane_p8                __vget_lane_p8
#define vget_lane_s16               __vget_lane_s16
#define vget_lane_s8                __vget_lane_s8
#define vget_lane_s32               __vget_lane_s32
#define vget_lane_u16               __vget_lane_u16
#define vget_lane_u8                __vget_lane_u8
#define vget_lane_u32               __vget_lane_u32
#define vset_lane_s64               __vset_lane_s64
#define vset_lane_u64               __vset_lane_u64
#define vsetq_lane_s64              __vsetq_lane_s64
#define vsetq_lane_u64              __vsetq_lane_u64
#define vget_lane_s64               __vget_lane_s64
#define vget_lane_u64               __vget_lane_u64
#define vgetq_lane_s64              __vgetq_lane_s64
#define vgetq_lane_u64              __vgetq_lane_u64
#define vsetq_lane_f32              __vsetq_lane_f32
#define vsetq_lane_p16              __vsetq_lane_p16
#define vsetq_lane_p8               __vsetq_lane_p8
#define vsetq_lane_s16              __vsetq_lane_s16
#define vsetq_lane_s32              __vsetq_lane_s32
#define vsetq_lane_s8               __vsetq_lane_s8
#define vsetq_lane_u16              __vsetq_lane_u16
#define vsetq_lane_u32              __vsetq_lane_u32
#define vsetq_lane_u8               __vsetq_lane_u8
#define vgetq_lane_f32              __vgetq_lane_f32
#define vgetq_lane_p16              __vgetq_lane_p16
#define vgetq_lane_p8               __vgetq_lane_p8
#define vgetq_lane_s16              __vgetq_lane_s16
#define vgetq_lane_s8               __vgetq_lane_s8
#define vgetq_lane_s32              __vgetq_lane_s32
#define vgetq_lane_u16              __vgetq_lane_u16
#define vgetq_lane_u8               __vgetq_lane_u8
#define vgetq_lane_u32              __vgetq_lane_u32
#define vmovl_s16                   __vmovl_s16
#define vmovl_s32                   __vmovl_s32
#define vmovl_s8                    __vmovl_s8
#define vmovl_u16                   __vmovl_u16
#define vmovl_u32                   __vmovl_u32
#define vmovl_u8                    __vmovl_u8
#define vmovn_s16                   __vmovn_s16
#define vmovn_s32                   __vmovn_s32
#define vmovn_s64                   __vmovn_s64
#define vmovn_u16                   __vmovn_u16
#define vmovn_u32                   __vmovn_u32
#define vmovn_u64                   __vmovn_u64
#define vmul_f32                    __vmul_f32
#define vmul_p8                     __vmul_p8
#define vmul_s16                    __vmul_s16
#define vmul_s32                    __vmul_s32
#define vmul_s8                     __vmul_s8
#define vmul_u16                    __vmul_u16
#define vmul_u32                    __vmul_u32
#define vmul_u8                     __vmul_u8
#define vmulq_f32                   __vmulq_f32
#define vmulq_p8                    __vmulq_p8
#define vmulq_s16                   __vmulq_s16
#define vmulq_s32                   __vmulq_s32
#define vmulq_s8                    __vmulq_s8
#define vmulq_u16                   __vmulq_u16
#define vmulq_u32                   __vmulq_u32
#define vmulq_u8                    __vmulq_u8
#define vmul_n_f32                  __vmul_n_f32
#define vmulq_n_f32                 __vmulq_n_f32
#define vmul_lane_f32               __vmul_lane_f32
#define vmul_lane_s16               __vmul_lane_s16
#define vmul_lane_s32               __vmul_lane_s32
#define vmul_lane_u16               __vmul_lane_u16
#define vmul_lane_u32               __vmul_lane_u32
#define vmulq_lane_f32              __vmulq_lane_f32
#define vmulq_lane_s16              __vmulq_lane_s16
#define vmulq_lane_s32              __vmulq_lane_s32
#define vmulq_lane_u16              __vmulq_lane_u16
#define vmulq_lane_u32              __vmulq_lane_u32
#define vmull_p64                   __vmull_p64
#define vmull_p8                    __vmull_p8
#define vmull_s16                   __vmull_s16
#define vmull_s32                   __vmull_s32
#define vmull_s8                    __vmull_s8
#define vmull_u16                   __vmull_u16
#define vmull_u32                   __vmull_u32
#define vmull_u8                    __vmull_u8
#define vmull_lane_s16              __vmull_lane_s16
#define vmull_lane_s32              __vmull_lane_s32
#define vmull_lane_u16              __vmull_lane_u16
#define vmull_lane_u32              __vmull_lane_u32
#define vmvn_p16                    __vmvn_p16
#define vmvn_p8                     __vmvn_p8
#define vmvn_s16                    __vmvn_s16
#define vmvn_s32                    __vmvn_s32
#define vmvn_s8                     __vmvn_s8
#define vmvn_u16                    __vmvn_u16
#define vmvn_u32                    __vmvn_u32
#define vmvn_u8                     __vmvn_u8
#define vmvnq_p16                   __vmvnq_p16
#define vmvnq_p8                    __vmvnq_p8
#define vmvnq_s16                   __vmvnq_s16
#define vmvnq_s32                   __vmvnq_s32
#define vmvnq_s8                    __vmvnq_s8
#define vmvnq_u16                   __vmvnq_u16
#define vmvnq_u32                   __vmvnq_u32
#define vmvnq_u8                    __vmvnq_u8
#define vpadal_s16                  __vpadal_s16
#define vpadal_s32                  __vpadal_s32
#define vpadal_s8                   __vpadal_s8
#define vpadal_u16                  __vpadal_u16
#define vpadal_u32                  __vpadal_u32
#define vpadal_u8                   __vpadal_u8
#define vpadalq_s16                 __vpadalq_s16
#define vpadalq_s32                 __vpadalq_s32
#define vpadalq_s8                  __vpadalq_s8
#define vpadalq_u16                 __vpadalq_u16
#define vpadalq_u32                 __vpadalq_u32
#define vpadalq_u8                  __vpadalq_u8
#define vpadd_f32                   __vpadd_f32
#define vpadd_s16                   __vpadd_s16
#define vpadd_s32                   __vpadd_s32
#define vpadd_s8                    __vpadd_s8
#define vpadd_u16                   __vpadd_u16
#define vpadd_u32                   __vpadd_u32
#define vpadd_u8                    __vpadd_u8
#define vpaddl_s16                  __vpaddl_s16
#define vpaddl_s32                  __vpaddl_s32
#define vpaddl_s8                   __vpaddl_s8
#define vpaddl_u16                  __vpaddl_u16
#define vpaddl_u32                  __vpaddl_u32
#define vpaddl_u8                   __vpaddl_u8
#define vpaddlq_s16                 __vpaddlq_s16
#define vpaddlq_s32                 __vpaddlq_s32
#define vpaddlq_s8                  __vpaddlq_s8
#define vpaddlq_u16                 __vpaddlq_u16
#define vpaddlq_u32                 __vpaddlq_u32
#define vpaddlq_u8                  __vpaddlq_u8
#define vpmax_f32                   __vpmax_f32
#define vpmin_f32                   __vpmin_f32
#define vpmax_s16                   __vpmax_s16
#define vpmax_s32                   __vpmax_s32
#define vpmax_s8                    __vpmax_s8
#define vpmax_u16                   __vpmax_u16
#define vpmax_u32                   __vpmax_u32
#define vpmax_u8                    __vpmax_u8
#define vpmin_s16                   __vpmin_s16
#define vpmin_s32                   __vpmin_s32
#define vpmin_s8                    __vpmin_s8
#define vpmin_u16                   __vpmin_u16
#define vpmin_u32                   __vpmin_u32
#define vpmin_u8                    __vpmin_u8
#define vqabs_s16                   __vqabs_s16
#define vqabs_s32                   __vqabs_s32
#define vqabs_s8                    __vqabs_s8
#define vqneg_s16                   __vqneg_s16
#define vqneg_s32                   __vqneg_s32
#define vqneg_s8                    __vqneg_s8
#define vqabsq_s16                  __vqabsq_s16
#define vqabsq_s32                  __vqabsq_s32
#define vqabsq_s8                   __vqabsq_s8
#define vqnegq_s16                  __vqnegq_s16
#define vqnegq_s32                  __vqnegq_s32
#define vqnegq_s8                   __vqnegq_s8
#define vqadd_s16                   __vqadd_s16
#define vqadd_s32                   __vqadd_s32
#define vqadd_s64                   __vqadd_s64
#define vqadd_s8                    __vqadd_s8
#define vqadd_u16                   __vqadd_u16
#define vqadd_u32                   __vqadd_u32
#define vqadd_u64                   __vqadd_u64
#define vqadd_u8                    __vqadd_u8
#define vqaddq_s16                  __vqaddq_s16
#define vqaddq_s32                  __vqaddq_s32
#define vqaddq_s64                  __vqaddq_s64
#define vqaddq_s8                   __vqaddq_s8
#define vqaddq_u16                  __vqaddq_u16
#define vqaddq_u32                  __vqaddq_u32
#define vqaddq_u64                  __vqaddq_u64
#define vqaddq_u8                   __vqaddq_u8
#define vqdmlal_s16                 __vqdmlal_s16
#define vqdmlal_s32                 __vqdmlal_s32
#define vqdmlsl_s16                 __vqdmlsl_s16
#define vqdmlsl_s32                 __vqdmlsl_s32
#define vqdmlal_lane_s16            __vqdmlal_lane_s16
#define vqdmlal_lane_s32            __vqdmlal_lane_s32
#define vqdmlsl_lane_s16            __vqdmlsl_lane_s16
#define vqdmlsl_lane_s32            __vqdmlsl_lane_s32
#define vqdmulh_lane_s16            __vqdmulh_lane_s16
#define vqdmulh_lane_s32            __vqdmulh_lane_s32
#define vqrdmulh_lane_s16           __vqrdmulh_lane_s16
#define vqrdmulh_lane_s32           __vqrdmulh_lane_s32
#define vqdmulhq_lane_s16           __vqdmulhq_lane_s16
#define vqdmulhq_lane_s32           __vqdmulhq_lane_s32
#define vqrdmulhq_lane_s16          __vqrdmulhq_lane_s16
#define vqrdmulhq_lane_s32          __vqrdmulhq_lane_s32
#define vqdmulh_s16                 __vqdmulh_s16
#define vqdmulh_s32                 __vqdmulh_s32
#define vqrdmulh_s16                __vqrdmulh_s16
#define vqrdmulh_s32                __vqrdmulh_s32
#define vqdmulhq_s16                __vqdmulhq_s16
#define vqdmulhq_s32                __vqdmulhq_s32
#define vqrdmulhq_s16               __vqrdmulhq_s16
#define vqrdmulhq_s32               __vqrdmulhq_s32
#define vqdmull_s16                 __vqdmull_s16
#define vqdmull_s32                 __vqdmull_s32
#define vqdmull_lane_s16            __vqdmull_lane_s16
#define vqdmull_lane_s32            __vqdmull_lane_s32
#define vqmovn_s16                  __vqmovn_s16
#define vqmovn_s32                  __vqmovn_s32
#define vqmovn_s64                  __vqmovn_s64
#define vqmovn_u16                  __vqmovn_u16
#define vqmovn_u32                  __vqmovn_u32
#define vqmovn_u64                  __vqmovn_u64
#define vqmovun_s16                 __vqmovun_s16
#define vqmovun_s32                 __vqmovun_s32
#define vqmovun_s64                 __vqmovun_s64
#define vqshl_n_s16                 __vqshl_n_s16
#define vqshl_n_s32                 __vqshl_n_s32
#define vqshl_n_s64                 __vqshl_n_s64
#define vqshl_n_s8                  __vqshl_n_s8
#define vqshl_n_u16                 __vqshl_n_u16
#define vqshl_n_u32                 __vqshl_n_u32
#define vqshl_n_u64                 __vqshl_n_u64
#define vqshl_n_u8                  __vqshl_n_u8
#define vqshlu_n_s16                __vqshlu_n_s16
#define vqshlu_n_s32                __vqshlu_n_s32
#define vqshlu_n_s64                __vqshlu_n_s64
#define vqshlu_n_s8                 __vqshlu_n_s8
#define vqshlq_n_s16                __vqshlq_n_s16
#define vqshlq_n_s32                __vqshlq_n_s32
#define vqshlq_n_s64                __vqshlq_n_s64
#define vqshlq_n_s8                 __vqshlq_n_s8
#define vqshlq_n_u16                __vqshlq_n_u16
#define vqshlq_n_u32                __vqshlq_n_u32
#define vqshlq_n_u64                __vqshlq_n_u64
#define vqshlq_n_u8                 __vqshlq_n_u8
#define vqshluq_n_s16               __vqshluq_n_s16
#define vqshluq_n_s32               __vqshluq_n_s32
#define vqshluq_n_s64               __vqshluq_n_s64
#define vqshluq_n_s8                __vqshluq_n_s8
#define vqrshrn_n_s16               __vqrshrn_n_s16
#define vqrshrn_n_s32               __vqrshrn_n_s32
#define vqrshrn_n_s64               __vqrshrn_n_s64
#define vqrshrn_n_u16               __vqrshrn_n_u16
#define vqrshrn_n_u32               __vqrshrn_n_u32
#define vqrshrn_n_u64               __vqrshrn_n_u64
#define vqrshrun_n_s16              __vqrshrun_n_s16
#define vqrshrun_n_s32              __vqrshrun_n_s32
#define vqrshrun_n_s64              __vqrshrun_n_s64
#define vqshrn_n_s16                __vqshrn_n_s16
#define vqshrn_n_s32                __vqshrn_n_s32
#define vqshrn_n_s64                __vqshrn_n_s64
#define vqshrn_n_u16                __vqshrn_n_u16
#define vqshrn_n_u32                __vqshrn_n_u32
#define vqshrn_n_u64                __vqshrn_n_u64
#define vqshrun_n_s16               __vqshrun_n_s16
#define vqshrun_n_s32               __vqshrun_n_s32
#define vqshrun_n_s64               __vqshrun_n_s64
#define vqsub_s16                   __vqsub_s16
#define vqsub_s32                   __vqsub_s32
#define vqsub_s64                   __vqsub_s64
#define vqsub_s8                    __vqsub_s8
#define vqsub_u16                   __vqsub_u16
#define vqsub_u32                   __vqsub_u32
#define vqsub_u64                   __vqsub_u64
#define vqsub_u8                    __vqsub_u8
#define vqsubq_s16                  __vqsubq_s16
#define vqsubq_s32                  __vqsubq_s32
#define vqsubq_s64                  __vqsubq_s64
#define vqsubq_s8                   __vqsubq_s8
#define vqsubq_u16                  __vqsubq_u16
#define vqsubq_u32                  __vqsubq_u32
#define vqsubq_u64                  __vqsubq_u64
#define vqsubq_u8                   __vqsubq_u8
#define vrecpe_f32                  __vrecpe_f32
#define vrecpe_u32                  __vrecpe_u32
#define vrsqrte_f32                 __vrsqrte_f32
#define vrsqrte_u32                 __vrsqrte_u32
#define vrecpeq_f32                 __vrecpeq_f32
#define vrecpeq_u32                 __vrecpeq_u32
#define vrsqrteq_f32                __vrsqrteq_f32
#define vrsqrteq_u32                __vrsqrteq_u32
#define vrecps_f32                  __vrecps_f32
#define vrecpsq_f32                 __vrecpsq_f32
#define vrev16_p8                   __vrev16_p8
#define vrev16_s8                   __vrev16_s8
#define vrev16_u8                   __vrev16_u8
#define vrev32_p16                  __vrev32_p16
#define vrev32_p8                   __vrev32_p8
#define vrev32_s16                  __vrev32_s16
#define vrev32_s8                   __vrev32_s8
#define vrev32_u16                  __vrev32_u16
#define vrev32_u8                   __vrev32_u8
#define vrev64_f32                  __vrev64_f32
#define vrev64_p16                  __vrev64_p16
#define vrev64_p8                   __vrev64_p8
#define vrev64_s16                  __vrev64_s16
#define vrev64_s32                  __vrev64_s32
#define vrev64_s8                   __vrev64_s8
#define vrev64_u16                  __vrev64_u16
#define vrev64_u32                  __vrev64_u32
#define vrev64_u8                   __vrev64_u8
#define vrev16q_p8                  __vrev16q_p8
#define vrev16q_s8                  __vrev16q_s8
#define vrev16q_u8                  __vrev16q_u8
#define vrev32q_p16                 __vrev32q_p16
#define vrev32q_p8                  __vrev32q_p8
#define vrev32q_s16                 __vrev32q_s16
#define vrev32q_s8                  __vrev32q_s8
#define vrev32q_u16                 __vrev32q_u16
#define vrev32q_u8                  __vrev32q_u8
#define vrev64q_f32                 __vrev64q_f32
#define vrev64q_p16                 __vrev64q_p16
#define vrev64q_p8                  __vrev64q_p8
#define vrev64q_s16                 __vrev64q_s16
#define vrev64q_s32                 __vrev64q_s32
#define vrev64q_s8                  __vrev64q_s8
#define vrev64q_u16                 __vrev64q_u16
#define vrev64q_u32                 __vrev64q_u32
#define vrev64q_u8                  __vrev64q_u8
#define vrnd_f32                    __vrnd_f32
#define vrnda_f32                   __vrnda_f32
#define vrndm_f32                   __vrndm_f32
#define vrndn_f32                   __vrndn_f32
#define vrndp_f32                   __vrndp_f32
#define vrndx_f32                   __vrndx_f32
#define vrndq_f32                   __vrndq_f32
#define vrndaq_f32                  __vrndaq_f32
#define vrndmq_f32                  __vrndmq_f32
#define vrndnq_f32                  __vrndnq_f32
#define vrndpq_f32                  __vrndpq_f32
#define vrndxq_f32                  __vrndxq_f32
#define vrsqrts_f32                 __vrsqrts_f32
#define vrsqrtsq_f32                __vrsqrtsq_f32
#define vshl_n_s16                  __vshl_n_s16
#define vshl_n_s32                  __vshl_n_s32
#define vshl_n_s64                  __vshl_n_s64
#define vshl_n_s8                   __vshl_n_s8
#define vshl_n_u16                  __vshl_n_u16
#define vshl_n_u32                  __vshl_n_u32
#define vshl_n_u64                  __vshl_n_u64
#define vshl_n_u8                   __vshl_n_u8
#define vshlq_n_s16                 __vshlq_n_s16
#define vshlq_n_s32                 __vshlq_n_s32
#define vshlq_n_s64                 __vshlq_n_s64
#define vshlq_n_s8                  __vshlq_n_s8
#define vshlq_n_u16                 __vshlq_n_u16
#define vshlq_n_u32                 __vshlq_n_u32
#define vshlq_n_u64                 __vshlq_n_u64
#define vshlq_n_u8                  __vshlq_n_u8
#define vqrshl_s16                  __vqrshl_s16
#define vqrshl_s32                  __vqrshl_s32
#define vqrshl_s64                  __vqrshl_s64
#define vqrshl_s8                   __vqrshl_s8
#define vqrshl_u16                  __vqrshl_u16
#define vqrshl_u32                  __vqrshl_u32
#define vqrshl_u64                  __vqrshl_u64
#define vqrshl_u8                   __vqrshl_u8
#define vqshl_s16                   __vqshl_s16
#define vqshl_s32                   __vqshl_s32
#define vqshl_s64                   __vqshl_s64
#define vqshl_s8                    __vqshl_s8
#define vqshl_u16                   __vqshl_u16
#define vqshl_u32                   __vqshl_u32
#define vqshl_u64                   __vqshl_u64
#define vqshl_u8                    __vqshl_u8
#define vrshl_s16                   __vrshl_s16
#define vrshl_s32                   __vrshl_s32
#define vrshl_s64                   __vrshl_s64
#define vrshl_s8                    __vrshl_s8
#define vrshl_u16                   __vrshl_u16
#define vrshl_u32                   __vrshl_u32
#define vrshl_u64                   __vrshl_u64
#define vrshl_u8                    __vrshl_u8
#define vshl_s16                    __vshl_s16
#define vshl_s32                    __vshl_s32
#define vshl_s64                    __vshl_s64
#define vshl_s8                     __vshl_s8
#define vshl_u16                    __vshl_u16
#define vshl_u32                    __vshl_u32
#define vshl_u64                    __vshl_u64
#define vshl_u8                     __vshl_u8
#define vqrshlq_s16                 __vqrshlq_s16
#define vqrshlq_s32                 __vqrshlq_s32
#define vqrshlq_s64                 __vqrshlq_s64
#define vqrshlq_s8                  __vqrshlq_s8
#define vqrshlq_u16                 __vqrshlq_u16
#define vqrshlq_u32                 __vqrshlq_u32
#define vqrshlq_u64                 __vqrshlq_u64
#define vqrshlq_u8                  __vqrshlq_u8
#define vqshlq_s16                  __vqshlq_s16
#define vqshlq_s32                  __vqshlq_s32
#define vqshlq_s64                  __vqshlq_s64
#define vqshlq_s8                   __vqshlq_s8
#define vqshlq_u16                  __vqshlq_u16
#define vqshlq_u32                  __vqshlq_u32
#define vqshlq_u64                  __vqshlq_u64
#define vqshlq_u8                   __vqshlq_u8
#define vrshlq_s16                  __vrshlq_s16
#define vrshlq_s32                  __vrshlq_s32
#define vrshlq_s64                  __vrshlq_s64
#define vrshlq_s8                   __vrshlq_s8
#define vrshlq_u16                  __vrshlq_u16
#define vrshlq_u32                  __vrshlq_u32
#define vrshlq_u64                  __vrshlq_u64
#define vrshlq_u8                   __vrshlq_u8
#define vshlq_s16                   __vshlq_s16
#define vshlq_s32                   __vshlq_s32
#define vshlq_s64                   __vshlq_s64
#define vshlq_s8                    __vshlq_s8
#define vshlq_u16                   __vshlq_u16
#define vshlq_u32                   __vshlq_u32
#define vshlq_u64                   __vshlq_u64
#define vshlq_u8                    __vshlq_u8
#define vshll_n_s16                 __vshll_n_s16
#define vshll_n_s32                 __vshll_n_s32
#define vshll_n_s8                  __vshll_n_s8
#define vshll_n_u16                 __vshll_n_u16
#define vshll_n_u32                 __vshll_n_u32
#define vshll_n_u8                  __vshll_n_u8
#define vrshr_n_s16                 __vrshr_n_s16
#define vrshr_n_s32                 __vrshr_n_s32
#define vrshr_n_s64                 __vrshr_n_s64
#define vrshr_n_s8                  __vrshr_n_s8
#define vrshr_n_u16                 __vrshr_n_u16
#define vrshr_n_u32                 __vrshr_n_u32
#define vrshr_n_u64                 __vrshr_n_u64
#define vrshr_n_u8                  __vrshr_n_u8
#define vshr_n_s16                  __vshr_n_s16
#define vshr_n_s32                  __vshr_n_s32
#define vshr_n_s64                  __vshr_n_s64
#define vshr_n_s8                   __vshr_n_s8
#define vshr_n_u16                  __vshr_n_u16
#define vshr_n_u32                  __vshr_n_u32
#define vshr_n_u64                  __vshr_n_u64
#define vshr_n_u8                   __vshr_n_u8
#define vrshrq_n_s16                __vrshrq_n_s16
#define vrshrq_n_s32                __vrshrq_n_s32
#define vrshrq_n_s64                __vrshrq_n_s64
#define vrshrq_n_s8                 __vrshrq_n_s8
#define vrshrq_n_u16                __vrshrq_n_u16
#define vrshrq_n_u32                __vrshrq_n_u32
#define vrshrq_n_u64                __vrshrq_n_u64
#define vrshrq_n_u8                 __vrshrq_n_u8
#define vshrq_n_s16                 __vshrq_n_s16
#define vshrq_n_s32                 __vshrq_n_s32
#define vshrq_n_s64                 __vshrq_n_s64
#define vshrq_n_s8                  __vshrq_n_s8
#define vshrq_n_u16                 __vshrq_n_u16
#define vshrq_n_u32                 __vshrq_n_u32
#define vshrq_n_u64                 __vshrq_n_u64
#define vshrq_n_u8                  __vshrq_n_u8
#define vrshrn_n_s16                __vrshrn_n_s16
#define vrshrn_n_s32                __vrshrn_n_s32
#define vrshrn_n_s64                __vrshrn_n_s64
#define vrshrn_n_u16                __vrshrn_n_u16
#define vrshrn_n_u32                __vrshrn_n_u32
#define vrshrn_n_u64                __vrshrn_n_u64
#define vshrn_n_s16                 __vshrn_n_s16
#define vshrn_n_s32                 __vshrn_n_s32
#define vshrn_n_s64                 __vshrn_n_s64
#define vshrn_n_u16                 __vshrn_n_u16
#define vshrn_n_u32                 __vshrn_n_u32
#define vshrn_n_u64                 __vshrn_n_u64
#define vsli_n_p16                  __vsli_n_p16
#define vsli_n_p8                   __vsli_n_p8
#define vsli_n_s16                  __vsli_n_s16
#define vsli_n_s32                  __vsli_n_s32
#define vsli_n_s64                  __vsli_n_s64
#define vsli_n_s8                   __vsli_n_s8
#define vsli_n_u16                  __vsli_n_u16
#define vsli_n_u32                  __vsli_n_u32
#define vsli_n_u64                  __vsli_n_u64
#define vsli_n_u8                   __vsli_n_u8
#define vsliq_n_p16                 __vsliq_n_p16
#define vsliq_n_p8                  __vsliq_n_p8
#define vsliq_n_s16                 __vsliq_n_s16
#define vsliq_n_s32                 __vsliq_n_s32
#define vsliq_n_s64                 __vsliq_n_s64
#define vsliq_n_s8                  __vsliq_n_s8
#define vsliq_n_u16                 __vsliq_n_u16
#define vsliq_n_u32                 __vsliq_n_u32
#define vsliq_n_u64                 __vsliq_n_u64
#define vsliq_n_u8                  __vsliq_n_u8
#define vrsra_n_s16                 __vrsra_n_s16
#define vrsra_n_s32                 __vrsra_n_s32
#define vrsra_n_s64                 __vrsra_n_s64
#define vrsra_n_s8                  __vrsra_n_s8
#define vrsra_n_u16                 __vrsra_n_u16
#define vrsra_n_u32                 __vrsra_n_u32
#define vrsra_n_u64                 __vrsra_n_u64
#define vrsra_n_u8                  __vrsra_n_u8
#define vsra_n_s16                  __vsra_n_s16
#define vsra_n_s32                  __vsra_n_s32
#define vsra_n_s64                  __vsra_n_s64
#define vsra_n_s8                   __vsra_n_s8
#define vsra_n_u16                  __vsra_n_u16
#define vsra_n_u32                  __vsra_n_u32
#define vsra_n_u64                  __vsra_n_u64
#define vsra_n_u8                   __vsra_n_u8
#define vrsraq_n_s16                __vrsraq_n_s16
#define vrsraq_n_s32                __vrsraq_n_s32
#define vrsraq_n_s64                __vrsraq_n_s64
#define vrsraq_n_s8                 __vrsraq_n_s8
#define vrsraq_n_u16                __vrsraq_n_u16
#define vrsraq_n_u32                __vrsraq_n_u32
#define vrsraq_n_u64                __vrsraq_n_u64
#define vrsraq_n_u8                 __vrsraq_n_u8
#define vsraq_n_s16                 __vsraq_n_s16
#define vsraq_n_s32                 __vsraq_n_s32
#define vsraq_n_s64                 __vsraq_n_s64
#define vsraq_n_s8                  __vsraq_n_s8
#define vsraq_n_u16                 __vsraq_n_u16
#define vsraq_n_u32                 __vsraq_n_u32
#define vsraq_n_u64                 __vsraq_n_u64
#define vsraq_n_u8                  __vsraq_n_u8
#define vsri_n_p16                  __vsri_n_p16
#define vsri_n_p8                   __vsri_n_p8
#define vsri_n_s16                  __vsri_n_s16
#define vsri_n_s32                  __vsri_n_s32
#define vsri_n_s64                  __vsri_n_s64
#define vsri_n_s8                   __vsri_n_s8
#define vsri_n_u16                  __vsri_n_u16
#define vsri_n_u32                  __vsri_n_u32
#define vsri_n_u64                  __vsri_n_u64
#define vsri_n_u8                   __vsri_n_u8
#define vsriq_n_p16                 __vsriq_n_p16
#define vsriq_n_p8                  __vsriq_n_p8
#define vsriq_n_s16                 __vsriq_n_s16
#define vsriq_n_s32                 __vsriq_n_s32
#define vsriq_n_s64                 __vsriq_n_s64
#define vsriq_n_s8                  __vsriq_n_s8
#define vsriq_n_u16                 __vsriq_n_u16
#define vsriq_n_u32                 __vsriq_n_u32
#define vsriq_n_u64                 __vsriq_n_u64
#define vsriq_n_u8                  __vsriq_n_u8
#define vst1_f32                    __vst1_f32
#define vst1_p16                    __vst1_p16
#define vst1_p8                     __vst1_p8
#define vst1_s16                    __vst1_s16
#define vst1_s32                    __vst1_s32
#define vst1_s64                    __vst1_s64
#define vst1_s8                     __vst1_s8
#define vst1_u16                    __vst1_u16
#define vst1_u32                    __vst1_u32
#define vst1_u64                    __vst1_u64
#define vst1_u8                     __vst1_u8
#define vst1_f32_ex                 __vst1_f32_ex
#define vst1_p16_ex                 __vst1_p16_ex
#define vst1_p8_ex                  __vst1_p8_ex
#define vst1_s16_ex                 __vst1_s16_ex
#define vst1_s32_ex                 __vst1_s32_ex
#define vst1_s64_ex                 __vst1_s64_ex
#define vst1_s8_ex                  __vst1_s8_ex
#define vst1_u16_ex                 __vst1_u16_ex
#define vst1_u32_ex                 __vst1_u32_ex
#define vst1_u64_ex                 __vst1_u64_ex
#define vst1_u8_ex                  __vst1_u8_ex
#define vst1q_f32                   __vst1q_f32
#define vst1q_p16                   __vst1q_p16
#define vst1q_p8                    __vst1q_p8
#define vst1q_s16                   __vst1q_s16
#define vst1q_s32                   __vst1q_s32
#define vst1q_s64                   __vst1q_s64
#define vst1q_s8                    __vst1q_s8
#define vst1q_u16                   __vst1q_u16
#define vst1q_u32                   __vst1q_u32
#define vst1q_u64                   __vst1q_u64
#define vst1q_u8                    __vst1q_u8
#define vst1q_f32_ex                __vst1q_f32_ex
#define vst1q_p16_ex                __vst1q_p16_ex
#define vst1q_p8_ex                 __vst1q_p8_ex
#define vst1q_s16_ex                __vst1q_s16_ex
#define vst1q_s32_ex                __vst1q_s32_ex
#define vst1q_s64_ex                __vst1q_s64_ex
#define vst1q_s8_ex                 __vst1q_s8_ex
#define vst1q_u16_ex                __vst1q_u16_ex
#define vst1q_u32_ex                __vst1q_u32_ex
#define vst1q_u64_ex                __vst1q_u64_ex
#define vst1q_u8_ex                 __vst1q_u8_ex
#define vst1_lane_f32               __vst1_lane_f32
#define vst1_lane_p16               __vst1_lane_p16
#define vst1_lane_p8                __vst1_lane_p8
#define vst1_lane_s16               __vst1_lane_s16
#define vst1_lane_s32               __vst1_lane_s32
#define vst1_lane_s8                __vst1_lane_s8
#define vst1_lane_u16               __vst1_lane_u16
#define vst1_lane_u32               __vst1_lane_u32
#define vst1_lane_u8                __vst1_lane_u8
#define vst1q_lane_f32              __vst1q_lane_f32
#define vst1q_lane_p16              __vst1q_lane_p16
#define vst1q_lane_p8               __vst1q_lane_p8
#define vst1q_lane_s16              __vst1q_lane_s16
#define vst1q_lane_s32              __vst1q_lane_s32
#define vst1q_lane_s8               __vst1q_lane_s8
#define vst1q_lane_u16              __vst1q_lane_u16
#define vst1q_lane_u32              __vst1q_lane_u32
#define vst1q_lane_u8               __vst1q_lane_u8
#define vst1_lane_f32_ex            __vst1_lane_f32_ex
#define vst1_lane_p16_ex            __vst1_lane_p16_ex
#define vst1_lane_s16_ex            __vst1_lane_s16_ex
#define vst1_lane_s32_ex            __vst1_lane_s32_ex
#define vst1_lane_u16_ex            __vst1_lane_u16_ex
#define vst1_lane_u32_ex            __vst1_lane_u32_ex
#define vst1q_lane_f32_ex           __vst1q_lane_f32_ex
#define vst1q_lane_p16_ex           __vst1q_lane_p16_ex
#define vst1q_lane_s16_ex           __vst1q_lane_s16_ex
#define vst1q_lane_s32_ex           __vst1q_lane_s32_ex
#define vst1q_lane_u16_ex           __vst1q_lane_u16_ex
#define vst1q_lane_u32_ex           __vst1q_lane_u32_ex
#define vst2_f32                    __vst2_f32
#define vst2_p16                    __vst2_p16
#define vst2_p8                     __vst2_p8
#define vst2_s16                    __vst2_s16
#define vst2_s32                    __vst2_s32
#define vst2_s8                     __vst2_s8
#define vst2_u16                    __vst2_u16
#define vst2_u32                    __vst2_u32
#define vst2_u8                     __vst2_u8
#define vst2_s64                    __vst2_s64
#define vst2_u64                    __vst2_u64
#define vst2_s64_ex                 __vst2_s64_ex
#define vst2_u64_ex                 __vst2_u64_ex
#define vst2_f32_ex                 __vst2_f32_ex
#define vst2_p16_ex                 __vst2_p16_ex
#define vst2_p8_ex                  __vst2_p8_ex
#define vst2_s16_ex                 __vst2_s16_ex
#define vst2_s32_ex                 __vst2_s32_ex
#define vst2_s8_ex                  __vst2_s8_ex
#define vst2_u16_ex                 __vst2_u16_ex
#define vst2_u32_ex                 __vst2_u32_ex
#define vst2_u8_ex                  __vst2_u8_ex
#define vst2q_f32                   __vst2q_f32
#define vst2q_p16                   __vst2q_p16
#define vst2q_p8                    __vst2q_p8
#define vst2q_s16                   __vst2q_s16
#define vst2q_s32                   __vst2q_s32
#define vst2q_s8                    __vst2q_s8
#define vst2q_u16                   __vst2q_u16
#define vst2q_u32                   __vst2q_u32
#define vst2q_u8                    __vst2q_u8
#define vst2q_f32_ex                __vst2q_f32_ex
#define vst2q_p16_ex                __vst2q_p16_ex
#define vst2q_p8_ex                 __vst2q_p8_ex
#define vst2q_s16_ex                __vst2q_s16_ex
#define vst2q_s32_ex                __vst2q_s32_ex
#define vst2q_s8_ex                 __vst2q_s8_ex
#define vst2q_u16_ex                __vst2q_u16_ex
#define vst2q_u32_ex                __vst2q_u32_ex
#define vst2q_u8_ex                 __vst2q_u8_ex
#define vst2_lane_f32               __vst2_lane_f32
#define vst2_lane_p16               __vst2_lane_p16
#define vst2_lane_p8                __vst2_lane_p8
#define vst2_lane_s16               __vst2_lane_s16
#define vst2_lane_s32               __vst2_lane_s32
#define vst2_lane_s8                __vst2_lane_s8
#define vst2_lane_u16               __vst2_lane_u16
#define vst2_lane_u32               __vst2_lane_u32
#define vst2_lane_u8                __vst2_lane_u8
#define vst2q_lane_f32              __vst2q_lane_f32
#define vst2q_lane_p16              __vst2q_lane_p16
#define vst2q_lane_s16              __vst2q_lane_s16
#define vst2q_lane_s32              __vst2q_lane_s32
#define vst2q_lane_u16              __vst2q_lane_u16
#define vst2q_lane_u32              __vst2q_lane_u32
#define vst2_lane_f32_ex            __vst2_lane_f32_ex
#define vst2_lane_p16_ex            __vst2_lane_p16_ex
#define vst2_lane_p8_ex             __vst2_lane_p8_ex
#define vst2_lane_s16_ex            __vst2_lane_s16_ex
#define vst2_lane_s32_ex            __vst2_lane_s32_ex
#define vst2_lane_s8_ex             __vst2_lane_s8_ex
#define vst2_lane_u16_ex            __vst2_lane_u16_ex
#define vst2_lane_u32_ex            __vst2_lane_u32_ex
#define vst2_lane_u8_ex             __vst2_lane_u8_ex
#define vst2q_lane_f32_ex           __vst2q_lane_f32_ex
#define vst2q_lane_p16_ex           __vst2q_lane_p16_ex
#define vst2q_lane_s16_ex           __vst2q_lane_s16_ex
#define vst2q_lane_s32_ex           __vst2q_lane_s32_ex
#define vst2q_lane_u16_ex           __vst2q_lane_u16_ex
#define vst2q_lane_u32_ex           __vst2q_lane_u32_ex
#define vst3_f32                    __vst3_f32
#define vst3_p16                    __vst3_p16
#define vst3_p8                     __vst3_p8
#define vst3_s16                    __vst3_s16
#define vst3_s32                    __vst3_s32
#define vst3_s8                     __vst3_s8
#define vst3_u16                    __vst3_u16
#define vst3_u32                    __vst3_u32
#define vst3_u8                     __vst3_u8
#define vst3_s64                    __vst3_s64
#define vst3_u64                    __vst3_u64
#define vst3_s64_ex                 __vst3_s64_ex
#define vst3_u64_ex                 __vst3_u64_ex
#define vst3_f32_ex                 __vst3_f32_ex
#define vst3_p16_ex                 __vst3_p16_ex
#define vst3_p8_ex                  __vst3_p8_ex
#define vst3_s16_ex                 __vst3_s16_ex
#define vst3_s32_ex                 __vst3_s32_ex
#define vst3_s8_ex                  __vst3_s8_ex
#define vst3_u16_ex                 __vst3_u16_ex
#define vst3_u32_ex                 __vst3_u32_ex
#define vst3_u8_ex                  __vst3_u8_ex
#define vst3q_f32                   __vst3q_f32
#define vst3q_p16                   __vst3q_p16
#define vst3q_p8                    __vst3q_p8
#define vst3q_s16                   __vst3q_s16
#define vst3q_s32                   __vst3q_s32
#define vst3q_s8                    __vst3q_s8
#define vst3q_u16                   __vst3q_u16
#define vst3q_u32                   __vst3q_u32
#define vst3q_u8                    __vst3q_u8
#define vst3q_f32_ex                __vst3q_f32_ex
#define vst3q_p16_ex                __vst3q_p16_ex
#define vst3q_p8_ex                 __vst3q_p8_ex
#define vst3q_s16_ex                __vst3q_s16_ex
#define vst3q_s32_ex                __vst3q_s32_ex
#define vst3q_s8_ex                 __vst3q_s8_ex
#define vst3q_u16_ex                __vst3q_u16_ex
#define vst3q_u32_ex                __vst3q_u32_ex
#define vst3q_u8_ex                 __vst3q_u8_ex
#define vst3_lane_f32               __vst3_lane_f32
#define vst3_lane_p16               __vst3_lane_p16
#define vst3_lane_p8                __vst3_lane_p8
#define vst3_lane_s16               __vst3_lane_s16
#define vst3_lane_s32               __vst3_lane_s32
#define vst3_lane_s8                __vst3_lane_s8
#define vst3_lane_u16               __vst3_lane_u16
#define vst3_lane_u32               __vst3_lane_u32
#define vst3_lane_u8                __vst3_lane_u8
#define vst3q_lane_f32              __vst3q_lane_f32
#define vst3q_lane_p16              __vst3q_lane_p16
#define vst3q_lane_s16              __vst3q_lane_s16
#define vst3q_lane_s32              __vst3q_lane_s32
#define vst3q_lane_u16              __vst3q_lane_u16
#define vst3q_lane_u32              __vst3q_lane_u32
#define vst4_f32                    __vst4_f32
#define vst4_p16                    __vst4_p16
#define vst4_p8                     __vst4_p8
#define vst4_s16                    __vst4_s16
#define vst4_s32                    __vst4_s32
#define vst4_s8                     __vst4_s8
#define vst4_u16                    __vst4_u16
#define vst4_u32                    __vst4_u32
#define vst4_u8                     __vst4_u8
#define vst4_s64                    __vst4_s64
#define vst4_u64                    __vst4_u64
#define vst4_s64_ex                 __vst4_s64_ex
#define vst4_u64_ex                 __vst4_u64_ex
#define vst4_f32_ex                 __vst4_f32_ex
#define vst4_p16_ex                 __vst4_p16_ex
#define vst4_p8_ex                  __vst4_p8_ex
#define vst4_s16_ex                 __vst4_s16_ex
#define vst4_s32_ex                 __vst4_s32_ex
#define vst4_s8_ex                  __vst4_s8_ex
#define vst4_u16_ex                 __vst4_u16_ex
#define vst4_u32_ex                 __vst4_u32_ex
#define vst4_u8_ex                  __vst4_u8_ex
#define vst4q_f32                   __vst4q_f32
#define vst4q_p16                   __vst4q_p16
#define vst4q_p8                    __vst4q_p8
#define vst4q_s16                   __vst4q_s16
#define vst4q_s32                   __vst4q_s32
#define vst4q_s8                    __vst4q_s8
#define vst4q_u16                   __vst4q_u16
#define vst4q_u32                   __vst4q_u32
#define vst4q_u8                    __vst4q_u8
#define vst4q_f32_ex                __vst4q_f32_ex
#define vst4q_p16_ex                __vst4q_p16_ex
#define vst4q_p8_ex                 __vst4q_p8_ex
#define vst4q_s16_ex                __vst4q_s16_ex
#define vst4q_s32_ex                __vst4q_s32_ex
#define vst4q_s8_ex                 __vst4q_s8_ex
#define vst4q_u16_ex                __vst4q_u16_ex
#define vst4q_u32_ex                __vst4q_u32_ex
#define vst4q_u8_ex                 __vst4q_u8_ex
#define vst4_lane_f32               __vst4_lane_f32
#define vst4_lane_p16               __vst4_lane_p16
#define vst4_lane_p8                __vst4_lane_p8
#define vst4_lane_s16               __vst4_lane_s16
#define vst4_lane_s32               __vst4_lane_s32
#define vst4_lane_s8                __vst4_lane_s8
#define vst4_lane_u16               __vst4_lane_u16
#define vst4_lane_u32               __vst4_lane_u32
#define vst4_lane_u8                __vst4_lane_u8
#define vst4q_lane_f32              __vst4q_lane_f32
#define vst4q_lane_p16              __vst4q_lane_p16
#define vst4q_lane_s16              __vst4q_lane_s16
#define vst4q_lane_s32              __vst4q_lane_s32
#define vst4q_lane_u16              __vst4q_lane_u16
#define vst4q_lane_u32              __vst4q_lane_u32
#define vst4_lane_f32_ex            __vst4_lane_f32_ex
#define vst4_lane_p16_ex            __vst4_lane_p16_ex
#define vst4_lane_p8_ex             __vst4_lane_p8_ex
#define vst4_lane_s16_ex            __vst4_lane_s16_ex
#define vst4_lane_s32_ex            __vst4_lane_s32_ex
#define vst4_lane_s8_ex             __vst4_lane_s8_ex
#define vst4_lane_u16_ex            __vst4_lane_u16_ex
#define vst4_lane_u32_ex            __vst4_lane_u32_ex
#define vst4_lane_u8_ex             __vst4_lane_u8_ex
#define vst4q_lane_f32_ex           __vst4q_lane_f32_ex
#define vst4q_lane_p16_ex           __vst4q_lane_p16_ex
#define vst4q_lane_s16_ex           __vst4q_lane_s16_ex
#define vst4q_lane_s32_ex           __vst4q_lane_s32_ex
#define vst4q_lane_u16_ex           __vst4q_lane_u16_ex
#define vst4q_lane_u32_ex           __vst4q_lane_u32_ex
#define vsub_f32                    __vsub_f32
#define vsub_s16                    __vsub_s16
#define vsub_s32                    __vsub_s32
#define vsub_s64                    __vsub_s64
#define vsub_s8                     __vsub_s8
#define vsub_u16                    __vsub_u16
#define vsub_u32                    __vsub_u32
#define vsub_u64                    __vsub_u64
#define vsub_u8                     __vsub_u8
#define vsubq_f32                   __vsubq_f32
#define vsubq_s16                   __vsubq_s16
#define vsubq_s32                   __vsubq_s32
#define vsubq_s64                   __vsubq_s64
#define vsubq_s8                    __vsubq_s8
#define vsubq_u16                   __vsubq_u16
#define vsubq_u32                   __vsubq_u32
#define vsubq_u64                   __vsubq_u64
#define vsubq_u8                    __vsubq_u8
#define vrsubhn_s16                 __vrsubhn_s16
#define vrsubhn_s32                 __vrsubhn_s32
#define vrsubhn_s64                 __vrsubhn_s64
#define vrsubhn_u16                 __vrsubhn_u16
#define vrsubhn_u32                 __vrsubhn_u32
#define vrsubhn_u64                 __vrsubhn_u64
#define vsubhn_s16                  __vsubhn_s16
#define vsubhn_s32                  __vsubhn_s32
#define vsubhn_s64                  __vsubhn_s64
#define vsubhn_u16                  __vsubhn_u16
#define vsubhn_u32                  __vsubhn_u32
#define vsubhn_u64                  __vsubhn_u64
#define vsubl_s16                   __vsubl_s16
#define vsubl_s32                   __vsubl_s32
#define vsubl_s8                    __vsubl_s8
#define vsubl_u16                   __vsubl_u16
#define vsubl_u32                   __vsubl_u32
#define vsubl_u8                    __vsubl_u8
#define vsubw_s16                   __vsubw_s16
#define vsubw_s32                   __vsubw_s32
#define vsubw_s8                    __vsubw_s8
#define vsubw_u16                   __vsubw_u16
#define vsubw_u32                   __vsubw_u32
#define vsubw_u8                    __vsubw_u8
#define vtbl2_p8                    __vtbl2_p8
#define vtbl2_s8                    __vtbl2_s8
#define vtbl2_u8                    __vtbl2_u8
#define vtbx2_p8                    __vtbx2_p8
#define vtbx2_s8                    __vtbx2_s8
#define vtbx2_u8                    __vtbx2_u8
#define vtbl3_p8                    __vtbl3_p8
#define vtbl3_s8                    __vtbl3_s8
#define vtbl3_u8                    __vtbl3_u8
#define vtbx3_p8                    __vtbx3_p8
#define vtbx3_s8                    __vtbx3_s8
#define vtbx3_u8                    __vtbx3_u8
#define vtbl4_p8                    __vtbl4_p8
#define vtbl4_s8                    __vtbl4_s8
#define vtbl4_u8                    __vtbl4_u8
#define vtbx4_p8                    __vtbx4_p8
#define vtbx4_s8                    __vtbx4_s8
#define vtbx4_u8                    __vtbx4_u8
#define vtbl1_p8                    __vtbl1_p8
#define vtbl1_s8                    __vtbl1_s8
#define vtbl1_u8                    __vtbl1_u8
#define vtbx1_p8                    __vtbx1_p8
#define vtbx1_s8                    __vtbx1_s8
#define vtbx1_u8                    __vtbx1_u8
#define vtrn_f32                    __vtrn_f32
#define vtrn_p16                    __vtrn_p16
#define vtrn_p8                     __vtrn_p8
#define vtrn_s16                    __vtrn_s16
#define vtrn_s32                    __vtrn_s32
#define vtrn_s8                     __vtrn_s8
#define vtrn_u16                    __vtrn_u16
#define vtrn_u32                    __vtrn_u32
#define vtrn_u8                     __vtrn_u8
#define vtrnq_f32                   __vtrnq_f32
#define vtrnq_p16                   __vtrnq_p16
#define vtrnq_p8                    __vtrnq_p8
#define vtrnq_s16                   __vtrnq_s16
#define vtrnq_s32                   __vtrnq_s32
#define vtrnq_s8                    __vtrnq_s8
#define vtrnq_u16                   __vtrnq_u16
#define vtrnq_u32                   __vtrnq_u32
#define vtrnq_u8                    __vtrnq_u8
#define vtrnq_s64                   __vtrnq_s64
#define vtrnq_u64                   __vtrnq_u64
#define vtst_p8                     __vtst_p8
#define vtst_s16                    __vtst_s16
#define vtst_s32                    __vtst_s32
#define vtst_s8                     __vtst_s8
#define vtst_u16                    __vtst_u16
#define vtst_u32                    __vtst_u32
#define vtst_u8                     __vtst_u8
#define vtstq_p8                    __vtstq_p8
#define vtstq_s16                   __vtstq_s16
#define vtstq_s32                   __vtstq_s32
#define vtstq_s8                    __vtstq_s8
#define vtstq_u16                   __vtstq_u16
#define vtstq_u32                   __vtstq_u32
#define vtstq_u8                    __vtstq_u8
#define vuzp_p16                    __vuzp_p16
#define vuzp_p8                     __vuzp_p8
#define vuzp_s16                    __vuzp_s16
#define vuzp_s8                     __vuzp_s8
#define vuzp_u16                    __vuzp_u16
#define vuzp_u8                     __vuzp_u8
#define vuzp_f32                    __vuzp_f32
#define vuzp_s32                    __vuzp_s32
#define vuzp_u32                    __vuzp_u32
#define vuzpq_f32                   __vuzpq_f32
#define vuzpq_p16                   __vuzpq_p16
#define vuzpq_p8                    __vuzpq_p8
#define vuzpq_s16                   __vuzpq_s16
#define vuzpq_s32                   __vuzpq_s32
#define vuzpq_s8                    __vuzpq_s8
#define vuzpq_u16                   __vuzpq_u16
#define vuzpq_u32                   __vuzpq_u32
#define vuzpq_u8                    __vuzpq_u8
#define vzip_p16                    __vzip_p16
#define vzip_p8                     __vzip_p8
#define vzip_s16                    __vzip_s16
#define vzip_s8                     __vzip_s8
#define vzip_u16                    __vzip_u16
#define vzip_u8                     __vzip_u8
#define vzip_f32                    __vzip_f32
#define vzip_s32                    __vzip_s32
#define vzip_u32                    __vzip_u32
#define vzipq_f32                   __vzipq_f32
#define vzipq_p16                   __vzipq_p16
#define vzipq_p8                    __vzipq_p8
#define vzipq_s16                   __vzipq_s16
#define vzipq_s32                   __vzipq_s32
#define vzipq_s8                    __vzipq_s8
#define vzipq_u16                   __vzipq_u16
#define vzipq_u32                   __vzipq_u32
#define vzipq_u8                    __vzipq_u8

#define vreinterpret_f32_s8         __vreinterpret_f32_s8
#define vreinterpret_f32_s16        __vreinterpret_f32_s16
#define vreinterpret_f32_s32        __vreinterpret_f32_s32
#define vreinterpret_f32_s64        __vreinterpret_f32_s64
#define vreinterpret_f32_p8         __vreinterpret_f32_p8
#define vreinterpret_f32_p16        __vreinterpret_f32_p16
#define vreinterpret_f32_u8         __vreinterpret_f32_u8
#define vreinterpret_f32_u16        __vreinterpret_f32_u16
#define vreinterpret_f32_u32        __vreinterpret_f32_u32
#define vreinterpret_f32_u64        __vreinterpret_f32_u64
#define vreinterpret_s8_f32         __vreinterpret_s8_f32
#define vreinterpret_s8_s16         __vreinterpret_s8_s16
#define vreinterpret_s8_s32         __vreinterpret_s8_s32
#define vreinterpret_s8_s64         __vreinterpret_s8_s64
#define vreinterpret_s8_p8          __vreinterpret_s8_p8
#define vreinterpret_s8_p16         __vreinterpret_s8_p16
#define vreinterpret_s8_u8          __vreinterpret_s8_u8
#define vreinterpret_s8_u16         __vreinterpret_s8_u16
#define vreinterpret_s8_u32         __vreinterpret_s8_u32
#define vreinterpret_s8_u64         __vreinterpret_s8_u64
#define vreinterpret_s16_f32        __vreinterpret_s16_f32
#define vreinterpret_s16_s8         __vreinterpret_s16_s8
#define vreinterpret_s16_s32        __vreinterpret_s16_s32
#define vreinterpret_s16_s64        __vreinterpret_s16_s64
#define vreinterpret_s16_p8         __vreinterpret_s16_p8
#define vreinterpret_s16_p16        __vreinterpret_s16_p16
#define vreinterpret_s16_u8         __vreinterpret_s16_u8
#define vreinterpret_s16_u16        __vreinterpret_s16_u16
#define vreinterpret_s16_u32        __vreinterpret_s16_u32
#define vreinterpret_s16_u64        __vreinterpret_s16_u64
#define vreinterpret_s32_f32        __vreinterpret_s32_f32
#define vreinterpret_s32_s8         __vreinterpret_s32_s8
#define vreinterpret_s32_s16        __vreinterpret_s32_s16
#define vreinterpret_s32_s64        __vreinterpret_s32_s64
#define vreinterpret_s32_p8         __vreinterpret_s32_p8
#define vreinterpret_s32_p16        __vreinterpret_s32_p16
#define vreinterpret_s32_u8         __vreinterpret_s32_u8
#define vreinterpret_s32_u16        __vreinterpret_s32_u16
#define vreinterpret_s32_u32        __vreinterpret_s32_u32
#define vreinterpret_s32_u64        __vreinterpret_s32_u64
#define vreinterpret_s64_f32        __vreinterpret_s64_f32
#define vreinterpret_s64_s8         __vreinterpret_s64_s8
#define vreinterpret_s64_s16        __vreinterpret_s64_s16
#define vreinterpret_s64_s32        __vreinterpret_s64_s32
#define vreinterpret_s64_p8         __vreinterpret_s64_p8
#define vreinterpret_s64_p16        __vreinterpret_s64_p16
#define vreinterpret_s64_u8         __vreinterpret_s64_u8
#define vreinterpret_s64_u16        __vreinterpret_s64_u16
#define vreinterpret_s64_u32        __vreinterpret_s64_u32
#define vreinterpret_s64_u64        __vreinterpret_s64_u64
#define vreinterpret_p8_f32         __vreinterpret_p8_f32
#define vreinterpret_p8_s8          __vreinterpret_p8_s8
#define vreinterpret_p8_s16         __vreinterpret_p8_s16
#define vreinterpret_p8_s32         __vreinterpret_p8_s32
#define vreinterpret_p8_s64         __vreinterpret_p8_s64
#define vreinterpret_p8_p16         __vreinterpret_p8_p16
#define vreinterpret_p8_u8          __vreinterpret_p8_u8
#define vreinterpret_p8_u16         __vreinterpret_p8_u16
#define vreinterpret_p8_u32         __vreinterpret_p8_u32
#define vreinterpret_p8_u64         __vreinterpret_p8_u64
#define vreinterpret_p16_f32        __vreinterpret_p16_f32
#define vreinterpret_p16_s8         __vreinterpret_p16_s8
#define vreinterpret_p16_s16        __vreinterpret_p16_s16
#define vreinterpret_p16_s32        __vreinterpret_p16_s32
#define vreinterpret_p16_s64        __vreinterpret_p16_s64
#define vreinterpret_p16_p8         __vreinterpret_p16_p8
#define vreinterpret_p16_u8         __vreinterpret_p16_u8
#define vreinterpret_p16_u16        __vreinterpret_p16_u16
#define vreinterpret_p16_u32        __vreinterpret_p16_u32
#define vreinterpret_p16_u64        __vreinterpret_p16_u64
#define vreinterpret_u8_f32         __vreinterpret_u8_f32
#define vreinterpret_u8_s8          __vreinterpret_u8_s8
#define vreinterpret_u8_s16         __vreinterpret_u8_s16
#define vreinterpret_u8_s32         __vreinterpret_u8_s32
#define vreinterpret_u8_s64         __vreinterpret_u8_s64
#define vreinterpret_u8_p8          __vreinterpret_u8_p8
#define vreinterpret_u8_p16         __vreinterpret_u8_p16
#define vreinterpret_u8_u16         __vreinterpret_u8_u16
#define vreinterpret_u8_u32         __vreinterpret_u8_u32
#define vreinterpret_u8_u64         __vreinterpret_u8_u64
#define vreinterpret_u16_f32        __vreinterpret_u16_f32
#define vreinterpret_u16_s8         __vreinterpret_u16_s8
#define vreinterpret_u16_s16        __vreinterpret_u16_s16
#define vreinterpret_u16_s32        __vreinterpret_u16_s32
#define vreinterpret_u16_s64        __vreinterpret_u16_s64
#define vreinterpret_u16_p8         __vreinterpret_u16_p8
#define vreinterpret_u16_p16        __vreinterpret_u16_p16
#define vreinterpret_u16_u8         __vreinterpret_u16_u8
#define vreinterpret_u16_u32        __vreinterpret_u16_u32
#define vreinterpret_u16_u64        __vreinterpret_u16_u64
#define vreinterpret_u32_f32        __vreinterpret_u32_f32
#define vreinterpret_u32_s8         __vreinterpret_u32_s8
#define vreinterpret_u32_s16        __vreinterpret_u32_s16
#define vreinterpret_u32_s32        __vreinterpret_u32_s32
#define vreinterpret_u32_s64        __vreinterpret_u32_s64
#define vreinterpret_u32_p8         __vreinterpret_u32_p8
#define vreinterpret_u32_p16        __vreinterpret_u32_p16
#define vreinterpret_u32_u8         __vreinterpret_u32_u8
#define vreinterpret_u32_u16        __vreinterpret_u32_u16
#define vreinterpret_u32_u64        __vreinterpret_u32_u64
#define vreinterpret_u64_f32        __vreinterpret_u64_f32
#define vreinterpret_u64_s8         __vreinterpret_u64_s8
#define vreinterpret_u64_s16        __vreinterpret_u64_s16
#define vreinterpret_u64_s32        __vreinterpret_u64_s32
#define vreinterpret_u64_s64        __vreinterpret_u64_s64
#define vreinterpret_u64_p8         __vreinterpret_u64_p8
#define vreinterpret_u64_p16        __vreinterpret_u64_p16
#define vreinterpret_u64_u8         __vreinterpret_u64_u8
#define vreinterpret_u64_u16        __vreinterpret_u64_u16
#define vreinterpret_u64_u32        __vreinterpret_u64_u32
#define vreinterpretq_f32_s8        __vreinterpretq_f32_s8
#define vreinterpretq_f32_s16       __vreinterpretq_f32_s16
#define vreinterpretq_f32_s32       __vreinterpretq_f32_s32
#define vreinterpretq_f32_s64       __vreinterpretq_f32_s64
#define vreinterpretq_f32_p8        __vreinterpretq_f32_p8
#define vreinterpretq_f32_p16       __vreinterpretq_f32_p16
#define vreinterpretq_f32_u8        __vreinterpretq_f32_u8
#define vreinterpretq_f32_u16       __vreinterpretq_f32_u16
#define vreinterpretq_f32_u32       __vreinterpretq_f32_u32
#define vreinterpretq_f32_u64       __vreinterpretq_f32_u64
#define vreinterpretq_s8_f32        __vreinterpretq_s8_f32
#define vreinterpretq_s8_s16        __vreinterpretq_s8_s16
#define vreinterpretq_s8_s32        __vreinterpretq_s8_s32
#define vreinterpretq_s8_s64        __vreinterpretq_s8_s64
#define vreinterpretq_s8_p8         __vreinterpretq_s8_p8
#define vreinterpretq_s8_p16        __vreinterpretq_s8_p16
#define vreinterpretq_s8_u8         __vreinterpretq_s8_u8
#define vreinterpretq_s8_u16        __vreinterpretq_s8_u16
#define vreinterpretq_s8_u32        __vreinterpretq_s8_u32
#define vreinterpretq_s8_u64        __vreinterpretq_s8_u64
#define vreinterpretq_s16_f32       __vreinterpretq_s16_f32
#define vreinterpretq_s16_s8        __vreinterpretq_s16_s8
#define vreinterpretq_s16_s32       __vreinterpretq_s16_s32
#define vreinterpretq_s16_s64       __vreinterpretq_s16_s64
#define vreinterpretq_s16_p8        __vreinterpretq_s16_p8
#define vreinterpretq_s16_p16       __vreinterpretq_s16_p16
#define vreinterpretq_s16_u8        __vreinterpretq_s16_u8
#define vreinterpretq_s16_u16       __vreinterpretq_s16_u16
#define vreinterpretq_s16_u32       __vreinterpretq_s16_u32
#define vreinterpretq_s16_u64       __vreinterpretq_s16_u64
#define vreinterpretq_s32_f32       __vreinterpretq_s32_f32
#define vreinterpretq_s32_s8        __vreinterpretq_s32_s8
#define vreinterpretq_s32_s16       __vreinterpretq_s32_s16
#define vreinterpretq_s32_s64       __vreinterpretq_s32_s64
#define vreinterpretq_s32_p8        __vreinterpretq_s32_p8
#define vreinterpretq_s32_p16       __vreinterpretq_s32_p16
#define vreinterpretq_s32_u8        __vreinterpretq_s32_u8
#define vreinterpretq_s32_u16       __vreinterpretq_s32_u16
#define vreinterpretq_s32_u32       __vreinterpretq_s32_u32
#define vreinterpretq_s32_u64       __vreinterpretq_s32_u64
#define vreinterpretq_s64_f32       __vreinterpretq_s64_f32
#define vreinterpretq_s64_s8        __vreinterpretq_s64_s8
#define vreinterpretq_s64_s16       __vreinterpretq_s64_s16
#define vreinterpretq_s64_s32       __vreinterpretq_s64_s32
#define vreinterpretq_s64_p8        __vreinterpretq_s64_p8
#define vreinterpretq_s64_p16       __vreinterpretq_s64_p16
#define vreinterpretq_s64_u8        __vreinterpretq_s64_u8
#define vreinterpretq_s64_u16       __vreinterpretq_s64_u16
#define vreinterpretq_s64_u32       __vreinterpretq_s64_u32
#define vreinterpretq_s64_u64       __vreinterpretq_s64_u64
#define vreinterpretq_p8_f32        __vreinterpretq_p8_f32
#define vreinterpretq_p8_s8         __vreinterpretq_p8_s8
#define vreinterpretq_p8_s16        __vreinterpretq_p8_s16
#define vreinterpretq_p8_s32        __vreinterpretq_p8_s32
#define vreinterpretq_p8_s64        __vreinterpretq_p8_s64
#define vreinterpretq_p8_p16        __vreinterpretq_p8_p16
#define vreinterpretq_p8_u8         __vreinterpretq_p8_u8
#define vreinterpretq_p8_u16        __vreinterpretq_p8_u16
#define vreinterpretq_p8_u32        __vreinterpretq_p8_u32
#define vreinterpretq_p8_u64        __vreinterpretq_p8_u64
#define vreinterpretq_p16_f32       __vreinterpretq_p16_f32
#define vreinterpretq_p16_s8        __vreinterpretq_p16_s8
#define vreinterpretq_p16_s16       __vreinterpretq_p16_s16
#define vreinterpretq_p16_s32       __vreinterpretq_p16_s32
#define vreinterpretq_p16_s64       __vreinterpretq_p16_s64
#define vreinterpretq_p16_p8        __vreinterpretq_p16_p8
#define vreinterpretq_p16_u8        __vreinterpretq_p16_u8
#define vreinterpretq_p16_u16       __vreinterpretq_p16_u16
#define vreinterpretq_p16_u32       __vreinterpretq_p16_u32
#define vreinterpretq_p16_u64       __vreinterpretq_p16_u64
#define vreinterpretq_u8_f32        __vreinterpretq_u8_f32
#define vreinterpretq_u8_s8         __vreinterpretq_u8_s8
#define vreinterpretq_u8_s16        __vreinterpretq_u8_s16
#define vreinterpretq_u8_s32        __vreinterpretq_u8_s32
#define vreinterpretq_u8_s64        __vreinterpretq_u8_s64
#define vreinterpretq_u8_p8         __vreinterpretq_u8_p8
#define vreinterpretq_u8_p16        __vreinterpretq_u8_p16
#define vreinterpretq_u8_u16        __vreinterpretq_u8_u16
#define vreinterpretq_u8_u32        __vreinterpretq_u8_u32
#define vreinterpretq_u8_u64        __vreinterpretq_u8_u64
#define vreinterpretq_u16_f32       __vreinterpretq_u16_f32
#define vreinterpretq_u16_s8        __vreinterpretq_u16_s8
#define vreinterpretq_u16_s16       __vreinterpretq_u16_s16
#define vreinterpretq_u16_s32       __vreinterpretq_u16_s32
#define vreinterpretq_u16_s64       __vreinterpretq_u16_s64
#define vreinterpretq_u16_p8        __vreinterpretq_u16_p8
#define vreinterpretq_u16_p16       __vreinterpretq_u16_p16
#define vreinterpretq_u16_u8        __vreinterpretq_u16_u8
#define vreinterpretq_u16_u32       __vreinterpretq_u16_u32
#define vreinterpretq_u16_u64       __vreinterpretq_u16_u64
#define vreinterpretq_u32_f32       __vreinterpretq_u32_f32
#define vreinterpretq_u32_s8        __vreinterpretq_u32_s8
#define vreinterpretq_u32_s16       __vreinterpretq_u32_s16
#define vreinterpretq_u32_s32       __vreinterpretq_u32_s32
#define vreinterpretq_u32_s64       __vreinterpretq_u32_s64
#define vreinterpretq_u32_p8        __vreinterpretq_u32_p8
#define vreinterpretq_u32_p16       __vreinterpretq_u32_p16
#define vreinterpretq_u32_u8        __vreinterpretq_u32_u8
#define vreinterpretq_u32_u16       __vreinterpretq_u32_u16
#define vreinterpretq_u32_u64       __vreinterpretq_u32_u64
#define vreinterpretq_u64_f32       __vreinterpretq_u64_f32
#define vreinterpretq_u64_s8        __vreinterpretq_u64_s8
#define vreinterpretq_u64_s16       __vreinterpretq_u64_s16
#define vreinterpretq_u64_s32       __vreinterpretq_u64_s32
#define vreinterpretq_u64_s64       __vreinterpretq_u64_s64
#define vreinterpretq_u64_p8        __vreinterpretq_u64_p8
#define vreinterpretq_u64_p16       __vreinterpretq_u64_p16
#define vreinterpretq_u64_u8        __vreinterpretq_u64_u8
#define vreinterpretq_u64_u16       __vreinterpretq_u64_u16
#define vreinterpretq_u64_u32       __vreinterpretq_u64_u32

#define vmul_n_s16                  __vmul_n_s16
#define vmul_n_s32                  __vmul_n_s32
#define vmul_n_u16                  __vmul_n_u16
#define vmul_n_u32                  __vmul_n_u32
#define vmulq_n_s16                 __vmulq_n_s16
#define vmulq_n_s32                 __vmulq_n_s32
#define vmulq_n_u16                 __vmulq_n_u16
#define vmulq_n_u32                 __vmulq_n_u32
#define vmull_n_s16                 __vmull_n_s16
#define vmull_n_s32                 __vmull_n_s32
#define vmull_n_u16                 __vmull_n_u16
#define vmull_n_u32                 __vmull_n_u32
#define vqdmulh_n_s16               __vqdmulh_n_s16
#define vqdmulh_n_s32               __vqdmulh_n_s32
#define vqdmulhq_n_s16              __vqdmulhq_n_s16
#define vqdmulhq_n_s32              __vqdmulhq_n_s32
#define vqdmull_n_s16               __vqdmull_n_s16
#define vqdmull_n_s32               __vqdmull_n_s32
#define vqrdmulh_n_s16              __vqrdmulh_n_s16
#define vqrdmulh_n_s32              __vqrdmulh_n_s32
#define vqrdmulhq_n_s16             __vqrdmulhq_n_s16
#define vqrdmulhq_n_s32             __vqrdmulhq_n_s32

#define vmla_n_s16                  __vmla_n_s16
#define vmla_n_s32                  __vmla_n_s32
#define vmla_n_u16                  __vmla_n_u16
#define vmla_n_u32                  __vmla_n_u32
#define vmlaq_n_s16                 __vmlaq_n_s16
#define vmlaq_n_s32                 __vmlaq_n_s32
#define vmlaq_n_u16                 __vmlaq_n_u16
#define vmlaq_n_u32                 __vmlaq_n_u32
#define vmlal_n_s16                 __vmlal_n_s16
#define vmlal_n_s32                 __vmlal_n_s32
#define vmlal_n_u16                 __vmlal_n_u16
#define vmlal_n_u32                 __vmlal_n_u32
#define vmls_n_s16                  __vmls_n_s16
#define vmls_n_s32                  __vmls_n_s32
#define vmls_n_u16                  __vmls_n_u16
#define vmls_n_u32                  __vmls_n_u32
#define vmlsq_n_s16                 __vmlsq_n_s16
#define vmlsq_n_s32                 __vmlsq_n_s32
#define vmlsq_n_u16                 __vmlsq_n_u16
#define vmlsq_n_u32                 __vmlsq_n_u32
#define vmlsl_n_s16                 __vmlsl_n_s16
#define vmlsl_n_s32                 __vmlsl_n_s32
#define vmlsl_n_u16                 __vmlsl_n_u16
#define vmlsl_n_u32                 __vmlsl_n_u32
#define vqdmlal_n_s16               __vqdmlal_n_s16
#define vqdmlal_n_s32               __vqdmlal_n_s32
#define vqdmlsl_n_s16               __vqdmlsl_n_s16
#define vqdmlsl_n_s32               __vqdmlsl_n_s32

#define vdup_lane_s64               __vdup_lane_s64
#define vdup_lane_u64               __vdup_lane_u64
#define vdupq_lane_s64              __vdupq_lane_s64
#define vdupq_lane_u64              __vdupq_lane_u64

#endif // !defined(_ARM_ISO_COMPATIBLE_INTRINSIC_NAMES)

#define vceqz_f32             vceq_z_f32_ex
#define vceqz_s16             vceq_z_s16_ex
#define vceqz_s32             vceq_z_s32_ex
#define vceqz_s8              vceq_z_s8_ex
#define vceqz_u16             vceq_z_u16_ex
#define vceqz_u32             vceq_z_u32_ex
#define vceqz_u8              vceq_z_u8_ex
#define vceqqz_f32            vceqq_z_f32_ex
#define vceqqz_s16            vceqq_z_s16_ex
#define vceqqz_s32            vceqq_z_s32_ex
#define vceqqz_s8             vceqq_z_s8_ex
#define vceqqz_u16            vceqq_z_u16_ex
#define vceqqz_u32            vceqq_z_u32_ex
#define vceqqz_u8             vceqq_z_u8_ex

#if defined (__cplusplus)
}
#endif  /* defined (__cplusplus) */

#endif

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS

#endif  /* defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) */
