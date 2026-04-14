/*
 *  Copyright (C) 2007-2025 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/***
* zmmintrin.h - Meta Header file for Intel(R) Architecture intrinsic functions.
*
*******************************************************************************/


#ifndef _ZMMINTRIN_H_INCLUDED
#define _ZMMINTRIN_H_INCLUDED

#ifndef _INCLUDED_IMM
#error "Header should only be included from <immintrin.h>."
#endif

//
// Definitions and declarations for use with 512-bit compiler intrinsics.
//

//
// A word about intrinsic function naming conventions.  Most 512-bit
// vector instructions have names such as v<operation><type>.  For
// example "vaddps" is an addition operation (add) on packed single-
// precision (ps) values.  The corresponding intrinsic is usually
// (not always) named _mm512_<operation>_<type>. For example, the
// "_mm512_add_ps" function generates VADDPS.  The corresponding
// masked flavor adds "_mask" to the name, e.g. "_mm512_mask_add_ps".
//
// The types include:
//
//    ps    -- packed single precision
//    pd    -- packed double precision
//    epi32 -- packed 32-bit integers
//    epu32 -- packed 32-bit unsigned integers
//    epi64 -- packed 64-bit integers
//

typedef unsigned char       __mmask8;
typedef unsigned short      __mmask16;
typedef unsigned int        __mmask32;
typedef unsigned __int64    __mmask64;

typedef union __declspec(intrin_type) __declspec(align(64)) __m512 {
    float m512_f32[16];
} __m512;

typedef struct __declspec(intrin_type) __declspec(align(64)) __m512d {
    double m512d_f64[8];
} __m512d;

typedef union  __declspec(intrin_type) __declspec(align(64)) __m512i {
    __int8              m512i_i8[64];
    __int16             m512i_i16[32];
    __int32             m512i_i32[16];
    __int64             m512i_i64[8];
    unsigned __int8     m512i_u8[64];
    unsigned __int16    m512i_u16[32];
    unsigned __int32    m512i_u32[16];
    unsigned __int64    m512i_u64[8];
} __m512i;


#ifdef __cplusplus
extern "C" {
// Intrinsic functions use C name-mangling.
#endif /* __cplusplus */

/* Conversion from one type to another, no change in value. */
extern __m256  __cdecl _mm512_castps512_ps256(__m512);
extern __m512  __cdecl _mm512_castpd_ps(__m512d);
extern __m512  __cdecl _mm512_castps256_ps512(__m256);
extern __m512  __cdecl _mm512_castsi512_ps(__m512i);
extern __m512  __cdecl _mm512_castps128_ps512(__m128);

extern __m256d __cdecl _mm512_castpd512_pd256(__m512d);
extern __m512d __cdecl _mm512_castpd256_pd512(__m256d);
extern __m512d __cdecl _mm512_castps_pd(__m512);
extern __m512d __cdecl _mm512_castsi512_pd(__m512i);
extern __m512d __cdecl _mm512_castpd128_pd512(__m128d);

extern __m256i __cdecl _mm512_castsi512_si256(__m512i);
extern __m512i __cdecl _mm512_castpd_si512(__m512d);
extern __m512i __cdecl _mm512_castps_si512(__m512);
extern __m512i __cdecl _mm512_castsi256_si512(__m256i);

/* Constant for special read-only mask register 'k0'. */
#define _MM_K0_REG8  (0xff)
#define _MM_K0_REG16 (0xffff)
#define _MM_K0_REG32 (0xffffffff)
#define _MM_K0_REG64 (0xffffffffffffffff)

/* Constants for broadcasts to vectors with 32-bit elements. */
typedef enum {
    _MM_BROADCAST32_NONE,   /* identity swizzle/convert */
#define _MM_BROADCAST_16X16 _MM_BROADCAST32_NONE
    _MM_BROADCAST_1X16,     /* broadcast x 16 ( aaaa aaaa aaaa aaaa ) */
    _MM_BROADCAST_4X16      /* broadcast x 4  ( dcba dcba dcba dcba ) */
} _MM_BROADCAST32_ENUM;

/* Constants for broadcasts to vectors with 64-bit elements. */
typedef enum {
    _MM_BROADCAST64_NONE,   /* identity swizzle/convert */
#define _MM_BROADCAST_8X8 _MM_BROADCAST64_NONE
    _MM_BROADCAST_1X8,      /* broadcast x 8 ( aaaa aaaa ) */
    _MM_BROADCAST_4X8       /* broadcast x 2 ( dcba dcba ) */
} _MM_BROADCAST64_ENUM;

/*
 * Constants for rounding mode.
 * These names beginning with "_MM_ROUND" are deprecated.
 * Use the names beginning with "_MM_FROUND" going forward.
 */
typedef enum {
    _MM_ROUND_MODE_NEAREST,             /* round to nearest (even) */
    _MM_ROUND_MODE_DOWN,                /* round toward negative infinity */
    _MM_ROUND_MODE_UP,                  /* round toward positive infinity */
    _MM_ROUND_MODE_TOWARD_ZERO,         /* round toward zero */
    _MM_ROUND_MODE_DEFAULT,             /* round mode from MXCSR */
    _MM_ROUND_MODE_NO_EXC = 8,          /* suppress all exceptions */
} _MM_ROUND_MODE_ENUM;

/* Constants for exponent adjustment. */
typedef enum {
    _MM_EXPADJ_NONE,               /* 2**0  (32.0 - no exp adjustment) */
    _MM_EXPADJ_4,                  /* 2**4  (28.4)  */
    _MM_EXPADJ_5,                  /* 2**5  (27.5)  */
    _MM_EXPADJ_8,                  /* 2**8  (24.8)  */
    _MM_EXPADJ_16,                 /* 2**16 (16.16) */
    _MM_EXPADJ_24,                 /* 2**24 (8.24)  */
    _MM_EXPADJ_31,                 /* 2**31 (1.31)  */
    _MM_EXPADJ_32                  /* 2**32 (0.32)  */
} _MM_EXP_ADJ_ENUM;

/* Constants for index scale (vgather/vscatter). */
typedef enum {
    _MM_SCALE_1 = 1,
    _MM_SCALE_2 = 2,
    _MM_SCALE_4 = 4,
    _MM_SCALE_8 = 8
} _MM_INDEX_SCALE_ENUM;

/* Constants for permute/shuffle. */
typedef enum {
    _MM_PERM_AAAA = 0x00, _MM_PERM_AAAB = 0x01, _MM_PERM_AAAC = 0x02,
    _MM_PERM_AAAD = 0x03, _MM_PERM_AABA = 0x04, _MM_PERM_AABB = 0x05,
    _MM_PERM_AABC = 0x06, _MM_PERM_AABD = 0x07, _MM_PERM_AACA = 0x08,
    _MM_PERM_AACB = 0x09, _MM_PERM_AACC = 0x0A, _MM_PERM_AACD = 0x0B,
    _MM_PERM_AADA = 0x0C, _MM_PERM_AADB = 0x0D, _MM_PERM_AADC = 0x0E,
    _MM_PERM_AADD = 0x0F, _MM_PERM_ABAA = 0x10, _MM_PERM_ABAB = 0x11,
    _MM_PERM_ABAC = 0x12, _MM_PERM_ABAD = 0x13, _MM_PERM_ABBA = 0x14,
    _MM_PERM_ABBB = 0x15, _MM_PERM_ABBC = 0x16, _MM_PERM_ABBD = 0x17,
    _MM_PERM_ABCA = 0x18, _MM_PERM_ABCB = 0x19, _MM_PERM_ABCC = 0x1A,
    _MM_PERM_ABCD = 0x1B, _MM_PERM_ABDA = 0x1C, _MM_PERM_ABDB = 0x1D,
    _MM_PERM_ABDC = 0x1E, _MM_PERM_ABDD = 0x1F, _MM_PERM_ACAA = 0x20,
    _MM_PERM_ACAB = 0x21, _MM_PERM_ACAC = 0x22, _MM_PERM_ACAD = 0x23,
    _MM_PERM_ACBA = 0x24, _MM_PERM_ACBB = 0x25, _MM_PERM_ACBC = 0x26,
    _MM_PERM_ACBD = 0x27, _MM_PERM_ACCA = 0x28, _MM_PERM_ACCB = 0x29,
    _MM_PERM_ACCC = 0x2A, _MM_PERM_ACCD = 0x2B, _MM_PERM_ACDA = 0x2C,
    _MM_PERM_ACDB = 0x2D, _MM_PERM_ACDC = 0x2E, _MM_PERM_ACDD = 0x2F,
    _MM_PERM_ADAA = 0x30, _MM_PERM_ADAB = 0x31, _MM_PERM_ADAC = 0x32,
    _MM_PERM_ADAD = 0x33, _MM_PERM_ADBA = 0x34, _MM_PERM_ADBB = 0x35,
    _MM_PERM_ADBC = 0x36, _MM_PERM_ADBD = 0x37, _MM_PERM_ADCA = 0x38,
    _MM_PERM_ADCB = 0x39, _MM_PERM_ADCC = 0x3A, _MM_PERM_ADCD = 0x3B,
    _MM_PERM_ADDA = 0x3C, _MM_PERM_ADDB = 0x3D, _MM_PERM_ADDC = 0x3E,
    _MM_PERM_ADDD = 0x3F, _MM_PERM_BAAA = 0x40, _MM_PERM_BAAB = 0x41,
    _MM_PERM_BAAC = 0x42, _MM_PERM_BAAD = 0x43, _MM_PERM_BABA = 0x44,
    _MM_PERM_BABB = 0x45, _MM_PERM_BABC = 0x46, _MM_PERM_BABD = 0x47,
    _MM_PERM_BACA = 0x48, _MM_PERM_BACB = 0x49, _MM_PERM_BACC = 0x4A,
    _MM_PERM_BACD = 0x4B, _MM_PERM_BADA = 0x4C, _MM_PERM_BADB = 0x4D,
    _MM_PERM_BADC = 0x4E, _MM_PERM_BADD = 0x4F, _MM_PERM_BBAA = 0x50,
    _MM_PERM_BBAB = 0x51, _MM_PERM_BBAC = 0x52, _MM_PERM_BBAD = 0x53,
    _MM_PERM_BBBA = 0x54, _MM_PERM_BBBB = 0x55, _MM_PERM_BBBC = 0x56,
    _MM_PERM_BBBD = 0x57, _MM_PERM_BBCA = 0x58, _MM_PERM_BBCB = 0x59,
    _MM_PERM_BBCC = 0x5A, _MM_PERM_BBCD = 0x5B, _MM_PERM_BBDA = 0x5C,
    _MM_PERM_BBDB = 0x5D, _MM_PERM_BBDC = 0x5E, _MM_PERM_BBDD = 0x5F,
    _MM_PERM_BCAA = 0x60, _MM_PERM_BCAB = 0x61, _MM_PERM_BCAC = 0x62,
    _MM_PERM_BCAD = 0x63, _MM_PERM_BCBA = 0x64, _MM_PERM_BCBB = 0x65,
    _MM_PERM_BCBC = 0x66, _MM_PERM_BCBD = 0x67, _MM_PERM_BCCA = 0x68,
    _MM_PERM_BCCB = 0x69, _MM_PERM_BCCC = 0x6A, _MM_PERM_BCCD = 0x6B,
    _MM_PERM_BCDA = 0x6C, _MM_PERM_BCDB = 0x6D, _MM_PERM_BCDC = 0x6E,
    _MM_PERM_BCDD = 0x6F, _MM_PERM_BDAA = 0x70, _MM_PERM_BDAB = 0x71,
    _MM_PERM_BDAC = 0x72, _MM_PERM_BDAD = 0x73, _MM_PERM_BDBA = 0x74,
    _MM_PERM_BDBB = 0x75, _MM_PERM_BDBC = 0x76, _MM_PERM_BDBD = 0x77,
    _MM_PERM_BDCA = 0x78, _MM_PERM_BDCB = 0x79, _MM_PERM_BDCC = 0x7A,
    _MM_PERM_BDCD = 0x7B, _MM_PERM_BDDA = 0x7C, _MM_PERM_BDDB = 0x7D,
    _MM_PERM_BDDC = 0x7E, _MM_PERM_BDDD = 0x7F, _MM_PERM_CAAA = 0x80,
    _MM_PERM_CAAB = 0x81, _MM_PERM_CAAC = 0x82, _MM_PERM_CAAD = 0x83,
    _MM_PERM_CABA = 0x84, _MM_PERM_CABB = 0x85, _MM_PERM_CABC = 0x86,
    _MM_PERM_CABD = 0x87, _MM_PERM_CACA = 0x88, _MM_PERM_CACB = 0x89,
    _MM_PERM_CACC = 0x8A, _MM_PERM_CACD = 0x8B, _MM_PERM_CADA = 0x8C,
    _MM_PERM_CADB = 0x8D, _MM_PERM_CADC = 0x8E, _MM_PERM_CADD = 0x8F,
    _MM_PERM_CBAA = 0x90, _MM_PERM_CBAB = 0x91, _MM_PERM_CBAC = 0x92,
    _MM_PERM_CBAD = 0x93, _MM_PERM_CBBA = 0x94, _MM_PERM_CBBB = 0x95,
    _MM_PERM_CBBC = 0x96, _MM_PERM_CBBD = 0x97, _MM_PERM_CBCA = 0x98,
    _MM_PERM_CBCB = 0x99, _MM_PERM_CBCC = 0x9A, _MM_PERM_CBCD = 0x9B,
    _MM_PERM_CBDA = 0x9C, _MM_PERM_CBDB = 0x9D, _MM_PERM_CBDC = 0x9E,
    _MM_PERM_CBDD = 0x9F, _MM_PERM_CCAA = 0xA0, _MM_PERM_CCAB = 0xA1,
    _MM_PERM_CCAC = 0xA2, _MM_PERM_CCAD = 0xA3, _MM_PERM_CCBA = 0xA4,
    _MM_PERM_CCBB = 0xA5, _MM_PERM_CCBC = 0xA6, _MM_PERM_CCBD = 0xA7,
    _MM_PERM_CCCA = 0xA8, _MM_PERM_CCCB = 0xA9, _MM_PERM_CCCC = 0xAA,
    _MM_PERM_CCCD = 0xAB, _MM_PERM_CCDA = 0xAC, _MM_PERM_CCDB = 0xAD,
    _MM_PERM_CCDC = 0xAE, _MM_PERM_CCDD = 0xAF, _MM_PERM_CDAA = 0xB0,
    _MM_PERM_CDAB = 0xB1, _MM_PERM_CDAC = 0xB2, _MM_PERM_CDAD = 0xB3,
    _MM_PERM_CDBA = 0xB4, _MM_PERM_CDBB = 0xB5, _MM_PERM_CDBC = 0xB6,
    _MM_PERM_CDBD = 0xB7, _MM_PERM_CDCA = 0xB8, _MM_PERM_CDCB = 0xB9,
    _MM_PERM_CDCC = 0xBA, _MM_PERM_CDCD = 0xBB, _MM_PERM_CDDA = 0xBC,
    _MM_PERM_CDDB = 0xBD, _MM_PERM_CDDC = 0xBE, _MM_PERM_CDDD = 0xBF,
    _MM_PERM_DAAA = 0xC0, _MM_PERM_DAAB = 0xC1, _MM_PERM_DAAC = 0xC2,
    _MM_PERM_DAAD = 0xC3, _MM_PERM_DABA = 0xC4, _MM_PERM_DABB = 0xC5,
    _MM_PERM_DABC = 0xC6, _MM_PERM_DABD = 0xC7, _MM_PERM_DACA = 0xC8,
    _MM_PERM_DACB = 0xC9, _MM_PERM_DACC = 0xCA, _MM_PERM_DACD = 0xCB,
    _MM_PERM_DADA = 0xCC, _MM_PERM_DADB = 0xCD, _MM_PERM_DADC = 0xCE,
    _MM_PERM_DADD = 0xCF, _MM_PERM_DBAA = 0xD0, _MM_PERM_DBAB = 0xD1,
    _MM_PERM_DBAC = 0xD2, _MM_PERM_DBAD = 0xD3, _MM_PERM_DBBA = 0xD4,
    _MM_PERM_DBBB = 0xD5, _MM_PERM_DBBC = 0xD6, _MM_PERM_DBBD = 0xD7,
    _MM_PERM_DBCA = 0xD8, _MM_PERM_DBCB = 0xD9, _MM_PERM_DBCC = 0xDA,
    _MM_PERM_DBCD = 0xDB, _MM_PERM_DBDA = 0xDC, _MM_PERM_DBDB = 0xDD,
    _MM_PERM_DBDC = 0xDE, _MM_PERM_DBDD = 0xDF, _MM_PERM_DCAA = 0xE0,
    _MM_PERM_DCAB = 0xE1, _MM_PERM_DCAC = 0xE2, _MM_PERM_DCAD = 0xE3,
    _MM_PERM_DCBA = 0xE4, _MM_PERM_DCBB = 0xE5, _MM_PERM_DCBC = 0xE6,
    _MM_PERM_DCBD = 0xE7, _MM_PERM_DCCA = 0xE8, _MM_PERM_DCCB = 0xE9,
    _MM_PERM_DCCC = 0xEA, _MM_PERM_DCCD = 0xEB, _MM_PERM_DCDA = 0xEC,
    _MM_PERM_DCDB = 0xED, _MM_PERM_DCDC = 0xEE, _MM_PERM_DCDD = 0xEF,
    _MM_PERM_DDAA = 0xF0, _MM_PERM_DDAB = 0xF1, _MM_PERM_DDAC = 0xF2,
    _MM_PERM_DDAD = 0xF3, _MM_PERM_DDBA = 0xF4, _MM_PERM_DDBB = 0xF5,
    _MM_PERM_DDBC = 0xF6, _MM_PERM_DDBD = 0xF7, _MM_PERM_DDCA = 0xF8,
    _MM_PERM_DDCB = 0xF9, _MM_PERM_DDCC = 0xFA, _MM_PERM_DDCD = 0xFB,
    _MM_PERM_DDDA = 0xFC, _MM_PERM_DDDB = 0xFD, _MM_PERM_DDDC = 0xFE,
    _MM_PERM_DDDD = 0xFF
} _MM_PERM_ENUM;

/*
 * Helper type and macro for computing the values of the immediate
 * used in mm512_fixup_ps.
 */
typedef enum {
    _MM_FIXUP_NO_CHANGE,
    _MM_FIXUP_NEG_INF,
    _MM_FIXUP_NEG_ZERO,
    _MM_FIXUP_POS_ZERO,
    _MM_FIXUP_POS_INF,
    _MM_FIXUP_NAN,
    _MM_FIXUP_MAX_FLOAT,
    _MM_FIXUP_MIN_FLOAT
} _MM_FIXUPRESULT_ENUM;

#define _MM_FIXUP(_NegInf, \
                  _Neg, \
                  _NegZero, \
                  _PosZero, \
                  _Pos, \
                  _PosInf, \
                  _Nan) \
   ((int) (_NegInf) | \
   ((int) (_Neg) << 3) | \
   ((int) (_NegZero) << 6) | \
   ((int) (_PosZero) << 9) | \
   ((int) (_Pos) << 12) | \
   ((int) (_PosInf) << 15) | \
   ((int) (_Nan) << 18))

/*
* Extract float32 or float64 normalized mantissas.
*/

/* Constants for mantissa extraction */
typedef enum {
    _MM_MANT_NORM_1_2,      /* interval [1, 2)      */
    _MM_MANT_NORM_p5_2,     /* interval [1.5, 2)    */
    _MM_MANT_NORM_p5_1,     /* interval [1.5, 1)    */
    _MM_MANT_NORM_p75_1p5   /* interval [0.75, 1.5) */
} _MM_MANTISSA_NORM_ENUM;

typedef enum {
    _MM_MANT_SIGN_src,      /* sign = sign(SRC)     */
    _MM_MANT_SIGN_zero,     /* sign = 0             */
    _MM_MANT_SIGN_nan       /* DEST = NaN if sign(SRC) = 1 */
} _MM_MANTISSA_SIGN_ENUM;

/*
* Compare float32, float64 or int32 vectors and set mask.
*/

/* Constants for integer comparison predicates */
typedef enum {
    _MM_CMPINT_EQ,      /* Equal */
    _MM_CMPINT_LT,      /* Less than */
    _MM_CMPINT_LE,      /* Less than or Equal */
    _MM_CMPINT_UNUSED,
    _MM_CMPINT_NE,      /* Not Equal */
    _MM_CMPINT_NLT,     /* Not Less than */
#define _MM_CMPINT_GE   _MM_CMPINT_NLT  /* Greater than or Equal */
    _MM_CMPINT_NLE      /* Not Less than or Equal */
#define _MM_CMPINT_GT   _MM_CMPINT_NLE  /* Greater than */
} _MM_CMPINT_ENUM;


/*
* Intel(R) AVX-512 intrinsic functions
*/
extern __m512  __cdecl _mm512_setzero_ps(void);
extern __m512d __cdecl _mm512_setzero_pd(void);

extern __m512  __cdecl _mm512_set_ps(float /* e15 */, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float /* e0 */);
extern __m512d __cdecl _mm512_set_pd(double /* e7 */, double, double, double, double, double, double, double /* e0 */);

extern __m512  __cdecl _mm512_setr_ps(float /* e0 */, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float /* e15 */);
extern __m512d __cdecl _mm512_setr_pd(double /* e0 */, double, double, double, double, double, double, double /* e7 */);

extern __m512  __cdecl _mm512_set1_ps(float);
extern __m512d __cdecl _mm512_set1_pd(double);

extern __m512  __cdecl _mm512_load_ps(void const*);
extern __m512d __cdecl _mm512_load_pd(void const*);
extern __m512  __cdecl _mm512_maskz_load_ps(__mmask16, void const*);
extern __m512d __cdecl _mm512_maskz_load_pd(__mmask8, void const*);
extern __m512  __cdecl _mm512_mask_load_ps(__m512, __mmask16, void const*);
extern __m512d __cdecl _mm512_mask_load_pd(__m512d, __mmask8, void const*);
extern __m512  __cdecl _mm512_loadu_ps(void const*);
extern __m512d __cdecl _mm512_loadu_pd(void const*);
extern __m512  __cdecl _mm512_maskz_loadu_ps(__mmask16, void const*);
extern __m512d __cdecl _mm512_maskz_loadu_pd(__mmask8, void const*);
extern __m512  __cdecl _mm512_mask_loadu_ps(__m512, __mmask16, void const*);
extern __m512d __cdecl _mm512_mask_loadu_pd(__m512d, __mmask8, void const*);

extern void    __cdecl _mm512_store_ps(void*, __m512);
extern void    __cdecl _mm512_store_pd(void*, __m512d);
extern void    __cdecl _mm512_storeu_ps(void*, __m512);
extern void    __cdecl _mm512_storeu_pd(void*, __m512d);
extern void    __cdecl _mm512_mask_store_ps(void*, __mmask16, __m512);
extern void    __cdecl _mm512_mask_store_pd(void*, __mmask8, __m512d);
extern void    __cdecl _mm512_mask_storeu_ps(void*, __mmask16, __m512);
extern void    __cdecl _mm512_mask_storeu_pd(void*, __mmask8, __m512d);

extern __m512  __cdecl _mm512_add_ps(__m512, __m512);
extern __m512  __cdecl _mm512_maskz_add_ps(__mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask_add_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_add_round_ps(__m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_add_round_ps(__mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_add_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_add_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_add_pd(__mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_add_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_add_round_pd(__m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_add_round_pd(__mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_add_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_sub_ps(__m512, __m512);
extern __m512  __cdecl _mm512_maskz_sub_ps(__mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask_sub_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_sub_round_ps(__m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_sub_round_ps(__mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_sub_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_sub_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_sub_pd(__mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_sub_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_sub_round_pd(__m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_sub_round_pd(__mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_sub_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_mul_ps(__m512, __m512);
extern __m512  __cdecl _mm512_maskz_mul_ps(__mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask_mul_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mul_round_ps( __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_mul_round_ps(__mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_mul_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_mul_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_mul_pd(__mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_mul_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mul_round_pd(__m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_mul_round_pd(__mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_mul_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_div_ps(__m512, __m512);
extern __m512  __cdecl _mm512_maskz_div_ps(__mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask_div_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_div_round_ps(__m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_div_round_ps(__mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_div_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_div_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_div_pd(__mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_div_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_div_round_pd(__m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_div_round_pd(__mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_div_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_fmadd_ps(__m512, __m512, __m512);
extern __m512  __cdecl _mm512_mask_fmadd_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask3_fmadd_ps(__m512, __m512, __m512, __mmask16);
extern __m512  __cdecl _mm512_maskz_fmadd_ps(__mmask16, __m512, __m512, __m512);
extern __m512  __cdecl _mm512_fmadd_round_ps(__m512, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_fmadd_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask3_fmadd_round_ps(__m512, __m512, __m512, __mmask16, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_fmadd_round_ps(__mmask16, __m512, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_fmadd_pd(__m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_fmadd_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask3_fmadd_pd(__m512d, __m512d, __m512d, __mmask8);
extern __m512d __cdecl _mm512_maskz_fmadd_pd(__mmask8, __m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_fmadd_round_pd(__m512d, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_fmadd_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask3_fmadd_round_pd(__m512d, __m512d, __m512d, __mmask8, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_fmadd_round_pd(__mmask8, __m512d, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_fmsub_ps(__m512, __m512, __m512);
extern __m512  __cdecl _mm512_mask_fmsub_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask3_fmsub_ps(__m512, __m512, __m512, __mmask16);
extern __m512  __cdecl _mm512_maskz_fmsub_ps(__mmask16, __m512, __m512, __m512);
extern __m512  __cdecl _mm512_fmsub_round_ps(__m512, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_fmsub_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask3_fmsub_round_ps(__m512, __m512, __m512, __mmask16, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_fmsub_round_ps(__mmask16, __m512, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_fmsub_pd(__m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_fmsub_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask3_fmsub_pd(__m512d, __m512d, __m512d, __mmask8);
extern __m512d __cdecl _mm512_maskz_fmsub_pd(__mmask8, __m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_fmsub_round_pd(__m512d, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_fmsub_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask3_fmsub_round_pd(__m512d, __m512d, __m512d, __mmask8, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_fmsub_round_pd(__mmask8, __m512d, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_fmaddsub_ps(__m512, __m512, __m512);
extern __m512  __cdecl _mm512_mask_fmaddsub_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask3_fmaddsub_ps(__m512, __m512, __m512, __mmask16);
extern __m512  __cdecl _mm512_maskz_fmaddsub_ps(__mmask16, __m512, __m512, __m512);
extern __m512  __cdecl _mm512_fmaddsub_round_ps(__m512, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_fmaddsub_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask3_fmaddsub_round_ps(__m512, __m512, __m512, __mmask16, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_fmaddsub_round_ps(__mmask16, __m512, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_fmaddsub_pd(__m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_fmaddsub_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask3_fmaddsub_pd(__m512d, __m512d, __m512d, __mmask8);
extern __m512d __cdecl _mm512_maskz_fmaddsub_pd(__mmask8, __m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_fmaddsub_round_pd(__m512d, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_fmaddsub_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask3_fmaddsub_round_pd(__m512d, __m512d, __m512d, __mmask8, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_fmaddsub_round_pd(__mmask8, __m512d, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_fmsubadd_ps(__m512, __m512, __m512);
extern __m512  __cdecl _mm512_mask_fmsubadd_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask3_fmsubadd_ps(__m512, __m512, __m512, __mmask16);
extern __m512  __cdecl _mm512_maskz_fmsubadd_ps(__mmask16, __m512, __m512, __m512);
extern __m512  __cdecl _mm512_fmsubadd_round_ps(__m512, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_fmsubadd_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask3_fmsubadd_round_ps(__m512, __m512, __m512, __mmask16, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_fmsubadd_round_ps(__mmask16, __m512, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_fmsubadd_pd(__m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_fmsubadd_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask3_fmsubadd_pd(__m512d, __m512d, __m512d, __mmask8);
extern __m512d __cdecl _mm512_maskz_fmsubadd_pd(__mmask8, __m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_fmsubadd_round_pd(__m512d, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_fmsubadd_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask3_fmsubadd_round_pd(__m512d, __m512d, __m512d, __mmask8, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_fmsubadd_round_pd(__mmask8, __m512d, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_fnmadd_ps(__m512, __m512, __m512);
extern __m512  __cdecl _mm512_mask_fnmadd_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask3_fnmadd_ps(__m512, __m512, __m512, __mmask16);
extern __m512  __cdecl _mm512_maskz_fnmadd_ps(__mmask16, __m512, __m512, __m512);
extern __m512  __cdecl _mm512_fnmadd_round_ps(__m512, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_fnmadd_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask3_fnmadd_round_ps(__m512, __m512, __m512, __mmask16, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_fnmadd_round_ps(__mmask16, __m512, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_fnmadd_pd(__m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_fnmadd_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask3_fnmadd_pd(__m512d, __m512d, __m512d, __mmask8);
extern __m512d __cdecl _mm512_maskz_fnmadd_pd(__mmask8, __m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_fnmadd_round_pd(__m512d, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_fnmadd_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask3_fnmadd_round_pd(__m512d, __m512d, __m512d, __mmask8, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_fnmadd_round_pd(__mmask8, __m512d, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_fnmsub_ps(__m512, __m512, __m512);
extern __m512  __cdecl _mm512_mask_fnmsub_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_mask3_fnmsub_ps(__m512, __m512, __m512, __mmask16);
extern __m512  __cdecl _mm512_maskz_fnmsub_ps(__mmask16, __m512, __m512, __m512);
extern __m512  __cdecl _mm512_fnmsub_round_ps(__m512, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_fnmsub_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512  __cdecl _mm512_mask3_fnmsub_round_ps(__m512, __m512, __m512, __mmask16, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_fnmsub_round_ps(__mmask16, __m512, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_fnmsub_pd(__m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask_fnmsub_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_mask3_fnmsub_pd(__m512d, __m512d, __m512d, __mmask8);
extern __m512d __cdecl _mm512_maskz_fnmsub_pd(__mmask8, __m512d, __m512d, __m512d);
extern __m512d __cdecl _mm512_fnmsub_round_pd(__m512d, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_fnmsub_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask3_fnmsub_round_pd(__m512d, __m512d, __m512d, __mmask8, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_fnmsub_round_pd(__mmask8, __m512d, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_sqrt_ps(__m512);
extern __m512d __cdecl _mm512_sqrt_pd(__m512d);
extern __m512  __cdecl _mm512_maskz_sqrt_ps(__mmask16, __m512);
extern __m512d __cdecl _mm512_maskz_sqrt_pd(__mmask8, __m512d);
extern __m512  __cdecl _mm512_mask_sqrt_ps(__m512, __mmask16, __m512);
extern __m512d __cdecl _mm512_mask_sqrt_pd(__m512d, __mmask8, __m512d);
extern __m512  __cdecl _mm512_sqrt_round_ps(__m512, const int /* rounding */);
extern __m512d __cdecl _mm512_sqrt_round_pd(__m512d, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_sqrt_round_ps(__mmask16, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_sqrt_round_pd(__mmask8, __m512d, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_sqrt_round_ps(__m512, __mmask16, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_sqrt_round_pd(__m512d, __mmask8, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_abs_ps(__m512);
extern __m512  __cdecl _mm512_maskz_abs_ps(__mmask16, __m512);
extern __m512  __cdecl _mm512_mask_abs_ps(__m512, __mmask16, __m512);
extern __m512d __cdecl _mm512_abs_pd(__m512d);
extern __m512d __cdecl _mm512_maskz_abs_pd(__mmask8, __m512d);
extern __m512d __cdecl _mm512_mask_abs_pd(__m512d, __mmask8, __m512d);

extern __m512  __cdecl _mm512_max_ps(__m512, __m512);
extern __m512d __cdecl _mm512_max_pd(__m512d, __m512d);
extern __m512  __cdecl _mm512_maskz_max_ps(__mmask16, __m512, __m512);
extern __m512d __cdecl _mm512_maskz_max_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_mask_max_ps(__m512, __mmask16, __m512, __m512);
extern __m512d __cdecl _mm512_mask_max_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_max_round_ps(__m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_max_round_pd(__m512d, __m512d, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_max_round_ps(__mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_max_round_pd(__mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_max_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_max_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_min_ps(__m512, __m512);
extern __m512d __cdecl _mm512_min_pd(__m512d, __m512d);
extern __m512  __cdecl _mm512_maskz_min_ps(__mmask16, __m512, __m512);
extern __m512d __cdecl _mm512_maskz_min_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_mask_min_ps(__m512, __mmask16, __m512, __m512);
extern __m512d __cdecl _mm512_mask_min_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_min_round_ps(__m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_min_round_pd(__m512d, __m512d, const int /* rounding */);
extern __m512  __cdecl _mm512_maskz_min_round_ps(__mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_min_round_pd(__mmask8, __m512d, __m512d, const int /* rounding */);
extern __m512  __cdecl _mm512_mask_min_round_ps(__m512, __mmask16, __m512, __m512, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_min_round_pd(__m512d, __mmask8, __m512d, __m512d, const int /* rounding */);

extern __m512  __cdecl _mm512_rcp14_ps(__m512);
extern __m512d __cdecl _mm512_rcp14_pd(__m512d);
extern __m512  __cdecl _mm512_maskz_rcp14_ps(__mmask16, __m512);
extern __m512d __cdecl _mm512_maskz_rcp14_pd(__mmask8, __m512d);
extern __m512  __cdecl _mm512_mask_rcp14_ps(__m512, __mmask16, __m512);
extern __m512d __cdecl _mm512_mask_rcp14_pd(__m512d, __mmask8, __m512d);

extern __m512  __cdecl _mm512_rsqrt14_ps(__m512);
extern __m512d __cdecl _mm512_rsqrt14_pd(__m512d);
extern __m512  __cdecl _mm512_maskz_rsqrt14_ps(__mmask16, __m512);
extern __m512d __cdecl _mm512_maskz_rsqrt14_pd(__mmask8, __m512d);
extern __m512  __cdecl _mm512_mask_rsqrt14_ps(__m512, __mmask16, __m512);
extern __m512d __cdecl _mm512_mask_rsqrt14_pd(__m512d, __mmask8, __m512d);

extern __m512d __cdecl _mm512_cvtps_pd(__m256);
extern __m256  __cdecl _mm512_cvtpd_ps(__m512d);
extern __m512d __cdecl _mm512_maskz_cvtps_pd(__mmask8, __m256);
extern __m256  __cdecl _mm512_maskz_cvtpd_ps(__mmask8, __m512d);
extern __m512d __cdecl _mm512_mask_cvtps_pd(__m512d, __mmask8, __m256);
extern __m256  __cdecl _mm512_mask_cvtpd_ps(__m256, __mmask8, __m512d);
extern __m512d __cdecl _mm512_cvt_roundps_pd(__m256, const int /* rounding */);
extern __m256  __cdecl _mm512_cvt_roundpd_ps(__m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_maskz_cvt_roundps_pd(__mmask8, __m256, const int /* rounding */);
extern __m256  __cdecl _mm512_maskz_cvt_roundpd_ps(__mmask8, __m512d, const int /* rounding */);
extern __m512d __cdecl _mm512_mask_cvt_roundps_pd(__m512d, __mmask8, __m256, const int /* rounding */);
extern __m256  __cdecl _mm512_mask_cvt_roundpd_ps(__m256, __mmask8, __m512d, const int /* rounding */);

extern __mmask16 __cdecl _mm512_cmp_ps_mask(__m512, __m512, const int);
extern __mmask16 __cdecl _mm512_mask_cmp_ps_mask(__mmask16, __m512, __m512, const int);
extern __mmask16 __cdecl _mm512_cmp_round_ps_mask(__m512, __m512, const int, const int /* rounding */);
extern __mmask16 __cdecl _mm512_mask_cmp_round_ps_mask(__mmask16, __m512, __m512, const int, const int /* rounding */);
extern __mmask8  __cdecl _mm512_cmp_pd_mask(__m512d, __m512d, const int);
extern __mmask8  __cdecl _mm512_mask_cmp_pd_mask(__mmask8, __m512d, __m512d, const int);
extern __mmask8  __cdecl _mm512_cmp_round_pd_mask(__m512d, __m512d, const int, const int /* rounding */);
extern __mmask8  __cdecl _mm512_mask_cmp_round_pd_mask(__mmask8, __m512d, __m512d, const int, const int /* rounding */);

extern __m512  __cdecl _mm512_broadcast_f32x2(__m128);
extern __m512  __cdecl _mm512_mask_broadcast_f32x2(__m512, __mmask16, __m128);
extern __m512  __cdecl _mm512_maskz_broadcast_f32x2(__mmask16, __m128);
extern __m512  __cdecl _mm512_broadcast_f32x4(__m128);
extern __m512  __cdecl _mm512_mask_broadcast_f32x4(__m512, __mmask16, __m128);
extern __m512  __cdecl _mm512_maskz_broadcast_f32x4(__mmask16, __m128);
extern __m512  __cdecl _mm512_broadcast_f32x8(__m256);
extern __m512  __cdecl _mm512_mask_broadcast_f32x8(__m512, __mmask16, __m256);
extern __m512  __cdecl _mm512_maskz_broadcast_f32x8(__mmask16, __m256);
extern __m512d __cdecl _mm512_broadcast_f64x2(__m128d);
extern __m512d __cdecl _mm512_mask_broadcast_f64x2(__m512d, __mmask8, __m128d);
extern __m512d __cdecl _mm512_maskz_broadcast_f64x2(__mmask8, __m128d);
extern __m512d __cdecl _mm512_broadcast_f64x4(__m256d);
extern __m512d __cdecl _mm512_mask_broadcast_f64x4(__m512d, __mmask8, __m256d);
extern __m512d __cdecl _mm512_maskz_broadcast_f64x4(__mmask8, __m256d);
extern __m512d __cdecl _mm512_broadcastsd_pd(__m128d);
extern __m512d __cdecl _mm512_mask_broadcastsd_pd(__m512d, __mmask8, __m128d);
extern __m512d __cdecl _mm512_maskz_broadcastsd_pd(__mmask8, __m128d);
extern __m512  __cdecl _mm512_broadcastss_ps(__m128);
extern __m512  __cdecl _mm512_mask_broadcastss_ps(__m512, __mmask16, __m128);
extern __m512  __cdecl _mm512_maskz_broadcastss_ps(__mmask16, __m128);

extern __m128  __cdecl _mm512_extractf32x4_ps(__m512, int);
extern __m128  __cdecl _mm512_mask_extractf32x4_ps(__m128, __mmask8, __m512, const int);
extern __m128  __cdecl _mm512_maskz_extractf32x4_ps(__mmask8, __m512, int);
extern __m256  __cdecl _mm512_extractf32x8_ps(__m512, int);
extern __m256  __cdecl _mm512_mask_extractf32x8_ps(__m256, __mmask8, __m512, const int);
extern __m256  __cdecl _mm512_maskz_extractf32x8_ps(__mmask8, __m512, int);
extern __m128d __cdecl _mm512_extractf64x2_pd(__m512d, int);
extern __m128d __cdecl _mm512_mask_extractf64x2_pd(__m128d, __mmask8, __m512d, const int);
extern __m128d __cdecl _mm512_maskz_extractf64x2_pd(__mmask8, __m512d, int);
extern __m256d __cdecl _mm512_extractf64x4_pd(__m512d, int);
extern __m256d __cdecl _mm512_mask_extractf64x4_pd(__m256d, __mmask8, __m512d, const int);
extern __m256d __cdecl _mm512_maskz_extractf64x4_pd(__mmask8, __m512d, int);

extern __m512  __cdecl _mm512_insertf32x4(__m512, __m128, int);
extern __m512  __cdecl _mm512_mask_insertf32x4(__m512, __mmask16, __m512, __m128, const int);
extern __m512  __cdecl _mm512_maskz_insertf32x4(__mmask16, __m512, __m128, int);
extern __m512  __cdecl _mm512_insertf32x8(__m512, __m256, int);
extern __m512  __cdecl _mm512_mask_insertf32x8(__m512, __mmask16, __m512, __m256, const int);
extern __m512  __cdecl _mm512_maskz_insertf32x8(__mmask16, __m512, __m256, int);
extern __m512d __cdecl _mm512_insertf64x2(__m512d, __m128d, int);
extern __m512d __cdecl _mm512_mask_insertf64x2(__m512d, __mmask8, __m512d, __m128d, const int);
extern __m512d __cdecl _mm512_maskz_insertf64x2(__mmask8, __m512d, __m128d, int);
extern __m512d __cdecl _mm512_insertf64x4(__m512d, __m256d, int);
extern __m512d __cdecl _mm512_mask_insertf64x4(__m512d, __mmask8, __m512d, __m256d, const int);
extern __m512d __cdecl _mm512_maskz_insertf64x4(__mmask8, __m512d, __m256d, int);

extern __m512  __cdecl _mm512_shuffle_f32x4(__m512, __m512, const int);
extern __m512  __cdecl _mm512_mask_shuffle_f32x4(__m512, __mmask16, __m512, __m512, const int);
extern __m512  __cdecl _mm512_maskz_shuffle_f32x4(__mmask16, __m512, __m512, const int);
extern __m512d __cdecl _mm512_shuffle_f64x2(__m512d, __m512d, const int);
extern __m512d __cdecl _mm512_mask_shuffle_f64x2(__m512d, __mmask8, __m512d, __m512d, const int);
extern __m512d __cdecl _mm512_maskz_shuffle_f64x2(__mmask8, __m512d, __m512d, const int);
extern __m512d __cdecl _mm512_shuffle_pd(__m512d, __m512d, const int);
extern __m512d __cdecl _mm512_mask_shuffle_pd(__m512d, __mmask8, __m512d, __m512d, const int);
extern __m512d __cdecl _mm512_maskz_shuffle_pd(__mmask8, __m512d, __m512d, const int);
extern __m512  __cdecl _mm512_shuffle_ps(__m512, __m512, const int);
extern __m512  __cdecl _mm512_mask_shuffle_ps(__m512, __mmask16, __m512, __m512, const int);
extern __m512  __cdecl _mm512_maskz_shuffle_ps(__mmask16, __m512, __m512, const int);

extern __mmask16 _mm512_cmpeq_ps_mask(__m512, __m512);
extern __mmask16 _mm512_cmple_ps_mask(__m512, __m512);
extern __mmask16 _mm512_cmplt_ps_mask(__m512, __m512);
extern __mmask16 _mm512_cmpneq_ps_mask(__m512, __m512);
extern __mmask16 _mm512_cmpnle_ps_mask(__m512, __m512);
extern __mmask16 _mm512_cmpnlt_ps_mask(__m512, __m512);
extern __mmask16 _mm512_cmpord_ps_mask(__m512, __m512);
extern __mmask16 _mm512_cmpunord_ps_mask(__m512, __m512);

extern __mmask16 _mm512_mask_cmpeq_ps_mask(__mmask16, __m512, __m512);
extern __mmask16 _mm512_mask_cmple_ps_mask(__mmask16, __m512, __m512);
extern __mmask16 _mm512_mask_cmplt_ps_mask(__mmask16, __m512, __m512);
extern __mmask16 _mm512_mask_cmpneq_ps_mask(__mmask16, __m512, __m512);
extern __mmask16 _mm512_mask_cmpnle_ps_mask(__mmask16, __m512, __m512);
extern __mmask16 _mm512_mask_cmpnlt_ps_mask(__mmask16, __m512, __m512);
extern __mmask16 _mm512_mask_cmpord_ps_mask(__mmask16, __m512, __m512);
extern __mmask16 _mm512_mask_cmpunord_ps_mask(__mmask16, __m512, __m512);

extern __mmask8 _mm512_cmpeq_pd_mask(__m512d, __m512d);
extern __mmask8 _mm512_cmple_pd_mask(__m512d, __m512d);
extern __mmask8 _mm512_cmplt_pd_mask(__m512d, __m512d);
extern __mmask8 _mm512_cmpneq_pd_mask(__m512d, __m512d);
extern __mmask8 _mm512_cmpnle_pd_mask(__m512d, __m512d);
extern __mmask8 _mm512_cmpnlt_pd_mask(__m512d, __m512d);
extern __mmask8 _mm512_cmpord_pd_mask(__m512d, __m512d);
extern __mmask8 _mm512_cmpunord_pd_mask(__m512d, __m512d);

extern __mmask8 _mm512_mask_cmpeq_pd_mask(__mmask8, __m512d, __m512d);
extern __mmask8 _mm512_mask_cmple_pd_mask(__mmask8, __m512d, __m512d);
extern __mmask8 _mm512_mask_cmplt_pd_mask(__mmask8, __m512d, __m512d);
extern __mmask8 _mm512_mask_cmpneq_pd_mask(__mmask8, __m512d, __m512d);
extern __mmask8 _mm512_mask_cmpnle_pd_mask(__mmask8, __m512d, __m512d);
extern __mmask8 _mm512_mask_cmpnlt_pd_mask(__mmask8, __m512d, __m512d);
extern __mmask8 _mm512_mask_cmpord_pd_mask(__mmask8, __m512d, __m512d);
extern __mmask8 _mm512_mask_cmpunord_pd_mask(__mmask8, __m512d, __m512d);


#define _mm512_setzero() _mm512_setzero_ps()
#define _mm512_undefined() _mm512_setzero()
#define _mm512_undefined_pd() _mm512_setzero_pd()
#define _mm512_undefined_ps() _mm512_undefined()

#define _mm512_set4_ps(a,b,c,d) \
    _mm512_set_ps((a), (b), (c), (d), (a), (b), (c), (d), \
    (a), (b), (c), (d), (a), (b), (c), (d))

#define _mm512_set4_pd(a,b,c,d) \
    _mm512_set_pd((a), (b), (c), (d), (a), (b), (c), (d))

#define _mm512_set_16to16_ps(e0,e1,e2,e3,e4,e5,e6,e7,e8, \
    e9, e10, e11, e12, e13, e14, e15) \
    _mm512_set_ps((e0), (e1), (e2), (e3), (e4), (e5), (e6), (e7), \
    (e8), (e9), (e10), (e11), (e12), (e13), (e14), (e15))

#define _mm512_set_8to8_pd(e0,e1,e2,e3,e4,e5,e6,e7) \
    _mm512_set_pd((e0), (e1), (e2), (e3), (e4), (e5), (e6), (e7))

#define _mm512_setr4_ps(a,b,c,d) \
    _mm512_set4_ps((d), (c), (b), (a))

#define _mm512_set_4to16_ps(a,b,c,d) \
    _mm512_setr4_ps((a), (b), (c), (d))

#define _mm512_setr4_pd(a,b,c,d) \
    _mm512_set4_pd((d), (c), (b), (a))

#define _mm512_set_4to8_pd(a,b,c,d) \
    _mm512_setr4_pd((a), (b), (c), (d))

#define _mm512_set_1to16_ps(x) _mm512_set1_ps((x))
#define _mm512_set_1to8_pd(x) _mm512_set1_pd((x))

extern __m512i __cdecl _mm512_setzero_si512(void);

extern __m512i __cdecl _mm512_set_epi8(char /* e63 */, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char,
                                       char, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char,
                                       char, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char,
                                       char, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char /* e0 */);
extern __m512i __cdecl _mm512_set_epi16(short /* e31 */, short, short, short, short, short, short, short, short, short, short, short, short, short, short, short,
                                        short, short, short, short, short, short, short, short, short, short, short, short, short, short, short, short /* e0 */);
extern __m512i __cdecl _mm512_set_epi32(int /* e15 */, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int /* e0 */);
extern __m512i __cdecl _mm512_set_epi64(__int64 /* e7 */, __int64, __int64, __int64, __int64, __int64, __int64, __int64 /* e0 */);

extern __m512i __cdecl _mm512_setr_epi8(char /* e0 */, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char,
                                        char, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char,
                                        char, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char,
                                        char, char, char, char, char, char, char, char, char, char, char, char, char, char, char, char /* e63 */);
extern __m512i __cdecl _mm512_setr_epi16(short /* e0 */, short, short, short, short, short, short, short, short, short, short, short, short, short, short, short,
                                         short, short, short, short, short, short, short, short, short, short, short, short, short, short, short, short /* e31 */);
extern __m512i __cdecl _mm512_setr_epi32(int /* e0 */, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int /* e15 */);
extern __m512i __cdecl _mm512_setr_epi64(__int64 /* e0 */, __int64, __int64, __int64, __int64, __int64, __int64, __int64 /* e7 */);

extern __m512i __cdecl _mm512_set1_epi8(char);
extern __m512i __cdecl _mm512_mask_set1_epi8(__m512i, __mmask64, char);
extern __m512i __cdecl _mm512_maskz_set1_epi8(__mmask64, char);
extern __m512i __cdecl _mm512_set1_epi16(short);
extern __m512i __cdecl _mm512_mask_set1_epi16(__m512i, __mmask32, short);
extern __m512i __cdecl _mm512_maskz_set1_epi16(__mmask32, short);
extern __m512i __cdecl _mm512_set1_epi32(int);
extern __m512i __cdecl _mm512_mask_set1_epi32(__m512i, __mmask16, int);
extern __m512i __cdecl _mm512_maskz_set1_epi32(__mmask16, int);
extern __m512i __cdecl _mm512_set1_epi64(__int64);
extern __m512i __cdecl _mm512_mask_set1_epi64(__m512i, __mmask8, __int64);
extern __m512i __cdecl _mm512_maskz_set1_epi64(__mmask8, __int64);

extern __m512i __cdecl _mm512_add_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_add_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_add_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_add_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_add_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_add_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_add_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_add_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_add_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_add_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_add_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_add_epi64(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_adds_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_adds_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_adds_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_adds_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_adds_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_adds_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_adds_epu8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_adds_epu8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_adds_epu8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_adds_epu16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_adds_epu16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_adds_epu16(__mmask32, __m512i, __m512i);

extern __m512i __cdecl _mm512_abs_epi8(__m512i);
extern __m512i __cdecl _mm512_mask_abs_epi8(__m512i, __mmask64, __m512i);
extern __m512i __cdecl _mm512_maskz_abs_epi8(__mmask64, __m512i);
extern __m512i __cdecl _mm512_abs_epi16(__m512i);
extern __m512i __cdecl _mm512_mask_abs_epi16(__m512i, __mmask32, __m512i);
extern __m512i __cdecl _mm512_maskz_abs_epi16(__mmask32, __m512i);
extern __m512i __cdecl _mm512_abs_epi32(__m512i);
extern __m512i __cdecl _mm512_mask_abs_epi32(__m512i, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_abs_epi32(__mmask16, __m512i);
extern __m512i __cdecl _mm512_abs_epi64(__m512i);
extern __m512i __cdecl _mm512_mask_abs_epi64(__m512i, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_abs_epi64(__mmask8, __m512i);

extern __m512i  __cdecl _mm512_broadcast_i32x2(__m128i);
extern __m512i  __cdecl _mm512_mask_broadcast_i32x2(__m512i, __mmask16, __m128i);
extern __m512i  __cdecl _mm512_maskz_broadcast_i32x2(__mmask16, __m128i);
extern __m512i  __cdecl _mm512_broadcast_i32x4(__m128i);
extern __m512i  __cdecl _mm512_mask_broadcast_i32x4(__m512i, __mmask16, __m128i);
extern __m512i  __cdecl _mm512_maskz_broadcast_i32x4(__mmask16, __m128i);
extern __m512i  __cdecl _mm512_broadcast_i32x8(__m256i);
extern __m512i  __cdecl _mm512_mask_broadcast_i32x8(__m512i, __mmask16, __m256i);
extern __m512i  __cdecl _mm512_maskz_broadcast_i32x8(__mmask16, __m256i);
extern __m512i  __cdecl _mm512_broadcast_i64x2(__m128i);
extern __m512i  __cdecl _mm512_mask_broadcast_i64x2(__m512i, __mmask8, __m128i);
extern __m512i  __cdecl _mm512_maskz_broadcast_i64x2(__mmask8, __m128i);
extern __m512i  __cdecl _mm512_broadcast_i64x4(__m256i);
extern __m512i  __cdecl _mm512_mask_broadcast_i64x4(__m512i, __mmask8, __m256i);
extern __m512i  __cdecl _mm512_maskz_broadcast_i64x4(__mmask8, __m256i);
extern __m512i __cdecl _mm512_broadcastb_epi8(__m128i);
extern __m512i __cdecl _mm512_mask_broadcastb_epi8(__m512i, __mmask64, __m128i);
extern __m512i __cdecl _mm512_maskz_broadcastb_epi8(__mmask64, __m128i);
extern __m512i __cdecl _mm512_broadcastw_epi16(__m128i);
extern __m512i __cdecl _mm512_mask_broadcastw_epi16(__m512i, __mmask32, __m128i);
extern __m512i __cdecl _mm512_maskz_broadcastw_epi16(__mmask32, __m128i);
extern __m512i __cdecl _mm512_broadcastd_epi32(__m128i);
extern __m512i __cdecl _mm512_mask_broadcastd_epi32(__m512i, __mmask16, __m128i);
extern __m512i __cdecl _mm512_maskz_broadcastd_epi32(__mmask16, __m128i);
extern __m512i __cdecl _mm512_broadcastq_epi64(__m128i);
extern __m512i __cdecl _mm512_mask_broadcastq_epi64(__m512i, __mmask8, __m128i);
extern __m512i __cdecl _mm512_maskz_broadcastq_epi64(__mmask8, __m128i);
extern __m512i __cdecl _mm512_broadcastmw_epi32(__mmask16);
extern __m512i __cdecl _mm512_broadcastmb_epi64(__mmask8);

extern __m512i __cdecl _mm512_sub_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_sub_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_sub_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_sub_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_sub_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_sub_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_sub_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_sub_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_sub_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_sub_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_sub_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_sub_epi64(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_subs_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_subs_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_subs_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_subs_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_subs_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_subs_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_subs_epu8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_subs_epu8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_subs_epu8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_subs_epu16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_subs_epu16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_subs_epu16(__mmask32, __m512i, __m512i);

extern __m512i __cdecl _mm512_max_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_max_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_max_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_max_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epi64(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_max_epu8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epu8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epu8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_max_epu16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epu16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epu16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_max_epu32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epu32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epu32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_max_epu64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_max_epu64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_max_epu64(__mmask8, __m512i, __m512i);

extern __m512i __cdecl _mm512_min_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_min_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_min_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_min_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epi64(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_min_epu8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epu8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epu8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_min_epu16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epu16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epu16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_min_epu32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epu32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epu32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_min_epu64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_min_epu64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_min_epu64(__mmask8, __m512i, __m512i);

extern __m512i __cdecl _mm512_mul_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mul_epi32(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mul_epi32(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_mul_epu32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mul_epu32(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mul_epu32(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_mulhi_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mulhi_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mulhi_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_mulhi_epu16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mulhi_epu16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mulhi_epu16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_mullo_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mullo_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mullo_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_mullo_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mullo_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mullo_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_mullo_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mullo_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mullo_epi64(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_mullox_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mullox_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_mulhrs_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_mulhrs_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_mulhrs_epi16(__mmask32, __m512i, __m512i);

extern __m512i __cdecl _mm512_load_epi32(void const*);
extern __m512i __cdecl _mm512_mask_load_epi32(__m512i, __mmask16, void const*);
extern __m512i __cdecl _mm512_maskz_load_epi32(__mmask16, void const*);
extern __m512i __cdecl _mm512_load_epi64(void const*);
extern __m512i __cdecl _mm512_mask_load_epi64(__m512i, __mmask8, void const*);
extern __m512i __cdecl _mm512_maskz_load_epi64(__mmask8, void const*);
extern __m512i __cdecl _mm512_loadu_epi8(void const*);
extern __m512i __cdecl _mm512_mask_loadu_epi8(__m512i, __mmask64, void const*);
extern __m512i __cdecl _mm512_maskz_loadu_epi8(__mmask64, void const*);
extern __m512i __cdecl _mm512_loadu_epi16(void const*);
extern __m512i __cdecl _mm512_mask_loadu_epi16(__m512i, __mmask32, void const*);
extern __m512i __cdecl _mm512_maskz_loadu_epi16(__mmask32, void const*);
extern __m512i __cdecl _mm512_loadu_epi32(void const*);
extern __m512i __cdecl _mm512_mask_loadu_epi32(__m512i, __mmask16, void const*);
extern __m512i __cdecl _mm512_maskz_loadu_epi32(__mmask16, void const*);
extern __m512i __cdecl _mm512_loadu_epi64(void const*);
extern __m512i __cdecl _mm512_mask_loadu_epi64(__m512i, __mmask8, void const*);
extern __m512i __cdecl _mm512_maskz_loadu_epi64(__mmask8, void const*);

extern void    __cdecl _mm512_store_epi32(void*, __m512i);
extern void    __cdecl _mm512_mask_store_epi32(void*, __mmask16, __m512i);
extern void    __cdecl _mm512_store_epi64(void*, __m512i);
extern void    __cdecl _mm512_mask_store_epi64(void*, __mmask8, __m512i);
extern void    __cdecl _mm512_storeu_epi8(void*, __m512i);
extern void    __cdecl _mm512_mask_storeu_epi8(void*, __mmask64, __m512i);
extern void    __cdecl _mm512_storeu_epi16(void*, __m512i);
extern void    __cdecl _mm512_mask_storeu_epi16(void*, __mmask32, __m512i);
extern void    __cdecl _mm512_storeu_epi32(void*, __m512i);
extern void    __cdecl _mm512_mask_storeu_epi32(void*, __mmask16, __m512i);
extern void    __cdecl _mm512_storeu_epi64(void*, __m512i);
extern void    __cdecl _mm512_mask_storeu_epi64(void*, __mmask8, __m512i);

extern __m128i __cdecl _mm512_extracti32x4_epi32(__m512i, int);
extern __m128i __cdecl _mm512_mask_extracti32x4_epi32(__m128i, __mmask8, __m512i, int);
extern __m128i __cdecl _mm512_maskz_extracti32x4_epi32(__mmask8, __m512i, int);
extern __m256i __cdecl _mm512_extracti32x8_epi32(__m512i, int);
extern __m256i __cdecl _mm512_mask_extracti32x8_epi32(__m256i, __mmask8, __m512i, int);
extern __m256i __cdecl _mm512_maskz_extracti32x8_epi32(__mmask8, __m512i, int);
extern __m128i __cdecl _mm512_extracti64x2_epi64(__m512i, int);
extern __m128i __cdecl _mm512_mask_extracti64x2_epi64(__m128i, __mmask8, __m512i, int);
extern __m128i __cdecl _mm512_maskz_extracti64x2_epi64(__mmask8, __m512i, int);
extern __m256i __cdecl _mm512_extracti64x4_epi64(__m512i, int);
extern __m256i __cdecl _mm512_mask_extracti64x4_epi64(__m256i, __mmask8, __m512i, int);
extern __m256i __cdecl _mm512_maskz_extracti64x4_epi64(__mmask8, __m512i, int);

extern __m512i __cdecl _mm512_inserti32x4(__m512i, __m128i, int);
extern __m512i __cdecl _mm512_mask_inserti32x4(__m512i, __mmask16, __m512i, __m128i, int);
extern __m512i __cdecl _mm512_maskz_inserti32x4(__mmask16, __m512i, __m128i, int);
extern __m512i __cdecl _mm512_inserti32x8(__m512i, __m256i, int);
extern __m512i __cdecl _mm512_mask_inserti32x8(__m512i, __mmask16, __m512i, __m256i, int);
extern __m512i __cdecl _mm512_maskz_inserti32x8(__mmask16, __m512i, __m256i, int);
extern __m512i __cdecl _mm512_inserti64x2(__m512i, __m128i, int);
extern __m512i __cdecl _mm512_mask_inserti64x2(__m512i, __mmask8, __m512i, __m128i, int);
extern __m512i __cdecl _mm512_maskz_inserti64x2(__mmask8, __m512i, __m128i, int);
extern __m512i __cdecl _mm512_inserti64x4(__m512i, __m256i, int);
extern __m512i __cdecl _mm512_mask_inserti64x4(__m512i, __mmask8, __m512i, __m256i, int);
extern __m512i __cdecl _mm512_maskz_inserti64x4(__mmask8, __m512i, __m256i, int);

extern __m512i __cdecl _mm512_shuffle_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_shuffle_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_shuffle_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_shuffle_epi32(__m512i, int);
extern __m512i __cdecl _mm512_mask_shuffle_epi32(__m512i, __mmask16, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shuffle_epi32(__mmask16, __m512i, int);
extern __m512i __cdecl _mm512_shuffle_i32x4(__m512i, __m512i, const int);
extern __m512i __cdecl _mm512_mask_shuffle_i32x4(__m512i, __mmask16, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_maskz_shuffle_i32x4(__mmask16, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_shuffle_i64x2(__m512i, __m512i, const int);
extern __m512i __cdecl _mm512_mask_shuffle_i64x2(__m512i, __mmask8, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_maskz_shuffle_i64x2(__mmask8, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_shufflehi_epi16(__m512i, int);
extern __m512i __cdecl _mm512_mask_shufflehi_epi16(__m512i, __mmask32, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shufflehi_epi16(__mmask32, __m512i, int);
extern __m512i __cdecl _mm512_shufflelo_epi16(__m512i, int);
extern __m512i __cdecl _mm512_mask_shufflelo_epi16(__m512i, __mmask32, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shufflelo_epi16(__mmask32, __m512i, int);

extern __m512  __cdecl _mm512_mask_mov_ps(__m512, __mmask16, __m512);
extern __m512  __cdecl _mm512_maskz_mov_ps(__mmask16, __m512);
extern __m512d __cdecl _mm512_mask_mov_pd(__m512d, __mmask8, __m512d);
extern __m512d __cdecl _mm512_maskz_mov_pd(__mmask8, __m512d);
extern __m512i __cdecl _mm512_mask_mov_epi8(__m512i, __mmask64, __m512i);
extern __m512i __cdecl _mm512_maskz_mov_epi8(__mmask64, __m512i);
extern __m512i __cdecl _mm512_mask_mov_epi16(__m512i, __mmask32, __m512i);
extern __m512i __cdecl _mm512_maskz_mov_epi16(__mmask32, __m512i);
extern __m512i __cdecl _mm512_mask_mov_epi32(__m512i, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_mov_epi32(__mmask16, __m512i);
extern __m512i __cdecl _mm512_mask_mov_epi64(__m512i, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_mov_epi64(__mmask8, __m512i);
extern __m512d __cdecl _mm512_movedup_pd(__m512d);
extern __m512d __cdecl _mm512_mask_movedup_pd(__m512d, __mmask8, __m512d);
extern __m512d __cdecl _mm512_maskz_movedup_pd(__mmask8, __m512d);
extern __m512  __cdecl _mm512_movehdup_ps(__m512);
extern __m512  __cdecl _mm512_mask_movehdup_ps(__m512, __mmask16, __m512);
extern __m512  __cdecl _mm512_maskz_movehdup_ps(__mmask16, __m512);
extern __m512  __cdecl _mm512_moveldup_ps(__m512);
extern __m512  __cdecl _mm512_mask_moveldup_ps(__m512, __mmask16, __m512);
extern __m512  __cdecl _mm512_maskz_moveldup_ps(__mmask16, __m512);

extern __m512i __cdecl _mm512_movm_epi8(__mmask64);
extern __m512i __cdecl _mm512_movm_epi16(__mmask32);
extern __m512i __cdecl _mm512_movm_epi32(__mmask16);
extern __m512i __cdecl _mm512_movm_epi64(__mmask8);
extern __mmask64 __cdecl _mm512_movepi8_mask(__m512i);
extern __mmask32 __cdecl _mm512_movepi16_mask(__m512i);
extern __mmask16 __cdecl _mm512_movepi32_mask(__m512i);
extern __mmask8  __cdecl _mm512_movepi64_mask(__m512i);

extern __m512i __cdecl _mm512_alignr_epi8(__m512i, __m512i, const int);
extern __m512i __cdecl _mm512_mask_alignr_epi8(__m512i, __mmask64, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_maskz_alignr_epi8(__mmask64, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_alignr_epi32(__m512i, __m512i, const int);
extern __m512i __cdecl _mm512_mask_alignr_epi32(__m512i, __mmask16, __m512i, __m512i, const int /* count */);
extern __m512i __cdecl _mm512_maskz_alignr_epi32(__mmask16, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_alignr_epi64(__m512i, __m512i, const int);
extern __m512i __cdecl _mm512_mask_alignr_epi64(__m512i, __mmask8, __m512i, __m512i, const int);
extern __m512i __cdecl _mm512_maskz_alignr_epi64(__mmask8, __m512i, __m512i, const int);

extern __m512d __cdecl _mm512_and_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_mask_and_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_and_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_and_ps(__m512, __m512);
extern __m512  __cdecl _mm512_mask_and_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_maskz_and_ps(__mmask16, __m512, __m512);
extern __m512i __cdecl _mm512_and_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_and_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_and_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_and_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_and_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_and_epi64(__mmask8, __m512i, __m512i);

extern __m512d __cdecl _mm512_andnot_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_mask_andnot_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_andnot_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_andnot_ps(__m512, __m512);
extern __m512  __cdecl _mm512_mask_andnot_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_maskz_andnot_ps(__mmask16, __m512, __m512);
extern __m512i __cdecl _mm512_andnot_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_andnot_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_andnot_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_andnot_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_andnot_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_andnot_epi64(__mmask8, __m512i, __m512i);

extern __m512d __cdecl _mm512_or_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_mask_or_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_or_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_or_ps(__m512, __m512);
extern __m512  __cdecl _mm512_mask_or_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_maskz_or_ps(__mmask16, __m512, __m512);
extern __m512i __cdecl _mm512_or_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_or_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_or_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_or_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_or_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_or_epi64(__mmask8, __m512i, __m512i);

extern __m512d __cdecl _mm512_xor_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_mask_xor_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_xor_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_xor_ps(__m512, __m512);
extern __m512  __cdecl _mm512_mask_xor_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_maskz_xor_ps(__mmask16, __m512, __m512);
extern __m512i __cdecl _mm512_xor_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_xor_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_xor_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_xor_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_xor_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_xor_epi64(__mmask8, __m512i, __m512i);

extern __m512  __cdecl _mm512_mask_blend_ps(__mmask16, __m512, __m512);
extern __m512d __cdecl _mm512_mask_blend_pd(__mmask8, __m512d, __m512d);
extern __m512i __cdecl _mm512_mask_blend_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_blend_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_blend_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_blend_epi64(__mmask8, __m512i, __m512i);

extern __m512i __cdecl _mm512_sll_epi16(__m512i, __m128i);
extern __m512i __cdecl _mm512_sll_epi32(__m512i, __m128i);
extern __m512i __cdecl _mm512_sll_epi64(__m512i, __m128i);
extern __m512i __cdecl _mm512_slli_epi16(__m512i, unsigned int);
extern __m512i __cdecl _mm512_slli_epi32(__m512i, unsigned int);
extern __m512i __cdecl _mm512_slli_epi64(__m512i, unsigned int);
extern __m512i __cdecl _mm512_sllv_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_sllv_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_sllv_epi64(__m512i, __m512i);

extern __m512i __cdecl _mm512_mask_sll_epi16(__m512i, __mmask32, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_sll_epi16(__mmask32, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_sll_epi32(__m512i, __mmask16, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_sll_epi32(__mmask16, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_sll_epi64(__m512i, __mmask8, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_sll_epi64(__mmask8, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_slli_epi16(__m512i, __mmask32, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_slli_epi16(__mmask32, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_slli_epi32(__m512i, __mmask16, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_slli_epi32(__mmask16, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_slli_epi64(__m512i, __mmask8, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_slli_epi64(__mmask8, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_sllv_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_sllv_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_sllv_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_sllv_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_sllv_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_sllv_epi64(__mmask8, __m512i, __m512i);

extern __m512i __cdecl _mm512_srl_epi16(__m512i, __m128i);
extern __m512i __cdecl _mm512_srl_epi32(__m512i, __m128i);
extern __m512i __cdecl _mm512_srl_epi64(__m512i, __m128i);
extern __m512i __cdecl _mm512_srli_epi16(__m512i, int);
extern __m512i __cdecl _mm512_srli_epi32(__m512i, unsigned int);
extern __m512i __cdecl _mm512_srli_epi64(__m512i, unsigned int);
extern __m512i __cdecl _mm512_srlv_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_srlv_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_srlv_epi64(__m512i, __m512i);

extern __m512i __cdecl _mm512_mask_srl_epi16(__m512i, __mmask32, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_srl_epi16(__mmask32, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_srl_epi32(__m512i, __mmask16, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_srl_epi32(__mmask16, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_srl_epi64(__m512i, __mmask8, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_srl_epi64(__mmask8, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_srli_epi16(__m512i, __mmask32, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_srli_epi16(__mmask32, __m512i, int);
extern __m512i __cdecl _mm512_mask_srli_epi32(__m512i, __mmask16, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_srli_epi32(__mmask16, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_srli_epi64(__m512i, __mmask8, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_srli_epi64(__mmask8, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_srlv_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_srlv_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_srlv_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_srlv_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_srlv_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_srlv_epi64(__mmask8, __m512i, __m512i);

extern __m512i __cdecl _mm512_sra_epi16(__m512i, __m128i);
extern __m512i __cdecl _mm512_sra_epi32(__m512i, __m128i);
extern __m512i __cdecl _mm512_sra_epi64(__m512i, __m128i);
extern __m512i __cdecl _mm512_srai_epi16(__m512i, unsigned int);
extern __m512i __cdecl _mm512_srai_epi32(__m512i, unsigned int);
extern __m512i __cdecl _mm512_srai_epi64(__m512i, unsigned int);
extern __m512i __cdecl _mm512_srav_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_srav_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_srav_epi64(__m512i, __m512i);

extern __m512i __cdecl _mm512_mask_sra_epi16(__m512i, __mmask32, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_sra_epi16(__mmask32, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_sra_epi32(__m512i, __mmask16, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_sra_epi32(__mmask16, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_sra_epi64(__m512i, __mmask8, __m512i, __m128i);
extern __m512i __cdecl _mm512_maskz_sra_epi64(__mmask8, __m512i, __m128i);
extern __m512i __cdecl _mm512_mask_srai_epi16(__m512i, __mmask32, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_srai_epi16(__mmask32, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_srai_epi32(__m512i, __mmask16, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_srai_epi32(__mmask16, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_srai_epi64(__m512i, __mmask8, __m512i, unsigned int);
extern __m512i __cdecl _mm512_maskz_srai_epi64(__mmask8, __m512i, unsigned int);
extern __m512i __cdecl _mm512_mask_srav_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_srav_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_srav_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_srav_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_srav_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_srav_epi64(__mmask8, __m512i, __m512i);

extern __m512i __cdecl _mm512_bslli_epi128(__m512i, int);
extern __m512i __cdecl _mm512_bsrli_epi128(__m512i, int);

extern __m512i __cdecl _mm512_rol_epi32(__m512i, const int);
extern __m512i __cdecl _mm512_mask_rol_epi32(__m512i, __mmask16, __m512i, const int);
extern __m512i __cdecl _mm512_maskz_rol_epi32(__mmask16, __m512i, const int);
extern __m512i __cdecl _mm512_rol_epi64(__m512i, const int);
extern __m512i __cdecl _mm512_mask_rol_epi64(__m512i, __mmask8, __m512i, const int);
extern __m512i __cdecl _mm512_maskz_rol_epi64(__mmask8, __m512i, const int);
extern __m512i __cdecl _mm512_rolv_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_rolv_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_rolv_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_rolv_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_rolv_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_rolv_epi64(__mmask8, __m512i, __m512i);

extern __m512i __cdecl _mm512_ror_epi32(__m512i, int);
extern __m512i __cdecl _mm512_mask_ror_epi32(__m512i, __mmask16, __m512i, int);
extern __m512i __cdecl _mm512_maskz_ror_epi32(__mmask16, __m512i, int);
extern __m512i __cdecl _mm512_ror_epi64(__m512i, int);
extern __m512i __cdecl _mm512_mask_ror_epi64(__m512i, __mmask8, __m512i, int);
extern __m512i __cdecl _mm512_maskz_ror_epi64(__mmask8, __m512i, int);
extern __m512i __cdecl _mm512_rorv_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_rorv_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_rorv_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_rorv_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_rorv_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_rorv_epi64(__mmask8, __m512i, __m512i);

extern __m512d __cdecl _mm512_unpackhi_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_mask_unpackhi_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_unpackhi_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_unpackhi_ps(__m512, __m512);
extern __m512  __cdecl _mm512_mask_unpackhi_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_maskz_unpackhi_ps(__mmask16, __m512, __m512);
extern __m512d __cdecl _mm512_unpacklo_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_mask_unpacklo_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_unpacklo_pd(__mmask8, __m512d, __m512d);
extern __m512  __cdecl _mm512_unpacklo_ps(__m512, __m512);
extern __m512  __cdecl _mm512_mask_unpacklo_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_maskz_unpacklo_ps(__mmask16, __m512, __m512);
extern __m512i __cdecl _mm512_unpackhi_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpackhi_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpackhi_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_unpackhi_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpackhi_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpackhi_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_unpackhi_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpackhi_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpackhi_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_unpackhi_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpackhi_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpackhi_epi64(__mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_unpacklo_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpacklo_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpacklo_epi8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_unpacklo_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpacklo_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpacklo_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_unpacklo_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpacklo_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpacklo_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_unpacklo_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_unpacklo_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_unpacklo_epi64(__mmask8, __m512i, __m512i);

extern __m512  __cdecl _mm512_getexp_ps(__m512);
extern __m512  __cdecl _mm512_mask_getexp_ps(__m512, __mmask16, __m512);
extern __m512  __cdecl _mm512_maskz_getexp_ps(__mmask16, __m512);
extern __m512  __cdecl _mm512_getexp_round_ps(__m512, int);
extern __m512  __cdecl _mm512_mask_getexp_round_ps(__m512, __mmask16, __m512, int);
extern __m512  __cdecl _mm512_maskz_getexp_round_ps(__mmask16, __m512, int);
extern __m512d __cdecl _mm512_getexp_pd(__m512d);
extern __m512d __cdecl _mm512_mask_getexp_pd(__m512d, __mmask8, __m512d);
extern __m512d __cdecl _mm512_maskz_getexp_pd(__mmask8, __m512d);
extern __m512d __cdecl _mm512_getexp_round_pd(__m512d, int);
extern __m512d __cdecl _mm512_mask_getexp_round_pd(__m512d, __mmask8, __m512d, int);
extern __m512d __cdecl _mm512_maskz_getexp_round_pd(__mmask8, __m512d, int);

extern __m512  __cdecl _mm512_getmant_ps(__m512, int, int);
extern __m512  __cdecl _mm512_mask_getmant_ps(__m512, __mmask16, __m512, int, int);
extern __m512  __cdecl _mm512_maskz_getmant_ps(__mmask16, __m512, int, int);
extern __m512  __cdecl _mm512_getmant_round_ps(__m512, int, int, int);
extern __m512  __cdecl _mm512_mask_getmant_round_ps(__m512, __mmask16, __m512, int, int, int);
extern __m512  __cdecl _mm512_maskz_getmant_round_ps(__mmask16, __m512, int, int, int);
extern __m512d __cdecl _mm512_getmant_pd(__m512d, int, int);
extern __m512d __cdecl _mm512_mask_getmant_pd(__m512d, __mmask8, __m512d, int, int);
extern __m512d __cdecl _mm512_maskz_getmant_pd(__mmask8, __m512d, int, int);
extern __m512d __cdecl _mm512_getmant_round_pd(__m512d, int, int, int);
extern __m512d __cdecl _mm512_mask_getmant_round_pd(__m512d, __mmask8, __m512d, int, int, int);
extern __m512d __cdecl _mm512_maskz_getmant_round_pd(__mmask8, __m512d, int, int, int);

extern __m512d __cdecl _mm512_permute_pd(__m512d, const int);
extern __m512d __cdecl _mm512_mask_permute_pd(__m512d, __mmask8, __m512d, const int);
extern __m512d __cdecl _mm512_maskz_permute_pd(__mmask8, __m512d, const int);
extern __m512  __cdecl _mm512_permute_ps(__m512, const int);
extern __m512  __cdecl _mm512_mask_permute_ps(__m512, __mmask16, __m512, const int);
extern __m512  __cdecl _mm512_maskz_permute_ps(__mmask16, __m512, const int);
extern __m512d __cdecl _mm512_permutevar_pd(__m512d, __m512i);
extern __m512d __cdecl _mm512_mask_permutevar_pd(__m512d, __mmask8, __m512d, __m512i);
extern __m512d __cdecl _mm512_maskz_permutevar_pd(__mmask8, __m512d, __m512i);
extern __m512  __cdecl _mm512_permutevar_ps(__m512, __m512i);
extern __m512  __cdecl _mm512_mask_permutevar_ps(__m512, __mmask16, __m512, __m512i);
extern __m512  __cdecl _mm512_maskz_permutevar_ps(__mmask16, __m512, __m512i);
extern __m512i __cdecl _mm512_permutevar_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_permutevar_epi32(__m512i, __mmask16, __m512i, __m512i);

extern __m512d __cdecl _mm512_permutex_pd(__m512d, const int);
extern __m512d __cdecl _mm512_mask_permutex_pd(__m512d, __mmask8, __m512d, const int);
extern __m512d __cdecl _mm512_maskz_permutex_pd(__mmask8, __m512d, const int);
extern __m512i __cdecl _mm512_permutex_epi64(__m512i, const int);
extern __m512i __cdecl _mm512_mask_permutex_epi64(__m512i, __mmask8, __m512i, const int);
extern __m512i __cdecl _mm512_maskz_permutex_epi64(__mmask8, __m512i, const int);
extern __m512d __cdecl _mm512_permutexvar_pd(__m512i, __m512d);
extern __m512d __cdecl _mm512_mask_permutexvar_pd(__m512d, __mmask8, __m512i, __m512d);
extern __m512d __cdecl _mm512_maskz_permutexvar_pd(__mmask8, __m512i, __m512d);
extern __m512  __cdecl _mm512_permutexvar_ps(__m512i, __m512);
extern __m512  __cdecl _mm512_mask_permutexvar_ps(__m512, __mmask16, __m512i, __m512);
extern __m512  __cdecl _mm512_maskz_permutexvar_ps(__mmask16, __m512i, __m512);
extern __m512i __cdecl _mm512_permutexvar_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_permutexvar_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_permutexvar_epi16(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_permutexvar_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_permutexvar_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_permutexvar_epi32(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_permutexvar_epi64(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_permutexvar_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_permutexvar_epi64(__mmask8, __m512i, __m512i);

extern __m512d __cdecl _mm512_permutex2var_pd(__m512d, __m512i /* index */, __m512d);
extern __m512d __cdecl _mm512_mask_permutex2var_pd(__m512d, __mmask8, __m512i /* index */, __m512d);
extern __m512d __cdecl _mm512_mask2_permutex2var_pd(__m512d, __m512i /* index */, __mmask8, __m512d);
extern __m512d __cdecl _mm512_maskz_permutex2var_pd(__mmask8, __m512d, __m512i /* index */, __m512d);
extern __m512  __cdecl _mm512_permutex2var_ps(__m512, __m512i /* index */, __m512);
extern __m512  __cdecl _mm512_mask_permutex2var_ps(__m512, __mmask16, __m512i /* index */, __m512);
extern __m512  __cdecl _mm512_mask2_permutex2var_ps(__m512, __m512i /* index */, __mmask16, __m512);
extern __m512  __cdecl _mm512_maskz_permutex2var_ps(__mmask16, __m512, __m512i /* index */, __m512);
extern __m512i __cdecl _mm512_permutex2var_epi16(__m512i, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_mask_permutex2var_epi16(__m512i, __mmask32, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_mask2_permutex2var_epi16(__m512i, __m512i /* idx */, __mmask32, __m512i);
extern __m512i __cdecl _mm512_maskz_permutex2var_epi16(__mmask32, __m512i, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_permutex2var_epi32(__m512i, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_mask_permutex2var_epi32(__m512i, __mmask16, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_mask2_permutex2var_epi32(__m512i, __m512i /* idx */, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_permutex2var_epi32(__mmask16, __m512i, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_permutex2var_epi64(__m512i, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_mask_permutex2var_epi64(__m512i, __mmask8, __m512i /* idx */, __m512i);
extern __m512i __cdecl _mm512_mask2_permutex2var_epi64(__m512i, __m512i /* idx */, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_permutex2var_epi64(__mmask8, __m512i, __m512i /* idx */, __m512i);

extern __m512d __cdecl _mm512_mask_compress_pd(__m512d, __mmask8, __m512d);
extern __m512d __cdecl _mm512_maskz_compress_pd(__mmask8, __m512d);
extern __m512  __cdecl _mm512_mask_compress_ps(__m512, __mmask16, __m512);
extern __m512  __cdecl _mm512_maskz_compress_ps(__mmask16, __m512);
extern __m512i __cdecl _mm512_mask_compress_epi8(__m512i, __mmask64, __m512i);
extern __m512i __cdecl _mm512_maskz_compress_epi8(__mmask64, __m512i);
extern __m512i __cdecl _mm512_mask_compress_epi16(__m512i, __mmask32, __m512i);
extern __m512i __cdecl _mm512_maskz_compress_epi16(__mmask32, __m512i);
extern __m512i __cdecl _mm512_mask_compress_epi32(__m512i, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_compress_epi32(__mmask16, __m512i);
extern __m512i __cdecl _mm512_mask_compress_epi64(__m512i, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_compress_epi64(__mmask8, __m512i);

extern void    __cdecl _mm512_mask_compressstoreu_pd(void*, __mmask8, __m512d);
extern void    __cdecl _mm512_mask_compressstoreu_ps(void*, __mmask16, __m512);
extern void    __cdecl _mm512_mask_compressstoreu_epi8(void*, __mmask64, __m512i);
extern void    __cdecl _mm512_mask_compressstoreu_epi16(void*, __mmask32, __m512i);
extern void    __cdecl _mm512_mask_compressstoreu_epi32(void*, __mmask16, __m512i);
extern void    __cdecl _mm512_mask_compressstoreu_epi64(void*, __mmask8, __m512i);

extern __m512d __cdecl _mm512_mask_expand_pd(__m512d, __mmask8, __m512d);
extern __m512d __cdecl _mm512_maskz_expand_pd(__mmask8, __m512d);
extern __m512  __cdecl _mm512_mask_expand_ps(__m512, __mmask16, __m512);
extern __m512  __cdecl _mm512_maskz_expand_ps(__mmask16, __m512);
extern __m512i __cdecl _mm512_mask_expand_epi8(__m512i, __mmask64, __m512i);
extern __m512i __cdecl _mm512_maskz_expand_epi8(__mmask64, __m512i);
extern __m512i __cdecl _mm512_mask_expand_epi16(__m512i, __mmask32, __m512i);
extern __m512i __cdecl _mm512_maskz_expand_epi16(__mmask32, __m512i);
extern __m512i __cdecl _mm512_mask_expand_epi32(__m512i, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_expand_epi32(__mmask16, __m512i);
extern __m512i __cdecl _mm512_mask_expand_epi64(__m512i, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_expand_epi64(__mmask8, __m512i);
extern __m512d __cdecl _mm512_mask_expandloadu_pd(__m512d, __mmask8, void const*);
extern __m512d __cdecl _mm512_maskz_expandloadu_pd(__mmask8, void const*);
extern __m512  __cdecl _mm512_mask_expandloadu_ps(__m512, __mmask16, void const*);
extern __m512  __cdecl _mm512_maskz_expandloadu_ps(__mmask16, void const*);
extern __m512i __cdecl _mm512_mask_expandloadu_epi8(__m512i, __mmask64, const void*);
extern __m512i __cdecl _mm512_maskz_expandloadu_epi8(__mmask64, const void*);
extern __m512i __cdecl _mm512_mask_expandloadu_epi16(__m512i, __mmask32, const void*);
extern __m512i __cdecl _mm512_maskz_expandloadu_epi16(__mmask32, const void*);
extern __m512i __cdecl _mm512_mask_expandloadu_epi32(__m512i, __mmask16, void const*);
extern __m512i __cdecl _mm512_maskz_expandloadu_epi32(__mmask16, void const*);
extern __m512i __cdecl _mm512_mask_expandloadu_epi64(__m512i, __mmask8, void const*);
extern __m512i __cdecl _mm512_maskz_expandloadu_epi64(__mmask8, void const*);

extern __m512i __cdecl _mm512_ternarylogic_epi32(__m512i, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_ternarylogic_epi32(__m512i, __mmask16, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_ternarylogic_epi32(__mmask16, __m512i, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_ternarylogic_epi64(__m512i, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_ternarylogic_epi64(__m512i, __mmask8, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_ternarylogic_epi64(__mmask8, __m512i, __m512i, __m512i, int);

extern __m512i __cdecl _mm512_conflict_epi32(__m512i);
extern __m512i __cdecl _mm512_mask_conflict_epi32(__m512i, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_conflict_epi32(__mmask16, __m512i);
extern __m512i __cdecl _mm512_conflict_epi64(__m512i);
extern __m512i __cdecl _mm512_mask_conflict_epi64(__m512i, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_conflict_epi64(__mmask8, __m512i);

extern __m512i __cdecl _mm512_lzcnt_epi8(__m512i);
extern __m256i __cdecl _mm256_lzcnt_epi8(__m256i);
extern __m128i __cdecl _mm_lzcnt_epi8(__m128i);

extern __m512i __cdecl _mm512_lzcnt_epi16(__m512i);
extern __m256i __cdecl _mm256_lzcnt_epi16(__m256i);
extern __m128i __cdecl _mm_lzcnt_epi16(__m128i);

extern __m512i __cdecl _mm512_lzcnt_epi32(__m512i);
extern __m512i __cdecl _mm512_mask_lzcnt_epi32(__m512i, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_lzcnt_epi32(__mmask16, __m512i);
extern __m512i __cdecl _mm512_lzcnt_epi64(__m512i);
extern __m512i __cdecl _mm512_mask_lzcnt_epi64(__m512i, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_lzcnt_epi64(__mmask8, __m512i);

extern __m512i __cdecl _mm512_avg_epu8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_avg_epu8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_avg_epu8(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_avg_epu16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_avg_epu16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_avg_epu16(__mmask32, __m512i, __m512i);

extern __m512i __cdecl _mm512_sad_epu8(__m512i, __m512i);
extern __m512i __cdecl _mm512_dbsad_epu8(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_dbsad_epu8(__m512i, __mmask32, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_dbsad_epu8(__mmask32, __m512i, __m512i, int);

extern float   __cdecl _mm512_reduce_add_ps(__m512);
extern float   __cdecl _mm512_mask_reduce_add_ps(__mmask16, __m512);
extern double  __cdecl _mm512_reduce_add_pd(__m512d);
extern double  __cdecl _mm512_mask_reduce_add_pd(__mmask8, __m512d);
extern int     __cdecl _mm512_reduce_add_epi8(__m512i);
extern int     __cdecl _mm512_mask_reduce_add_epi8(__mmask64, __m512i);
extern int     __cdecl _mm512_reduce_add_epi16(__m512i);
extern int     __cdecl _mm512_mask_reduce_add_epi16(__mmask32, __m512i);
extern int     __cdecl _mm512_reduce_add_epi32(__m512i);
extern int     __cdecl _mm512_mask_reduce_add_epi32(__mmask16, __m512i);
extern __int64 __cdecl _mm512_reduce_add_epi64(__m512i);
extern __int64 __cdecl _mm512_mask_reduce_add_epi64(__mmask8, __m512i);
extern int     __cdecl _mm512_reduce_add_epu8(__m512i);
extern int     __cdecl _mm512_mask_reduce_add_epu8(__mmask64, __m512i);
extern int     __cdecl _mm512_reduce_add_epu16(__m512i);
extern int     __cdecl _mm512_mask_reduce_add_epu16(__mmask32, __m512i);

extern float   __cdecl _mm512_reduce_mul_ps(__m512);
extern float   __cdecl _mm512_mask_reduce_mul_ps(__mmask16, __m512);
extern double  __cdecl _mm512_reduce_mul_pd(__m512d);
extern double  __cdecl _mm512_mask_reduce_mul_pd(__mmask8, __m512d);
extern int     __cdecl _mm512_reduce_mul_epi32(__m512i);
extern int     __cdecl _mm512_mask_reduce_mul_epi32(__mmask16, __m512i);
extern __int64 __cdecl _mm512_reduce_mul_epi64(__m512i);
extern __int64 __cdecl _mm512_mask_reduce_mul_epi64(__mmask8, __m512i);

extern float   __cdecl _mm512_reduce_min_ps(__m512);
extern float   __cdecl _mm512_mask_reduce_min_ps(__mmask16, __m512);
extern double  __cdecl _mm512_reduce_min_pd(__m512d);
extern double  __cdecl _mm512_mask_reduce_min_pd(__mmask8, __m512d);
extern int     __cdecl _mm512_reduce_min_epi32(__m512i);
extern int     __cdecl _mm512_mask_reduce_min_epi32(__mmask16, __m512i);
extern __int64 __cdecl _mm512_reduce_min_epi64(__m512i);
extern __int64 __cdecl _mm512_mask_reduce_min_epi64(__mmask8, __m512i);
extern unsigned int     __cdecl _mm512_reduce_min_epu32(__m512i);
extern unsigned int     __cdecl _mm512_mask_reduce_min_epu32(__mmask16, __m512i);
extern unsigned __int64 __cdecl _mm512_reduce_min_epu64(__m512i);
extern unsigned __int64 __cdecl _mm512_mask_reduce_min_epu64(__mmask8, __m512i);

extern float   __cdecl _mm512_reduce_max_ps(__m512);
extern float   __cdecl _mm512_mask_reduce_max_ps(__mmask16, __m512);
extern double  __cdecl _mm512_reduce_max_pd(__m512d);
extern double  __cdecl _mm512_mask_reduce_max_pd(__mmask8, __m512d);
extern int     __cdecl _mm512_reduce_max_epi32(__m512i);
extern int     __cdecl _mm512_mask_reduce_max_epi32(__mmask16, __m512i);
extern __int64 __cdecl _mm512_reduce_max_epi64(__m512i);
extern __int64 __cdecl _mm512_mask_reduce_max_epi64(__mmask8, __m512i);
extern unsigned int     __cdecl _mm512_reduce_max_epu32(__m512i);
extern unsigned int     __cdecl _mm512_mask_reduce_max_epu32(__mmask16, __m512i);
extern unsigned __int64 __cdecl _mm512_reduce_max_epu64(__m512i);
extern unsigned __int64 __cdecl _mm512_mask_reduce_max_epu64(__mmask8, __m512i);

extern int     __cdecl _mm512_reduce_and_epi32(__m512i);
extern int     __cdecl _mm512_mask_reduce_and_epi32(__mmask16, __m512i);
extern __int64 __cdecl _mm512_reduce_and_epi64(__m512i);
extern __int64 __cdecl _mm512_mask_reduce_and_epi64(__mmask8, __m512i);

extern int     __cdecl _mm512_reduce_or_epi32(__m512i);
extern int     __cdecl _mm512_mask_reduce_or_epi32(__mmask16, __m512i);
extern __int64 __cdecl _mm512_reduce_or_epi64(__m512i);
extern __int64 __cdecl _mm512_mask_reduce_or_epi64(__mmask8, __m512i);

extern int     __cdecl _mm512_reduce_xor_epi32(__m512i);
extern int     __cdecl _mm512_mask_reduce_xor_epi32(__mmask16, __m512i);
extern __int64 __cdecl _mm512_reduce_xor_epi64(__m512i);
extern __int64 __cdecl _mm512_mask_reduce_xor_epi64(__mmask8, __m512i);

extern __m512d __cdecl _mm512_reduce_pd(__m512d, int);
extern __m512d __cdecl _mm512_mask_reduce_pd(__m512d, __mmask8, __m512d, int);
extern __m512d __cdecl _mm512_maskz_reduce_pd(__mmask8, __m512d, int);
extern __m512d __cdecl _mm512_reduce_round_pd(__m512d, int, int);
extern __m512d __cdecl _mm512_mask_reduce_round_pd(__m512d, __mmask8, __m512d, int, int);
extern __m512d __cdecl _mm512_maskz_reduce_round_pd(__mmask8, __m512d, int, int);
extern __m512  __cdecl _mm512_reduce_ps(__m512, int);
extern __m512  __cdecl _mm512_mask_reduce_ps(__m512, __mmask16, __m512, int);
extern __m512  __cdecl _mm512_maskz_reduce_ps(__mmask16, __m512, int);
extern __m512  __cdecl _mm512_reduce_round_ps(__m512, int, int);
extern __m512  __cdecl _mm512_mask_reduce_round_ps(__m512, __mmask16, __m512, int, int);
extern __m512  __cdecl _mm512_maskz_reduce_round_ps(__mmask16, __m512, int, int);

extern __m512d __cdecl _mm512_roundscale_pd(__m512d, int);
extern __m512d __cdecl _mm512_mask_roundscale_pd(__m512d, __mmask8, __m512d, int);
extern __m512d __cdecl _mm512_maskz_roundscale_pd(__mmask8, __m512d, int);
extern __m512d __cdecl _mm512_roundscale_round_pd(__m512d, int, int);
extern __m512d __cdecl _mm512_mask_roundscale_round_pd(__m512d, __mmask8, __m512d, int, int);
extern __m512d __cdecl _mm512_maskz_roundscale_round_pd(__mmask8, __m512d, int, int);
extern __m512  __cdecl _mm512_roundscale_ps(__m512, int);
extern __m512  __cdecl _mm512_mask_roundscale_ps(__m512, __mmask16, __m512, int);
extern __m512  __cdecl _mm512_maskz_roundscale_ps(__mmask16, __m512, int);
extern __m512  __cdecl _mm512_roundscale_round_ps(__m512, int, int);
extern __m512  __cdecl _mm512_mask_roundscale_round_ps(__m512, __mmask16, __m512, int, int);
extern __m512  __cdecl _mm512_maskz_roundscale_round_ps(__mmask16, __m512, int, int);

extern __m512d __cdecl _mm512_scalef_pd(__m512d, __m512d);
extern __m512d __cdecl _mm512_mask_scalef_pd(__m512d, __mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_maskz_scalef_pd(__mmask8, __m512d, __m512d);
extern __m512d __cdecl _mm512_scalef_round_pd(__m512d, __m512d, int);
extern __m512d __cdecl _mm512_mask_scalef_round_pd(__m512d, __mmask8, __m512d, __m512d, int);
extern __m512d __cdecl _mm512_maskz_scalef_round_pd(__mmask8, __m512d, __m512d, int);
extern __m512  __cdecl _mm512_scalef_ps(__m512, __m512);
extern __m512  __cdecl _mm512_mask_scalef_ps(__m512, __mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_maskz_scalef_ps(__mmask16, __m512, __m512);
extern __m512  __cdecl _mm512_scalef_round_ps(__m512, __m512, int);
extern __m512  __cdecl _mm512_mask_scalef_round_ps(__m512, __mmask16, __m512, __m512, int);
extern __m512  __cdecl _mm512_maskz_scalef_round_ps(__mmask16, __m512, __m512, int);

extern __m512d __cdecl _mm512_fixupimm_pd(__m512d, __m512d, __m512i, const int);
extern __m512d __cdecl _mm512_mask_fixupimm_pd(__m512d, __mmask8, __m512d, __m512i, const int);
extern __m512d __cdecl _mm512_maskz_fixupimm_pd(__mmask8, __m512d, __m512d, __m512i, const int);
extern __m512d __cdecl _mm512_fixupimm_round_pd(__m512d, __m512d, __m512i, const int, const int);
extern __m512d __cdecl _mm512_mask_fixupimm_round_pd(__m512d, __mmask8, __m512d, __m512i, const int, const int);
extern __m512d __cdecl _mm512_maskz_fixupimm_round_pd(__mmask8, __m512d, __m512d, __m512i, const int, const int);
extern __m512  __cdecl _mm512_fixupimm_ps(__m512, __m512, __m512i, const int);
extern __m512  __cdecl _mm512_mask_fixupimm_ps(__m512, __mmask16, __m512, __m512i, const int);
extern __m512  __cdecl _mm512_maskz_fixupimm_ps(__mmask16, __m512, __m512, __m512i, const int);
extern __m512  __cdecl _mm512_fixupimm_round_ps(__m512, __m512, __m512i, const int, const int);
extern __m512  __cdecl _mm512_mask_fixupimm_round_ps(__m512, __mmask16, __m512, __m512i, const int, const int);
extern __m512  __cdecl _mm512_maskz_fixupimm_round_ps(__mmask16, __m512, __m512, __m512i, const int, const int);

extern void    __cdecl _mm512_stream_pd(void*, __m512d);
extern void    __cdecl _mm512_stream_ps(void*, __m512);
extern void    __cdecl _mm512_stream_si512(void*, __m512i);
extern __m512i __cdecl _mm512_stream_load_si512(void const*);

extern __m128d __cdecl _mm512_castpd512_pd128(__m512d);
extern __m128  __cdecl _mm512_castps512_ps128(__m512);
extern __m128i __cdecl _mm512_castsi512_si128(__m512i);
extern __m512i __cdecl _mm512_castsi128_si512(__m128i);

extern __mmask16 __cdecl _mm512_fpclass_ps_mask(__m512, int);
extern __mmask16 __cdecl _mm512_mask_fpclass_ps_mask(__mmask16, __m512, int);
extern __mmask8  __cdecl _mm512_fpclass_pd_mask(__m512d, int);
extern __mmask8  __cdecl _mm512_mask_fpclass_pd_mask(__mmask8, __m512d, int);

extern __m512d __cdecl _mm512_range_pd(__m512d, __m512d, int);
extern __m512d __cdecl _mm512_mask_range_pd(__m512d, __mmask8, __m512d, __m512d, int);
extern __m512d __cdecl _mm512_maskz_range_pd(__mmask8, __m512d, __m512d, int);
extern __m512d __cdecl _mm512_range_round_pd(__m512d, __m512d, int, int);
extern __m512d __cdecl _mm512_mask_range_round_pd(__m512d, __mmask8, __m512d, __m512d, int, int);
extern __m512d __cdecl _mm512_maskz_range_round_pd(__mmask8, __m512d, __m512d, int, int);
extern __m512  __cdecl _mm512_range_ps(__m512, __m512, int);
extern __m512  __cdecl _mm512_mask_range_ps(__m512, __mmask16, __m512, __m512, int);
extern __m512  __cdecl _mm512_maskz_range_ps(__mmask16, __m512, __m512, int);
extern __m512  __cdecl _mm512_range_round_ps(__m512, __m512, int, int);
extern __m512  __cdecl _mm512_mask_range_round_ps(__m512, __mmask16, __m512, __m512, int, int);
extern __m512  __cdecl _mm512_maskz_range_round_ps(__mmask16, __m512, __m512, int, int);

extern __m512i __cdecl _mm512_madd_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_madd_epi16(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_madd_epi16(__mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maddubs_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_maddubs_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_maddubs_epi16(__mmask32, __m512i, __m512i);

extern __m512i __cdecl _mm512_packs_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_packs_epi16(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_packs_epi16(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_packs_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_packs_epi32(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_packs_epi32(__mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_packus_epi16(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_packus_epi16(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_packus_epi16(__mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_packus_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_packus_epi32(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_packus_epi32(__mmask32, __m512i, __m512i);

extern __mmask64 __cdecl _mm512_cmp_epi8_mask(__m512i, __m512i, const int);
extern __mmask64 __cdecl _mm512_mask_cmp_epi8_mask(__mmask64, __m512i, __m512i, const int);
extern __mmask32 __cdecl _mm512_cmp_epi16_mask(__m512i, __m512i, const int);
extern __mmask32 __cdecl _mm512_mask_cmp_epi16_mask(__mmask32, __m512i, __m512i, const int);
extern __mmask16 __cdecl _mm512_cmp_epi32_mask(__m512i, __m512i, const int);
extern __mmask16 __cdecl _mm512_mask_cmp_epi32_mask(__mmask16, __m512i, __m512i, const int);
extern __mmask8  __cdecl _mm512_cmp_epi64_mask(__m512i, __m512i, const int);
extern __mmask8  __cdecl _mm512_mask_cmp_epi64_mask(__mmask8, __m512i, __m512i, const int);
extern __mmask64 __cdecl _mm512_cmp_epu8_mask(__m512i, __m512i, const int);
extern __mmask64 __cdecl _mm512_mask_cmp_epu8_mask(__mmask64, __m512i, __m512i, const int);
extern __mmask32 __cdecl _mm512_cmp_epu16_mask(__m512i, __m512i, const int);
extern __mmask32 __cdecl _mm512_mask_cmp_epu16_mask(__mmask32, __m512i, __m512i, const int);
extern __mmask16 __cdecl _mm512_cmp_epu32_mask(__m512i, __m512i, const int);
extern __mmask16 __cdecl _mm512_mask_cmp_epu32_mask(__mmask16, __m512i, __m512i, const int);
extern __mmask8  __cdecl _mm512_cmp_epu64_mask(__m512i, __m512i, const int);
extern __mmask8  __cdecl _mm512_mask_cmp_epu64_mask(__mmask8, __m512i, __m512i, const int);

extern __mmask64 __cdecl _mm512_test_epi8_mask(__m512i, __m512i);
extern __mmask64 __cdecl _mm512_mask_test_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask32 __cdecl _mm512_test_epi16_mask(__m512i, __m512i);
extern __mmask32 __cdecl _mm512_mask_test_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask64 __cdecl _mm512_testn_epi8_mask(__m512i, __m512i);
extern __mmask64 __cdecl _mm512_mask_testn_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask32 __cdecl _mm512_testn_epi16_mask(__m512i, __m512i);
extern __mmask32 __cdecl _mm512_mask_testn_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask16 __cdecl _mm512_test_epi32_mask(__m512i, __m512i);
extern __mmask16 __cdecl _mm512_mask_test_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_test_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_test_epi64_mask(__mmask8, __m512i, __m512i);
extern __mmask16 __cdecl _mm512_testn_epi32_mask(__m512i, __m512i);
extern __mmask16 __cdecl _mm512_mask_testn_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_testn_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_testn_epi64_mask(__mmask8, __m512i, __m512i);

extern __mmask16 __cdecl _mm512_kunpackb(__mmask16, __mmask16);
extern __mmask32 __cdecl _mm512_kunpackw(__mmask32, __mmask32);
extern __mmask64 __cdecl _mm512_kunpackd(__mmask64, __mmask64);

extern unsigned char __cdecl _mm512_testz_and_mask8(__mmask8, __mmask8);
extern unsigned char __cdecl _mm512_testz_and_mask16(__mmask16, __mmask16);
extern unsigned char __cdecl _mm512_testz_and_mask32(__mmask32, __mmask32);
extern unsigned char __cdecl _mm512_testz_and_mask64(__mmask64, __mmask64);
extern unsigned char __cdecl _mm512_testz_andn_mask8(__mmask8, __mmask8);
extern unsigned char __cdecl _mm512_testz_andn_mask16(__mmask16, __mmask16);
extern unsigned char __cdecl _mm512_testz_andn_mask32(__mmask32, __mmask32);
extern unsigned char __cdecl _mm512_testz_andn_mask64(__mmask64, __mmask64);
extern unsigned char __cdecl _mm512_testz_or_mask8(__mmask8, __mmask8);
extern unsigned char __cdecl _mm512_testz_or_mask16(__mmask16, __mmask16);
extern unsigned char __cdecl _mm512_testz_or_mask32(__mmask32, __mmask32);
extern unsigned char __cdecl _mm512_testz_or_mask64(__mmask64, __mmask64);
extern unsigned char __cdecl _mm512_testz_nor_mask8(__mmask8, __mmask8);
extern unsigned char __cdecl _mm512_testz_nor_mask16(__mmask16, __mmask16);
extern unsigned char __cdecl _mm512_testz_nor_mask32(__mmask32, __mmask32);
extern unsigned char __cdecl _mm512_testz_nor_mask64(__mmask64, __mmask64);

extern __m512  __cdecl _mm512_i32gather_ps(__m512i, void const*, int);
extern __m512  __cdecl _mm512_mask_i32gather_ps(__m512, __mmask16, __m512i, void const*, int);
extern void    __cdecl _mm512_i32scatter_ps(void*, __m512i, __m512, int);
extern void    __cdecl _mm512_mask_i32scatter_ps(void*, __mmask16, __m512i, __m512, int);
extern __m512d __cdecl _mm512_i64gather_pd(__m512i, void const*, int);
extern __m512d __cdecl _mm512_mask_i64gather_pd(__m512d, __mmask8, __m512i, void const*, int);
extern void    __cdecl _mm512_i64scatter_pd(void*, __m512i, __m512d, int);
extern void    __cdecl _mm512_mask_i64scatter_pd(void*, __mmask8, __m512i, __m512d, int);
extern __m512d __cdecl _mm512_i32gather_pd(__m256i, void const*, int);
extern __m512d __cdecl _mm512_mask_i32gather_pd(__m512d, __mmask8, __m256i, void const*, int);
extern void    __cdecl _mm512_i32scatter_pd(void*, __m256i, __m512d, int);
extern void    __cdecl _mm512_mask_i32scatter_pd(void*, __mmask8, __m256i, __m512d, int);
extern __m512i __cdecl _mm512_i32gather_epi32(__m512i, void const*, int);
extern __m512i __cdecl _mm512_mask_i32gather_epi32(__m512i, __mmask16, __m512i, void const*, int);
extern void    __cdecl _mm512_i32scatter_epi32(void*, __m512i, __m512i, int);
extern void    __cdecl _mm512_mask_i32scatter_epi32(void*, __mmask16, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_i32gather_epi64(__m256i, void const*, int);
extern __m512i __cdecl _mm512_mask_i32gather_epi64(__m512i, __mmask8, __m256i, void const*, int);
extern __m512i __cdecl _mm512_i64gather_epi64(__m512i, void const*, int);
extern __m512i __cdecl _mm512_mask_i64gather_epi64(__m512i, __mmask8, __m512i, void const*, int);
extern void    __cdecl _mm512_i32scatter_epi64(void*, __m256i, __m512i, int);
extern void    __cdecl _mm512_mask_i32scatter_epi64(void*, __mmask8, __m256i, __m512i, int);
extern void    __cdecl _mm512_i64scatter_epi64(void*, __m512i, __m512i, int);
extern void    __cdecl _mm512_mask_i64scatter_epi64(void*, __mmask8, __m512i, __m512i, int);
extern __m256  __cdecl _mm512_i64gather_ps(__m512i, void const*, int);
extern __m256  __cdecl _mm512_mask_i64gather_ps(__m256, __mmask8, __m512i, void const*, int);
extern void    __cdecl _mm512_i64scatter_ps(void*, __m512i, __m256, int);
extern void    __cdecl _mm512_mask_i64scatter_ps(void*, __mmask8, __m512i, __m256, int);
extern __m256i __cdecl _mm512_i64gather_epi32(__m512i, void const*, int);
extern __m256i __cdecl _mm512_mask_i64gather_epi32(__m256i, __mmask8, __m512i, void const*, int);
extern void    __cdecl _mm512_i64scatter_epi32(void*, __m512i, __m256i, int);
extern void    __cdecl _mm512_mask_i64scatter_epi32(void*, __mmask8, __m512i, __m256i, int);

extern __m512d __cdecl _mm512_cvtpslo_pd(__m512);
extern __m512d __cdecl _mm512_mask_cvtpslo_pd(__m512d, __mmask8, __m512);
extern __m512d __cdecl _mm512_cvtepi32lo_pd(__m512i);
extern __m512d __cdecl _mm512_mask_cvtepi32lo_pd(__m512d, __mmask8, __m512i);
extern __m512d __cdecl _mm512_cvtepu32lo_pd(__m512i);
extern __m512d __cdecl _mm512_mask_cvtepu32lo_pd(__m512d, __mmask8, __m512i);
extern __m512d __cdecl _mm512_cvtepi32_pd(__m256i);
extern __m512d __cdecl _mm512_mask_cvtepi32_pd(__m512d, __mmask8, __m256i);
extern __m512d __cdecl _mm512_maskz_cvtepi32_pd(__mmask8, __m256i);
extern __m512d __cdecl _mm512_cvtepu32_pd(__m256i);
extern __m512d __cdecl _mm512_mask_cvtepu32_pd(__m512d, __mmask8, __m256i);
extern __m512d __cdecl _mm512_maskz_cvtepu32_pd(__mmask8, __m256i);

extern __m512  __cdecl _mm512_cvtepi32_ps( __m512i);
extern __m512  __cdecl _mm512_mask_cvtepi32_ps(__m512, __mmask16, __m512i);
extern __m512  __cdecl _mm512_maskz_cvtepi32_ps(__mmask16, __m512i);
extern __m512  __cdecl _mm512_cvt_roundepi32_ps(__m512i, int);
extern __m512  __cdecl _mm512_mask_cvt_roundepi32_ps(__m512, __mmask16, __m512i, int);
extern __m512  __cdecl _mm512_maskz_cvt_roundepi32_ps(__mmask16, __m512i, int);
extern __m512  __cdecl _mm512_cvtepu32_ps( __m512i);
extern __m512  __cdecl _mm512_mask_cvtepu32_ps(__m512, __mmask16, __m512i);
extern __m512  __cdecl _mm512_maskz_cvtepu32_ps(__mmask16, __m512i);
extern __m512  __cdecl _mm512_cvt_roundepu32_ps(__m512i, int);
extern __m512  __cdecl _mm512_mask_cvt_roundepu32_ps(__m512, __mmask16, __m512i, int);
extern __m512  __cdecl _mm512_maskz_cvt_roundepu32_ps(__mmask16, __m512i, int);
extern __m512  __cdecl _mm512_cvtph_ps(__m256i);
extern __m512  __cdecl _mm512_mask_cvtph_ps(__m512, __mmask16, __m256i);
extern __m512  __cdecl _mm512_maskz_cvtph_ps(__mmask16, __m256i);
extern __m512  __cdecl _mm512_cvt_roundph_ps(__m256i, int);
extern __m512  __cdecl _mm512_mask_cvt_roundph_ps(__m512, __mmask16, __m256i, int);
extern __m512  __cdecl _mm512_maskz_cvt_roundph_ps(__mmask16, __m256i, int);
extern __m256i __cdecl _mm512_cvtps_ph(__m512, int);
extern __m256i __cdecl _mm512_mask_cvtps_ph(__m256i, __mmask16, __m512, int);
extern __m256i __cdecl _mm512_maskz_cvtps_ph(__mmask16, __m512, int);
extern __m256i __cdecl _mm512_cvt_roundps_ph(__m512, int);
extern __m256i __cdecl _mm512_mask_cvt_roundps_ph(__m256i, __mmask16, __m512, int);
extern __m256i __cdecl _mm512_maskz_cvt_roundps_ph(__mmask16, __m512, int);
extern __m256  __cdecl _mm512_cvtepi64_ps(__m512i);
extern __m256  __cdecl _mm512_mask_cvtepi64_ps(__m256, __mmask8, __m512i);
extern __m256  __cdecl _mm512_maskz_cvtepi64_ps(__mmask8, __m512i);
extern __m256  __cdecl _mm512_cvt_roundepi64_ps(__m512i, int);
extern __m256  __cdecl _mm512_mask_cvt_roundepi64_ps(__m256, __mmask8, __m512i, int);
extern __m256  __cdecl _mm512_maskz_cvt_roundepi64_ps(__mmask8, __m512i, int);
extern __m256  __cdecl _mm512_cvtepu64_ps(__m512i);
extern __m256  __cdecl _mm512_mask_cvtepu64_ps(__m256, __mmask8, __m512i);
extern __m256  __cdecl _mm512_maskz_cvtepu64_ps(__mmask8, __m512i);
extern __m256  __cdecl _mm512_cvt_roundepu64_ps(__m512i, int);
extern __m256  __cdecl _mm512_mask_cvt_roundepu64_ps(__m256, __mmask8, __m512i, int);
extern __m256  __cdecl _mm512_maskz_cvt_roundepu64_ps(__mmask8, __m512i, int);

extern __m512i __cdecl _mm512_cvtepi8_epi32(__m128i);
extern __m512i __cdecl _mm512_mask_cvtepi8_epi32(__m512i, __mmask16, __m128i);
extern __m512i __cdecl _mm512_maskz_cvtepi8_epi32(__mmask16, __m128i);
extern __m512i __cdecl _mm512_cvtepi8_epi64(__m128i);
extern __m512i __cdecl _mm512_mask_cvtepi8_epi64(__m512i, __mmask8, __m128i);
extern __m512i __cdecl _mm512_maskz_cvtepi8_epi64(__mmask8, __m128i);
extern __m512i __cdecl _mm512_cvtepi16_epi32(__m256i);
extern __m512i __cdecl _mm512_mask_cvtepi16_epi32(__m512i, __mmask16, __m256i);
extern __m512i __cdecl _mm512_maskz_cvtepi16_epi32(__mmask16, __m256i);
extern __m512i __cdecl _mm512_cvtepi16_epi64(__m128i);
extern __m512i __cdecl _mm512_mask_cvtepi16_epi64(__m512i, __mmask8, __m128i);
extern __m512i __cdecl _mm512_maskz_cvtepi16_epi64(__mmask8, __m128i);
extern __m128i __cdecl _mm512_cvtepi32_epi8(__m512i);
extern __m128i __cdecl _mm512_mask_cvtepi32_epi8(__m128i, __mmask16, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtepi32_epi8(__mmask16, __m512i);
extern void    __cdecl _mm512_mask_cvtepi32_storeu_epi8(void*, __mmask16, __m512i);
extern __m128i __cdecl _mm512_cvtsepi32_epi8(__m512i);
extern __m128i __cdecl _mm512_mask_cvtsepi32_epi8(__m128i, __mmask16, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtsepi32_epi8(__mmask16, __m512i);
extern void    __cdecl _mm512_mask_cvtsepi32_storeu_epi8(void*, __mmask16, __m512i);
extern __m128i __cdecl _mm512_cvtusepi32_epi8(__m512i);
extern __m128i __cdecl _mm512_mask_cvtusepi32_epi8(__m128i, __mmask16, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtusepi32_epi8(__mmask16, __m512i);
extern void    __cdecl _mm512_mask_cvtusepi32_storeu_epi8(void*, __mmask16, __m512i);
extern __m256i __cdecl _mm512_cvtepi32_epi16(__m512i);
extern __m256i __cdecl _mm512_mask_cvtepi32_epi16(__m256i, __mmask16, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtepi32_epi16(__mmask16, __m512i);
extern void    __cdecl _mm512_mask_cvtepi32_storeu_epi16(void*, __mmask16, __m512i);
extern __m256i __cdecl _mm512_cvtsepi32_epi16(__m512i);
extern __m256i __cdecl _mm512_mask_cvtsepi32_epi16(__m256i, __mmask16, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtsepi32_epi16(__mmask16, __m512i);
extern void    __cdecl _mm512_mask_cvtsepi32_storeu_epi16(void*, __mmask16, __m512i);
extern __m256i __cdecl _mm512_cvtusepi32_epi16(__m512i);
extern __m256i __cdecl _mm512_mask_cvtusepi32_epi16(__m256i, __mmask16, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtusepi32_epi16(__mmask16, __m512i);
extern void    __cdecl _mm512_mask_cvtusepi32_storeu_epi16(void*, __mmask16, __m512i);
extern __m512i __cdecl _mm512_cvtepi32_epi64(__m256i);
extern __m512i __cdecl _mm512_mask_cvtepi32_epi64(__m512i, __mmask8, __m256i);
extern __m512i __cdecl _mm512_maskz_cvtepi32_epi64(__mmask8, __m256i);
extern __m128i __cdecl _mm512_cvtepi64_epi8(__m512i);
extern __m128i __cdecl _mm512_mask_cvtepi64_epi8(__m128i, __mmask8, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtepi64_epi8(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtepi64_storeu_epi8(void*, __mmask8, __m512i);
extern __m128i __cdecl _mm512_cvtsepi64_epi8(__m512i);
extern __m128i __cdecl _mm512_mask_cvtsepi64_epi8(__m128i, __mmask8, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtsepi64_epi8(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtsepi64_storeu_epi8(void*, __mmask8, __m512i);
extern __m128i __cdecl _mm512_cvtusepi64_epi8(__m512i);
extern __m128i __cdecl _mm512_mask_cvtusepi64_epi8(__m128i, __mmask8, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtusepi64_epi8(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtusepi64_storeu_epi8(void*, __mmask8, __m512i);
extern __m128i __cdecl _mm512_cvtepi64_epi16(__m512i);
extern __m128i __cdecl _mm512_mask_cvtepi64_epi16(__m128i, __mmask8, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtepi64_epi16(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtepi64_storeu_epi16(void*, __mmask8, __m512i);
extern __m128i __cdecl _mm512_cvtsepi64_epi16(__m512i);
extern __m128i __cdecl _mm512_mask_cvtsepi64_epi16(__m128i, __mmask8, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtsepi64_epi16(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtsepi64_storeu_epi16(void*, __mmask8, __m512i);
extern __m128i __cdecl _mm512_cvtusepi64_epi16(__m512i);
extern __m128i __cdecl _mm512_mask_cvtusepi64_epi16(__m128i, __mmask8, __m512i);
extern __m128i __cdecl _mm512_maskz_cvtusepi64_epi16(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtusepi64_storeu_epi16(void*, __mmask8, __m512i);
extern __m256i __cdecl _mm512_cvtepi64_epi32(__m512i);
extern __m256i __cdecl _mm512_mask_cvtepi64_epi32(__m256i, __mmask8, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtepi64_epi32(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtepi64_storeu_epi32(void*, __mmask8, __m512i);
extern __m256i __cdecl _mm512_cvtsepi64_epi32(__m512i);
extern __m256i __cdecl _mm512_mask_cvtsepi64_epi32(__m256i, __mmask8, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtsepi64_epi32(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtsepi64_storeu_epi32(void*, __mmask8, __m512i);
extern __m256i __cdecl _mm512_cvtusepi64_epi32(__m512i);
extern __m256i __cdecl _mm512_mask_cvtusepi64_epi32(__m256i, __mmask8, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtusepi64_epi32(__mmask8, __m512i);
extern void    __cdecl _mm512_mask_cvtusepi64_storeu_epi32(void*, __mmask8, __m512i);
extern __m512i __cdecl _mm512_cvtepu8_epi32(__m128i);
extern __m512i __cdecl _mm512_mask_cvtepu8_epi32(__m512i, __mmask16, __m128i);
extern __m512i __cdecl _mm512_maskz_cvtepu8_epi32(__mmask16, __m128i);
extern __m512i __cdecl _mm512_cvtepu8_epi64(__m128i);
extern __m512i __cdecl _mm512_mask_cvtepu8_epi64(__m512i, __mmask8, __m128i);
extern __m512i __cdecl _mm512_maskz_cvtepu8_epi64(__mmask8, __m128i);
extern __m512i __cdecl _mm512_cvtepu16_epi32(__m256i);
extern __m512i __cdecl _mm512_mask_cvtepu16_epi32(__m512i, __mmask16, __m256i);
extern __m512i __cdecl _mm512_maskz_cvtepu16_epi32(__mmask16, __m256i);
extern __m512i __cdecl _mm512_cvtepu16_epi64(__m128i);
extern __m512i __cdecl _mm512_mask_cvtepu16_epi64(__m512i, __mmask8, __m128i);
extern __m512i __cdecl _mm512_maskz_cvtepu16_epi64(__mmask8, __m128i);
extern __m512i __cdecl _mm512_cvtepu32_epi64(__m256i);
extern __m512i __cdecl _mm512_mask_cvtepu32_epi64(__m512i, __mmask8, __m256i);
extern __m512i __cdecl _mm512_maskz_cvtepu32_epi64(__mmask8, __m256i);

extern __m512i __cdecl _mm512_cvtps_epi32(__m512);
extern __m512i __cdecl _mm512_mask_cvtps_epi32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_cvtps_epi32(__mmask16, __m512);
extern __m512i __cdecl _mm512_cvt_roundps_epi32(__m512, int);
extern __m512i __cdecl _mm512_mask_cvt_roundps_epi32(__m512i, __mmask16, __m512, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundps_epi32(__mmask16, __m512, int);
extern __m512i __cdecl _mm512_cvttps_epi32(__m512);
extern __m512i __cdecl _mm512_mask_cvttps_epi32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_cvttps_epi32(__mmask16, __m512);
extern __m512i __cdecl _mm512_cvtt_roundps_epi32(__m512, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundps_epi32(__m512i, __mmask16, __m512, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundps_epi32(__mmask16, __m512, int);
extern __m512i __cdecl _mm512_cvtps_epu32(__m512);
extern __m512i __cdecl _mm512_mask_cvtps_epu32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_cvtps_epu32(__mmask16, __m512);
extern __m512i __cdecl _mm512_cvt_roundps_epu32(__m512, int);
extern __m512i __cdecl _mm512_mask_cvt_roundps_epu32(__m512i, __mmask16, __m512, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundps_epu32(__mmask16, __m512, int);
extern __m512i __cdecl _mm512_cvttps_epu32(__m512);
extern __m512i __cdecl _mm512_mask_cvttps_epu32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_cvttps_epu32(__mmask16, __m512);
extern __m512i __cdecl _mm512_cvtt_roundps_epu32(__m512, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundps_epu32(__m512i, __mmask16, __m512, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundps_epu32(__mmask16, __m512, int);
extern __m256i __cdecl _mm512_cvtpd_epi32(__m512d);
extern __m256i __cdecl _mm512_mask_cvtpd_epi32(__m256i, __mmask8, __m512d);
extern __m256i __cdecl _mm512_maskz_cvtpd_epi32(__mmask8, __m512d);
extern __m256i __cdecl _mm512_cvt_roundpd_epi32(__m512d, int);
extern __m256i __cdecl _mm512_mask_cvt_roundpd_epi32(__m256i, __mmask8, __m512d, int);
extern __m256i __cdecl _mm512_maskz_cvt_roundpd_epi32(__mmask8, __m512d, int);
extern __m256i __cdecl _mm512_cvttpd_epi32(__m512d);
extern __m256i __cdecl _mm512_mask_cvttpd_epi32(__m256i, __mmask8, __m512d);
extern __m256i __cdecl _mm512_maskz_cvttpd_epi32(__mmask8, __m512d);
extern __m256i __cdecl _mm512_cvtt_roundpd_epi32(__m512d, int);
extern __m256i __cdecl _mm512_mask_cvtt_roundpd_epi32(__m256i, __mmask8, __m512d, int);
extern __m256i __cdecl _mm512_maskz_cvtt_roundpd_epi32(__mmask8, __m512d, int);
extern __m256i __cdecl _mm512_cvtpd_epu32(__m512d);
extern __m256i __cdecl _mm512_mask_cvtpd_epu32(__m256i, __mmask8, __m512d);
extern __m256i __cdecl _mm512_maskz_cvtpd_epu32(__mmask8, __m512d);
extern __m256i __cdecl _mm512_cvt_roundpd_epu32(__m512d, int);
extern __m256i __cdecl _mm512_mask_cvt_roundpd_epu32(__m256i, __mmask8, __m512d, int);
extern __m256i __cdecl _mm512_maskz_cvt_roundpd_epu32(__mmask8, __m512d, int);
extern __m256i __cdecl _mm512_cvttpd_epu32(__m512d);
extern __m256i __cdecl _mm512_mask_cvttpd_epu32(__m256i, __mmask8, __m512d);
extern __m256i __cdecl _mm512_maskz_cvttpd_epu32(__mmask8, __m512d);
extern __m256i __cdecl _mm512_cvtt_roundpd_epu32(__m512d, int);
extern __m256i __cdecl _mm512_mask_cvtt_roundpd_epu32(__m256i, __mmask8, __m512d, int);
extern __m256i __cdecl _mm512_maskz_cvtt_roundpd_epu32(__mmask8, __m512d, int);

extern __m512i __cdecl _mm512_cvtepi8_epi16(__m256i);
extern __m512i __cdecl _mm512_mask_cvtepi8_epi16(__m512i, __mmask32, __m256i);
extern __m512i __cdecl _mm512_maskz_cvtepi8_epi16(__mmask32, __m256i);
extern __m512i __cdecl _mm512_cvtepu8_epi16(__m256i);
extern __m512i __cdecl _mm512_mask_cvtepu8_epi16(__m512i, __mmask32, __m256i);
extern __m512i __cdecl _mm512_maskz_cvtepu8_epi16(__mmask32, __m256i);
extern __m256i __cdecl _mm512_cvtepi16_epi8(__m512i);
extern __m256i __cdecl _mm512_mask_cvtepi16_epi8(__m256i, __mmask32, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtepi16_epi8(__mmask32, __m512i);
extern void    __cdecl _mm512_mask_cvtepi16_storeu_epi8(void*, __mmask32, __m512i);
extern __m256i __cdecl _mm512_cvtsepi16_epi8(__m512i);
extern __m256i __cdecl _mm512_mask_cvtsepi16_epi8(__m256i, __mmask32, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtsepi16_epi8(__mmask32, __m512i);
extern void    __cdecl _mm512_mask_cvtsepi16_storeu_epi8(void*, __mmask32, __m512i);
extern __m256i __cdecl _mm512_cvtusepi16_epi8(__m512i);
extern __m256i __cdecl _mm512_mask_cvtusepi16_epi8(__m256i, __mmask32, __m512i);
extern __m256i __cdecl _mm512_maskz_cvtusepi16_epi8(__mmask32, __m512i);
extern void    __cdecl _mm512_mask_cvtusepi16_storeu_epi8(void*, __mmask32, __m512i);

extern __m512d __cdecl _mm512_cvtepi64_pd(__m512i);
extern __m512d __cdecl _mm512_mask_cvtepi64_pd(__m512d, __mmask8, __m512i);
extern __m512d __cdecl _mm512_maskz_cvtepi64_pd(__mmask8, __m512i);
extern __m512d __cdecl _mm512_cvt_roundepi64_pd(__m512i, int);
extern __m512d __cdecl _mm512_mask_cvt_roundepi64_pd(__m512d, __mmask8, __m512i, int);
extern __m512d __cdecl _mm512_maskz_cvt_roundepi64_pd(__mmask8, __m512i, int);
extern __m512d __cdecl _mm512_cvtepu64_pd(__m512i);
extern __m512d __cdecl _mm512_mask_cvtepu64_pd(__m512d, __mmask8, __m512i);
extern __m512d __cdecl _mm512_maskz_cvtepu64_pd(__mmask8, __m512i);
extern __m512d __cdecl _mm512_cvt_roundepu64_pd(__m512i, int);
extern __m512d __cdecl _mm512_mask_cvt_roundepu64_pd(__m512d, __mmask8, __m512i, int);
extern __m512d __cdecl _mm512_maskz_cvt_roundepu64_pd(__mmask8, __m512i, int);
extern __m512i __cdecl _mm512_cvtpd_epi64(__m512d);
extern __m512i __cdecl _mm512_mask_cvtpd_epi64(__m512i, __mmask8, __m512d);
extern __m512i __cdecl _mm512_maskz_cvtpd_epi64(__mmask8, __m512d);
extern __m512i __cdecl _mm512_cvt_roundpd_epi64(__m512d, int);
extern __m512i __cdecl _mm512_mask_cvt_roundpd_epi64(__m512i, __mmask8, __m512d, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundpd_epi64(__mmask8, __m512d, int);
extern __m512i __cdecl _mm512_cvtpd_epu64(__m512d);
extern __m512i __cdecl _mm512_mask_cvtpd_epu64(__m512i, __mmask8, __m512d);
extern __m512i __cdecl _mm512_maskz_cvtpd_epu64(__mmask8, __m512d);
extern __m512i __cdecl _mm512_cvt_roundpd_epu64(__m512d, int);
extern __m512i __cdecl _mm512_mask_cvt_roundpd_epu64(__m512i, __mmask8, __m512d, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundpd_epu64(__mmask8, __m512d, int);
extern __m512i __cdecl _mm512_cvttpd_epi64(__m512d);
extern __m512i __cdecl _mm512_mask_cvttpd_epi64(__m512i, __mmask8, __m512d);
extern __m512i __cdecl _mm512_maskz_cvttpd_epi64(__mmask8, __m512d);
extern __m512i __cdecl _mm512_cvtt_roundpd_epi64(__m512d, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundpd_epi64(__m512i, __mmask8, __m512d, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundpd_epi64(__mmask8, __m512d, int);
extern __m512i __cdecl _mm512_cvttpd_epu64(__m512d);
extern __m512i __cdecl _mm512_mask_cvttpd_epu64(__m512i, __mmask8, __m512d);
extern __m512i __cdecl _mm512_maskz_cvttpd_epu64(__mmask8, __m512d);
extern __m512i __cdecl _mm512_cvtt_roundpd_epu64(__m512d, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundpd_epu64(__m512i, __mmask8, __m512d, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundpd_epu64(__mmask8, __m512d, int);
extern __m512i __cdecl _mm512_cvtps_epi64(__m256);
extern __m512i __cdecl _mm512_mask_cvtps_epi64(__m512i, __mmask8, __m256);
extern __m512i __cdecl _mm512_maskz_cvtps_epi64(__mmask8, __m256);
extern __m512i __cdecl _mm512_cvt_roundps_epi64(__m256, int);
extern __m512i __cdecl _mm512_mask_cvt_roundps_epi64(__m512i, __mmask8, __m256, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundps_epi64(__mmask8, __m256, int);
extern __m512i __cdecl _mm512_cvtps_epu64(__m256);
extern __m512i __cdecl _mm512_mask_cvtps_epu64(__m512i, __mmask8, __m256);
extern __m512i __cdecl _mm512_maskz_cvtps_epu64(__mmask8, __m256);
extern __m512i __cdecl _mm512_cvt_roundps_epu64(__m256, int);
extern __m512i __cdecl _mm512_mask_cvt_roundps_epu64(__m512i, __mmask8, __m256, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundps_epu64(__mmask8, __m256, int);
extern __m512i __cdecl _mm512_cvttps_epi64(__m256);
extern __m512i __cdecl _mm512_mask_cvttps_epi64(__m512i, __mmask8, __m256);
extern __m512i __cdecl _mm512_maskz_cvttps_epi64(__mmask8, __m256);
extern __m512i __cdecl _mm512_cvtt_roundps_epi64(__m256, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundps_epi64(__m512i, __mmask8, __m256, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundps_epi64(__mmask8, __m256, int);
extern __m512i __cdecl _mm512_cvttps_epu64(__m256);
extern __m512i __cdecl _mm512_mask_cvttps_epu64(__m512i, __mmask8, __m256);
extern __m512i __cdecl _mm512_maskz_cvttps_epu64(__mmask8, __m256);
extern __m512i __cdecl _mm512_cvtt_roundps_epu64(__m256, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundps_epu64(__m512i, __mmask8, __m256, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundps_epu64(__mmask8, __m256, int);

extern __mmask64  __cdecl _mm512_cmpeq_epi8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmpge_epi8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmpgt_epi8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmple_epi8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmplt_epi8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmpneq_epi8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmpeq_epu8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmpge_epu8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmpgt_epu8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmple_epu8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmplt_epu8_mask(__m512i, __m512i);
extern __mmask64  __cdecl _mm512_cmpneq_epu8_mask(__m512i, __m512i);

extern __mmask64  __cdecl _mm512_mask_cmpeq_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmpge_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmpgt_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmple_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmplt_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmpneq_epi8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmpeq_epu8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmpge_epu8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmpgt_epu8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmple_epu8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmplt_epu8_mask(__mmask64, __m512i, __m512i);
extern __mmask64  __cdecl _mm512_mask_cmpneq_epu8_mask(__mmask64, __m512i, __m512i);

extern __mmask32  __cdecl _mm512_cmpeq_epi16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmpge_epi16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmpgt_epi16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmple_epi16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmplt_epi16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmpneq_epi16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmpeq_epu16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmpge_epu16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmpgt_epu16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmple_epu16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmplt_epu16_mask(__m512i, __m512i);
extern __mmask32  __cdecl _mm512_cmpneq_epu16_mask(__m512i, __m512i);

extern __mmask32  __cdecl _mm512_mask_cmpeq_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmpge_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmpgt_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmple_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmplt_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmpneq_epi16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmpeq_epu16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmpge_epu16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmpgt_epu16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmple_epu16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmplt_epu16_mask(__mmask32, __m512i, __m512i);
extern __mmask32  __cdecl _mm512_mask_cmpneq_epu16_mask(__mmask32, __m512i, __m512i);

extern __mmask16  __cdecl _mm512_cmpeq_epi32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmpge_epi32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmpgt_epi32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmple_epi32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmplt_epi32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmpneq_epi32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmpeq_epu32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmpge_epu32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmpgt_epu32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmple_epu32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmplt_epu32_mask(__m512i, __m512i);
extern __mmask16  __cdecl _mm512_cmpneq_epu32_mask(__m512i, __m512i);

extern __mmask16  __cdecl _mm512_mask_cmpeq_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmpge_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmpgt_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmple_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmplt_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmpneq_epi32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmpeq_epu32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmpge_epu32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmpgt_epu32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmple_epu32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmplt_epu32_mask(__mmask16, __m512i, __m512i);
extern __mmask16  __cdecl _mm512_mask_cmpneq_epu32_mask(__mmask16, __m512i, __m512i);

extern __mmask8  __cdecl _mm512_cmpeq_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmpge_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmpgt_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmple_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmplt_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmpneq_epi64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmpeq_epu64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmpge_epu64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmpgt_epu64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmple_epu64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmplt_epu64_mask(__m512i, __m512i);
extern __mmask8  __cdecl _mm512_cmpneq_epu64_mask(__m512i, __m512i);

extern __mmask8  __cdecl _mm512_mask_cmpeq_epi64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmpge_epi64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmpgt_epi64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmple_epi64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmplt_epi64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmpneq_epi64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmpeq_epu64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmpge_epu64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmpgt_epu64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmple_epu64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmplt_epu64_mask(__mmask8, __m512i, __m512i);
extern __mmask8  __cdecl _mm512_mask_cmpneq_epu64_mask(__mmask8, __m512i, __m512i);

#define _mm512_setzero_epi32 _mm512_setzero_si512
#define _mm512_set_1to8_pq(x) _mm512_set1_epi64((x))
#define _mm512_set_1to8_epi64(x) _mm512_set1_epi64((x))
#define _mm512_set_1to16_pi(x) _mm512_set1_epi32((x))
#define _mm512_set_1to16_epi32(x) _mm512_set1_epi32((x))
#define _mm512_set4_epi64(a,b,c,d) \
    _mm512_set_epi64((a), (b), (c), (d), (a), (b), (c), (d))
#define _mm512_set4_epi32(a,b,c,d) \
    _mm512_set_epi32((a), (b), (c), (d), (a), (b), (c), (d), \
                    (a), (b), (c), (d), (a), (b), (c), (d))
#define _mm512_setr4_epi64(a,b,c,d) \
    _mm512_set4_epi64((d),(c),(b),(a))
#define _mm512_set_4to8_pq(a,b,c,d) \
    _mm512_setr4_epi64((a),(b),(c),(d))
#define _mm512_set_4to8_epi64(a,b,c,d) \
    _mm512_setr4_epi64((a),(b),(c),(d))
#define _mm512_setr4_epi32(a,b,c,d) \
    _mm512_set4_epi32((d),(c),(b),(a))
#define _mm512_set_4to16_pi(a,b,c,d) \
    _mm512_setr4_epi32((a),(b),(c),(d))
#define _mm512_set_4to16_epi32(a,b,c,d) \
    _mm512_setr4_epi32((a),(b),(c),(d))
#define _mm512_set_16to16_pi(e0,e1,e2,e3,e4,e5,e6,e7,e8, \
                                e9,e10,e11,e12,e13,e14,e15) \
    _mm512_set_epi32((e0),(e1),(e2),(e3),(e4),(e5),(e6),(e7), \
                        (e8),(e9),(e10),(e11),(e12),(e13),(e14),(e15))
#define _mm512_set_16to16_epi32(e0,e1,e2,e3,e4,e5,e6,e7,e8, \
                                e9,e10,e11,e12,e13,e14,e15) \
    _mm512_set_epi32((e0),(e1),(e2),(e3),(e4),(e5),(e6),(e7), \
                        (e8),(e9),(e10),(e11),(e12),(e13),(e14),(e15))
#define _mm512_set_8to8_pq(e0,e1,e2,e3,e4,e5,e6,e7) \
    _mm512_set_epi64((e0),(e1),(e2),(e3),(e4),(e5),(e6),(e7))
#define _mm512_set_8to8_epi64(e0,e1,e2,e3,e4,e5,e6,e7) \
    _mm512_set_epi64((e0),(e1),(e2),(e3),(e4),(e5),(e6),(e7))

#define _mm512_load_si512   _mm512_load_epi32
#define _mm512_loadu_si512  _mm512_loadu_epi32
#define _mm512_store_si512  _mm512_store_epi32
#define _mm512_storeu_si512 _mm512_storeu_epi32

#define _mm512_and_si512      _mm512_and_epi32
#define _mm512_andnot_si512   _mm512_andnot_epi32
#define _mm512_or_si512       _mm512_or_epi32
#define _mm512_xor_si512      _mm512_xor_epi32

#define _mm512_undefined_epi32() _mm512_setzero_si512()

#define _mm512_i32logather_epi64(index, addr, scale) \
    _mm512_i32gather_epi64(_mm512_castsi512_si256(index), (addr), (scale))
#define _mm512_mask_i32logather_epi64(v1, k1, index, addr, scale) \
    _mm512_mask_i32gather_epi64((v1), (k1), _mm512_castsi512_si256(index), (addr), (scale))
#define _mm512_i32logather_pd(index, addr, scale) \
    _mm512_i32gather_pd(_mm512_castsi512_si256(index), (addr), (scale))
#define _mm512_mask_i32logather_pd(v1, k1, index, addr, scale) \
    _mm512_mask_i32gather_pd((v1), (k1), _mm512_castsi512_si256(index), (addr), (scale))
#define _mm512_i32loscatter_pd(addr, index, v1, scale) \
    _mm512_i32scatter_pd((addr), _mm512_castsi512_si256(index), (v1), (scale))
#define _mm512_mask_i32loscatter_pd(addr, k1, index, v1, scale) \
    _mm512_mask_i32scatter_pd((addr), (k1), _mm512_castsi512_si256(index), (v1), (scale))


// AVX512VL functions
extern __m128i   __cdecl _mm_mask_abs_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_abs_epi16(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_abs_epi16(__m256i, __mmask16, __m256i);
extern __m256i   __cdecl _mm256_maskz_abs_epi16(__mmask16, __m256i);
extern __m128i   __cdecl _mm_mask_abs_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_abs_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_abs_epi32(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_abs_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_abs_epi64(__m128i);
extern __m128i   __cdecl _mm_mask_abs_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_abs_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_abs_epi64(__m256i);
extern __m256i   __cdecl _mm256_mask_abs_epi64(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_abs_epi64(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_abs_epi8(__m128i, __mmask16, __m128i);
extern __m128i   __cdecl _mm_maskz_abs_epi8(__mmask16, __m128i);
extern __m256i   __cdecl _mm256_mask_abs_epi8(__m256i, __mmask32, __m256i);
extern __m256i   __cdecl _mm256_maskz_abs_epi8(__mmask32, __m256i);
extern __m128i   __cdecl _mm_mask_add_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_add_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_add_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_add_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_add_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_add_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_add_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_add_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_add_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_add_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_add_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_add_epi64(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_add_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_add_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_add_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_add_epi8(__mmask32, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_add_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_add_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_add_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_add_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_add_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_add_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_add_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_add_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_adds_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_adds_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_adds_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_adds_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_adds_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_adds_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_adds_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_adds_epi8(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_adds_epu16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_adds_epu16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_adds_epu16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_adds_epu16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_adds_epu8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_adds_epu8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_adds_epu8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_adds_epu8(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_alignr_epi32(__m128i, __m128i, const int);
extern __m128i   __cdecl _mm_mask_alignr_epi32(__m128i, __mmask8, __m128i, __m128i, const int);
extern __m128i   __cdecl _mm_maskz_alignr_epi32(__mmask8, __m128i, __m128i, const int);
extern __m256i   __cdecl _mm256_alignr_epi32(__m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_mask_alignr_epi32(__m256i, __mmask8, __m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_alignr_epi32(__mmask8, __m256i, __m256i, const int);
extern __m128i   __cdecl _mm_alignr_epi64(__m128i, __m128i, const int);
extern __m128i   __cdecl _mm_mask_alignr_epi64(__m128i, __mmask8, __m128i, __m128i, const int);
extern __m128i   __cdecl _mm_maskz_alignr_epi64(__mmask8, __m128i, __m128i, const int);
extern __m256i   __cdecl _mm256_alignr_epi64(__m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_mask_alignr_epi64(__m256i, __mmask8, __m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_alignr_epi64(__mmask8, __m256i, __m256i, const int);
extern __m128i   __cdecl _mm_mask_alignr_epi8(__m128i, __mmask16, __m128i, __m128i, const int);
extern __m128i   __cdecl _mm_maskz_alignr_epi8(__mmask16, __m128i, __m128i, const int);
extern __m256i   __cdecl _mm256_mask_alignr_epi8(__m256i, __mmask32, __m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_alignr_epi8(__mmask32, __m256i, __m256i, const int);
extern __m128i   __cdecl _mm_and_epi32(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_and_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_and_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_and_epi32(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_and_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_and_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_and_epi64(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_and_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_and_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_and_epi64(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_and_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_and_epi64(__mmask8, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_and_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_and_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_and_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_and_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_and_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_and_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_and_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_and_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_andnot_epi32(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_andnot_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_andnot_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_andnot_epi32(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_andnot_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_andnot_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_andnot_epi64(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_andnot_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_andnot_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_andnot_epi64(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_andnot_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_andnot_epi64(__mmask8, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_andnot_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_andnot_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_andnot_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_andnot_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_andnot_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_andnot_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_andnot_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_andnot_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_avg_epu16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_avg_epu16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_avg_epu16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_avg_epu16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_avg_epu8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_avg_epu8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_avg_epu8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_avg_epu8(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_blend_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_blend_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_blend_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_blend_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_blend_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_blend_epi64(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_blend_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_blend_epi8(__mmask32, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_blend_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_blend_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_blend_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_blend_ps(__mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_broadcast_f32x2(__m128);
extern __m256    __cdecl _mm256_mask_broadcast_f32x2(__m256, __mmask8, __m128);
extern __m256    __cdecl _mm256_maskz_broadcast_f32x2(__mmask8, __m128);
extern __m256    __cdecl _mm256_broadcast_f32x4(__m128);
extern __m256    __cdecl _mm256_mask_broadcast_f32x4(__m256, __mmask8, __m128);
extern __m256    __cdecl _mm256_maskz_broadcast_f32x4(__mmask8, __m128);
extern __m256d   __cdecl _mm256_broadcast_f64x2(__m128d);
extern __m256d   __cdecl _mm256_mask_broadcast_f64x2(__m256d, __mmask8, __m128d);
extern __m256d   __cdecl _mm256_maskz_broadcast_f64x2(__mmask8, __m128d);
extern __m128i   __cdecl _mm_broadcast_i32x2(__m128i);
extern __m128i   __cdecl _mm_mask_broadcast_i32x2(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_broadcast_i32x2(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_broadcast_i32x2(__m128i);
extern __m256i   __cdecl _mm256_mask_broadcast_i32x2(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_broadcast_i32x2(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_broadcast_i32x4(__m128i);
extern __m256i   __cdecl _mm256_mask_broadcast_i32x4(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_broadcast_i32x4(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_broadcast_i64x2(__m128i);
extern __m256i   __cdecl _mm256_mask_broadcast_i64x2(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_broadcast_i64x2(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_broadcastb_epi8(__m128i, __mmask16, __m128i);
extern __m128i   __cdecl _mm_maskz_broadcastb_epi8(__mmask16, __m128i);
extern __m256i   __cdecl _mm256_mask_broadcastb_epi8(__m256i, __mmask32, __m128i);
extern __m256i   __cdecl _mm256_maskz_broadcastb_epi8(__mmask32, __m128i);
extern __m128i   __cdecl _mm_mask_broadcastd_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_broadcastd_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_broadcastd_epi32(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_broadcastd_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm_broadcastmb_epi64(__mmask8);
extern __m256i   __cdecl _mm256_broadcastmb_epi64(__mmask8);
extern __m128i   __cdecl _mm_broadcastmw_epi32(__mmask16);
extern __m256i   __cdecl _mm256_broadcastmw_epi32(__mmask16);
extern __m128i   __cdecl _mm_mask_broadcastq_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_broadcastq_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_broadcastq_epi64(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_broadcastq_epi64(__mmask8, __m128i);
extern __m256d   __cdecl _mm256_mask_broadcastsd_pd(__m256d, __mmask8, __m128d);
extern __m256d   __cdecl _mm256_maskz_broadcastsd_pd(__mmask8, __m128d);
extern __m128    __cdecl _mm_mask_broadcastss_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_broadcastss_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_broadcastss_ps(__m256, __mmask8, __m128);
extern __m256    __cdecl _mm256_maskz_broadcastss_ps(__mmask8, __m128);
extern __m128i   __cdecl _mm_mask_broadcastw_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_broadcastw_epi16(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_broadcastw_epi16(__m256i, __mmask16, __m128i);
extern __m256i   __cdecl _mm256_maskz_broadcastw_epi16(__mmask16, __m128i);
extern __mmask8  __cdecl _mm_cmp_epi16_mask(__m128i, __m128i, const int);
extern __mmask8  __cdecl _mm_mask_cmp_epi16_mask(__mmask8, __m128i, __m128i, const int);
extern __mmask16 __cdecl _mm256_cmp_epi16_mask(__m256i, __m256i, const int);
extern __mmask16 __cdecl _mm256_mask_cmp_epi16_mask(__mmask16, __m256i, __m256i, const int);
extern __mmask8  __cdecl _mm_cmp_epi32_mask(__m128i, __m128i, int);
extern __mmask8  __cdecl _mm_mask_cmp_epi32_mask(__mmask8, __m128i, __m128i, int);
extern __mmask8  __cdecl _mm256_cmp_epi32_mask(__m256i, __m256i, int);
extern __mmask8  __cdecl _mm256_mask_cmp_epi32_mask(__mmask8, __m256i, __m256i, int);
extern __mmask8  __cdecl _mm_cmp_epi64_mask(__m128i, __m128i, int);
extern __mmask8  __cdecl _mm_mask_cmp_epi64_mask(__mmask8, __m128i, __m128i, int);
extern __mmask8  __cdecl _mm256_cmp_epi64_mask(__m256i, __m256i, int);
extern __mmask8  __cdecl _mm256_mask_cmp_epi64_mask(__mmask8, __m256i, __m256i, int);
extern __mmask16 __cdecl _mm_cmp_epi8_mask(__m128i, __m128i, const int);
extern __mmask16 __cdecl _mm_mask_cmp_epi8_mask(__mmask16, __m128i, __m128i, const int);
extern __mmask32 __cdecl _mm256_cmp_epi8_mask(__m256i, __m256i, const int);
extern __mmask32 __cdecl _mm256_mask_cmp_epi8_mask(__mmask32, __m256i, __m256i, const int);
extern __mmask8  __cdecl _mm_cmp_epu16_mask(__m128i, __m128i, const int);
extern __mmask8  __cdecl _mm_mask_cmp_epu16_mask(__mmask8, __m128i, __m128i, const int);
extern __mmask16 __cdecl _mm256_cmp_epu16_mask(__m256i, __m256i, const int);
extern __mmask16 __cdecl _mm256_mask_cmp_epu16_mask(__mmask16, __m256i, __m256i, const int);
extern __mmask8  __cdecl _mm_cmp_epu32_mask(__m128i, __m128i, int);
extern __mmask8  __cdecl _mm_mask_cmp_epu32_mask(__mmask8, __m128i, __m128i, int);
extern __mmask8  __cdecl _mm256_cmp_epu32_mask(__m256i, __m256i, int);
extern __mmask8  __cdecl _mm256_mask_cmp_epu32_mask(__mmask8, __m256i, __m256i, int);
extern __mmask8  __cdecl _mm_cmp_epu64_mask(__m128i, __m128i, int);
extern __mmask8  __cdecl _mm_mask_cmp_epu64_mask(__mmask8, __m128i, __m128i, int);
extern __mmask8  __cdecl _mm256_cmp_epu64_mask(__m256i, __m256i, int);
extern __mmask8  __cdecl _mm256_mask_cmp_epu64_mask(__mmask8, __m256i, __m256i, int);
extern __mmask16 __cdecl _mm_cmp_epu8_mask(__m128i, __m128i, const int);
extern __mmask16 __cdecl _mm_mask_cmp_epu8_mask(__mmask16, __m128i, __m128i, const int);
extern __mmask32 __cdecl _mm256_cmp_epu8_mask(__m256i, __m256i, const int);
extern __mmask32 __cdecl _mm256_mask_cmp_epu8_mask(__mmask32, __m256i, __m256i, const int);
extern __mmask8  __cdecl _mm_cmp_pd_mask(__m128d, __m128d, const int);
extern __mmask8  __cdecl _mm_mask_cmp_pd_mask(__mmask8, __m128d, __m128d, const int);
extern __mmask8  __cdecl _mm256_cmp_pd_mask(__m256d, __m256d, const int);
extern __mmask8  __cdecl _mm256_mask_cmp_pd_mask(__mmask8, __m256d, __m256d, const int);
extern __mmask8  __cdecl _mm_cmp_ps_mask(__m128, __m128, const int);
extern __mmask8  __cdecl _mm_mask_cmp_ps_mask(__mmask8, __m128, __m128, const int);
extern __mmask8  __cdecl _mm256_cmp_ps_mask(__m256, __m256, const int);
extern __mmask8  __cdecl _mm256_mask_cmp_ps_mask(__mmask8, __m256, __m256, const int);
extern __m128i   __cdecl _mm_mask_compress_epi8(__m128i, __mmask16, __m128i);
extern __m128i   __cdecl _mm_maskz_compress_epi8(__mmask16, __m128i);
extern __m256i   __cdecl _mm256_mask_compress_epi8(__m256i, __mmask32, __m256i);
extern __m256i   __cdecl _mm256_maskz_compress_epi8(__mmask32, __m256i);
extern __m128i   __cdecl _mm_mask_compress_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_compress_epi16(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_compress_epi16(__m256i, __mmask16, __m256i);
extern __m256i   __cdecl _mm256_maskz_compress_epi16(__mmask16, __m256i);
extern __m128i   __cdecl _mm_mask_compress_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_compress_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_compress_epi32(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_compress_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_compress_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_compress_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_compress_epi64(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_compress_epi64(__mmask8, __m256i);
extern __m128d   __cdecl _mm_mask_compress_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_compress_pd(__mmask8, __m128d);
extern __m256d   __cdecl _mm256_mask_compress_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_compress_pd(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_compress_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_compress_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_compress_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_compress_ps(__mmask8, __m256);
extern void      __cdecl _mm_mask_compressstoreu_epi8(void*, __mmask16, __m128i);
extern void      __cdecl _mm256_mask_compressstoreu_epi8(void*, __mmask32, __m256i);
extern void      __cdecl _mm_mask_compressstoreu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_compressstoreu_epi16(void*, __mmask16, __m256i);
extern void      __cdecl _mm_mask_compressstoreu_epi32(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_compressstoreu_epi32(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_compressstoreu_epi64(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_compressstoreu_epi64(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_compressstoreu_pd(void*, __mmask8, __m128d);
extern void      __cdecl _mm256_mask_compressstoreu_pd(void*, __mmask8, __m256d);
extern void      __cdecl _mm_mask_compressstoreu_ps(void*, __mmask8, __m128);
extern void      __cdecl _mm256_mask_compressstoreu_ps(void*, __mmask8, __m256);
extern __m128i   __cdecl _mm_conflict_epi32(__m128i);
extern __m128i   __cdecl _mm_mask_conflict_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_conflict_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_conflict_epi32(__m256i);
extern __m256i   __cdecl _mm256_mask_conflict_epi32(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_conflict_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_conflict_epi64(__m128i);
extern __m128i   __cdecl _mm_mask_conflict_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_conflict_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_conflict_epi64(__m256i);
extern __m256i   __cdecl _mm256_mask_conflict_epi64(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_conflict_epi64(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_cvtps_ph(__m128i, __mmask8, __m128, int);
extern __m128i   __cdecl _mm_maskz_cvtps_ph(__mmask8, __m128, int);
extern __m128i   __cdecl _mm_mask_cvt_roundps_ph(__m128i, __mmask8, __m128, int);
extern __m128i   __cdecl _mm_maskz_cvt_roundps_ph(__mmask8, __m128, int);
extern __m128i   __cdecl _mm256_mask_cvtps_ph(__m128i, __mmask8, __m256, int);
extern __m128i   __cdecl _mm256_maskz_cvtps_ph(__mmask8, __m256, int);
extern __m128i   __cdecl _mm256_mask_cvt_roundps_ph(__m128i, __mmask8, __m256, int);
extern __m128i   __cdecl _mm256_maskz_cvt_roundps_ph(__mmask8, __m256, int);
extern __m128i   __cdecl _mm_mask_cvtepi16_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi16_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepi16_epi32(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepi16_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepi16_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi16_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepi16_epi64(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepi16_epi64(__mmask8, __m128i);
extern __m128i   __cdecl _mm_cvtepi16_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtepi16_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi16_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtepi16_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtepi16_epi8(__m128i, __mmask16, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtepi16_epi8(__mmask16, __m256i);
extern void      __cdecl _mm_mask_cvtepi16_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtepi16_storeu_epi8(void*, __mmask16, __m256i);
extern __m128i   __cdecl _mm_cvtepi32_epi16(__m128i);
extern __m128i   __cdecl _mm_mask_cvtepi32_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi32_epi16(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtepi32_epi16(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtepi32_epi16(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtepi32_epi16(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_cvtepi32_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi32_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepi32_epi64(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepi32_epi64(__mmask8, __m128i);
extern __m128i   __cdecl _mm_cvtepi32_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtepi32_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi32_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtepi32_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtepi32_epi8(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtepi32_epi8(__mmask8, __m256i);
extern __m128d   __cdecl _mm_mask_cvtepi32_pd(__m128d, __mmask8, __m128i);
extern __m128d   __cdecl _mm_maskz_cvtepi32_pd(__mmask8, __m128i);
extern __m256d   __cdecl _mm256_mask_cvtepi32_pd(__m256d, __mmask8, __m128i);
extern __m256d   __cdecl _mm256_maskz_cvtepi32_pd(__mmask8, __m128i);
extern __m128    __cdecl _mm_mask_cvtepi32_ps(__m128, __mmask8, __m128i);
extern __m128    __cdecl _mm_maskz_cvtepi32_ps(__mmask8, __m128i);
extern __m256    __cdecl _mm256_mask_cvtepi32_ps(__m256, __mmask8, __m256i);
extern __m256    __cdecl _mm256_maskz_cvtepi32_ps(__mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtepi32_storeu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtepi32_storeu_epi16(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtepi32_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtepi32_storeu_epi8(void*, __mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtepi64_epi16(__m128i);
extern __m128i   __cdecl _mm_mask_cvtepi64_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi64_epi16(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtepi64_epi16(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtepi64_epi16(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtepi64_epi16(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtepi64_epi32(__m128i);
extern __m128i   __cdecl _mm_mask_cvtepi64_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi64_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtepi64_epi32(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtepi64_epi32(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtepi64_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtepi64_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtepi64_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi64_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtepi64_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtepi64_epi8(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtepi64_epi8(__mmask8, __m256i);
extern __m128d   __cdecl _mm_cvtepi64_pd(__m128i);
extern __m128d   __cdecl _mm_mask_cvtepi64_pd(__m128d, __mmask8, __m128i);
extern __m128d   __cdecl _mm_maskz_cvtepi64_pd(__mmask8, __m128i);
extern __m256d   __cdecl _mm256_cvtepi64_pd(__m256i);
extern __m256d   __cdecl _mm256_mask_cvtepi64_pd(__m256d, __mmask8, __m256i);
extern __m256d   __cdecl _mm256_maskz_cvtepi64_pd(__mmask8, __m256i);
extern __m128    __cdecl _mm_cvtepi64_ps(__m128i);
extern __m128    __cdecl _mm_mask_cvtepi64_ps(__m128, __mmask8, __m128i);
extern __m128    __cdecl _mm_maskz_cvtepi64_ps(__mmask8, __m128i);
extern __m128    __cdecl _mm256_cvtepi64_ps(__m256i);
extern __m128    __cdecl _mm256_mask_cvtepi64_ps(__m128, __mmask8, __m256i);
extern __m128    __cdecl _mm256_maskz_cvtepi64_ps(__mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtepi64_storeu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtepi64_storeu_epi16(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtepi64_storeu_epi32(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtepi64_storeu_epi32(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtepi64_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtepi64_storeu_epi8(void*, __mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_cvtepi8_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi8_epi16(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepi8_epi16(__m256i, __mmask16, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepi8_epi16(__mmask16, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepi8_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi8_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepi8_epi32(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepi8_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepi8_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepi8_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepi8_epi64(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepi8_epi64(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepu16_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepu16_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepu16_epi32(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepu16_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepu16_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepu16_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepu16_epi64(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepu16_epi64(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepu32_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepu32_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepu32_epi64(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepu32_epi64(__mmask8, __m128i);
extern __m128    __cdecl _mm_cvtepu32_ps(__m128i);
extern __m128    __cdecl _mm_mask_cvtepu32_ps(__m128, __mmask8, __m128i);
extern __m128    __cdecl _mm_maskz_cvtepu32_ps(__mmask8, __m128i);
extern __m256    __cdecl _mm256_cvtepu32_ps(__m256i);
extern __m256    __cdecl _mm256_mask_cvtepu32_ps(__m256, __mmask8, __m256i);
extern __m256    __cdecl _mm256_maskz_cvtepu32_ps(__mmask8, __m256i);
extern __m128d   __cdecl _mm_cvtepu32_pd(__m128i);
extern __m128d   __cdecl _mm_mask_cvtepu32_pd(__m128d, __mmask8, __m128i);
extern __m128d   __cdecl _mm_maskz_cvtepu32_pd(__mmask8, __m128i);
extern __m256d   __cdecl _mm256_cvtepu32_pd(__m128i);
extern __m256d   __cdecl _mm256_mask_cvtepu32_pd(__m256d, __mmask8, __m128i);
extern __m256d   __cdecl _mm256_maskz_cvtepu32_pd(__mmask8, __m128i);
extern __m128d   __cdecl _mm_cvtepu64_pd(__m128i);
extern __m128d   __cdecl _mm_mask_cvtepu64_pd(__m128d, __mmask8, __m128i);
extern __m128d   __cdecl _mm_maskz_cvtepu64_pd(__mmask8, __m128i);
extern __m256d   __cdecl _mm256_cvtepu64_pd(__m256i);
extern __m256d   __cdecl _mm256_mask_cvtepu64_pd(__m256d, __mmask8, __m256i);
extern __m256d   __cdecl _mm256_maskz_cvtepu64_pd(__mmask8, __m256i);
extern __m128    __cdecl _mm_cvtepu64_ps(__m128i);
extern __m128    __cdecl _mm_mask_cvtepu64_ps(__m128, __mmask8, __m128i);
extern __m128    __cdecl _mm_maskz_cvtepu64_ps(__mmask8, __m128i);
extern __m128    __cdecl _mm256_cvtepu64_ps(__m256i);
extern __m128    __cdecl _mm256_mask_cvtepu64_ps(__m128, __mmask8, __m256i);
extern __m128    __cdecl _mm256_maskz_cvtepu64_ps(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_cvtepu8_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepu8_epi16(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepu8_epi16(__m256i, __mmask16, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepu8_epi16(__mmask16, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepu8_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepu8_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepu8_epi32(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepu8_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtepu8_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtepu8_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_cvtepu8_epi64(__m256i, __mmask8, __m128i);
extern __m256i   __cdecl _mm256_maskz_cvtepu8_epi64(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtpd_epi32(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvtpd_epi32(__mmask8, __m128d);
extern __m128i   __cdecl _mm256_mask_cvtpd_epi32(__m128i, __mmask8, __m256d);
extern __m128i   __cdecl _mm256_maskz_cvtpd_epi32(__mmask8, __m256d);
extern __m128i   __cdecl _mm_cvtpd_epi64(__m128d);
extern __m128i   __cdecl _mm_mask_cvtpd_epi64(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvtpd_epi64(__mmask8, __m128d);
extern __m256i   __cdecl _mm256_cvtpd_epi64(__m256d);
extern __m256i   __cdecl _mm256_mask_cvtpd_epi64(__m256i, __mmask8, __m256d);
extern __m256i   __cdecl _mm256_maskz_cvtpd_epi64(__mmask8, __m256d);
extern __m128i   __cdecl _mm_cvtpd_epu32(__m128d);
extern __m128i   __cdecl _mm_mask_cvtpd_epu32(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvtpd_epu32(__mmask8, __m128d);
extern __m128i   __cdecl _mm256_cvtpd_epu32(__m256d);
extern __m128i   __cdecl _mm256_mask_cvtpd_epu32(__m128i, __mmask8, __m256d);
extern __m128i   __cdecl _mm256_maskz_cvtpd_epu32(__mmask8, __m256d);
extern __m128i   __cdecl _mm_cvtpd_epu64(__m128d);
extern __m128i   __cdecl _mm_mask_cvtpd_epu64(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvtpd_epu64(__mmask8, __m128d);
extern __m256i   __cdecl _mm256_cvtpd_epu64(__m256d);
extern __m256i   __cdecl _mm256_mask_cvtpd_epu64(__m256i, __mmask8, __m256d);
extern __m256i   __cdecl _mm256_maskz_cvtpd_epu64(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_cvtpd_ps(__m128, __mmask8, __m128d);
extern __m128    __cdecl _mm_maskz_cvtpd_ps(__mmask8, __m128d);
extern __m128    __cdecl _mm256_mask_cvtpd_ps(__m128, __mmask8, __m256d);
extern __m128    __cdecl _mm256_maskz_cvtpd_ps(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_cvtph_ps(__m128, __mmask8, __m128i);
extern __m128    __cdecl _mm_maskz_cvtph_ps(__mmask8, __m128i);
extern __m256    __cdecl _mm256_mask_cvtph_ps(__m256, __mmask8, __m128i);
extern __m256    __cdecl _mm256_maskz_cvtph_ps(__mmask8, __m128i);
extern __m128i   __cdecl _mm_mask_cvtps_epi32(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvtps_epi32(__mmask8, __m128);
extern __m256i   __cdecl _mm256_mask_cvtps_epi32(__m256i, __mmask8, __m256);
extern __m256i   __cdecl _mm256_maskz_cvtps_epi32(__mmask8, __m256);
extern __m128i   __cdecl _mm_cvtps_epi64(__m128);
extern __m128i   __cdecl _mm_mask_cvtps_epi64(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvtps_epi64(__mmask8, __m128);
extern __m256i   __cdecl _mm256_cvtps_epi64(__m128);
extern __m256i   __cdecl _mm256_mask_cvtps_epi64(__m256i, __mmask8, __m128);
extern __m256i   __cdecl _mm256_maskz_cvtps_epi64(__mmask8, __m128);
extern __m128i   __cdecl _mm_cvtps_epu32(__m128);
extern __m128i   __cdecl _mm_mask_cvtps_epu32(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvtps_epu32(__mmask8, __m128);
extern __m256i   __cdecl _mm256_cvtps_epu32(__m256);
extern __m256i   __cdecl _mm256_mask_cvtps_epu32(__m256i, __mmask8, __m256);
extern __m256i   __cdecl _mm256_maskz_cvtps_epu32(__mmask8, __m256);
extern __m128i   __cdecl _mm_cvtps_epu64(__m128);
extern __m128i   __cdecl _mm_mask_cvtps_epu64(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvtps_epu64(__mmask8, __m128);
extern __m256i   __cdecl _mm256_cvtps_epu64(__m128);
extern __m256i   __cdecl _mm256_mask_cvtps_epu64(__m256i, __mmask8, __m128);
extern __m256i   __cdecl _mm256_maskz_cvtps_epu64(__mmask8, __m128);
extern __m128d   __cdecl _mm_mask_cvtps_pd(__m128d, __mmask8, __m128);
extern __m128d   __cdecl _mm_maskz_cvtps_pd(__mmask8, __m128);
extern __m256d   __cdecl _mm256_mask_cvtps_pd(__m256d, __mmask8, __m128);
extern __m256d   __cdecl _mm256_maskz_cvtps_pd(__mmask8, __m128);
extern __m128i   __cdecl _mm_cvtsepi16_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtsepi16_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtsepi16_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtsepi16_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtsepi16_epi8(__m128i, __mmask16, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtsepi16_epi8(__mmask16, __m256i);
extern void      __cdecl _mm_mask_cvtsepi16_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtsepi16_storeu_epi8(void*, __mmask16, __m256i);
extern __m128i   __cdecl _mm_cvtsepi32_epi16(__m128i);
extern __m128i   __cdecl _mm_mask_cvtsepi32_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtsepi32_epi16(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtsepi32_epi16(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtsepi32_epi16(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtsepi32_epi16(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtsepi32_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtsepi32_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtsepi32_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtsepi32_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtsepi32_epi8(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtsepi32_epi8(__mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtsepi32_storeu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtsepi32_storeu_epi16(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtsepi32_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtsepi32_storeu_epi8(void*, __mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtsepi64_epi16(__m128i);
extern __m128i   __cdecl _mm_mask_cvtsepi64_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtsepi64_epi16(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtsepi64_epi16(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtsepi64_epi16(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtsepi64_epi16(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtsepi64_epi32(__m128i);
extern __m128i   __cdecl _mm_mask_cvtsepi64_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtsepi64_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtsepi64_epi32(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtsepi64_epi32(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtsepi64_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtsepi64_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtsepi64_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtsepi64_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtsepi64_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtsepi64_epi8(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtsepi64_epi8(__mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtsepi64_storeu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtsepi64_storeu_epi16(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtsepi64_storeu_epi32(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtsepi64_storeu_epi32(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtsepi64_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtsepi64_storeu_epi8(void*, __mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_cvttpd_epi32(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvttpd_epi32(__mmask8, __m128d);
extern __m128i   __cdecl _mm256_mask_cvttpd_epi32(__m128i, __mmask8, __m256d);
extern __m128i   __cdecl _mm256_maskz_cvttpd_epi32(__mmask8, __m256d);
extern __m128i   __cdecl _mm_cvttpd_epi64(__m128d);
extern __m128i   __cdecl _mm_mask_cvttpd_epi64(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvttpd_epi64(__mmask8, __m128d);
extern __m256i   __cdecl _mm256_cvttpd_epi64(__m256d);
extern __m256i   __cdecl _mm256_mask_cvttpd_epi64(__m256i, __mmask8, __m256d);
extern __m256i   __cdecl _mm256_maskz_cvttpd_epi64(__mmask8, __m256d);
extern __m128i   __cdecl _mm_cvttpd_epu32(__m128d);
extern __m128i   __cdecl _mm_mask_cvttpd_epu32(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvttpd_epu32(__mmask8, __m128d);
extern __m128i   __cdecl _mm256_cvttpd_epu32(__m256d);
extern __m128i   __cdecl _mm256_mask_cvttpd_epu32(__m128i, __mmask8, __m256d);
extern __m128i   __cdecl _mm256_maskz_cvttpd_epu32(__mmask8, __m256d);
extern __m128i   __cdecl _mm_cvttpd_epu64(__m128d);
extern __m128i   __cdecl _mm_mask_cvttpd_epu64(__m128i, __mmask8, __m128d);
extern __m128i   __cdecl _mm_maskz_cvttpd_epu64(__mmask8, __m128d);
extern __m256i   __cdecl _mm256_cvttpd_epu64(__m256d);
extern __m256i   __cdecl _mm256_mask_cvttpd_epu64(__m256i, __mmask8, __m256d);
extern __m256i   __cdecl _mm256_maskz_cvttpd_epu64(__mmask8, __m256d);
extern __m128i   __cdecl _mm_mask_cvttps_epi32(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvttps_epi32(__mmask8, __m128);
extern __m256i   __cdecl _mm256_mask_cvttps_epi32(__m256i, __mmask8, __m256);
extern __m256i   __cdecl _mm256_maskz_cvttps_epi32(__mmask8, __m256);
extern __m128i   __cdecl _mm_cvttps_epi64(__m128);
extern __m128i   __cdecl _mm_mask_cvttps_epi64(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvttps_epi64(__mmask8, __m128);
extern __m256i   __cdecl _mm256_cvttps_epi64(__m128);
extern __m256i   __cdecl _mm256_mask_cvttps_epi64(__m256i, __mmask8, __m128);
extern __m256i   __cdecl _mm256_maskz_cvttps_epi64(__mmask8, __m128);
extern __m128i   __cdecl _mm_cvttps_epu32(__m128);
extern __m128i   __cdecl _mm_mask_cvttps_epu32(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvttps_epu32(__mmask8, __m128);
extern __m256i   __cdecl _mm256_cvttps_epu32(__m256);
extern __m256i   __cdecl _mm256_mask_cvttps_epu32(__m256i, __mmask8, __m256);
extern __m256i   __cdecl _mm256_maskz_cvttps_epu32(__mmask8, __m256);
extern __m128i   __cdecl _mm_cvttps_epu64(__m128);
extern __m128i   __cdecl _mm_mask_cvttps_epu64(__m128i, __mmask8, __m128);
extern __m128i   __cdecl _mm_maskz_cvttps_epu64(__mmask8, __m128);
extern __m256i   __cdecl _mm256_cvttps_epu64(__m128);
extern __m256i   __cdecl _mm256_mask_cvttps_epu64(__m256i, __mmask8, __m128);
extern __m256i   __cdecl _mm256_maskz_cvttps_epu64(__mmask8, __m128);
extern __m128i   __cdecl _mm_cvtusepi16_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtusepi16_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtusepi16_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtusepi16_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtusepi16_epi8(__m128i, __mmask16, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtusepi16_epi8(__mmask16, __m256i);
extern void      __cdecl _mm_mask_cvtusepi16_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtusepi16_storeu_epi8(void*, __mmask16, __m256i);
extern __m128i   __cdecl _mm_cvtusepi32_epi16(__m128i);
extern __m128i   __cdecl _mm_mask_cvtusepi32_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtusepi32_epi16(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtusepi32_epi16(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtusepi32_epi16(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtusepi32_epi16(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtusepi32_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtusepi32_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtusepi32_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtusepi32_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtusepi32_epi8(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtusepi32_epi8(__mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtusepi32_storeu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtusepi32_storeu_epi16(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtusepi32_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtusepi32_storeu_epi8(void*, __mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtusepi64_epi16(__m128i);
extern __m128i   __cdecl _mm_mask_cvtusepi64_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtusepi64_epi16(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtusepi64_epi16(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtusepi64_epi16(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtusepi64_epi16(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtusepi64_epi32(__m128i);
extern __m128i   __cdecl _mm_mask_cvtusepi64_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtusepi64_epi32(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtusepi64_epi32(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtusepi64_epi32(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtusepi64_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_cvtusepi64_epi8(__m128i);
extern __m128i   __cdecl _mm_mask_cvtusepi64_epi8(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_cvtusepi64_epi8(__mmask8, __m128i);
extern __m128i   __cdecl _mm256_cvtusepi64_epi8(__m256i);
extern __m128i   __cdecl _mm256_mask_cvtusepi64_epi8(__m128i, __mmask8, __m256i);
extern __m128i   __cdecl _mm256_maskz_cvtusepi64_epi8(__mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtusepi64_storeu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtusepi64_storeu_epi16(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtusepi64_storeu_epi32(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtusepi64_storeu_epi32(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_cvtusepi64_storeu_epi8(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_cvtusepi64_storeu_epi8(void*, __mmask8, __m256i);
extern __m128i   __cdecl _mm_dbsad_epu8(__m128i, __m128i, int);
extern __m128i   __cdecl _mm_mask_dbsad_epu8(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i   __cdecl _mm_maskz_dbsad_epu8(__mmask8, __m128i, __m128i, int);
extern __m256i   __cdecl _mm256_dbsad_epu8(__m256i, __m256i, int);
extern __m256i   __cdecl _mm256_mask_dbsad_epu8(__m256i, __mmask16, __m256i, __m256i, int);
extern __m256i   __cdecl _mm256_maskz_dbsad_epu8(__mmask16, __m256i, __m256i, int);
extern __m128d   __cdecl _mm_mask_div_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_div_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_div_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_div_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_div_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_div_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_div_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_div_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_expand_epi8(__m128i, __mmask16, __m128i);
extern __m128i   __cdecl _mm_maskz_expand_epi8(__mmask16, __m128i);
extern __m256i   __cdecl _mm256_mask_expand_epi8(__m256i, __mmask32, __m256i);
extern __m256i   __cdecl _mm256_maskz_expand_epi8(__mmask32, __m256i);
extern __m128i   __cdecl _mm_mask_expand_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_expand_epi16(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_expand_epi16(__m256i, __mmask16, __m256i);
extern __m256i   __cdecl _mm256_maskz_expand_epi16(__mmask16, __m256i);
extern __m128i   __cdecl _mm_mask_expand_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_expand_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_expand_epi32(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_expand_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_expand_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_expand_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_expand_epi64(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_expand_epi64(__mmask8, __m256i);
extern __m128d   __cdecl _mm_mask_expand_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_expand_pd(__mmask8, __m128d);
extern __m256d   __cdecl _mm256_mask_expand_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_expand_pd(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_expand_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_expand_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_expand_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_expand_ps(__mmask8, __m256);
extern __m128i   __cdecl _mm_mask_expandloadu_epi8(__m128i, __mmask16, const void*);
extern __m128i   __cdecl _mm_maskz_expandloadu_epi8(__mmask16, const void*);
extern __m256i   __cdecl _mm256_mask_expandloadu_epi8(__m256i, __mmask32, const void*);
extern __m256i   __cdecl _mm256_maskz_expandloadu_epi8(__mmask32, const void*);
extern __m128i   __cdecl _mm_mask_expandloadu_epi16(__m128i, __mmask8, const void*);
extern __m128i   __cdecl _mm_maskz_expandloadu_epi16(__mmask8, const void*);
extern __m256i   __cdecl _mm256_mask_expandloadu_epi16(__m256i, __mmask16, const void*);
extern __m256i   __cdecl _mm256_maskz_expandloadu_epi16(__mmask16, const void*);
extern __m128i   __cdecl _mm_mask_expandloadu_epi32(__m128i, __mmask8, void const*);
extern __m128i   __cdecl _mm_maskz_expandloadu_epi32(__mmask8, void const*);
extern __m256i   __cdecl _mm256_mask_expandloadu_epi32(__m256i, __mmask8, void const*);
extern __m256i   __cdecl _mm256_maskz_expandloadu_epi32(__mmask8, void const*);
extern __m128i   __cdecl _mm_mask_expandloadu_epi64(__m128i, __mmask8, void const*);
extern __m128i   __cdecl _mm_maskz_expandloadu_epi64(__mmask8, void const*);
extern __m256i   __cdecl _mm256_mask_expandloadu_epi64(__m256i, __mmask8, void const*);
extern __m256i   __cdecl _mm256_maskz_expandloadu_epi64(__mmask8, void const*);
extern __m128d   __cdecl _mm_mask_expandloadu_pd(__m128d, __mmask8, void const*);
extern __m128d   __cdecl _mm_maskz_expandloadu_pd(__mmask8, void const*);
extern __m256d   __cdecl _mm256_mask_expandloadu_pd(__m256d, __mmask8, void const*);
extern __m256d   __cdecl _mm256_maskz_expandloadu_pd(__mmask8, void const*);
extern __m128    __cdecl _mm_mask_expandloadu_ps(__m128, __mmask8, void const*);
extern __m128    __cdecl _mm_maskz_expandloadu_ps(__mmask8, void const*);
extern __m256    __cdecl _mm256_mask_expandloadu_ps(__m256, __mmask8, void const*);
extern __m256    __cdecl _mm256_maskz_expandloadu_ps(__mmask8, void const*);
extern __m128    __cdecl _mm256_extractf32x4_ps(__m256, int);
extern __m128    __cdecl _mm256_mask_extractf32x4_ps(__m128, __mmask8, __m256, int);
extern __m128    __cdecl _mm256_maskz_extractf32x4_ps(__mmask8, __m256, int);
extern __m128d   __cdecl _mm256_extractf64x2_pd(__m256d, int);
extern __m128d   __cdecl _mm256_mask_extractf64x2_pd(__m128d, __mmask8, __m256d, int);
extern __m128d   __cdecl _mm256_maskz_extractf64x2_pd(__mmask8, __m256d, int);
extern __m128i   __cdecl _mm256_extracti32x4_epi32(__m256i, int);
extern __m128i   __cdecl _mm256_mask_extracti32x4_epi32(__m128i, __mmask8, __m256i, int);
extern __m128i   __cdecl _mm256_maskz_extracti32x4_epi32(__mmask8, __m256i, int);
extern __m128i   __cdecl _mm256_extracti64x2_epi64(__m256i, int);
extern __m128i   __cdecl _mm256_mask_extracti64x2_epi64(__m128i, __mmask8, __m256i, int);
extern __m128i   __cdecl _mm256_maskz_extracti64x2_epi64(__mmask8, __m256i, int);
extern __m128d   __cdecl _mm_fixupimm_pd(__m128d, __m128d, __m128i, const int);
extern __m128d   __cdecl _mm_mask_fixupimm_pd(__m128d, __mmask8, __m128d, __m128i, const int);
extern __m128d   __cdecl _mm_maskz_fixupimm_pd(__mmask8, __m128d, __m128d, __m128i, const int);
extern __m256d   __cdecl _mm256_fixupimm_pd(__m256d, __m256d, __m256i, const int);
extern __m256d   __cdecl _mm256_mask_fixupimm_pd(__m256d, __mmask8, __m256d, __m256i, const int);
extern __m256d   __cdecl _mm256_maskz_fixupimm_pd(__mmask8, __m256d, __m256d, __m256i, const int);
extern __m128    __cdecl _mm_fixupimm_ps(__m128, __m128, __m128i, const int);
extern __m128    __cdecl _mm_mask_fixupimm_ps(__m128, __mmask8, __m128, __m128i, const int);
extern __m128    __cdecl _mm_maskz_fixupimm_ps(__mmask8, __m128, __m128, __m128i, const int);
extern __m256    __cdecl _mm256_fixupimm_ps(__m256, __m256, __m256i, const int);
extern __m256    __cdecl _mm256_mask_fixupimm_ps(__m256, __mmask8, __m256, __m256i, const int);
extern __m256    __cdecl _mm256_maskz_fixupimm_ps(__mmask8, __m256, __m256, __m256i, const int);
extern __m128d   __cdecl _mm_mask_fmadd_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fmadd_pd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fmadd_pd(__mmask8, __m128d, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_fmadd_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_mask3_fmadd_pd(__m256d, __m256d, __m256d, __mmask8);
extern __m256d   __cdecl _mm256_maskz_fmadd_pd(__mmask8, __m256d, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_fmadd_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fmadd_ps(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fmadd_ps(__mmask8, __m128, __m128, __m128);
extern __m256    __cdecl _mm256_mask_fmadd_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_mask3_fmadd_ps(__m256, __m256, __m256, __mmask8);
extern __m256    __cdecl _mm256_maskz_fmadd_ps(__mmask8, __m256, __m256, __m256);
extern __m128d   __cdecl _mm_mask_fmaddsub_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fmaddsub_pd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fmaddsub_pd(__mmask8, __m128d, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_fmaddsub_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_mask3_fmaddsub_pd(__m256d, __m256d, __m256d, __mmask8);
extern __m256d   __cdecl _mm256_maskz_fmaddsub_pd(__mmask8, __m256d, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_fmaddsub_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fmaddsub_ps(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fmaddsub_ps(__mmask8, __m128, __m128, __m128);
extern __m256    __cdecl _mm256_mask_fmaddsub_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_mask3_fmaddsub_ps(__m256, __m256, __m256, __mmask8);
extern __m256    __cdecl _mm256_maskz_fmaddsub_ps(__mmask8, __m256, __m256, __m256);
extern __m128d   __cdecl _mm_mask_fmsub_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fmsub_pd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fmsub_pd(__mmask8, __m128d, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_fmsub_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_mask3_fmsub_pd(__m256d, __m256d, __m256d, __mmask8);
extern __m256d   __cdecl _mm256_maskz_fmsub_pd(__mmask8, __m256d, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_fmsub_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fmsub_ps(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fmsub_ps(__mmask8, __m128, __m128, __m128);
extern __m256    __cdecl _mm256_mask_fmsub_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_mask3_fmsub_ps(__m256, __m256, __m256, __mmask8);
extern __m256    __cdecl _mm256_maskz_fmsub_ps(__mmask8, __m256, __m256, __m256);
extern __m128d   __cdecl _mm_mask_fmsubadd_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fmsubadd_pd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fmsubadd_pd(__mmask8, __m128d, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_fmsubadd_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_mask3_fmsubadd_pd(__m256d, __m256d, __m256d, __mmask8);
extern __m256d   __cdecl _mm256_maskz_fmsubadd_pd(__mmask8, __m256d, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_fmsubadd_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fmsubadd_ps(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fmsubadd_ps(__mmask8, __m128, __m128, __m128);
extern __m256    __cdecl _mm256_mask_fmsubadd_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_mask3_fmsubadd_ps(__m256, __m256, __m256, __mmask8);
extern __m256    __cdecl _mm256_maskz_fmsubadd_ps(__mmask8, __m256, __m256, __m256);
extern __m128d   __cdecl _mm_mask_fnmadd_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fnmadd_pd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fnmadd_pd(__mmask8, __m128d, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_fnmadd_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_mask3_fnmadd_pd(__m256d, __m256d, __m256d, __mmask8);
extern __m256d   __cdecl _mm256_maskz_fnmadd_pd(__mmask8, __m256d, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_fnmadd_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fnmadd_ps(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fnmadd_ps(__mmask8, __m128, __m128, __m128);
extern __m256    __cdecl _mm256_mask_fnmadd_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_mask3_fnmadd_ps(__m256, __m256, __m256, __mmask8);
extern __m256    __cdecl _mm256_maskz_fnmadd_ps(__mmask8, __m256, __m256, __m256);
extern __m128d   __cdecl _mm_mask_fnmsub_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fnmsub_pd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fnmsub_pd(__mmask8, __m128d, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_fnmsub_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_mask3_fnmsub_pd(__m256d, __m256d, __m256d, __mmask8);
extern __m256d   __cdecl _mm256_maskz_fnmsub_pd(__mmask8, __m256d, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_fnmsub_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fnmsub_ps(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fnmsub_ps(__mmask8, __m128, __m128, __m128);
extern __m256    __cdecl _mm256_mask_fnmsub_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_mask3_fnmsub_ps(__m256, __m256, __m256, __mmask8);
extern __m256    __cdecl _mm256_maskz_fnmsub_ps(__mmask8, __m256, __m256, __m256);
extern __mmask8  __cdecl _mm_fpclass_pd_mask(__m128d, int);
extern __mmask8  __cdecl _mm_mask_fpclass_pd_mask(__mmask8, __m128d, int);
extern __mmask8  __cdecl _mm256_fpclass_pd_mask(__m256d, int);
extern __mmask8  __cdecl _mm256_mask_fpclass_pd_mask(__mmask8, __m256d, int);
extern __mmask8  __cdecl _mm_fpclass_ps_mask(__m128, int);
extern __mmask8  __cdecl _mm_mask_fpclass_ps_mask(__mmask8, __m128, int);
extern __mmask8  __cdecl _mm256_fpclass_ps_mask(__m256, int);
extern __mmask8  __cdecl _mm256_mask_fpclass_ps_mask(__mmask8, __m256, int);
extern __m128d   __cdecl _mm_getexp_pd(__m128d);
extern __m128d   __cdecl _mm_mask_getexp_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_getexp_pd(__mmask8, __m128d);
extern __m256d   __cdecl _mm256_getexp_pd(__m256d);
extern __m256d   __cdecl _mm256_mask_getexp_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_getexp_pd(__mmask8, __m256d);
extern __m128    __cdecl _mm_getexp_ps(__m128);
extern __m128    __cdecl _mm_mask_getexp_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_getexp_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_getexp_ps(__m256);
extern __m256    __cdecl _mm256_mask_getexp_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_getexp_ps(__mmask8, __m256);
extern __m128d   __cdecl _mm_getmant_pd(__m128d, int, int);
extern __m128d   __cdecl _mm_mask_getmant_pd(__m128d, __mmask8, __m128d, int, int);
extern __m128d   __cdecl _mm_maskz_getmant_pd(__mmask8, __m128d, int, int);
extern __m256d   __cdecl _mm256_getmant_pd(__m256d, int, int);
extern __m256d   __cdecl _mm256_mask_getmant_pd(__m256d, __mmask8, __m256d, int, int);
extern __m256d   __cdecl _mm256_maskz_getmant_pd(__mmask8, __m256d, int, int);
extern __m128    __cdecl _mm_getmant_ps(__m128, int, int);
extern __m128    __cdecl _mm_mask_getmant_ps(__m128, __mmask8, __m128, int, int);
extern __m128    __cdecl _mm_maskz_getmant_ps(__mmask8, __m128, int, int);
extern __m256    __cdecl _mm256_getmant_ps(__m256, int, int);
extern __m256    __cdecl _mm256_mask_getmant_ps(__m256, __mmask8, __m256, int, int);
extern __m256    __cdecl _mm256_maskz_getmant_ps(__mmask8, __m256, int, int);
extern __m128i   __cdecl _mm_mmask_i32gather_epi32(__m128i, __mmask8, __m128i, void const*, const int);
extern __m256i   __cdecl _mm256_mmask_i32gather_epi32(__m256i, __mmask8, __m256i, void const*, const int);
extern __m128i   __cdecl _mm_mmask_i32gather_epi64(__m128i, __mmask8, __m128i, void const*, const int);
extern __m256i   __cdecl _mm256_mmask_i32gather_epi64(__m256i, __mmask8, __m128i, void const*, const int);
extern __m128d   __cdecl _mm_mmask_i32gather_pd(__m128d, __mmask8, __m128i, void const*, const int);
extern __m256d   __cdecl _mm256_mmask_i32gather_pd(__m256d, __mmask8, __m128i, void const*, const int);
extern __m128    __cdecl _mm_mmask_i32gather_ps(__m128, __mmask8, __m128i, void const*, const int);
extern __m256    __cdecl _mm256_mmask_i32gather_ps(__m256, __mmask8, __m256i, void const*, const int);
extern void      __cdecl _mm_i32scatter_epi32(void*, __m128i, __m128i, const int);
extern void      __cdecl _mm_mask_i32scatter_epi32(void*, __mmask8, __m128i, __m128i, const int);
extern void      __cdecl _mm256_i32scatter_epi32(void*, __m256i, __m256i, const int);
extern void      __cdecl _mm256_mask_i32scatter_epi32(void*, __mmask8, __m256i, __m256i, const int);
extern void      __cdecl _mm_i32scatter_epi64(void*, __m128i, __m128i, const int);
extern void      __cdecl _mm_mask_i32scatter_epi64(void*, __mmask8, __m128i, __m128i, const int);
extern void      __cdecl _mm256_i32scatter_epi64(void*, __m128i, __m256i, const int);
extern void      __cdecl _mm256_mask_i32scatter_epi64(void*, __mmask8, __m128i, __m256i, const int);
extern void      __cdecl _mm_i32scatter_pd(void*, __m128i, __m128d, const int);
extern void      __cdecl _mm_mask_i32scatter_pd(void*, __mmask8, __m128i, __m128d, const int);
extern void      __cdecl _mm256_i32scatter_pd(void*, __m128i, __m256d, const int);
extern void      __cdecl _mm256_mask_i32scatter_pd(void*, __mmask8, __m128i, __m256d, const int);
extern void      __cdecl _mm_i32scatter_ps(void*, __m128i, __m128, const int);
extern void      __cdecl _mm_mask_i32scatter_ps(void*, __mmask8, __m128i, __m128, const int);
extern void      __cdecl _mm256_i32scatter_ps(void*, __m256i, __m256, const int);
extern void      __cdecl _mm256_mask_i32scatter_ps(void*, __mmask8, __m256i, __m256, const int);
extern __m128i   __cdecl _mm_mmask_i64gather_epi32(__m128i, __mmask8, __m128i, void const*, const int);
extern __m128i   __cdecl _mm256_mmask_i64gather_epi32(__m128i, __mmask8, __m256i, void const*, const int);
extern __m128i   __cdecl _mm_mmask_i64gather_epi64(__m128i, __mmask8, __m128i, void const*, const int);
extern __m256i   __cdecl _mm256_mmask_i64gather_epi64(__m256i, __mmask8, __m256i, void const*, const int);
extern __m128d   __cdecl _mm_mmask_i64gather_pd(__m128d, __mmask8, __m128i, void const*, const int);
extern __m256d   __cdecl _mm256_mmask_i64gather_pd(__m256d, __mmask8, __m256i, void const*, const int);
extern __m128    __cdecl _mm_mmask_i64gather_ps(__m128, __mmask8, __m128i, void const*, const int);
extern __m128    __cdecl _mm256_mmask_i64gather_ps(__m128, __mmask8, __m256i, void const*, const int);
extern void      __cdecl _mm_i64scatter_epi32(void*, __m128i, __m128i, const int);
extern void      __cdecl _mm_mask_i64scatter_epi32(void*, __mmask8, __m128i, __m128i, const int);
extern void      __cdecl _mm256_i64scatter_epi32(void*, __m256i, __m128i, const int);
extern void      __cdecl _mm256_mask_i64scatter_epi32(void*, __mmask8, __m256i, __m128i, const int);
extern void      __cdecl _mm_i64scatter_epi64(void*, __m128i, __m128i, const int);
extern void      __cdecl _mm_mask_i64scatter_epi64(void*, __mmask8, __m128i, __m128i, const int);
extern void      __cdecl _mm256_i64scatter_epi64(void*, __m256i, __m256i, const int);
extern void      __cdecl _mm256_mask_i64scatter_epi64(void*, __mmask8, __m256i, __m256i, const int);
extern void      __cdecl _mm_i64scatter_pd(void*, __m128i, __m128d, const int);
extern void      __cdecl _mm_mask_i64scatter_pd(void*, __mmask8, __m128i, __m128d, const int);
extern void      __cdecl _mm256_i64scatter_pd(void*, __m256i, __m256d, const int);
extern void      __cdecl _mm256_mask_i64scatter_pd(void*, __mmask8, __m256i, __m256d, const int);
extern void      __cdecl _mm_i64scatter_ps(void*, __m128i, __m128, const int);
extern void      __cdecl _mm_mask_i64scatter_ps(void*, __mmask8, __m128i, __m128, const int);
extern void      __cdecl _mm256_i64scatter_ps(void*, __m256i, __m128, const int);
extern void      __cdecl _mm256_mask_i64scatter_ps(void*, __mmask8, __m256i, __m128, const int);
extern __m256    __cdecl _mm256_insertf32x4(__m256, __m128, int);
extern __m256    __cdecl _mm256_mask_insertf32x4(__m256, __mmask8, __m256, __m128, int);
extern __m256    __cdecl _mm256_maskz_insertf32x4(__mmask8, __m256, __m128, int);
extern __m256d   __cdecl _mm256_insertf64x2(__m256d, __m128d, int);
extern __m256d   __cdecl _mm256_mask_insertf64x2(__m256d, __mmask8, __m256d, __m128d, int);
extern __m256d   __cdecl _mm256_maskz_insertf64x2(__mmask8, __m256d, __m128d, int);
extern __m256i   __cdecl _mm256_inserti32x4(__m256i, __m128i, int);
extern __m256i   __cdecl _mm256_mask_inserti32x4(__m256i, __mmask8, __m256i, __m128i, int);
extern __m256i   __cdecl _mm256_maskz_inserti32x4(__mmask8, __m256i, __m128i, int);
extern __m256i   __cdecl _mm256_inserti64x2(__m256i, __m128i, int);
extern __m256i   __cdecl _mm256_mask_inserti64x2(__m256i, __mmask8, __m256i, __m128i, int);
extern __m256i   __cdecl _mm256_maskz_inserti64x2(__mmask8, __m256i, __m128i, int);
extern __m128i   __cdecl _mm_mask_load_epi32(__m128i, __mmask8, void const*);
extern __m128i   __cdecl _mm_maskz_load_epi32(__mmask8, void const*);
extern __m256i   __cdecl _mm256_mask_load_epi32(__m256i, __mmask8, void const*);
extern __m256i   __cdecl _mm256_maskz_load_epi32(__mmask8, void const*);
extern __m128i   __cdecl _mm_mask_load_epi64(__m128i, __mmask8, void const*);
extern __m128i   __cdecl _mm_maskz_load_epi64(__mmask8, void const*);
extern __m256i   __cdecl _mm256_mask_load_epi64(__m256i, __mmask8, void const*);
extern __m256i   __cdecl _mm256_maskz_load_epi64(__mmask8, void const*);
extern __m128d   __cdecl _mm_mask_load_pd(__m128d, __mmask8, void const*);
extern __m128d   __cdecl _mm_maskz_load_pd(__mmask8, void const*);
extern __m256d   __cdecl _mm256_mask_load_pd(__m256d, __mmask8, void const*);
extern __m256d   __cdecl _mm256_maskz_load_pd(__mmask8, void const*);
extern __m128    __cdecl _mm_mask_load_ps(__m128, __mmask8, void const*);
extern __m128    __cdecl _mm_maskz_load_ps(__mmask8, void const*);
extern __m256    __cdecl _mm256_mask_load_ps(__m256, __mmask8, void const*);
extern __m256    __cdecl _mm256_maskz_load_ps(__mmask8, void const*);
extern __m128i   __cdecl _mm_loadu_epi16(void const*);
extern __m128i   __cdecl _mm_mask_loadu_epi16(__m128i, __mmask8, void const*);
extern __m128i   __cdecl _mm_maskz_loadu_epi16(__mmask8, void const*);
extern __m256i   __cdecl _mm256_loadu_epi16(void const*);
extern __m256i   __cdecl _mm256_mask_loadu_epi16(__m256i, __mmask16, void const*);
extern __m256i   __cdecl _mm256_maskz_loadu_epi16(__mmask16, void const*);
extern __m128i   __cdecl _mm_loadu_epi32(void const*);
extern __m128i   __cdecl _mm_mask_loadu_epi32(__m128i, __mmask8, void const*);
extern __m128i   __cdecl _mm_maskz_loadu_epi32(__mmask8, void const*);
extern __m256i   __cdecl _mm256_loadu_epi32(void const*);
extern __m256i   __cdecl _mm256_mask_loadu_epi32(__m256i, __mmask8, void const*);
extern __m256i   __cdecl _mm256_maskz_loadu_epi32(__mmask8, void const*);
extern __m128i   __cdecl _mm_loadu_epi64(void const*);
extern __m128i   __cdecl _mm_mask_loadu_epi64(__m128i, __mmask8, void const*);
extern __m128i   __cdecl _mm_maskz_loadu_epi64(__mmask8, void const*);
extern __m256i   __cdecl _mm256_loadu_epi64(void const*);
extern __m256i   __cdecl _mm256_mask_loadu_epi64(__m256i, __mmask8, void const*);
extern __m256i   __cdecl _mm256_maskz_loadu_epi64(__mmask8, void const*);
extern __m128i   __cdecl _mm_loadu_epi8(void const*);
extern __m128i   __cdecl _mm_mask_loadu_epi8(__m128i, __mmask16, void const*);
extern __m128i   __cdecl _mm_maskz_loadu_epi8(__mmask16, void const*);
extern __m256i   __cdecl _mm256_loadu_epi8(void const*);
extern __m256i   __cdecl _mm256_mask_loadu_epi8(__m256i, __mmask32, void const*);
extern __m256i   __cdecl _mm256_maskz_loadu_epi8(__mmask32, void const*);
extern __m128d   __cdecl _mm_mask_loadu_pd(__m128d, __mmask8, void const*);
extern __m128d   __cdecl _mm_maskz_loadu_pd(__mmask8, void const*);
extern __m256d   __cdecl _mm256_mask_loadu_pd(__m256d, __mmask8, void const*);
extern __m256d   __cdecl _mm256_maskz_loadu_pd(__mmask8, void const*);
extern __m128    __cdecl _mm_mask_loadu_ps(__m128, __mmask8, void const*);
extern __m128    __cdecl _mm_maskz_loadu_ps(__mmask8, void const*);
extern __m256    __cdecl _mm256_mask_loadu_ps(__m256, __mmask8, void const*);
extern __m256    __cdecl _mm256_maskz_loadu_ps(__mmask8, void const*);
extern __m128i   __cdecl _mm_lzcnt_epi32(__m128i);
extern __m128i   __cdecl _mm_mask_lzcnt_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_lzcnt_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_lzcnt_epi32(__m256i);
extern __m256i   __cdecl _mm256_mask_lzcnt_epi32(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_lzcnt_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_lzcnt_epi64(__m128i);
extern __m128i   __cdecl _mm_mask_lzcnt_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_lzcnt_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_lzcnt_epi64(__m256i);
extern __m256i   __cdecl _mm256_mask_lzcnt_epi64(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_lzcnt_epi64(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_madd_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_madd_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_madd_epi16(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_madd_epi16(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_maddubs_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_maddubs_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_maddubs_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_maddubs_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epi64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_max_epi64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epi64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_max_epi64(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epi8(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epu16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epu16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epu16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epu16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epu32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epu32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epu32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epu32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epu64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epu64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_max_epu64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epu64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epu64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_max_epu64(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_max_epu8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_max_epu8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_max_epu8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_max_epu8(__mmask32, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_max_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_max_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_max_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_max_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_max_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_max_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_max_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_max_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_min_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_min_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_min_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epi64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_min_epi64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epi64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_min_epi64(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_min_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epi8(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_min_epu16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epu16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epu16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epu16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_min_epu32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epu32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epu32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epu32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_min_epu64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epu64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_min_epu64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epu64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epu64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_min_epu64(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_min_epu8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_min_epu8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_min_epu8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_min_epu8(__mmask32, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_min_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_min_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_min_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_min_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_min_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_min_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_min_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_min_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_mov_epi16(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_mov_epi16(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_mov_epi16(__m256i, __mmask16, __m256i);
extern __m256i   __cdecl _mm256_maskz_mov_epi16(__mmask16, __m256i);
extern __m128i   __cdecl _mm_mask_mov_epi32(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_mov_epi32(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_mov_epi32(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_mov_epi32(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_mov_epi64(__m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_mov_epi64(__mmask8, __m128i);
extern __m256i   __cdecl _mm256_mask_mov_epi64(__m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_mov_epi64(__mmask8, __m256i);
extern __m128i   __cdecl _mm_mask_mov_epi8(__m128i, __mmask16, __m128i);
extern __m128i   __cdecl _mm_maskz_mov_epi8(__mmask16, __m128i);
extern __m256i   __cdecl _mm256_mask_mov_epi8(__m256i, __mmask32, __m256i);
extern __m256i   __cdecl _mm256_maskz_mov_epi8(__mmask32, __m256i);
extern __m128d   __cdecl _mm_mask_mov_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_mov_pd(__mmask8, __m128d);
extern __m256d   __cdecl _mm256_mask_mov_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_mov_pd(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_mov_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_mov_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_mov_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_mov_ps(__mmask8, __m256);
extern __m128d   __cdecl _mm_mask_movedup_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_movedup_pd(__mmask8, __m128d);
extern __m256d   __cdecl _mm256_mask_movedup_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_movedup_pd(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_movehdup_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_movehdup_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_movehdup_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_movehdup_ps(__mmask8, __m256);
extern __m128    __cdecl _mm_mask_moveldup_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_moveldup_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_moveldup_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_moveldup_ps(__mmask8, __m256);
extern __mmask8  __cdecl _mm_movepi16_mask(__m128i);
extern __mmask16 __cdecl _mm256_movepi16_mask(__m256i);
extern __mmask8  __cdecl _mm_movepi32_mask(__m128i);
extern __mmask8  __cdecl _mm256_movepi32_mask(__m256i);
extern __mmask8  __cdecl _mm_movepi64_mask(__m128i);
extern __mmask8  __cdecl _mm256_movepi64_mask(__m256i);
extern __mmask16 __cdecl _mm_movepi8_mask(__m128i);
extern __mmask32 __cdecl _mm256_movepi8_mask(__m256i);
extern __m128i   __cdecl _mm_movm_epi16(__mmask8);
extern __m256i   __cdecl _mm256_movm_epi16(__mmask16);
extern __m128i   __cdecl _mm_movm_epi32(__mmask8);
extern __m256i   __cdecl _mm256_movm_epi32(__mmask8);
extern __m128i   __cdecl _mm_movm_epi64(__mmask8);
extern __m256i   __cdecl _mm256_movm_epi64(__mmask8);
extern __m128i   __cdecl _mm_movm_epi8(__mmask16);
extern __m256i   __cdecl _mm256_movm_epi8(__mmask32);
extern __m128i   __cdecl _mm_mask_mul_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mul_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mul_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mul_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_mul_epu32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mul_epu32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mul_epu32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mul_epu32(__mmask8, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_mul_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_mul_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_mul_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_mul_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_mul_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_mul_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_mul_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_mul_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_mulhi_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mulhi_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mulhi_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mulhi_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_mulhi_epu16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mulhi_epu16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mulhi_epu16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mulhi_epu16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_mulhrs_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mulhrs_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mulhrs_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mulhrs_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_mullo_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mullo_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mullo_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mullo_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_mullo_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mullo_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mullo_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mullo_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_mullo_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_mullo_epi64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_mullo_epi64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_mullo_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_mullo_epi64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_mullo_epi64(__m256i, __m256i);
extern __m128i   __cdecl _mm_or_epi32(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_or_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_or_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_or_epi32(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_or_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_or_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_or_epi64(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_or_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_or_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_or_epi64(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_or_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_or_epi64(__mmask8, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_or_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_or_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_or_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_or_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_or_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_or_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_or_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_or_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_packs_epi16(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_packs_epi16(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_packs_epi16(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_packs_epi16(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_packs_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_packs_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_packs_epi32(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_packs_epi32(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_packus_epi16(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_packus_epi16(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_packus_epi16(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_packus_epi16(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_packus_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_packus_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_packus_epi32(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_packus_epi32(__mmask16, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_permute_pd(__m128d, __mmask8, __m128d, const int);
extern __m128d   __cdecl _mm_maskz_permute_pd(__mmask8, __m128d, const int);
extern __m256d   __cdecl _mm256_mask_permute_pd(__m256d, __mmask8, __m256d, const int);
extern __m256d   __cdecl _mm256_maskz_permute_pd(__mmask8, __m256d, const int);
extern __m128    __cdecl _mm_mask_permute_ps(__m128, __mmask8, __m128, const int);
extern __m128    __cdecl _mm_maskz_permute_ps(__mmask8, __m128, const int);
extern __m256    __cdecl _mm256_mask_permute_ps(__m256, __mmask8, __m256, const int);
extern __m256    __cdecl _mm256_maskz_permute_ps(__mmask8, __m256, const int);
extern __m128d   __cdecl _mm_mask_permutevar_pd(__m128d, __mmask8, __m128d, __m128i);
extern __m128d   __cdecl _mm_maskz_permutevar_pd(__mmask8, __m128d, __m128i);
extern __m256d   __cdecl _mm256_mask_permutevar_pd(__m256d, __mmask8, __m256d, __m256i);
extern __m256d   __cdecl _mm256_maskz_permutevar_pd(__mmask8, __m256d, __m256i);
extern __m128    __cdecl _mm_mask_permutevar_ps(__m128, __mmask8, __m128, __m128i);
extern __m128    __cdecl _mm_maskz_permutevar_ps(__mmask8, __m128, __m128i);
extern __m256    __cdecl _mm256_mask_permutevar_ps(__m256, __mmask8, __m256, __m256i);
extern __m256    __cdecl _mm256_maskz_permutevar_ps(__mmask8, __m256, __m256i);
extern __m256i   __cdecl _mm256_mask_permutex_epi64(__m256i, __mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_permutex_epi64(__mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_permutex_epi64(__m256i, const int);
extern __m256d   __cdecl _mm256_mask_permutex_pd(__m256d, __mmask8, __m256d, const int);
extern __m256d   __cdecl _mm256_maskz_permutex_pd(__mmask8, __m256d, const int);
extern __m256d   __cdecl _mm256_permutex_pd(__m256d, const int);
extern __m128i   __cdecl _mm_mask_permutex2var_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_mask2_permutex2var_epi16(__m128i, __m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_permutex2var_epi16(__mmask8, __m128i, __m128i, __m128i);
extern __m128i   __cdecl _mm_permutex2var_epi16(__m128i, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_permutex2var_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_mask2_permutex2var_epi16(__m256i, __m256i, __mmask16, __m256i);
extern __m256i   __cdecl _mm256_maskz_permutex2var_epi16(__mmask16, __m256i, __m256i, __m256i);
extern __m256i   __cdecl _mm256_permutex2var_epi16(__m256i, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_permutex2var_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_mask2_permutex2var_epi32(__m128i, __m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_permutex2var_epi32(__mmask8, __m128i, __m128i, __m128i);
extern __m128i   __cdecl _mm_permutex2var_epi32(__m128i, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_permutex2var_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_mask2_permutex2var_epi32(__m256i, __m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_permutex2var_epi32(__mmask8, __m256i, __m256i, __m256i);
extern __m256i   __cdecl _mm256_permutex2var_epi32(__m256i, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_permutex2var_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_mask2_permutex2var_epi64(__m128i, __m128i, __mmask8, __m128i);
extern __m128i   __cdecl _mm_maskz_permutex2var_epi64(__mmask8, __m128i, __m128i, __m128i);
extern __m128i   __cdecl _mm_permutex2var_epi64(__m128i, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_permutex2var_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_mask2_permutex2var_epi64(__m256i, __m256i, __mmask8, __m256i);
extern __m256i   __cdecl _mm256_maskz_permutex2var_epi64(__mmask8, __m256i, __m256i, __m256i);
extern __m256i   __cdecl _mm256_permutex2var_epi64(__m256i, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_permutex2var_pd(__m128d, __mmask8, __m128i, __m128d);
extern __m128d   __cdecl _mm_mask2_permutex2var_pd(__m128d, __m128i, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_permutex2var_pd(__mmask8, __m128d, __m128i, __m128d);
extern __m128d   __cdecl _mm_permutex2var_pd(__m128d, __m128i, __m128d);
extern __m256d   __cdecl _mm256_mask_permutex2var_pd(__m256d, __mmask8, __m256i, __m256d);
extern __m256d   __cdecl _mm256_mask2_permutex2var_pd(__m256d, __m256i, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_permutex2var_pd(__mmask8, __m256d, __m256i, __m256d);
extern __m256d   __cdecl _mm256_permutex2var_pd(__m256d, __m256i, __m256d);
extern __m128    __cdecl _mm_mask_permutex2var_ps(__m128, __mmask8, __m128i, __m128);
extern __m128    __cdecl _mm_mask2_permutex2var_ps(__m128, __m128i, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_permutex2var_ps(__mmask8, __m128, __m128i, __m128);
extern __m128    __cdecl _mm_permutex2var_ps(__m128, __m128i, __m128);
extern __m256    __cdecl _mm256_mask_permutex2var_ps(__m256, __mmask8, __m256i, __m256);
extern __m256    __cdecl _mm256_mask2_permutex2var_ps(__m256, __m256i, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_permutex2var_ps(__mmask8, __m256, __m256i, __m256);
extern __m256    __cdecl _mm256_permutex2var_ps(__m256, __m256i, __m256);
extern __m128i   __cdecl _mm_mask_permutexvar_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_permutexvar_epi16(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_permutexvar_epi16(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_permutexvar_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_permutexvar_epi16(__mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_permutexvar_epi16(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_permutexvar_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_permutexvar_epi32(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_permutexvar_epi32(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_permutexvar_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_permutexvar_epi64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_permutexvar_epi64(__m256i, __m256i);
extern __m256d   __cdecl _mm256_mask_permutexvar_pd(__m256d, __mmask8, __m256i, __m256d);
extern __m256d   __cdecl _mm256_maskz_permutexvar_pd(__mmask8, __m256i, __m256d);
extern __m256d   __cdecl _mm256_permutexvar_pd(__m256i, __m256d);
extern __m256    __cdecl _mm256_mask_permutexvar_ps(__m256, __mmask8, __m256i, __m256);
extern __m256    __cdecl _mm256_maskz_permutexvar_ps(__mmask8, __m256i, __m256);
extern __m256    __cdecl _mm256_permutexvar_ps(__m256i, __m256);
extern __m128d   __cdecl _mm_mask_range_pd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_range_pd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_range_pd(__m128d, __m128d, int);
extern __m256d   __cdecl _mm256_mask_range_pd(__m256d, __mmask8, __m256d, __m256d, int);
extern __m256d   __cdecl _mm256_maskz_range_pd(__mmask8, __m256d, __m256d, int);
extern __m256d   __cdecl _mm256_range_pd(__m256d, __m256d, int);
extern __m128    __cdecl _mm_mask_range_ps(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_range_ps(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_range_ps(__m128, __m128, int);
extern __m256    __cdecl _mm256_mask_range_ps(__m256, __mmask8, __m256, __m256, int);
extern __m256    __cdecl _mm256_maskz_range_ps(__mmask8, __m256, __m256, int);
extern __m256    __cdecl _mm256_range_ps(__m256, __m256, int);
extern __m128d   __cdecl _mm_mask_rcp14_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_rcp14_pd(__mmask8, __m128d);
extern __m128d   __cdecl _mm_rcp14_pd(__m128d);
extern __m256d   __cdecl _mm256_mask_rcp14_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_rcp14_pd(__mmask8, __m256d);
extern __m256d   __cdecl _mm256_rcp14_pd(__m256d);
extern __m128    __cdecl _mm_mask_rcp14_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_rcp14_ps(__mmask8, __m128);
extern __m128    __cdecl _mm_rcp14_ps(__m128);
extern __m256    __cdecl _mm256_mask_rcp14_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_rcp14_ps(__mmask8, __m256);
extern __m256    __cdecl _mm256_rcp14_ps(__m256);
extern __m128d   __cdecl _mm_mask_reduce_pd(__m128d, __mmask8, __m128d, int);
extern __m128d   __cdecl _mm_maskz_reduce_pd(__mmask8, __m128d, int);
extern __m128d   __cdecl _mm_reduce_pd(__m128d, int);
extern __m256d   __cdecl _mm256_mask_reduce_pd(__m256d, __mmask8, __m256d, int);
extern __m256d   __cdecl _mm256_maskz_reduce_pd(__mmask8, __m256d, int);
extern __m256d   __cdecl _mm256_reduce_pd(__m256d, int);
extern __m128    __cdecl _mm_mask_reduce_ps(__m128, __mmask8, __m128, int);
extern __m128    __cdecl _mm_maskz_reduce_ps(__mmask8, __m128, int);
extern __m128    __cdecl _mm_reduce_ps(__m128, int);
extern __m256    __cdecl _mm256_mask_reduce_ps(__m256, __mmask8, __m256, int);
extern __m256    __cdecl _mm256_maskz_reduce_ps(__mmask8, __m256, int);
extern __m256    __cdecl _mm256_reduce_ps(__m256, int);
extern __m128i   __cdecl _mm_mask_rol_epi32(__m128i, __mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_maskz_rol_epi32(__mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_rol_epi32(__m128i, int);
extern __m256i   __cdecl _mm256_mask_rol_epi32(__m256i, __mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_rol_epi32(__mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_rol_epi32(__m256i, const int);
extern __m128i   __cdecl _mm_mask_rol_epi64(__m128i, __mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_maskz_rol_epi64(__mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_rol_epi64(__m128i, const int);
extern __m256i   __cdecl _mm256_mask_rol_epi64(__m256i, __mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_rol_epi64(__mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_rol_epi64(__m256i, const int);
extern __m128i   __cdecl _mm_mask_rolv_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_rolv_epi32(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_rolv_epi32(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_rolv_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_rolv_epi32(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_rolv_epi32(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_rolv_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_rolv_epi64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_rolv_epi64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_rolv_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_rolv_epi64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_rolv_epi64(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_ror_epi32(__m128i, __mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_maskz_ror_epi32(__mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_ror_epi32(__m128i, const int);
extern __m256i   __cdecl _mm256_mask_ror_epi32(__m256i, __mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_ror_epi32(__mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_ror_epi32(__m256i, const int);
extern __m128i   __cdecl _mm_mask_ror_epi64(__m128i, __mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_maskz_ror_epi64(__mmask8, __m128i, const int);
extern __m128i   __cdecl _mm_ror_epi64(__m128i, const int);
extern __m256i   __cdecl _mm256_mask_ror_epi64(__m256i, __mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_ror_epi64(__mmask8, __m256i, const int);
extern __m256i   __cdecl _mm256_ror_epi64(__m256i, const int);
extern __m128i   __cdecl _mm_mask_rorv_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_rorv_epi32(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_rorv_epi32(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_rorv_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_rorv_epi32(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_rorv_epi32(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_rorv_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_rorv_epi64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_rorv_epi64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_rorv_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_rorv_epi64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_rorv_epi64(__m256i, __m256i);
extern __m128d   __cdecl _mm_mask_roundscale_pd(__m128d, __mmask8, __m128d, int);
extern __m128d   __cdecl _mm_maskz_roundscale_pd(__mmask8, __m128d, int);
extern __m128d   __cdecl _mm_roundscale_pd(__m128d, int);
extern __m256d   __cdecl _mm256_mask_roundscale_pd(__m256d, __mmask8, __m256d, int);
extern __m256d   __cdecl _mm256_maskz_roundscale_pd(__mmask8, __m256d, int);
extern __m256d   __cdecl _mm256_roundscale_pd(__m256d, int);
extern __m128    __cdecl _mm_mask_roundscale_ps(__m128, __mmask8, __m128, int);
extern __m128    __cdecl _mm_maskz_roundscale_ps(__mmask8, __m128, int);
extern __m128    __cdecl _mm_roundscale_ps(__m128, int);
extern __m256    __cdecl _mm256_mask_roundscale_ps(__m256, __mmask8, __m256, int);
extern __m256    __cdecl _mm256_maskz_roundscale_ps(__mmask8, __m256, int);
extern __m256    __cdecl _mm256_roundscale_ps(__m256, int);
extern __m128d   __cdecl _mm_mask_rsqrt14_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_rsqrt14_pd(__mmask8, __m128d);
extern __m256d   __cdecl _mm256_mask_rsqrt14_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_rsqrt14_pd(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_rsqrt14_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_rsqrt14_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_rsqrt14_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_rsqrt14_ps(__mmask8, __m256);
extern __m128d   __cdecl _mm_mask_scalef_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_scalef_pd(__mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_scalef_pd(__m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_scalef_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_scalef_pd(__mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_scalef_pd(__m256d, __m256d);
extern __m128    __cdecl _mm_mask_scalef_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_scalef_ps(__mmask8, __m128, __m128);
extern __m128    __cdecl _mm_scalef_ps(__m128, __m128);
extern __m256    __cdecl _mm256_mask_scalef_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_scalef_ps(__mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_scalef_ps(__m256, __m256);
extern __m128i   __cdecl _mm_mask_set1_epi16(__m128i, __mmask8, short);
extern __m128i   __cdecl _mm_maskz_set1_epi16(__mmask8, short);
extern __m256i   __cdecl _mm256_mask_set1_epi16(__m256i, __mmask16, short);
extern __m256i   __cdecl _mm256_maskz_set1_epi16(__mmask16, short);
extern __m128i   __cdecl _mm_mask_set1_epi32(__m128i, __mmask8, int);
extern __m128i   __cdecl _mm_maskz_set1_epi32(__mmask8, int);
extern __m256i   __cdecl _mm256_mask_set1_epi32(__m256i, __mmask8, int);
extern __m256i   __cdecl _mm256_maskz_set1_epi32(__mmask8, int);
extern __m128i   __cdecl _mm_mask_set1_epi64(__m128i, __mmask8, __int64);
extern __m128i   __cdecl _mm_maskz_set1_epi64(__mmask8, __int64);
extern __m256i   __cdecl _mm256_mask_set1_epi64(__m256i, __mmask8, __int64);
extern __m256i   __cdecl _mm256_maskz_set1_epi64(__mmask8, __int64);
extern __m128i   __cdecl _mm_mask_set1_epi8(__m128i, __mmask16, char);
extern __m128i   __cdecl _mm_maskz_set1_epi8(__mmask16, char);
extern __m256i   __cdecl _mm256_mask_set1_epi8(__m256i, __mmask32, char);
extern __m256i   __cdecl _mm256_maskz_set1_epi8(__mmask32, char);
extern __m128i   __cdecl _mm_mask_shuffle_epi32(__m128i, __mmask8, __m128i, int);
extern __m128i   __cdecl _mm_maskz_shuffle_epi32(__mmask8, __m128i, int);
extern __m256i   __cdecl _mm256_mask_shuffle_epi32(__m256i, __mmask8, __m256i, int);
extern __m256i   __cdecl _mm256_maskz_shuffle_epi32(__mmask8, __m256i, int);
extern __m128i   __cdecl _mm_mask_shuffle_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_shuffle_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_shuffle_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_shuffle_epi8(__mmask32, __m256i, __m256i);
extern __m256    __cdecl _mm256_mask_shuffle_f32x4(__m256, __mmask8, __m256, __m256, const int);
extern __m256    __cdecl _mm256_maskz_shuffle_f32x4(__mmask8, __m256, __m256, const int);
extern __m256    __cdecl _mm256_shuffle_f32x4(__m256, __m256, const int);
extern __m256d   __cdecl _mm256_mask_shuffle_f64x2(__m256d, __mmask8, __m256d, __m256d, const int);
extern __m256d   __cdecl _mm256_maskz_shuffle_f64x2(__mmask8, __m256d, __m256d, const int);
extern __m256d   __cdecl _mm256_shuffle_f64x2(__m256d, __m256d, const int);
extern __m256i   __cdecl _mm256_mask_shuffle_i32x4(__m256i, __mmask8, __m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_shuffle_i32x4(__mmask8, __m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_shuffle_i32x4(__m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_mask_shuffle_i64x2(__m256i, __mmask8, __m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_maskz_shuffle_i64x2(__mmask8, __m256i, __m256i, const int);
extern __m256i   __cdecl _mm256_shuffle_i64x2(__m256i, __m256i, const int);
extern __m128d   __cdecl _mm_mask_shuffle_pd(__m128d, __mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_maskz_shuffle_pd(__mmask8, __m128d, __m128d, const int);
extern __m256d   __cdecl _mm256_mask_shuffle_pd(__m256d, __mmask8, __m256d, __m256d, const int);
extern __m256d   __cdecl _mm256_maskz_shuffle_pd(__mmask8, __m256d, __m256d, const int);
extern __m128    __cdecl _mm_mask_shuffle_ps(__m128, __mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_maskz_shuffle_ps(__mmask8, __m128, __m128, const int);
extern __m256    __cdecl _mm256_mask_shuffle_ps(__m256, __mmask8, __m256, __m256, const int);
extern __m256    __cdecl _mm256_maskz_shuffle_ps(__mmask8, __m256, __m256, const int);
extern __m128i   __cdecl _mm_mask_shufflehi_epi16(__m128i, __mmask8, __m128i, int);
extern __m128i   __cdecl _mm_maskz_shufflehi_epi16(__mmask8, __m128i, int);
extern __m256i   __cdecl _mm256_mask_shufflehi_epi16(__m256i, __mmask16, __m256i, int);
extern __m256i   __cdecl _mm256_maskz_shufflehi_epi16(__mmask16, __m256i, int);
extern __m128i   __cdecl _mm_mask_shufflelo_epi16(__m128i, __mmask8, __m128i, int);
extern __m128i   __cdecl _mm_maskz_shufflelo_epi16(__mmask8, __m128i, int);
extern __m256i   __cdecl _mm256_mask_shufflelo_epi16(__m256i, __mmask16, __m256i, int);
extern __m256i   __cdecl _mm256_maskz_shufflelo_epi16(__mmask16, __m256i, int);
extern __m128i   __cdecl _mm_mask_sll_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sll_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sll_epi16(__m256i, __mmask16, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_sll_epi16(__mmask16, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_sll_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sll_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sll_epi32(__m256i, __mmask8, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_sll_epi32(__mmask8, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_sll_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sll_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sll_epi64(__m256i, __mmask8, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_sll_epi64(__mmask8, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_slli_epi16(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_slli_epi16(__mmask8, __m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_slli_epi16(__m256i, __mmask16, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_slli_epi16(__mmask16, __m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_slli_epi32(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_slli_epi32(__mmask8, __m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_slli_epi32(__m256i, __mmask8, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_slli_epi32(__mmask8, __m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_slli_epi64(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_slli_epi64(__mmask8, __m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_slli_epi64(__m256i, __mmask8, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_slli_epi64(__mmask8, __m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_sllv_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sllv_epi16(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_sllv_epi16(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sllv_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_sllv_epi16(__mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_sllv_epi16(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_sllv_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sllv_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sllv_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_sllv_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_sllv_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sllv_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sllv_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_sllv_epi64(__mmask8, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_sqrt_pd(__m128d, __mmask8, __m128d);
extern __m128d   __cdecl _mm_maskz_sqrt_pd(__mmask8, __m128d);
extern __m256d   __cdecl _mm256_mask_sqrt_pd(__m256d, __mmask8, __m256d);
extern __m256d   __cdecl _mm256_maskz_sqrt_pd(__mmask8, __m256d);
extern __m128    __cdecl _mm_mask_sqrt_ps(__m128, __mmask8, __m128);
extern __m128    __cdecl _mm_maskz_sqrt_ps(__mmask8, __m128);
extern __m256    __cdecl _mm256_mask_sqrt_ps(__m256, __mmask8, __m256);
extern __m256    __cdecl _mm256_maskz_sqrt_ps(__mmask8, __m256);
extern __m128i   __cdecl _mm_mask_sra_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sra_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sra_epi16(__m256i, __mmask16, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_sra_epi16(__mmask16, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_sra_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sra_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sra_epi32(__m256i, __mmask8, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_sra_epi32(__mmask8, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_sra_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sra_epi64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_sra_epi64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sra_epi64(__m256i, __mmask8, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_sra_epi64(__mmask8, __m256i, __m128i);
extern __m256i   __cdecl _mm256_sra_epi64(__m256i, __m128i);
extern __m128i   __cdecl _mm_mask_srai_epi16(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_srai_epi16(__mmask8, __m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_srai_epi16(__m256i, __mmask16, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_srai_epi16(__mmask16, __m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_srai_epi32(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_srai_epi32(__mmask8, __m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_srai_epi32(__m256i, __mmask8, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_srai_epi32(__mmask8, __m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_srai_epi64(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_srai_epi64(__mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_srai_epi64(__m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_srai_epi64(__m256i, __mmask8, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_srai_epi64(__mmask8, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_srai_epi64(__m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_srav_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srav_epi16(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_srav_epi16(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srav_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_srav_epi16(__mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_srav_epi16(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_srav_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srav_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srav_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_srav_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_srav_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srav_epi64(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_srav_epi64(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srav_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_srav_epi64(__mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_srav_epi64(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_srl_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srl_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srl_epi16(__m256i, __mmask16, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_srl_epi16(__mmask16, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_srl_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srl_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srl_epi32(__m256i, __mmask8, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_srl_epi32(__mmask8, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_srl_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srl_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srl_epi64(__m256i, __mmask8, __m256i, __m128i);
extern __m256i   __cdecl _mm256_maskz_srl_epi64(__mmask8, __m256i, __m128i);
extern __m128i   __cdecl _mm_mask_srli_epi16(__m128i, __mmask8, __m128i, int);
extern __m128i   __cdecl _mm_maskz_srli_epi16(__mmask8, __m128i, int);
extern __m256i   __cdecl _mm256_mask_srli_epi16(__m256i, __mmask16, __m256i, int);
extern __m256i   __cdecl _mm256_maskz_srli_epi16(__mmask16, __m256i, int);
extern __m128i   __cdecl _mm_mask_srli_epi32(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_srli_epi32(__mmask8, __m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_srli_epi32(__m256i, __mmask8, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_srli_epi32(__mmask8, __m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_srli_epi64(__m128i, __mmask8, __m128i, unsigned int);
extern __m128i   __cdecl _mm_maskz_srli_epi64(__mmask8, __m128i, unsigned int);
extern __m256i   __cdecl _mm256_mask_srli_epi64(__m256i, __mmask8, __m256i, unsigned int);
extern __m256i   __cdecl _mm256_maskz_srli_epi64(__mmask8, __m256i, unsigned int);
extern __m128i   __cdecl _mm_mask_srlv_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srlv_epi16(__mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_srlv_epi16(__m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srlv_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_srlv_epi16(__mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_srlv_epi16(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_srlv_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srlv_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srlv_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_srlv_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_srlv_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_srlv_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_srlv_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_srlv_epi64(__mmask8, __m256i, __m256i);
extern void      __cdecl _mm_mask_store_epi32(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_store_epi32(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_store_epi64(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_store_epi64(void*, __mmask8, __m256i);
extern void      __cdecl _mm_mask_store_pd(void*, __mmask8, __m128d);
extern void      __cdecl _mm256_mask_store_pd(void*, __mmask8, __m256d);
extern void      __cdecl _mm_mask_store_ps(void*, __mmask8, __m128);
extern void      __cdecl _mm256_mask_store_ps(void*, __mmask8, __m256);
extern void      __cdecl _mm_storeu_epi16(void*, __m128i);
extern void      __cdecl _mm256_storeu_epi16(void*, __m256i);
extern void      __cdecl _mm_mask_storeu_epi16(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_storeu_epi16(void*, __mmask16, __m256i);
extern void      __cdecl _mm_storeu_epi32(void*, __m128i);
extern void      __cdecl _mm256_storeu_epi32(void*, __m256i);
extern void      __cdecl _mm_mask_storeu_epi32(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_storeu_epi32(void*, __mmask8, __m256i);
extern void      __cdecl _mm_storeu_epi64(void*, __m128i);
extern void      __cdecl _mm256_storeu_epi64(void*, __m256i);
extern void      __cdecl _mm_mask_storeu_epi64(void*, __mmask8, __m128i);
extern void      __cdecl _mm256_mask_storeu_epi64(void*, __mmask8, __m256i);
extern void      __cdecl _mm_storeu_epi8(void*, __m128i);
extern void      __cdecl _mm256_storeu_epi8(void*, __m256i);
extern void      __cdecl _mm_mask_storeu_epi8(void*, __mmask16, __m128i);
extern void      __cdecl _mm256_mask_storeu_epi8(void*, __mmask32, __m256i);
extern void      __cdecl _mm_mask_storeu_pd(void*, __mmask8, __m128d);
extern void      __cdecl _mm256_mask_storeu_pd(void*, __mmask8, __m256d);
extern void      __cdecl _mm_mask_storeu_ps(void*, __mmask8, __m128);
extern void      __cdecl _mm256_mask_storeu_ps(void*, __mmask8, __m256);
extern __m128i   __cdecl _mm_mask_sub_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sub_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sub_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_sub_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_sub_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sub_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sub_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_sub_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_sub_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sub_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sub_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_sub_epi64(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_sub_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_sub_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_sub_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_sub_epi8(__mmask32, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_sub_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_sub_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_sub_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_sub_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_sub_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_sub_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_sub_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_sub_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_subs_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_subs_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_subs_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_subs_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_subs_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_subs_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_subs_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_subs_epi8(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_subs_epu16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_subs_epu16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_subs_epu16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_subs_epu16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_subs_epu8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_subs_epu8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_subs_epu8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_subs_epu8(__mmask32, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_ternarylogic_epi32(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i   __cdecl _mm_maskz_ternarylogic_epi32(__mmask8, __m128i, __m128i, __m128i, int);
extern __m128i   __cdecl _mm_ternarylogic_epi32(__m128i, __m128i, __m128i, int);
extern __m256i   __cdecl _mm256_mask_ternarylogic_epi32(__m256i, __mmask8, __m256i, __m256i, int);
extern __m256i   __cdecl _mm256_maskz_ternarylogic_epi32(__mmask8, __m256i, __m256i, __m256i, int);
extern __m256i   __cdecl _mm256_ternarylogic_epi32(__m256i, __m256i, __m256i, int);
extern __m128i   __cdecl _mm_mask_ternarylogic_epi64(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i   __cdecl _mm_maskz_ternarylogic_epi64(__mmask8, __m128i, __m128i, __m128i, int);
extern __m128i   __cdecl _mm_ternarylogic_epi64(__m128i, __m128i, __m128i, int);
extern __m256i   __cdecl _mm256_mask_ternarylogic_epi64(__m256i, __mmask8, __m256i, __m256i, int);
extern __m256i   __cdecl _mm256_maskz_ternarylogic_epi64(__mmask8, __m256i, __m256i, __m256i, int);
extern __m256i   __cdecl _mm256_ternarylogic_epi64(__m256i, __m256i, __m256i, int);
extern __mmask8  __cdecl _mm_mask_test_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_test_epi16_mask(__m128i, __m128i);
extern __mmask16 __cdecl _mm256_mask_test_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16 __cdecl _mm256_test_epi16_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm_mask_test_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_test_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm256_mask_test_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_test_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm_mask_test_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_test_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm256_mask_test_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_test_epi64_mask(__m256i, __m256i);
extern __mmask16 __cdecl _mm_mask_test_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16 __cdecl _mm_test_epi8_mask(__m128i, __m128i);
extern __mmask32 __cdecl _mm256_mask_test_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32 __cdecl _mm256_test_epi8_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm_mask_testn_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_testn_epi16_mask(__m128i, __m128i);
extern __mmask16 __cdecl _mm256_mask_testn_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16 __cdecl _mm256_testn_epi16_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm_mask_testn_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_testn_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm256_mask_testn_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_testn_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm_mask_testn_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_testn_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm256_mask_testn_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_testn_epi64_mask(__m256i, __m256i);
extern __mmask16 __cdecl _mm_mask_testn_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16 __cdecl _mm_testn_epi8_mask(__m128i, __m128i);
extern __mmask32 __cdecl _mm256_mask_testn_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32 __cdecl _mm256_testn_epi8_mask(__m256i, __m256i);
extern __m128i   __cdecl _mm_mask_unpackhi_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpackhi_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpackhi_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpackhi_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_unpackhi_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpackhi_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpackhi_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpackhi_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_unpackhi_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpackhi_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpackhi_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpackhi_epi64(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_unpackhi_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpackhi_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpackhi_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpackhi_epi8(__mmask32, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_unpackhi_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_unpackhi_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_unpackhi_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_unpackhi_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_unpackhi_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_unpackhi_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_unpackhi_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_unpackhi_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_mask_unpacklo_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpacklo_epi16(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpacklo_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpacklo_epi16(__mmask16, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_unpacklo_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpacklo_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpacklo_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpacklo_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_unpacklo_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpacklo_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpacklo_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpacklo_epi64(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_mask_unpacklo_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_unpacklo_epi8(__mmask16, __m128i, __m128i);
extern __m256i   __cdecl _mm256_mask_unpacklo_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_unpacklo_epi8(__mmask32, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_unpacklo_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_unpacklo_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_unpacklo_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_unpacklo_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_unpacklo_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_unpacklo_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_unpacklo_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_unpacklo_ps(__mmask8, __m256, __m256);
extern __m128i   __cdecl _mm_xor_epi32(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_xor_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_xor_epi32(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_xor_epi32(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_xor_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_xor_epi32(__mmask8, __m256i, __m256i);
extern __m128i   __cdecl _mm_xor_epi64(__m128i, __m128i);
extern __m128i   __cdecl _mm_mask_xor_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i   __cdecl _mm_maskz_xor_epi64(__mmask8, __m128i, __m128i);
extern __m256i   __cdecl _mm256_xor_epi64(__m256i, __m256i);
extern __m256i   __cdecl _mm256_mask_xor_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i   __cdecl _mm256_maskz_xor_epi64(__mmask8, __m256i, __m256i);
extern __m128d   __cdecl _mm_mask_xor_pd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_xor_pd(__mmask8, __m128d, __m128d);
extern __m256d   __cdecl _mm256_mask_xor_pd(__m256d, __mmask8, __m256d, __m256d);
extern __m256d   __cdecl _mm256_maskz_xor_pd(__mmask8, __m256d, __m256d);
extern __m128    __cdecl _mm_mask_xor_ps(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_xor_ps(__mmask8, __m128, __m128);
extern __m256    __cdecl _mm256_mask_xor_ps(__m256, __mmask8, __m256, __m256);
extern __m256    __cdecl _mm256_maskz_xor_ps(__mmask8, __m256, __m256);

extern __mmask16  __cdecl _mm_cmpeq_epi8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmpge_epi8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmpgt_epi8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmple_epi8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmplt_epi8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmpneq_epi8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmpeq_epu8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmpge_epu8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmpgt_epu8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmple_epu8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmplt_epu8_mask(__m128i, __m128i);
extern __mmask16  __cdecl _mm_cmpneq_epu8_mask(__m128i, __m128i);

extern __mmask16  __cdecl _mm_mask_cmpeq_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmpge_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmpgt_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmple_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmplt_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmpneq_epi8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmpeq_epu8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmpge_epu8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmpgt_epu8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmple_epu8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmplt_epu8_mask(__mmask16, __m128i, __m128i);
extern __mmask16  __cdecl _mm_mask_cmpneq_epu8_mask(__mmask16, __m128i, __m128i);

extern __mmask8  __cdecl _mm_cmpeq_epi16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpge_epi16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpgt_epi16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmple_epi16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmplt_epi16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpneq_epi16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpeq_epu16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpge_epu16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpgt_epu16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmple_epu16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmplt_epu16_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpneq_epu16_mask(__m128i, __m128i);

extern __mmask8  __cdecl _mm_mask_cmpeq_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpge_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpgt_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmple_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmplt_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpneq_epi16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpeq_epu16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpge_epu16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpgt_epu16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmple_epu16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmplt_epu16_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpneq_epu16_mask(__mmask8, __m128i, __m128i);

extern __mmask8  __cdecl _mm_cmpeq_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpge_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpgt_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmple_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmplt_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpneq_epi32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpeq_epu32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpge_epu32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpgt_epu32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmple_epu32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmplt_epu32_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpneq_epu32_mask(__m128i, __m128i);

extern __mmask8  __cdecl _mm_mask_cmpeq_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpge_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpgt_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmple_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmplt_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpneq_epi32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpeq_epu32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpge_epu32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpgt_epu32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmple_epu32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmplt_epu32_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpneq_epu32_mask(__mmask8, __m128i, __m128i);

extern __mmask8  __cdecl _mm_cmpeq_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpge_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpgt_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmple_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmplt_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpneq_epi64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpeq_epu64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpge_epu64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpgt_epu64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmple_epu64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmplt_epu64_mask(__m128i, __m128i);
extern __mmask8  __cdecl _mm_cmpneq_epu64_mask(__m128i, __m128i);

extern __mmask8  __cdecl _mm_mask_cmpeq_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpge_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpgt_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmple_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmplt_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpneq_epi64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpeq_epu64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpge_epu64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpgt_epu64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmple_epu64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmplt_epu64_mask(__mmask8, __m128i, __m128i);
extern __mmask8  __cdecl _mm_mask_cmpneq_epu64_mask(__mmask8, __m128i, __m128i);

extern __mmask32  __cdecl _mm256_cmpeq_epi8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmpge_epi8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmpgt_epi8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmple_epi8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmplt_epi8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmpneq_epi8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmpeq_epu8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmpge_epu8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmpgt_epu8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmple_epu8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmplt_epu8_mask(__m256i, __m256i);
extern __mmask32  __cdecl _mm256_cmpneq_epu8_mask(__m256i, __m256i);

extern __mmask32  __cdecl _mm256_mask_cmpeq_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmpge_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmpgt_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmple_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmplt_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmpneq_epi8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmpeq_epu8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmpge_epu8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmpgt_epu8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmple_epu8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmplt_epu8_mask(__mmask32, __m256i, __m256i);
extern __mmask32  __cdecl _mm256_mask_cmpneq_epu8_mask(__mmask32, __m256i, __m256i);

extern __mmask16  __cdecl _mm256_cmpeq_epi16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmpge_epi16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmpgt_epi16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmple_epi16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmplt_epi16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmpneq_epi16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmpeq_epu16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmpge_epu16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmpgt_epu16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmple_epu16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmplt_epu16_mask(__m256i, __m256i);
extern __mmask16  __cdecl _mm256_cmpneq_epu16_mask(__m256i, __m256i);

extern __mmask16  __cdecl _mm256_mask_cmpeq_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmpge_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmpgt_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmple_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmplt_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmpneq_epi16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmpeq_epu16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmpge_epu16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmpgt_epu16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmple_epu16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmplt_epu16_mask(__mmask16, __m256i, __m256i);
extern __mmask16  __cdecl _mm256_mask_cmpneq_epu16_mask(__mmask16, __m256i, __m256i);

extern __mmask8  __cdecl _mm256_cmpeq_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpge_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpgt_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmple_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmplt_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpneq_epi32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpeq_epu32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpge_epu32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpgt_epu32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmple_epu32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmplt_epu32_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpneq_epu32_mask(__m256i, __m256i);

extern __mmask8  __cdecl _mm256_mask_cmpeq_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpge_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpgt_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmple_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmplt_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpneq_epi32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpeq_epu32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpge_epu32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpgt_epu32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmple_epu32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmplt_epu32_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpneq_epu32_mask(__mmask8, __m256i, __m256i);

extern __mmask8  __cdecl _mm256_cmpeq_epi64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpge_epi64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpgt_epi64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmple_epi64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmplt_epi64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpneq_epi64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpeq_epu64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpge_epu64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpgt_epu64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmple_epu64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmplt_epu64_mask(__m256i, __m256i);
extern __mmask8  __cdecl _mm256_cmpneq_epu64_mask(__m256i, __m256i);

extern __mmask8  __cdecl _mm256_mask_cmpeq_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpge_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpgt_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmple_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmplt_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpneq_epi64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpeq_epu64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpge_epu64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpgt_epu64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmple_epu64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmplt_epu64_mask(__mmask8, __m256i, __m256i);
extern __mmask8  __cdecl _mm256_mask_cmpneq_epu64_mask(__mmask8, __m256i, __m256i);

// AVX-512 scalar functions
extern __m128d   __cdecl _mm_add_round_sd(__m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask_add_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_add_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128    __cdecl _mm_add_round_ss(__m128, __m128, int);
extern __m128    __cdecl _mm_mask_add_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_add_round_ss(__mmask8, __m128, __m128, int);
extern __m128d   __cdecl _mm_mask_add_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_add_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_add_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_add_ss(__mmask8, __m128, __m128);
extern __mmask8  __cdecl _mm_cmp_round_sd_mask(__m128d, __m128d, const int, const int);
extern __mmask8  __cdecl _mm_mask_cmp_round_sd_mask(__mmask8, __m128d, __m128d, const int, const int);
extern __mmask8  __cdecl _mm_cmp_round_ss_mask(__m128, __m128, const int, const int);
extern __mmask8  __cdecl _mm_mask_cmp_round_ss_mask(__mmask8, __m128, __m128, const int, const int);
extern __mmask8  __cdecl _mm_cmp_sd_mask(__m128d, __m128d, const int);
extern __mmask8  __cdecl _mm_mask_cmp_sd_mask(__mmask8, __m128d, __m128d, const int);
extern __mmask8  __cdecl _mm_cmp_ss_mask(__m128, __m128, const int);
extern __mmask8  __cdecl _mm_mask_cmp_ss_mask(__mmask8, __m128, __m128, const int);
extern int       __cdecl _mm_comi_round_sd(__m128d, __m128d, const int, const int);
extern int       __cdecl _mm_comi_round_ss(__m128, __m128, const int, const int);
extern __m128    __cdecl _mm_cvt_roundi32_ss(__m128, int, int);
extern int       __cdecl _mm_cvt_roundsd_i32(__m128d, int);
extern int       __cdecl _mm_cvt_roundsd_si32(__m128d, int);
extern __m128    __cdecl _mm_cvt_roundsd_ss(__m128, __m128d, int);
extern __m128    __cdecl _mm_mask_cvt_roundsd_ss(__m128, __mmask8, __m128, __m128d, int);
extern __m128    __cdecl _mm_maskz_cvt_roundsd_ss(__mmask8, __m128, __m128d, int);
extern unsigned int __cdecl _mm_cvt_roundsd_u32(__m128d, int);
extern __m128    __cdecl _mm_cvt_roundsi32_ss(__m128, int, int);
extern int       __cdecl _mm_cvt_roundss_i32(__m128, int);
extern __m128d   __cdecl _mm_cvt_roundss_sd(__m128d, __m128, int);
extern __m128d   __cdecl _mm_mask_cvt_roundss_sd(__m128d, __mmask8, __m128d, __m128, int);
extern __m128d   __cdecl _mm_maskz_cvt_roundss_sd(__mmask8, __m128d, __m128, int);
extern int       __cdecl _mm_cvt_roundss_si32(__m128, int);
extern unsigned int __cdecl _mm_cvt_roundss_u32(__m128, int);
extern __m128    __cdecl _mm_cvt_roundu32_ss(__m128, unsigned int, int);
extern __m128d   __cdecl _mm_cvti32_sd(__m128d, int);
extern __m128    __cdecl _mm_cvti32_ss(__m128, int);
extern int       __cdecl _mm_cvtsd_i32(__m128d);
extern __m128    __cdecl _mm_mask_cvtsd_ss(__m128, __mmask8, __m128, __m128d);
extern __m128    __cdecl _mm_maskz_cvtsd_ss(__mmask8, __m128, __m128d);
extern unsigned int __cdecl _mm_cvtsd_u32(__m128d);
extern int       __cdecl _mm_cvtss_i32(__m128);
extern __m128d   __cdecl _mm_mask_cvtss_sd(__m128d, __mmask8, __m128d, __m128);
extern __m128d   __cdecl _mm_maskz_cvtss_sd(__mmask8, __m128d, __m128);
extern unsigned int __cdecl _mm_cvtss_u32(__m128);
extern int       __cdecl _mm_cvtt_roundsd_i32(__m128d, int);
extern int       __cdecl _mm_cvtt_roundsd_si32(__m128d, int);
extern unsigned int __cdecl _mm_cvtt_roundsd_u32(__m128d, int);
extern int       __cdecl _mm_cvtt_roundss_i32(__m128, int);
extern int       __cdecl _mm_cvtt_roundss_si32(__m128, int);
extern unsigned int __cdecl _mm_cvtt_roundss_u32(__m128, int);
extern int       __cdecl _mm_cvttsd_i32(__m128d);
extern unsigned int __cdecl _mm_cvttsd_u32(__m128d);
extern int       __cdecl _mm_cvttss_i32(__m128);
extern unsigned int __cdecl _mm_cvttss_u32(__m128);
extern __m128d   __cdecl _mm_cvtu32_sd(__m128d, unsigned int);
extern __m128    __cdecl _mm_cvtu32_ss(__m128, unsigned int);
extern __m128d   __cdecl _mm_div_round_sd(__m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask_div_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_div_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128    __cdecl _mm_div_round_ss(__m128, __m128, int);
extern __m128    __cdecl _mm_mask_div_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_div_round_ss(__mmask8, __m128, __m128, int);
extern __m128d   __cdecl _mm_mask_div_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_div_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_div_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_div_ss(__mmask8, __m128, __m128);
extern __m128d   __cdecl _mm_fixupimm_round_sd(__m128d, __m128d, __m128i, const int, int);
extern __m128d   __cdecl _mm_mask_fixupimm_round_sd(__m128d, __mmask8, __m128d, __m128i, const int, const int);
extern __m128d   __cdecl _mm_maskz_fixupimm_round_sd(__mmask8, __m128d, __m128d, __m128i, const int, const int);
extern __m128    __cdecl _mm_fixupimm_round_ss(__m128, __m128, __m128i, const int, const int);
extern __m128    __cdecl _mm_mask_fixupimm_round_ss(__m128, __mmask8, __m128, __m128i, const int, const int);
extern __m128    __cdecl _mm_maskz_fixupimm_round_ss(__mmask8, __m128, __m128, __m128i, const int, const int);
extern __m128d   __cdecl _mm_fixupimm_sd(__m128d, __m128d, __m128i, const int);
extern __m128d   __cdecl _mm_mask_fixupimm_sd(__m128d, __mmask8, __m128d, __m128i, const int);
extern __m128d   __cdecl _mm_maskz_fixupimm_sd(__mmask8, __m128d, __m128d, __m128i, const int);
extern __m128    __cdecl _mm_fixupimm_ss(__m128, __m128, __m128i, const int);
extern __m128    __cdecl _mm_mask_fixupimm_ss(__m128, __mmask8, __m128, __m128i, const int);
extern __m128    __cdecl _mm_maskz_fixupimm_ss(__mmask8, __m128, __m128, __m128i, const int);
extern __m128d   __cdecl _mm_fmadd_round_sd(__m128d, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask_fmadd_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask3_fmadd_round_sd(__m128d, __m128d, __m128d, __mmask8, int);
extern __m128d   __cdecl _mm_maskz_fmadd_round_sd(__mmask8, __m128d, __m128d, __m128d, int);
extern __m128    __cdecl _mm_fmadd_round_ss(__m128, __m128, __m128, int);
extern __m128    __cdecl _mm_mask_fmadd_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_mask3_fmadd_round_ss(__m128, __m128, __m128, __mmask8, int);
extern __m128    __cdecl _mm_maskz_fmadd_round_ss(__mmask8, __m128, __m128, __m128, int);
extern __m128d   __cdecl _mm_mask_fmadd_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fmadd_sd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fmadd_sd(__mmask8, __m128d, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_fmadd_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fmadd_ss(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fmadd_ss(__mmask8, __m128, __m128, __m128);
extern __m128d   __cdecl _mm_fmsub_round_sd(__m128d, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask_fmsub_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask3_fmsub_round_sd(__m128d, __m128d, __m128d, __mmask8, int);
extern __m128d   __cdecl _mm_maskz_fmsub_round_sd(__mmask8, __m128d, __m128d, __m128d, int);
extern __m128    __cdecl _mm_fmsub_round_ss(__m128, __m128, __m128, int);
extern __m128    __cdecl _mm_mask_fmsub_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_mask3_fmsub_round_ss(__m128, __m128, __m128, __mmask8, int);
extern __m128    __cdecl _mm_maskz_fmsub_round_ss(__mmask8, __m128, __m128, __m128, int);
extern __m128d   __cdecl _mm_mask_fmsub_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fmsub_sd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fmsub_sd(__mmask8, __m128d, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_fmsub_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fmsub_ss(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fmsub_ss(__mmask8, __m128, __m128, __m128);
extern __m128d   __cdecl _mm_fnmadd_round_sd(__m128d, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask_fnmadd_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask3_fnmadd_round_sd(__m128d, __m128d, __m128d, __mmask8, int);
extern __m128d   __cdecl _mm_maskz_fnmadd_round_sd(__mmask8, __m128d, __m128d, __m128d, int);
extern __m128    __cdecl _mm_fnmadd_round_ss(__m128, __m128, __m128, int);
extern __m128    __cdecl _mm_mask_fnmadd_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_mask3_fnmadd_round_ss(__m128, __m128, __m128, __mmask8, int);
extern __m128    __cdecl _mm_maskz_fnmadd_round_ss(__mmask8, __m128, __m128, __m128, int);
extern __m128d   __cdecl _mm_mask_fnmadd_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fnmadd_sd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fnmadd_sd(__mmask8, __m128d, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_fnmadd_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fnmadd_ss(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fnmadd_ss(__mmask8, __m128, __m128, __m128);
extern __m128d   __cdecl _mm_fnmsub_round_sd(__m128d, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask_fnmsub_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mask3_fnmsub_round_sd(__m128d, __m128d, __m128d, __mmask8, int);
extern __m128d   __cdecl _mm_maskz_fnmsub_round_sd(__mmask8, __m128d, __m128d, __m128d, int);
extern __m128    __cdecl _mm_fnmsub_round_ss(__m128, __m128, __m128, int);
extern __m128    __cdecl _mm_mask_fnmsub_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_mask3_fnmsub_round_ss(__m128, __m128, __m128, __mmask8, int);
extern __m128    __cdecl _mm_maskz_fnmsub_round_ss(__mmask8, __m128, __m128, __m128, int);
extern __m128d   __cdecl _mm_mask_fnmsub_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_mask3_fnmsub_sd(__m128d, __m128d, __m128d, __mmask8);
extern __m128d   __cdecl _mm_maskz_fnmsub_sd(__mmask8, __m128d, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_fnmsub_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_mask3_fnmsub_ss(__m128, __m128, __m128, __mmask8);
extern __m128    __cdecl _mm_maskz_fnmsub_ss(__mmask8, __m128, __m128, __m128);
extern __mmask8  __cdecl _mm_fpclass_sd_mask(__m128d, int);
extern __mmask8  __cdecl _mm_mask_fpclass_sd_mask(__mmask8, __m128d, int);
extern __mmask8  __cdecl _mm_fpclass_ss_mask(__m128, int);
extern __mmask8  __cdecl _mm_mask_fpclass_ss_mask(__mmask8, __m128, int);
extern __m128d   __cdecl _mm_getexp_round_sd(__m128d, __m128d, const int);
extern __m128d   __cdecl _mm_mask_getexp_round_sd(__m128d, __mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_maskz_getexp_round_sd(__mmask8, __m128d, __m128d, const int);
extern __m128    __cdecl _mm_getexp_round_ss(__m128, __m128, const int);
extern __m128    __cdecl _mm_mask_getexp_round_ss(__m128, __mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_maskz_getexp_round_ss(__mmask8, __m128, __m128, const int);
extern __m128d   __cdecl _mm_getexp_sd(__m128d, __m128d);
extern __m128d   __cdecl _mm_mask_getexp_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_getexp_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_getexp_ss(__m128, __m128);
extern __m128    __cdecl _mm_mask_getexp_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_getexp_ss(__mmask8, __m128, __m128);
extern __m128d   __cdecl _mm_getmant_round_sd(__m128d, __m128d, int, int, int);
extern __m128d   __cdecl _mm_mask_getmant_round_sd(__m128d, __mmask8, __m128d, __m128d, int, int, int);
extern __m128d   __cdecl _mm_maskz_getmant_round_sd(__mmask8, __m128d, __m128d, int, int, int);
extern __m128    __cdecl _mm_getmant_round_ss(__m128, __m128, int, int, int);
extern __m128    __cdecl _mm_mask_getmant_round_ss(__m128, __mmask8, __m128, __m128, int, int, int);
extern __m128    __cdecl _mm_maskz_getmant_round_ss(__mmask8, __m128, __m128, int, int, int);
extern __m128d   __cdecl _mm_getmant_sd(__m128d, __m128d, int, int);
extern __m128d   __cdecl _mm_mask_getmant_sd(__m128d, __mmask8, __m128d, __m128d, int, int);
extern __m128d   __cdecl _mm_maskz_getmant_sd(__mmask8, __m128d, __m128d, int, int);
extern __m128    __cdecl _mm_getmant_ss(__m128, __m128, int, int);
extern __m128    __cdecl _mm_mask_getmant_ss(__m128, __mmask8, __m128, __m128, int, int);
extern __m128    __cdecl _mm_maskz_getmant_ss(__mmask8, __m128, __m128, int, int);
extern __m128d   __cdecl _mm_mask_load_sd(__m128d, __mmask8, const double*);
extern __m128d   __cdecl _mm_maskz_load_sd(__mmask8, const double*);
extern __m128    __cdecl _mm_mask_load_ss(__m128, __mmask8, const float*);
extern __m128    __cdecl _mm_maskz_load_ss(__mmask8, const float*);
extern __m128d   __cdecl _mm_mask_max_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_max_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_max_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_max_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_max_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_max_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_max_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_max_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_max_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_max_ss(__mmask8, __m128, __m128);
extern __m128d   __cdecl _mm_mask_min_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_min_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_min_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_min_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_min_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_min_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_min_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_min_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_min_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_min_ss(__mmask8, __m128, __m128);
extern __m128d   __cdecl _mm_mask_move_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_move_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_move_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_move_ss(__mmask8, __m128, __m128);
extern __m128d   __cdecl _mm_mask_mul_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_mul_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_mul_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_mul_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_mul_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_mul_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_mul_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_mul_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_mul_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_mul_ss(__mmask8, __m128, __m128);
extern __m128d   __cdecl _mm_range_sd(__m128d, __m128d, const int);
extern __m128d   __cdecl _mm_mask_range_sd(__m128d, __mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_maskz_range_sd(__mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_range_round_sd(__m128d, __m128d, const int, int);
extern __m128d   __cdecl _mm_mask_range_round_sd(__m128d, __mmask8, __m128d, __m128d, const int, int);
extern __m128d   __cdecl _mm_maskz_range_round_sd(__mmask8, __m128d, __m128d, const int, int);
extern __m128    __cdecl _mm_range_ss(__m128, __m128, const int);
extern __m128    __cdecl _mm_mask_range_ss(__m128, __mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_maskz_range_ss(__mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_range_round_ss(__m128, __m128, const int, int);
extern __m128    __cdecl _mm_mask_range_round_ss(__m128, __mmask8, __m128, __m128, const int, int);
extern __m128    __cdecl _mm_maskz_range_round_ss(__mmask8, __m128, __m128, const int, int);
extern __m128d   __cdecl _mm_mask_rcp14_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_rcp14_sd(__mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_rcp14_sd(__m128d, __m128d);
extern __m128    __cdecl _mm_mask_rcp14_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_rcp14_ss(__mmask8, __m128, __m128);
extern __m128    __cdecl _mm_rcp14_ss(__m128, __m128);
extern __m128d   __cdecl _mm_mask_rcp28_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_rcp28_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_rcp28_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_rcp28_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_rcp28_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_rcp28_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_rcp28_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_rcp28_sd(__mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_rcp28_sd(__m128d, __m128d);
extern __m128    __cdecl _mm_mask_rcp28_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_rcp28_ss(__mmask8, __m128, __m128);
extern __m128    __cdecl _mm_rcp28_ss(__m128, __m128);
extern __m128d   __cdecl _mm_mask_reduce_round_sd(__m128d, __mmask8, __m128d, __m128d, const int, int);
extern __m128d   __cdecl _mm_maskz_reduce_round_sd(__mmask8, __m128d, __m128d, const int, int);
extern __m128d   __cdecl _mm_reduce_round_sd(__m128d, __m128d, const int, int);
extern __m128    __cdecl _mm_mask_reduce_round_ss(__m128, __mmask8, __m128, __m128, const int, int);
extern __m128    __cdecl _mm_maskz_reduce_round_ss(__mmask8, __m128, __m128, const int, int);
extern __m128    __cdecl _mm_reduce_round_ss(__m128, __m128, const int, int);
extern __m128d   __cdecl _mm_mask_reduce_sd(__m128d, __mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_maskz_reduce_sd(__mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_reduce_sd(__m128d, __m128d, const int);
extern __m128    __cdecl _mm_mask_reduce_ss(__m128, __mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_maskz_reduce_ss(__mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_reduce_ss(__m128, __m128, const int);
extern __m128d   __cdecl _mm_mask_roundscale_round_sd(__m128d, __mmask8, __m128d, __m128d, const int, const int);
extern __m128d   __cdecl _mm_maskz_roundscale_round_sd(__mmask8, __m128d, __m128d, const int, const int);
extern __m128d   __cdecl _mm_roundscale_round_sd(__m128d, __m128d, const int, const int);
extern __m128    __cdecl _mm_mask_roundscale_round_ss(__m128, __mmask8, __m128, __m128, const int, const int);
extern __m128    __cdecl _mm_maskz_roundscale_round_ss(__mmask8, __m128, __m128, const int, const int);
extern __m128    __cdecl _mm_roundscale_round_ss(__m128, __m128, const int, const int);
extern __m128d   __cdecl _mm_mask_roundscale_sd(__m128d, __mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_maskz_roundscale_sd(__mmask8, __m128d, __m128d, const int);
extern __m128d   __cdecl _mm_roundscale_sd(__m128d, __m128d, const int);
extern __m128    __cdecl _mm_mask_roundscale_ss(__m128, __mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_maskz_roundscale_ss(__mmask8, __m128, __m128, const int);
extern __m128    __cdecl _mm_roundscale_ss(__m128, __m128, const int);
extern __m128d   __cdecl _mm_mask_rsqrt14_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_rsqrt14_sd(__mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_rsqrt14_sd(__m128d, __m128d);
extern __m128    __cdecl _mm_mask_rsqrt14_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_rsqrt14_ss(__mmask8, __m128, __m128);
extern __m128    __cdecl _mm_rsqrt14_ss(__m128, __m128);
extern __m128d   __cdecl _mm_mask_rsqrt28_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_rsqrt28_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_rsqrt28_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_rsqrt28_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_rsqrt28_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_rsqrt28_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_rsqrt28_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_rsqrt28_sd(__mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_rsqrt28_sd(__m128d, __m128d);
extern __m128    __cdecl _mm_mask_rsqrt28_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_rsqrt28_ss(__mmask8, __m128, __m128);
extern __m128    __cdecl _mm_rsqrt28_ss(__m128, __m128);
extern __m128d   __cdecl _mm_mask_scalef_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_scalef_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_scalef_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_scalef_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_scalef_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_scalef_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_scalef_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_scalef_sd(__mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_scalef_sd(__m128d, __m128d);
extern __m128    __cdecl _mm_mask_scalef_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_scalef_ss(__mmask8, __m128, __m128);
extern __m128    __cdecl _mm_scalef_ss(__m128, __m128);
extern __m128d   __cdecl _mm_mask_sqrt_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_sqrt_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_sqrt_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_sqrt_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_sqrt_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_sqrt_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_sqrt_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_sqrt_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_sqrt_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_sqrt_ss(__mmask8, __m128, __m128);
extern void      __cdecl _mm_mask_store_sd(double*, __mmask8, __m128d);
extern void      __cdecl _mm_mask_store_ss(float*, __mmask8, __m128);
extern __m128d   __cdecl _mm_mask_sub_round_sd(__m128d, __mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_maskz_sub_round_sd(__mmask8, __m128d, __m128d, int);
extern __m128d   __cdecl _mm_sub_round_sd(__m128d, __m128d, int);
extern __m128    __cdecl _mm_mask_sub_round_ss(__m128, __mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_maskz_sub_round_ss(__mmask8, __m128, __m128, int);
extern __m128    __cdecl _mm_sub_round_ss(__m128, __m128, int);
extern __m128d   __cdecl _mm_mask_sub_sd(__m128d, __mmask8, __m128d, __m128d);
extern __m128d   __cdecl _mm_maskz_sub_sd(__mmask8, __m128d, __m128d);
extern __m128    __cdecl _mm_mask_sub_ss(__m128, __mmask8, __m128, __m128);
extern __m128    __cdecl _mm_maskz_sub_ss(__mmask8, __m128, __m128);

#if defined (_M_X64)

extern unsigned __int64 __cdecl _mm_cvtsd_u64(__m128d);
extern unsigned __int64 __cdecl _mm_cvtss_u64(__m128);
extern unsigned __int64 __cdecl _mm_cvttsd_u64(__m128d);
extern unsigned __int64 __cdecl _mm_cvttss_u64(__m128);
extern unsigned __int64 __cdecl _mm_cvt_roundsd_u64(__m128d, int);
extern unsigned __int64 __cdecl _mm_cvt_roundss_u64(__m128, int);
extern unsigned __int64 __cdecl _mm_cvtt_roundsd_u64(__m128d, int);
extern unsigned __int64 __cdecl _mm_cvtt_roundss_u64(__m128, int);

extern __m128d   __cdecl _mm_cvti64_sd(__m128d, __int64);
extern __m128    __cdecl _mm_cvti64_ss(__m128, __int64);
extern __int64   __cdecl _mm_cvtsd_i64(__m128d);
extern __int64   __cdecl _mm_cvtss_i64(__m128);
extern __int64   __cdecl _mm_cvttsd_i64(__m128d);
extern __int64   __cdecl _mm_cvttss_i64(__m128);
extern __int64   __cdecl _mm_cvtt_roundsd_i64(__m128d, int);
extern __int64   __cdecl _mm_cvtt_roundsd_si64(__m128d, int);
extern __int64   __cdecl _mm_cvtt_roundss_i64(__m128, int);
extern __int64   __cdecl _mm_cvtt_roundss_si64(__m128, int);
extern __m128d   __cdecl _mm_cvtu64_sd(__m128d, unsigned __int64);
extern __m128    __cdecl _mm_cvtu64_ss(__m128, unsigned __int64);
extern __m128d   __cdecl _mm_cvt_roundi64_sd(__m128d, __int64, int);
extern __m128    __cdecl _mm_cvt_roundi64_ss(__m128, __int64, int);
extern __int64   __cdecl _mm_cvt_roundsd_i64(__m128d, int);
extern __int64   __cdecl _mm_cvt_roundsd_si64(__m128d, int);
extern __m128d   __cdecl _mm_cvt_roundsi64_sd(__m128d, __int64, int);
extern __m128    __cdecl _mm_cvt_roundsi64_ss(__m128, __int64, int);
extern __int64   __cdecl _mm_cvt_roundss_i64(__m128, int);
extern __int64   __cdecl _mm_cvt_roundss_si64(__m128, int);
extern __m128d   __cdecl _mm_cvt_roundu64_sd(__m128d, unsigned __int64, int);
extern __m128    __cdecl _mm_cvt_roundu64_ss(__m128, unsigned __int64, int);

#endif  /* defined (_M_X64) */

// Zero-extended cast functions
extern __m512d   __cdecl _mm512_zextpd128_pd512(__m128d);
extern __m512d   __cdecl _mm512_zextpd256_pd512(__m256d);
extern __m512    __cdecl _mm512_zextps128_ps512(__m128);
extern __m512    __cdecl _mm512_zextps256_ps512(__m256);
extern __m512i   __cdecl _mm512_zextsi128_si512(__m128i);
extern __m512i   __cdecl _mm512_zextsi256_si512(__m256i);

/*
 * Convert Scalar Single-Precision Floating-point value in 512-bit vector to
 * equivalent C/C++ float type.
 */
// float _mm512_cvtss_f32 (__m512 a)
#define _mm512_cvtss_f32(a) (_mm_cvtss_f32(_mm512_castps512_ps128(a)))

/*
 * Convert Scalar Double-Precision Floating-point value in 512-bit vector to
 * equivalent C/C++ double type.
 */
// double _mm512_cvtsd_f64 (__m512d a)
#define _mm512_cvtsd_f64(a) (_mm_cvtsd_f64(_mm512_castpd512_pd128(a)))

/*
 * Convert 32-bit Scalar integer in 512-bit vector to equivalent C/C++ int type.
 */
// int _mm512_cvtsi512_si32 (__m512i a)
#define _mm512_cvtsi512_si32(a) (_mm_cvtsi128_si32(_mm512_castsi512_si128(a)))

#if defined (_M_X64)
/*
 * Convert 64-bit Scalar integer in 512-bit vector to equivalent C/C++ __int64 type.
 */
// __int64 _mm512_cvtsi512_si64 (__m512i a)
#define _mm512_cvtsi512_si64(a) (_mm_cvtsi128_si64(_mm512_castsi512_si128(a)))
#endif  /* defined (_M_X64) */

// AVX512_IFMA intrinsics
extern __m128i __cdecl _mm_madd52hi_epu64(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_madd52hi_epu64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_madd52hi_epu64(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_madd52hi_epu64(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_madd52hi_epu64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_madd52hi_epu64(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_madd52hi_epu64(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_madd52hi_epu64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_madd52hi_epu64(__mmask8, __m512i, __m512i, __m512i);

extern __m128i __cdecl _mm_madd52lo_epu64(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_madd52lo_epu64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_madd52lo_epu64(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_madd52lo_epu64(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_madd52lo_epu64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_madd52lo_epu64(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_madd52lo_epu64(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_madd52lo_epu64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_madd52lo_epu64(__mmask8, __m512i, __m512i, __m512i);

// AVX512_VBMI intrinsics
extern __m128i __cdecl _mm_permutexvar_epi8(__m128i, __m128i);
extern __m128i __cdecl _mm_mask_permutexvar_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_permutexvar_epi8(__mmask16, __m128i, __m128i);
extern __m256i __cdecl _mm256_permutexvar_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_mask_permutexvar_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_permutexvar_epi8(__mmask32, __m256i, __m256i);
extern __m512i __cdecl _mm512_permutexvar_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_permutexvar_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_permutexvar_epi8(__mmask64, __m512i, __m512i);

extern __m128i __cdecl _mm_permutex2var_epi8(__m128i, __m128i /* idx */, __m128i);
extern __m128i __cdecl _mm_mask_permutex2var_epi8(__m128i, __mmask16, __m128i /* idx */, __m128i);
extern __m128i __cdecl _mm_mask2_permutex2var_epi8(__m128i, __m128i /* idx */, __mmask16, __m128i);
extern __m128i __cdecl _mm_maskz_permutex2var_epi8(__mmask16, __m128i, __m128i /* idx */, __m128i);
extern __m256i __cdecl _mm256_permutex2var_epi8(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_permutex2var_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask2_permutex2var_epi8(__m256i, __m256i, __mmask32, __m256i);
extern __m256i __cdecl _mm256_maskz_permutex2var_epi8(__mmask32, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_permutex2var_epi8(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_permutex2var_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask2_permutex2var_epi8(__m512i, __m512i, __mmask64, __m512i);
extern __m512i __cdecl _mm512_maskz_permutex2var_epi8(__mmask64, __m512i, __m512i, __m512i);

extern __m128i __cdecl _mm_multishift_epi64_epi8(__m128i, __m128i);
extern __m128i __cdecl _mm_mask_multishift_epi64_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_multishift_epi64_epi8(__mmask16, __m128i, __m128i);
extern __m256i __cdecl _mm256_multishift_epi64_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_mask_multishift_epi64_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_multishift_epi64_epi8(__mmask32, __m256i, __m256i);
extern __m512i __cdecl _mm512_multishift_epi64_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_multishift_epi64_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_multishift_epi64_epi8(__mmask64, __m512i, __m512i);

// AVX512_VNNI intrinsics
extern __m128i __cdecl _mm_dpbusd_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_dpbusd_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_dpbusd_epi32(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbusd_epi32(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_dpbusd_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_dpbusd_epi32(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_dpbusd_epi32(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_dpbusd_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_dpbusd_epi32(__mmask16, __m512i, __m512i, __m512i);

extern __m128i __cdecl _mm_dpbusds_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_dpbusds_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_dpbusds_epi32(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbusds_epi32(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_dpbusds_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_dpbusds_epi32(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_dpbusds_epi32(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_dpbusds_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_dpbusds_epi32(__mmask16, __m512i, __m512i, __m512i);

extern __m128i __cdecl _mm_dpwssd_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_dpwssd_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_dpwssd_epi32(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwssd_epi32(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_dpwssd_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_dpwssd_epi32(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_dpwssd_epi32(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_dpwssd_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_dpwssd_epi32(__mmask16, __m512i, __m512i, __m512i);

extern __m128i __cdecl _mm_dpwssds_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_dpwssds_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_dpwssds_epi32(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwssds_epi32(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_dpwssds_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_dpwssds_epi32(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_dpwssds_epi32(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_dpwssds_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_dpwssds_epi32(__mmask16, __m512i, __m512i, __m512i);

// VAES
extern __m256i __cdecl _mm256_aesenc_epi128(__m256i, __m256i);
extern __m512i __cdecl _mm512_aesenc_epi128(__m512i, __m512i);
extern __m256i __cdecl _mm256_aesenclast_epi128(__m256i, __m256i);
extern __m512i __cdecl _mm512_aesenclast_epi128(__m512i, __m512i);
extern __m256i __cdecl _mm256_aesdec_epi128(__m256i, __m256i);
extern __m512i __cdecl _mm512_aesdec_epi128(__m512i, __m512i);
extern __m256i __cdecl _mm256_aesdeclast_epi128(__m256i, __m256i);
extern __m512i __cdecl _mm512_aesdeclast_epi128(__m512i, __m512i);

// VPCLMULQDQ
extern __m256i __cdecl _mm256_clmulepi64_epi128(__m256i, __m256i, const int);
extern __m512i __cdecl _mm512_clmulepi64_epi128(__m512i, __m512i, const int);

// AVX512_VPOPCNTDQ
extern __m128i __cdecl _mm_popcnt_epi32(__m128i);
extern __m128i __cdecl _mm_mask_popcnt_epi32(__m128i, __mmask8, __m128i);
extern __m128i __cdecl _mm_maskz_popcnt_epi32(__mmask8, __m128i);
extern __m256i __cdecl _mm256_popcnt_epi32(__m256i);
extern __m256i __cdecl _mm256_mask_popcnt_epi32(__m256i, __mmask8, __m256i);
extern __m256i __cdecl _mm256_maskz_popcnt_epi32(__mmask8, __m256i);
extern __m512i __cdecl _mm512_popcnt_epi32(__m512i);
extern __m512i __cdecl _mm512_mask_popcnt_epi32(__m512i, __mmask16, __m512i);
extern __m512i __cdecl _mm512_maskz_popcnt_epi32(__mmask16, __m512i);

extern __m128i __cdecl _mm_popcnt_epi64(__m128i);
extern __m128i __cdecl _mm_mask_popcnt_epi64(__m128i, __mmask8, __m128i);
extern __m128i __cdecl _mm_maskz_popcnt_epi64(__mmask8, __m128i);
extern __m256i __cdecl _mm256_popcnt_epi64(__m256i);
extern __m256i __cdecl _mm256_mask_popcnt_epi64(__m256i, __mmask8, __m256i);
extern __m256i __cdecl _mm256_maskz_popcnt_epi64(__mmask8, __m256i);
extern __m512i __cdecl _mm512_popcnt_epi64(__m512i);
extern __m512i __cdecl _mm512_mask_popcnt_epi64(__m512i, __mmask8, __m512i);
extern __m512i __cdecl _mm512_maskz_popcnt_epi64(__mmask8, __m512i);

// AVX512_BITALG
extern __m128i __cdecl _mm_popcnt_epi8(__m128i);
extern __m128i __cdecl _mm_mask_popcnt_epi8(__m128i, __mmask16, __m128i);
extern __m128i __cdecl _mm_maskz_popcnt_epi8(__mmask16, __m128i);
extern __m256i __cdecl _mm256_popcnt_epi8(__m256i);
extern __m256i __cdecl _mm256_mask_popcnt_epi8(__m256i, __mmask32, __m256i);
extern __m256i __cdecl _mm256_maskz_popcnt_epi8(__mmask32, __m256i);
extern __m512i __cdecl _mm512_popcnt_epi8(__m512i);
extern __m512i __cdecl _mm512_mask_popcnt_epi8(__m512i, __mmask64, __m512i);
extern __m512i __cdecl _mm512_maskz_popcnt_epi8(__mmask64, __m512i);

extern __m128i __cdecl _mm_popcnt_epi16(__m128i);
extern __m128i __cdecl _mm_mask_popcnt_epi16(__m128i, __mmask8, __m128i);
extern __m128i __cdecl _mm_maskz_popcnt_epi16(__mmask8, __m128i);
extern __m256i __cdecl _mm256_popcnt_epi16(__m256i);
extern __m256i __cdecl _mm256_mask_popcnt_epi16(__m256i, __mmask16, __m256i);
extern __m256i __cdecl _mm256_maskz_popcnt_epi16(__mmask16, __m256i);
extern __m512i __cdecl _mm512_popcnt_epi16(__m512i);
extern __m512i __cdecl _mm512_mask_popcnt_epi16(__m512i, __mmask32, __m512i);
extern __m512i __cdecl _mm512_maskz_popcnt_epi16(__mmask32, __m512i);

extern __mmask16 __cdecl _mm_bitshuffle_epi64_mask(__m128i, __m128i);
extern __mmask16 __cdecl _mm_mask_bitshuffle_epi64_mask(__mmask16, __m128i, __m128i);
extern __mmask32 __cdecl _mm256_bitshuffle_epi64_mask(__m256i, __m256i);
extern __mmask32 __cdecl _mm256_mask_bitshuffle_epi64_mask(__mmask32, __m256i, __m256i);
extern __mmask64 __cdecl _mm512_bitshuffle_epi64_mask(__m512i, __m512i);
extern __mmask64 __cdecl _mm512_mask_bitshuffle_epi64_mask(__mmask64, __m512i, __m512i);

// GFNI
extern __m128i __cdecl _mm_gf2p8affineinv_epi64_epi8(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_gf2p8affineinv_epi64_epi8(__m128i, __mmask16, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_gf2p8affineinv_epi64_epi8(__mmask16, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_gf2p8affineinv_epi64_epi8(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_gf2p8affineinv_epi64_epi8(__m256i, __mmask32, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_gf2p8affineinv_epi64_epi8(__mmask32, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_gf2p8affineinv_epi64_epi8(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_gf2p8affineinv_epi64_epi8(__m512i, __mmask64, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_gf2p8affineinv_epi64_epi8(__mmask64, __m512i, __m512i, int);
extern __m128i __cdecl _mm_gf2p8affine_epi64_epi8(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_gf2p8affine_epi64_epi8(__m128i, __mmask16, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_gf2p8affine_epi64_epi8(__mmask16, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_gf2p8affine_epi64_epi8(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_gf2p8affine_epi64_epi8(__m256i, __mmask32, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_gf2p8affine_epi64_epi8(__mmask32, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_gf2p8affine_epi64_epi8(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_gf2p8affine_epi64_epi8(__m512i, __mmask64, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_gf2p8affine_epi64_epi8(__mmask64, __m512i, __m512i, int);
extern __m128i __cdecl _mm_gf2p8mul_epi8(__m128i, __m128i);
extern __m128i __cdecl _mm_mask_gf2p8mul_epi8(__m128i, __mmask16, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_gf2p8mul_epi8(__mmask16, __m128i, __m128i);
extern __m256i __cdecl _mm256_gf2p8mul_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_mask_gf2p8mul_epi8(__m256i, __mmask32, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_gf2p8mul_epi8(__mmask32, __m256i, __m256i);
extern __m512i __cdecl _mm512_gf2p8mul_epi8(__m512i, __m512i);
extern __m512i __cdecl _mm512_mask_gf2p8mul_epi8(__m512i, __mmask64, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_gf2p8mul_epi8(__mmask64, __m512i, __m512i);

// VPSHLDW|D|Q, VPSHLDVW|D|Q
extern __m128i __cdecl _mm_shldi_epi16(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_shldi_epi16(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_shldi_epi16(__mmask8, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_shldi_epi16(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_shldi_epi16(__m256i, __mmask16, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_shldi_epi16(__mmask16, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_shldi_epi16(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_shldi_epi16(__m512i, __mmask32, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shldi_epi16(__mmask32, __m512i, __m512i, int);
extern __m128i __cdecl _mm_shldi_epi32(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_shldi_epi32(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_shldi_epi32(__mmask8, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_shldi_epi32(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_shldi_epi32(__m256i, __mmask8, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_shldi_epi32(__mmask8, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_shldi_epi32(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_shldi_epi32(__m512i, __mmask16, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shldi_epi32(__mmask16, __m512i, __m512i, int);
extern __m128i __cdecl _mm_shldi_epi64(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_shldi_epi64(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_shldi_epi64(__mmask8, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_shldi_epi64(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_shldi_epi64(__m256i, __mmask8, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_shldi_epi64(__mmask8, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_shldi_epi64(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_shldi_epi64(__m512i, __mmask8, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shldi_epi64(__mmask8, __m512i, __m512i, int);
extern __m128i __cdecl _mm_shldv_epi16(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_shldv_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_shldv_epi16(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_shldv_epi16(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_shldv_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_shldv_epi16(__mmask16, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_shldv_epi16(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_shldv_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_shldv_epi16(__mmask32, __m512i, __m512i, __m512i);
extern __m128i __cdecl _mm_shldv_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_shldv_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_shldv_epi32(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_shldv_epi32(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_shldv_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_shldv_epi32(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_shldv_epi32(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_shldv_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_shldv_epi32(__mmask16, __m512i, __m512i, __m512i);
extern __m128i __cdecl _mm_shldv_epi64(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_shldv_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_shldv_epi64(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_shldv_epi64(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_shldv_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_shldv_epi64(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_shldv_epi64(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_shldv_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_shldv_epi64(__mmask8, __m512i, __m512i, __m512i);

// VPSHRDW|D|Q, VPSHRDVW|D|Q
extern __m128i __cdecl _mm_shrdi_epi16(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_shrdi_epi16(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_shrdi_epi16(__mmask8, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_shrdi_epi16(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_shrdi_epi16(__m256i, __mmask16, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_shrdi_epi16(__mmask16, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_shrdi_epi16(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_shrdi_epi16(__m512i, __mmask32, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shrdi_epi16(__mmask32, __m512i, __m512i, int);
extern __m128i __cdecl _mm_shrdi_epi32(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_shrdi_epi32(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_shrdi_epi32(__mmask8, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_shrdi_epi32(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_shrdi_epi32(__m256i, __mmask8, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_shrdi_epi32(__mmask8, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_shrdi_epi32(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_shrdi_epi32(__m512i, __mmask16, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shrdi_epi32(__mmask16, __m512i, __m512i, int);
extern __m128i __cdecl _mm_shrdi_epi64(__m128i, __m128i, int);
extern __m128i __cdecl _mm_mask_shrdi_epi64(__m128i, __mmask8, __m128i, __m128i, int);
extern __m128i __cdecl _mm_maskz_shrdi_epi64(__mmask8, __m128i, __m128i, int);
extern __m256i __cdecl _mm256_shrdi_epi64(__m256i, __m256i, int);
extern __m256i __cdecl _mm256_mask_shrdi_epi64(__m256i, __mmask8, __m256i, __m256i, int);
extern __m256i __cdecl _mm256_maskz_shrdi_epi64(__mmask8, __m256i, __m256i, int);
extern __m512i __cdecl _mm512_shrdi_epi64(__m512i, __m512i, int);
extern __m512i __cdecl _mm512_mask_shrdi_epi64(__m512i, __mmask8, __m512i, __m512i, int);
extern __m512i __cdecl _mm512_maskz_shrdi_epi64(__mmask8, __m512i, __m512i, int);
extern __m128i __cdecl _mm_shrdv_epi16(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_shrdv_epi16(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_shrdv_epi16(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_shrdv_epi16(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_shrdv_epi16(__m256i, __mmask16, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_shrdv_epi16(__mmask16, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_shrdv_epi16(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_shrdv_epi16(__m512i, __mmask32, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_shrdv_epi16(__mmask32, __m512i, __m512i, __m512i);
extern __m128i __cdecl _mm_shrdv_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_shrdv_epi32(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_shrdv_epi32(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_shrdv_epi32(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_shrdv_epi32(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_shrdv_epi32(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_shrdv_epi32(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_shrdv_epi32(__m512i, __mmask16, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_shrdv_epi32(__mmask16, __m512i, __m512i, __m512i);
extern __m128i __cdecl _mm_shrdv_epi64(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_mask_shrdv_epi64(__m128i, __mmask8, __m128i, __m128i);
extern __m128i __cdecl _mm_maskz_shrdv_epi64(__mmask8, __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_shrdv_epi64(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_mask_shrdv_epi64(__m256i, __mmask8, __m256i, __m256i);
extern __m256i __cdecl _mm256_maskz_shrdv_epi64(__mmask8, __m256i, __m256i, __m256i);
extern __m512i __cdecl _mm512_shrdv_epi64(__m512i, __m512i, __m512i);
extern __m512i __cdecl _mm512_mask_shrdv_epi64(__m512i, __mmask8, __m512i, __m512i);
extern __m512i __cdecl _mm512_maskz_shrdv_epi64(__mmask8, __m512i, __m512i, __m512i);

/*
* Intrinsic functions for Short Vector Math Library (SVML)
*/

// vector integer divide and remainder
extern __m512i _mm512_div_epi8(__m512i, __m512i);
extern __m512i _mm512_div_epi16(__m512i, __m512i);
extern __m512i _mm512_div_epi32(__m512i, __m512i);
extern __m512i _mm512_div_epi64(__m512i, __m512i);
extern __m512i _mm512_div_epu8(__m512i, __m512i);
extern __m512i _mm512_div_epu16(__m512i, __m512i);
extern __m512i _mm512_div_epu32(__m512i, __m512i);
extern __m512i _mm512_div_epu64(__m512i, __m512i);
extern __m512i _mm512_mask_div_epi32(__m512i /*src*/, __mmask16, __m512i, __m512i);
extern __m512i _mm512_mask_div_epu32(__m512i /*src*/, __mmask16, __m512i, __m512i);
extern __m512i _mm512_rem_epi8(__m512i, __m512i);
extern __m512i _mm512_rem_epi16(__m512i, __m512i);
extern __m512i _mm512_rem_epi32(__m512i, __m512i);
extern __m512i _mm512_rem_epi64(__m512i, __m512i);
extern __m512i _mm512_rem_epu8(__m512i, __m512i);
extern __m512i _mm512_rem_epu16(__m512i, __m512i);
extern __m512i _mm512_rem_epu32(__m512i, __m512i);
extern __m512i _mm512_rem_epu64(__m512i, __m512i);
extern __m512i _mm512_mask_rem_epi32(__m512i /*src*/, __mmask16, __m512i, __m512i);
extern __m512i _mm512_mask_rem_epu32(__m512i /*src*/, __mmask16, __m512i, __m512i);

// Math functions
extern __m512  _mm512_sin_ps(__m512);
extern __m512  _mm512_mask_sin_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_sin_pd(__m512d);
extern __m512d _mm512_mask_sin_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_cos_ps(__m512);
extern __m512  _mm512_mask_cos_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_cos_pd(__m512d);
extern __m512d _mm512_mask_cos_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_sincos_ps(__m512  * /*cos_res*/, __m512);
extern __m512  _mm512_mask_sincos_ps(__m512  * /*cos_res*/, __m512  /*sin_src*/, __m512  /*cos_src*/, __mmask16, __m512);
extern __m512d _mm512_sincos_pd(__m512d * /*cos_res*/, __m512d);
extern __m512d _mm512_mask_sincos_pd(__m512d * /*cos_res*/, __m512d /*sin_src*/, __m512d /*cos_src*/, __mmask8, __m512d);
extern __m512  _mm512_tan_ps(__m512);
extern __m512  _mm512_mask_tan_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_tan_pd(__m512d);
extern __m512d _mm512_mask_tan_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_asin_ps(__m512);
extern __m512  _mm512_mask_asin_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_asin_pd(__m512d);
extern __m512d _mm512_mask_asin_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_acos_ps(__m512);
extern __m512  _mm512_mask_acos_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_acos_pd(__m512d);
extern __m512d _mm512_mask_acos_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_atan_ps(__m512);
extern __m512  _mm512_mask_atan_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_atan_pd(__m512d);
extern __m512d _mm512_mask_atan_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_atan2_ps(__m512, __m512);
extern __m512  _mm512_mask_atan2_ps(__m512  /*src*/, __mmask16, __m512, __m512);
extern __m512d _mm512_atan2_pd(__m512d, __m512d);
extern __m512d _mm512_mask_atan2_pd(__m512d /*src*/, __mmask8, __m512d, __m512d);
extern __m512  _mm512_sind_ps(__m512);
extern __m512  _mm512_mask_sind_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_sind_pd(__m512d);
extern __m512d _mm512_mask_sind_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_cosd_ps(__m512);
extern __m512  _mm512_mask_cosd_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_cosd_pd(__m512d);
extern __m512d _mm512_mask_cosd_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_tand_ps(__m512);
extern __m512  _mm512_mask_tand_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_tand_pd(__m512d);
extern __m512d _mm512_mask_tand_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_sinh_ps(__m512);
extern __m512  _mm512_mask_sinh_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_sinh_pd(__m512d);
extern __m512d _mm512_mask_sinh_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_cosh_ps(__m512);
extern __m512  _mm512_mask_cosh_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_cosh_pd(__m512d);
extern __m512d _mm512_mask_cosh_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_tanh_ps(__m512);
extern __m512  _mm512_mask_tanh_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_tanh_pd(__m512d);
extern __m512d _mm512_mask_tanh_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_asinh_ps(__m512);
extern __m512  _mm512_mask_asinh_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_asinh_pd(__m512d);
extern __m512d _mm512_mask_asinh_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_acosh_ps(__m512);
extern __m512  _mm512_mask_acosh_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_acosh_pd(__m512d);
extern __m512d _mm512_mask_acosh_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_atanh_ps(__m512);
extern __m512  _mm512_mask_atanh_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_atanh_pd(__m512d);
extern __m512d _mm512_mask_atanh_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_log_ps(__m512);
extern __m512  _mm512_mask_log_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_log_pd(__m512d);
extern __m512d _mm512_mask_log_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_log1p_ps(__m512);
extern __m512  _mm512_mask_log1p_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_log1p_pd(__m512d);
extern __m512d _mm512_mask_log1p_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_log10_ps(__m512);
extern __m512  _mm512_mask_log10_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_log10_pd(__m512d);
extern __m512d _mm512_mask_log10_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_log2_ps(__m512);
extern __m512  _mm512_mask_log2_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_log2_pd(__m512d);
extern __m512d _mm512_mask_log2_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_logb_ps(__m512);
extern __m512  _mm512_mask_logb_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_logb_pd(__m512d);
extern __m512d _mm512_mask_logb_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_exp_ps(__m512);
extern __m512  _mm512_mask_exp_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_exp_pd(__m512d);
extern __m512d _mm512_mask_exp_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_exp10_ps(__m512);
extern __m512  _mm512_mask_exp10_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_exp10_pd(__m512d);
extern __m512d _mm512_mask_exp10_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_exp2_ps(__m512);
extern __m512  _mm512_mask_exp2_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_exp2_pd(__m512d);
extern __m512d _mm512_mask_exp2_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_expm1_ps(__m512);
extern __m512  _mm512_mask_expm1_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_expm1_pd(__m512d);
extern __m512d _mm512_mask_expm1_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_pow_ps(__m512, __m512);
extern __m512  _mm512_mask_pow_ps(__m512  /*src*/, __mmask16, __m512, __m512);
extern __m512d _mm512_pow_pd(__m512d, __m512d);
extern __m512d _mm512_mask_pow_pd(__m512d /*src*/, __mmask8, __m512d, __m512d);
extern __m512  _mm512_trunc_ps(__m512);
extern __m512  _mm512_mask_trunc_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_trunc_pd(__m512d);
extern __m512d _mm512_mask_trunc_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_floor_ps(__m512);
extern __m512  _mm512_mask_floor_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_floor_pd(__m512d);
extern __m512d _mm512_mask_floor_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_ceil_ps(__m512);
extern __m512  _mm512_mask_ceil_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_ceil_pd(__m512d);
extern __m512d _mm512_mask_ceil_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512 _mm512_svml_round_ps(__m512);
extern __m512 _mm512_mask_svml_round_ps(__m512 /*src*/, __mmask16, __m512);
extern __m512d _mm512_svml_round_pd(__m512d);
extern __m512d _mm512_mask_svml_round_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_fmod_ps(__m512, __m512);
extern __m512  _mm512_mask_fmod_ps(__m512  /*src*/, __mmask16, __m512, __m512);
extern __m512d _mm512_fmod_pd(__m512d, __m512d);
extern __m512d _mm512_mask_fmod_pd(__m512d /*src*/, __mmask8, __m512d, __m512d);
extern __m512  _mm512_rint_ps(__m512);
extern __m512  _mm512_mask_rint_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_rint_pd(__m512d);
extern __m512d _mm512_mask_rint_pd(__m512d /*src*/, __mmask8, __m512d);

#define _mm512_recip_ps(a)    _mm512_div_ps(_mm512_set1_ps(1.0f), (a))
#define _mm512_mask_recip_ps(src,k,a)    _mm512_mask_div_ps((src), (k), _mm512_set1_ps(1.0f), (a))
#define _mm512_recip_pd(a)    _mm512_div_pd(_mm512_set1_pd(1.0), (a))
#define _mm512_mask_recip_pd(src,k,a)    _mm512_mask_div_pd((src), (k), _mm512_set1_pd(1.0), (a))

extern __m512  _mm512_invsqrt_ps(__m512);
extern __m512  _mm512_mask_invsqrt_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_invsqrt_pd(__m512d);
extern __m512d _mm512_mask_invsqrt_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_cbrt_ps(__m512);
extern __m512  _mm512_mask_cbrt_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_cbrt_pd(__m512d);
extern __m512d _mm512_mask_cbrt_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_invcbrt_ps(__m512);
extern __m512  _mm512_mask_invcbrt_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_invcbrt_pd(__m512d);
extern __m512d _mm512_mask_invcbrt_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_hypot_ps(__m512, __m512);
extern __m512  _mm512_mask_hypot_ps(__m512  /*src*/, __mmask16, __m512, __m512);
extern __m512d _mm512_hypot_pd(__m512d, __m512d);
extern __m512d _mm512_mask_hypot_pd(__m512d /*src*/, __mmask8, __m512d, __m512d);
extern __m512  _mm512_cdfnorm_ps(__m512);
extern __m512  _mm512_mask_cdfnorm_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_cdfnorm_pd(__m512d);
extern __m512d _mm512_mask_cdfnorm_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_cdfnorminv_ps(__m512);
extern __m512  _mm512_mask_cdfnorminv_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_cdfnorminv_pd(__m512d);
extern __m512d _mm512_mask_cdfnorminv_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_erf_ps(__m512);
extern __m512  _mm512_mask_erf_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_erf_pd(__m512d);
extern __m512d _mm512_mask_erf_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_erfc_ps(__m512);
extern __m512  _mm512_mask_erfc_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_erfc_pd(__m512d);
extern __m512d _mm512_mask_erfc_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_erfcinv_ps(__m512);
extern __m512  _mm512_mask_erfcinv_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_erfcinv_pd(__m512d);
extern __m512d _mm512_mask_erfcinv_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_erfinv_ps(__m512);
extern __m512  _mm512_mask_erfinv_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_erfinv_pd(__m512d);
extern __m512d _mm512_mask_erfinv_pd(__m512d /*src*/, __mmask8, __m512d);
extern __m512  _mm512_nearbyint_ps(__m512);
extern __m512  _mm512_mask_nearbyint_ps(__m512  /*src*/, __mmask16, __m512);
extern __m512d _mm512_nearbyint_pd(__m512d);
extern __m512d _mm512_mask_nearbyint_pd(__m512d /*src*/, __mmask8, __m512d);

// BFLOAT16
typedef __m128i __m128bh;
typedef __m256i __m256bh;
typedef __m512i __m512bh;
typedef unsigned short __bfloat16;

extern __m128bh _mm_cvtneps_pbh(__m128);
extern __m128bh _mm_mask_cvtneps_pbh(__m128bh, __mmask8, __m128);
extern __m128bh _mm_maskz_cvtneps_pbh(__mmask8, __m128);
extern __m128bh _mm_cvtne2ps_pbh(__m128, __m128);
extern __m128bh _mm_mask_cvtne2ps_pbh(__m128bh, __mmask8, __m128, __m128);
extern __m128bh _mm_maskz_cvtne2ps_pbh(__mmask8, __m128, __m128);
extern __m128   _mm_dpbf16_ps(__m128, __m128bh, __m128bh);
extern __m128   _mm_mask_dpbf16_ps(__m128, __mmask8, __m128bh, __m128bh);
extern __m128   _mm_maskz_dpbf16_ps(__mmask8, __m128, __m128bh, __m128bh);
extern __m128bh _mm256_cvtneps_pbh(__m256);
extern __m128bh _mm256_mask_cvtneps_pbh(__m128bh, __mmask8, __m256);
extern __m128bh _mm256_maskz_cvtneps_pbh(__mmask8, __m256);
extern __m256bh _mm256_cvtne2ps_pbh(__m256, __m256);
extern __m256bh _mm256_mask_cvtne2ps_pbh(__m256bh, __mmask16, __m256, __m256);
extern __m256bh _mm256_maskz_cvtne2ps_pbh(__mmask16, __m256, __m256);
extern __m256   _mm256_dpbf16_ps(__m256, __m256bh, __m256bh);
extern __m256   _mm256_mask_dpbf16_ps(__m256, __mmask8, __m256bh, __m256bh);
extern __m256   _mm256_maskz_dpbf16_ps(__mmask8, __m256, __m256bh, __m256bh);
extern __m256bh _mm512_cvtneps_pbh(__m512);
extern __m256bh _mm512_mask_cvtneps_pbh(__m256bh, __mmask16, __m512);
extern __m256bh _mm512_maskz_cvtneps_pbh(__mmask16, __m512);
extern __m512bh _mm512_cvtne2ps_pbh(__m512, __m512);
extern __m512bh _mm512_mask_cvtne2ps_pbh(__m512bh, __mmask32, __m512, __m512);
extern __m512bh _mm512_maskz_cvtne2ps_pbh(__mmask32, __m512, __m512);
extern __m512   _mm512_dpbf16_ps(__m512, __m512bh, __m512bh);
extern __m512   _mm512_mask_dpbf16_ps(__m512, __mmask16, __m512bh, __m512bh);
extern __m512   _mm512_maskz_dpbf16_ps(__mmask16, __m512, __m512bh, __m512bh);
extern __bfloat16 _mm_cvtness_sbh(float);
extern float      _mm_cvtsbh_ss(__bfloat16);
extern __m128     _mm_cvtpbh_ps(__m128bh);
extern __m128     _mm_mask_cvtpbh_ps(__m128, __mmask8, __m128bh);
extern __m128     _mm_maskz_cvtpbh_ps(__mmask8, __m128bh);
extern __m256     _mm256_cvtpbh_ps(__m128bh);
extern __m256     _mm256_mask_cvtpbh_ps(__m256, __mmask8, __m128bh);
extern __m256     _mm256_maskz_cvtpbh_ps(__mmask8, __m128bh);
extern __m512     _mm512_cvtpbh_ps(__m256bh);
extern __m512     _mm512_mask_cvtpbh_ps(__m512, __mmask16, __m256bh);
extern __m512     _mm512_maskz_cvtpbh_ps(__mmask16, __m256bh);

#define _mm_cvtneps2bf16                _mm_cvtneps_pbh
#define _mm_mask_cvtneps2bf16           _mm_mask_cvtneps_pbh
#define _mm_maskz_cvtneps2bf16          _mm_maskz_cvtneps_pbh
#define _mm256_cvtneps2bf16             _mm256_cvtneps_pbh
#define _mm256_mask_cvtneps2bf16        _mm256_mask_cvtneps_pbh
#define _mm256_maskz_cvtneps2bf16       _mm256_maskz_cvtneps_pbh
#define _mm512_cvtneps2bf16             _mm512_cvtneps_pbh
#define _mm512_mask_cvtneps2bf16        _mm512_mask_cvtneps_pbh
#define _mm512_maskz_cvtneps2bf16       _mm512_maskz_cvtneps_pbh
#define _mm_cvtne2ps2bf16               _mm_cvtne2ps_pbh
#define _mm_mask_cvtne2ps2bf16          _mm_mask_cvtne2ps_pbh
#define _mm_maskz_cvtne2ps2bf16         _mm_maskz_cvtne2ps_pbh
#define _mm256_cvtne2ps2bf16            _mm256_cvtne2ps_pbh
#define _mm256_mask_cvtne2ps2bf16       _mm256_mask_cvtne2ps_pbh
#define _mm256_maskz_cvtne2ps2bf16      _mm256_maskz_cvtne2ps_pbh
#define _mm512_cvtne2ps2bf16            _mm512_cvtne2ps_pbh
#define _mm512_mask_cvtne2ps2bf16       _mm512_mask_cvtne2ps_pbh
#define _mm512_maskz_cvtne2ps2bf16      _mm512_maskz_cvtne2ps_pbh

extern __mmask8  __cdecl _kadd_mask8(__mmask8, __mmask8);
extern __mmask16 __cdecl _kadd_mask16(__mmask16, __mmask16);
extern __mmask32 __cdecl _kadd_mask32(__mmask32, __mmask32);
extern __mmask64 __cdecl _kadd_mask64(__mmask64, __mmask64);
extern __mmask8  __cdecl _kand_mask8(__mmask8, __mmask8);
extern __mmask16 __cdecl _kand_mask16(__mmask16, __mmask16);
extern __mmask32 __cdecl _kand_mask32(__mmask32, __mmask32);
extern __mmask64 __cdecl _kand_mask64(__mmask64, __mmask64);
extern __mmask8  __cdecl _kandn_mask8(__mmask8, __mmask8);
extern __mmask16 __cdecl _kandn_mask16(__mmask16, __mmask16);
extern __mmask32 __cdecl _kandn_mask32(__mmask32, __mmask32);
extern __mmask64 __cdecl _kandn_mask64(__mmask64, __mmask64);
extern __mmask8  __cdecl _knot_mask8(__mmask8);
extern __mmask16 __cdecl _knot_mask16(__mmask16);
extern __mmask32 __cdecl _knot_mask32(__mmask32);
extern __mmask64 __cdecl _knot_mask64(__mmask64);
extern __mmask8  __cdecl _kor_mask8(__mmask8, __mmask8);
extern __mmask16 __cdecl _kor_mask16(__mmask16, __mmask16);
extern __mmask32 __cdecl _kor_mask32(__mmask32, __mmask32);
extern __mmask64 __cdecl _kor_mask64(__mmask64, __mmask64);
extern __mmask8  __cdecl _kxnor_mask8(__mmask8, __mmask8);
extern __mmask16 __cdecl _kxnor_mask16(__mmask16, __mmask16);
extern __mmask32 __cdecl _kxnor_mask32(__mmask32, __mmask32);
extern __mmask64 __cdecl _kxnor_mask64(__mmask64, __mmask64);
extern __mmask8  __cdecl _kxor_mask8(__mmask8, __mmask8);
extern __mmask16 __cdecl _kxor_mask16(__mmask16, __mmask16);
extern __mmask32 __cdecl _kxor_mask32(__mmask32, __mmask32);
extern __mmask64 __cdecl _kxor_mask64(__mmask64, __mmask64);
extern __mmask8  __cdecl _kshiftli_mask8(__mmask8, unsigned int);
extern __mmask16 __cdecl _kshiftli_mask16(__mmask16, unsigned int);
extern __mmask32 __cdecl _kshiftli_mask32(__mmask32, unsigned int);
extern __mmask64 __cdecl _kshiftli_mask64(__mmask64, unsigned int);
extern __mmask8  __cdecl _kshiftri_mask8(__mmask8, unsigned int);
extern __mmask16 __cdecl _kshiftri_mask16(__mmask16, unsigned int);
extern __mmask32 __cdecl _kshiftri_mask32(__mmask32, unsigned int);
extern __mmask64 __cdecl _kshiftri_mask64(__mmask64, unsigned int);
extern __mmask8  __cdecl _load_mask8(__mmask8 *);
extern __mmask16 __cdecl _load_mask16(__mmask16 *);
extern __mmask32 __cdecl _load_mask32(__mmask32 *);
extern __mmask64 __cdecl _load_mask64(__mmask64 *);
extern void      __cdecl _store_mask8(__mmask8 *, __mmask8);
extern void      __cdecl _store_mask16(__mmask16 *, __mmask16);
extern void      __cdecl _store_mask32(__mmask32 *, __mmask32);
extern void      __cdecl _store_mask64(__mmask64 *, __mmask64);
extern unsigned int     __cdecl _cvtmask8_u32(__mmask8);
extern unsigned int     __cdecl _cvtmask16_u32(__mmask16);
extern unsigned int     __cdecl _cvtmask32_u32(__mmask32);
extern unsigned __int64 __cdecl _cvtmask64_u64(__mmask64);
extern __mmask8         __cdecl _cvtu32_mask8(unsigned int);
extern __mmask16        __cdecl _cvtu32_mask16(unsigned int);
extern __mmask32        __cdecl _cvtu32_mask32(unsigned int);
extern __mmask64        __cdecl _cvtu64_mask64(unsigned __int64);
extern __mmask16        __cdecl _mm512_kmov(__mmask16);
extern unsigned char __cdecl _kortest_mask8_u8(__mmask8, __mmask8, unsigned char *);
extern unsigned char __cdecl _kortest_mask16_u8(__mmask16, __mmask16, unsigned char *);
extern unsigned char __cdecl _kortest_mask32_u8(__mmask32, __mmask32, unsigned char *);
extern unsigned char __cdecl _kortest_mask64_u8(__mmask64, __mmask64, unsigned char *);
extern unsigned char __cdecl _ktest_mask8_u8(__mmask8, __mmask8, unsigned char *);
extern unsigned char __cdecl _ktest_mask16_u8(__mmask16, __mmask16, unsigned char *);
extern unsigned char __cdecl _ktest_mask32_u8(__mmask32, __mmask32, unsigned char *);
extern unsigned char __cdecl _ktest_mask64_u8(__mmask64, __mmask64, unsigned char *);

#define _mm512_kand(k1, k2)    _kand_mask16((k1), (k2))
#define _mm512_kandn(k1, k2)   _kandn_mask16((k1), (k2))
#define _mm512_kor(k1, k2)     _kor_mask16((k1), (k2))
#define _mm512_kxor(k1, k2)    _kxor_mask16((k1), (k2))
#define _mm512_kxnor(k1, k2)   _kxnor_mask16((k1), (k2))
#define _mm512_knot(k1)        _knot_mask16((k1))

#define _kortestc_mask8_u8(k1, k2)    _mm512_testz_nor_mask8((k1), (k2))
#define _kortestc_mask16_u8(k1, k2)   _mm512_testz_nor_mask16((k1), (k2))
#define _kortestc_mask32_u8(k1, k2)   _mm512_testz_nor_mask32((k1), (k2))
#define _kortestc_mask64_u8(k1, k2)   _mm512_testz_nor_mask64((k1), (k2))

#define _kortestz_mask8_u8(k1, k2)    _mm512_testz_or_mask8((k1), (k2))
#define _kortestz_mask16_u8(k1, k2)   _mm512_testz_or_mask16((k1), (k2))
#define _kortestz_mask32_u8(k1, k2)   _mm512_testz_or_mask32((k1), (k2))
#define _kortestz_mask64_u8(k1, k2)   _mm512_testz_or_mask64((k1), (k2))

#define _ktestc_mask8_u8(k1, k2)    _mm512_testz_andn_mask8((k1), (k2))
#define _ktestc_mask16_u8(k1, k2)   _mm512_testz_andn_mask16((k1), (k2))
#define _ktestc_mask32_u8(k1, k2)   _mm512_testz_andn_mask32((k1), (k2))
#define _ktestc_mask64_u8(k1, k2)   _mm512_testz_andn_mask64((k1), (k2))

#define _ktestz_mask8_u8(k1, k2)    _mm512_testz_and_mask8((k1), (k2))
#define _ktestz_mask16_u8(k1, k2)   _mm512_testz_and_mask16((k1), (k2))
#define _ktestz_mask32_u8(k1, k2)   _mm512_testz_and_mask32((k1), (k2))
#define _ktestz_mask64_u8(k1, k2)   _mm512_testz_and_mask64((k1), (k2))

#define _mm512_mask2int(m)    _cvtmask16_u32((m))
#define _mm512_int2mask(i)    _cvtu32_mask16((i))

#define _mm512_kortestz(m1, m2) ((int)_mm512_testz_or_mask16((m1), (m2)))
#define _mm512_kortestc(m1, m2) ((int)_mm512_testz_nor_mask16((m1), (m2)))

// These functions are not supported with Windows.
// They are deprecated and will be removed in a future release

extern __m512  __cdecl _mm512_mask_exp2a23_round_ps(__m512, __mmask16, __m512, int);
extern __m512  __cdecl _mm512_maskz_exp2a23_round_ps(__mmask16, __m512, int);
extern __m512d __cdecl _mm512_mask_exp2a23_round_pd(__m512d, __mmask8, __m512d, int);
extern __m512d __cdecl _mm512_maskz_exp2a23_round_pd(__mmask8, __m512d, int);

#define _mm512_exp2a23_round_ps(v1, e1) \
    _mm512_maskz_exp2a23_round_ps(_MM_K0_REG16, (v1), (e1))
#define _mm512_mask_exp2a23_ps(v1, k1, v2) \
    _mm512_mask_exp2a23_round_ps((v1), (k1), (v2), _MM_FROUND_CUR_DIRECTION)
#define _mm512_maskz_exp2a23_ps(k1, v1) \
    _mm512_maskz_exp2a23_round_ps((k1), (v1), _MM_FROUND_CUR_DIRECTION)
#define _mm512_exp2a23_ps(v1) \
    _mm512_maskz_exp2a23_ps(_MM_K0_REG16, (v1))
#define _mm512_exp2a23_round_pd(v1, e1) \
    _mm512_maskz_exp2a23_round_pd(_MM_K0_REG8, (v1), (e1))
#define _mm512_mask_exp2a23_pd(v1, k1, v2) \
    _mm512_mask_exp2a23_round_pd((v1), (k1), (v2), _MM_FROUND_CUR_DIRECTION)
#define _mm512_maskz_exp2a23_pd(k1, v1) \
    _mm512_maskz_exp2a23_round_pd((k1), (v1), _MM_FROUND_CUR_DIRECTION)
#define _mm512_exp2a23_pd(v1) \
    _mm512_maskz_exp2a23_pd(_MM_K0_REG8, (v1))

extern __m512  __cdecl _mm512_mask_rcp28_round_ps(__m512, __mmask16, __m512, const int);
extern __m512  __cdecl _mm512_maskz_rcp28_round_ps(__mmask16, __m512, const int);
extern __m512d __cdecl _mm512_mask_rcp28_round_pd(__m512d, __mmask8, __m512d, const int);
extern __m512d __cdecl _mm512_maskz_rcp28_round_pd(__mmask8, __m512d, const int);

#define _mm512_mask_rcp28_ps(v1, k1, v2) \
    _mm512_mask_rcp28_round_ps((v1), (k1), (v2), _MM_FROUND_CUR_DIRECTION)
#define _mm512_maskz_rcp28_ps(k1, v1) \
    _mm512_maskz_rcp28_round_ps((k1), (v1), _MM_FROUND_CUR_DIRECTION)
#define _mm512_rcp28_round_ps(v1, e1) \
    _mm512_maskz_rcp28_round_ps(_MM_K0_REG16, (v1), (e1))
#define _mm512_rcp28_ps(v1) \
    _mm512_maskz_rcp28_ps(_MM_K0_REG16, (v1))
#define _mm512_mask_rcp28_pd(v1, k1, v2) \
    _mm512_mask_rcp28_round_pd((v1), (k1), (v2), _MM_FROUND_CUR_DIRECTION)
#define _mm512_maskz_rcp28_pd(k1, v1) \
    _mm512_maskz_rcp28_round_pd((k1), (v1), _MM_FROUND_CUR_DIRECTION)
#define _mm512_rcp28_round_pd(v1, e1) \
    _mm512_maskz_rcp28_round_pd(_MM_K0_REG8, (v1), (e1))
#define _mm512_rcp28_pd(v1) \
    _mm512_maskz_rcp28_pd(_MM_K0_REG8, (v1))

extern __m512  __cdecl _mm512_mask_rsqrt28_round_ps(__m512, __mmask16, __m512, const int);
extern __m512  __cdecl _mm512_maskz_rsqrt28_round_ps(__mmask16, __m512, const int);
extern __m512d __cdecl _mm512_mask_rsqrt28_round_pd(__m512d, __mmask8, __m512d, const int);
extern __m512d __cdecl _mm512_maskz_rsqrt28_round_pd(__mmask8, __m512d, const int);

#define _mm512_mask_rsqrt28_ps(v1, k1, v2) \
    _mm512_mask_rsqrt28_round_ps((v1), (k1), (v2), _MM_FROUND_CUR_DIRECTION)
#define _mm512_maskz_rsqrt28_ps(k1, v1) \
    _mm512_maskz_rsqrt28_round_ps((k1), (v1), _MM_FROUND_CUR_DIRECTION)
#define _mm512_rsqrt28_round_ps(v1, e1) \
    _mm512_maskz_rsqrt28_round_ps(_MM_K0_REG16, (v1), (e1))
#define _mm512_rsqrt28_ps(v1) \
    _mm512_maskz_rsqrt28_ps(_MM_K0_REG16, (v1))
#define _mm512_mask_rsqrt28_pd(v1, k1, v2) \
    _mm512_mask_rsqrt28_round_pd((v1), (k1), (v2), _MM_FROUND_CUR_DIRECTION)
#define _mm512_maskz_rsqrt28_pd(k1, v1) \
    _mm512_maskz_rsqrt28_round_pd((k1), (v1), _MM_FROUND_CUR_DIRECTION)
#define _mm512_rsqrt28_round_pd(v1, e1) \
    _mm512_maskz_rsqrt28_round_pd(_MM_K0_REG8, (v1), (e1))
#define _mm512_rsqrt28_pd(v1) \
    _mm512_maskz_rsqrt28_pd(_MM_K0_REG8, (v1))

extern void __cdecl _mm512_prefetch_i32gather_pd(__m256i vindex, void const* base_addr, int scale, const int hint);
extern void __cdecl _mm512_prefetch_i32gather_ps(__m512i index, void const* mv, int scale, const int hint);
extern void __cdecl _mm512_prefetch_i32scatter_pd(void* base_addr, __m256i vindex, int scale, const int hint);
extern void __cdecl _mm512_prefetch_i32scatter_ps(void* mv, __m512i index, int scale, const int hint);
extern void __cdecl _mm512_prefetch_i64gather_pd(__m512i vindex, void const* base_addr, int scale, const int hint);
extern void __cdecl _mm512_prefetch_i64gather_ps(__m512i vindex, void const* base_addr, int scale, const int hint);
extern void __cdecl _mm512_prefetch_i64scatter_pd(void* base_addr, __m512i vindex, int scale, const int hint);
extern void __cdecl _mm512_prefetch_i64scatter_ps(void* base_addr, __m512i vindex, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i32gather_pd(__m256i vindex, __mmask8 mask, void const* base_addr, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i32gather_ps(__m512i vindex, __mmask16 mask, void const* base_addr, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i32scatter_pd(void* base_addr, __mmask8 mask, __m256i vinde, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i32scatter_ps(void* mv, __mmask16 k, __m512i index, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i64gather_pd(__m512i vindex, __mmask8 mask, void const* base_addr, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i64gather_ps(__m512i vindex, __mmask8 mask, void const* base_addr, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i64scatter_pd(void* base_addr, __mmask8 mask, __m512i vindex, int scale, const int hint);
extern void __cdecl _mm512_mask_prefetch_i64scatter_ps(void* base_addr, __mmask8 mask, __m512i vindex, int scale, const int hint);

// AVX512_VP2INTERSECT
extern void __cdecl _mm_2intersect_epi32(__m128i, __m128i, __mmask8 *, __mmask8 *);
extern void __cdecl _mm256_2intersect_epi32(__m256i, __m256i, __mmask8 *, __mmask8 *);
extern void __cdecl _mm512_2intersect_epi32(__m512i, __m512i, __mmask16 *, __mmask16 *);
extern void __cdecl _mm_2intersect_epi64(__m128i, __m128i, __mmask8 *, __mmask8 *);
extern void __cdecl _mm256_2intersect_epi64(__m256i, __m256i, __mmask8 *, __mmask8 *);
extern void __cdecl _mm512_2intersect_epi64(__m512i, __m512i, __mmask8 *, __mmask8 *);

#if defined (_M_X64)

// AMX
typedef int __tile;

extern void __cdecl _tile_loadconfig(const void *);
extern void __cdecl _tile_storeconfig(void *);
extern void __cdecl _tile_release(void);

extern void __cdecl _tile_loadd(__tile dst, const void *base, int stride);
extern void __cdecl _tile_loaddrs(__tile dst, const void *base, int stride);
extern void __cdecl _tile_loaddrst1(__tile dst, const void *base, int stride);
extern void __cdecl _tile_stream_loadd(__tile dst, const void *base, int stride);
extern void __cdecl _tile_stored(__tile src, void *base, int stride);
extern void __cdecl _tile_zero(__tile dst);

extern void __cdecl _tile_dpbf16ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dpfp16ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dpbssd(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dpbsud(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dpbusd(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dpbuud(__tile dst, __tile src1, __tile src2);

// AMX-COMPLEX
extern void __cdecl _tile_cmmimfp16ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_cmmrlfp16ps(__tile dst, __tile src1, __tile src2);

// AMX-FP8
extern void __cdecl _tile_dpbf8ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dpbhf8ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dphbf8ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_dphf8ps(__tile dst, __tile src1, __tile src2);

// AMX-TF32
extern void __cdecl _tile_mmultf32ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_tmmultf32ps(__tile dst, __tile src1, __tile src2);

// AMX-TRANSPOSE
extern void __cdecl _tile_2rpntlvwz0(__tile dst, const void *base, int stride);
extern void __cdecl _tile_2rpntlvwz0t1(__tile dst, const void *base, int stride);
extern void __cdecl _tile_2rpntlvwz1(__tile dst, const void *base, int stride);
extern void __cdecl _tile_2rpntlvwz1t1(__tile dst, const void *base, int stride);
extern void __cdecl _tile_transposed(__tile dst, __tile src1);

// AMX-TRANSPOSE / AMX-MOVRS
extern void __cdecl _tile_2rpntlvwz0rs(__tile dst, const void *base, int stride);
extern void __cdecl _tile_2rpntlvwz0rst1(__tile dst, const void *base, int stride);
extern void __cdecl _tile_2rpntlvwz1rs(__tile dst, const void *base, int stride);
extern void __cdecl _tile_2rpntlvwz1rst1(__tile dst, const void *base, int stride);

// AMX-TRANSPOSE / AMX-COMPLEX
extern void __cdecl _tile_conjtcmmimfp16ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_conjtfp16(__tile dst, __tile src1);
extern void __cdecl _tile_tcmmimfp16ps(__tile dst, __tile src1, __tile src2);
extern void __cdecl _tile_tcmmrlfp16ps(__tile dst, __tile src1, __tile src2);

// AMX-TRANSPOSE / AMX-BF16
extern void __cdecl _tile_tdpbf16ps(__tile dst, __tile src1, __tile src2);

// AMX-TRANSPOSE / AMX-FP16
extern void __cdecl _tile_tdpfp16ps(__tile dst, __tile src1, __tile src2);

#endif  /* defined (_M_X64) */

/* hfni */
typedef __m128i __m128h;
typedef __m256i __m256h;
typedef __m512i __m512h;

#if defined (_M_X64)

// AMX-AVX512 / AVX10.2
extern __m512   __cdecl _tile_cvtrowd2ps(__tile dst, unsigned int a);
extern __m512   __cdecl _tile_cvtrowd2psi(__tile dst, const unsigned int);
extern __m512bh __cdecl _tile_cvtrowps2bf16h(__tile dst, unsigned int a);
extern __m512bh __cdecl _tile_cvtrowps2bf16hi(__tile dst, const unsigned int);
extern __m512bh __cdecl _tile_cvtrowps2bf16l(__tile dst, unsigned int a);
extern __m512bh __cdecl _tile_cvtrowps2bf16li(__tile dst, const unsigned int);
extern __m512h  __cdecl _tile_cvtrowps2phh(__tile dst, unsigned int a);
extern __m512h  __cdecl _tile_cvtrowps2phhi(__tile dst, const unsigned int);
extern __m512h  __cdecl _tile_cvtrowps2phl(__tile dst, unsigned int a);
extern __m512h  __cdecl _tile_cvtrowps2phli(__tile dst, const unsigned int);
extern __m512   __cdecl _tile_movrow(__tile dst, unsigned int a);
extern __m512   __cdecl _tile_movrowi(__tile dst, const unsigned int);

#endif

// VADDPH
extern __m128h __cdecl _mm_add_ph(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_add_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_add_ph(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_add_ph(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_add_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_add_ph(__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_add_ph(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_add_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_add_ph(__mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_add_round_ph(__m512h, __m512h, int);
extern __m512h __cdecl _mm512_mask_add_round_ph(__m512h, __mmask32, __m512h, __m512h, int);
extern __m512h __cdecl _mm512_maskz_add_round_ph(__mmask32, __m512h, __m512h, int);

// VADDSH
extern __m128h __cdecl _mm_add_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_add_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_add_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_add_round_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_add_round_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_add_round_sh(__mmask8, __m128h, __m128h, int);
/* Intrinsics for broadcasting an fp16 value to vector */
/*short float data type is not supported in utc*/
// extern __m128h _mm_set1_ph(short float);
// extern __m256h _mm256_set1_ph(short float);
// extern __m512h _mm512_set1_ph(short float);

// VCMPPH
extern __mmask8 __cdecl _mm_cmp_ph_mask(__m128h, __m128h, const int);
extern __mmask8 __cdecl _mm_mask_cmp_ph_mask(__mmask8, __m128h, __m128h, const int);
extern __mmask16 __cdecl _mm256_cmp_ph_mask(__m256h, __m256h, const int);
extern __mmask16 __cdecl _mm256_mask_cmp_ph_mask(__mmask16, __m256h, __m256h, const int);
extern __mmask32 __cdecl _mm512_cmp_ph_mask(__m512h, __m512h, const int);
extern __mmask32 __cdecl _mm512_mask_cmp_ph_mask(__mmask32, __m512h, __m512h, const int);
extern __mmask32 __cdecl _mm512_cmp_round_ph_mask(__m512h, __m512h, const int, const int);
extern __mmask32 __cdecl _mm512_mask_cmp_round_ph_mask(__mmask32, __m512h, __m512h, const int, const int);

// VCMPSH
extern __mmask8 __cdecl _mm_cmp_sh_mask(__m128h, __m128h, const int);
extern __mmask8 __cdecl _mm_mask_cmp_sh_mask(__mmask8, __m128h, __m128h, const int);
extern __mmask8 __cdecl _mm_cmp_round_sh_mask(__m128h, __m128h, const int, const int);
extern __mmask8 __cdecl _mm_mask_cmp_round_sh_mask(__mmask8, __m128h, __m128h, const int, const int);

// VCOMISH
extern int __cdecl _mm_comi_sh(__m128h, __m128h, const int);
extern int __cdecl _mm_comi_round_sh(__m128h, __m128h, const int, const int);

// VCVTDQ2PH
extern __m128h __cdecl _mm_cvtepi32_ph(__m128i);
extern __m128h __cdecl _mm_mask_cvtepi32_ph(__m128h, __mmask8, __m128i);
extern __m128h __cdecl _mm_maskz_cvtepi32_ph(__mmask8, __m128i);
extern __m128h __cdecl _mm256_cvtepi32_ph(__m256i);
extern __m128h __cdecl _mm256_mask_cvtepi32_ph(__m128h, __mmask8, __m256i);
extern __m128h __cdecl _mm256_maskz_cvtepi32_ph(__mmask8, __m256i);
extern __m256h __cdecl _mm512_cvtepi32_ph (__m512i);
extern __m256h __cdecl _mm512_mask_cvtepi32_ph (__m256h, __mmask16, __m512i);
extern __m256h __cdecl _mm512_maskz_cvtepi32_ph (__mmask16, __m512i);
extern __m256h __cdecl _mm512_cvt_roundepi32_ph(__m512i, int);
extern __m256h __cdecl _mm512_mask_cvt_roundepi32_ph(__m256h, __mmask16, __m512i, int);
extern __m256h __cdecl _mm512_maskz_cvt_roundepi32_ph(__mmask16, __m512i, int);

//VCVTPD2PH
extern __m128h __cdecl _mm_cvtpd_ph(__m128d);
extern __m128h __cdecl _mm_mask_cvtpd_ph(__m128h, __mmask8, __m128d);
extern __m128h __cdecl _mm_maskz_cvtpd_ph(__mmask8, __m128d);
extern __m128h __cdecl _mm256_cvtpd_ph(__m256d);
extern __m128h __cdecl _mm256_mask_cvtpd_ph(__m128h, __mmask8, __m256d);
extern __m128h __cdecl _mm256_maskz_cvtpd_ph(__mmask8, __m256d);
extern __m128h __cdecl _mm512_cvtpd_ph (__m512d);
extern __m128h __cdecl _mm512_mask_cvtpd_ph (__m128h, __mmask8, __m512d);
extern __m128h __cdecl _mm512_maskz_cvtpd_ph (__mmask8, __m512d);
extern __m128h __cdecl _mm512_cvt_roundpd_ph(__m512d, int);
extern __m128h __cdecl _mm512_mask_cvt_roundpd_ph(__m128h, __mmask8, __m512d, int);
extern __m128h __cdecl _mm512_maskz_cvt_roundpd_ph(__mmask8, __m512d, int);

// VCVTPH2DQ
extern __m128i __cdecl _mm_cvtph_epi32(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_epi32(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_epi32(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvtph_epi32(__m128h);
extern __m256i __cdecl _mm256_mask_cvtph_epi32(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvtph_epi32(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvtph_epi32 (__m256h);
extern __m512i __cdecl _mm512_mask_cvtph_epi32 (__m512i, __mmask16, __m256h);
extern __m512i __cdecl _mm512_maskz_cvtph_epi32 (__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvt_roundph_epi32(__m256h, int);
extern __m512i __cdecl _mm512_mask_cvt_roundph_epi32(__m512i, __mmask16, __m256h, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundph_epi32(__mmask16, __m256h, int);

// VCVTPH2PD
extern __m128d __cdecl _mm_cvtph_pd(__m128h);
extern __m128d __cdecl _mm_mask_cvtph_pd(__m128d, __mmask8, __m128h);
extern __m128d __cdecl _mm_maskz_cvtph_pd(__mmask8, __m128h);
extern __m256d __cdecl _mm256_cvtph_pd(__m128h);
extern __m256d __cdecl _mm256_mask_cvtph_pd(__m256d, __mmask8, __m128h);
extern __m256d __cdecl _mm256_maskz_cvtph_pd(__mmask8, __m128h);
extern __m512d __cdecl _mm512_cvtph_pd (__m128h);
extern __m512d __cdecl _mm512_mask_cvtph_pd (__m512d, __mmask8, __m128h);
extern __m512d __cdecl _mm512_maskz_cvtph_pd (__mmask8, __m128h);
extern __m512d __cdecl _mm512_cvt_roundph_pd(__m128h, int);
extern __m512d __cdecl _mm512_mask_cvt_roundph_pd(__m512d, __mmask8, __m128h, int);
extern __m512d __cdecl _mm512_maskz_cvt_roundph_pd(__mmask8, __m128h, int);

// VCVTPH2PSX
extern __m128 __cdecl _mm_cvtxph_ps(__m128h);
extern __m128 __cdecl _mm_mask_cvtxph_ps(__m128, __mmask8, __m128h);
extern __m128 __cdecl _mm_maskz_cvtxph_ps(__mmask8, __m128h);
extern __m256 __cdecl _mm256_cvtxph_ps(__m128h);
extern __m256 __cdecl _mm256_mask_cvtxph_ps(__m256, __mmask8, __m128h);
extern __m256 __cdecl _mm256_maskz_cvtxph_ps(__mmask8, __m128h);
extern __m512 __cdecl _mm512_cvtxph_ps(__m256h);
extern __m512 __cdecl _mm512_mask_cvtxph_ps(__m512, __mmask16, __m256h);
extern __m512 __cdecl _mm512_maskz_cvtxph_ps(__mmask16, __m256h);
extern __m512 __cdecl _mm512_cvtx_roundph_ps(__m256h, int);
extern __m512 __cdecl _mm512_mask_cvtx_roundph_ps(__m512, __mmask16, __m256h, int);
extern __m512 __cdecl _mm512_maskz_cvtx_roundph_ps(__mmask16, __m256h, int);

// VCVTPH2QQ
extern __m128i __cdecl _mm_cvtph_epi64(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_epi64(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_epi64(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvtph_epi64(__m128h);
extern __m256i __cdecl _mm256_mask_cvtph_epi64(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvtph_epi64(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvtph_epi64 (__m128h);
extern __m512i __cdecl _mm512_mask_cvtph_epi64 (__m512i, __mmask8, __m128h);
extern __m512i __cdecl _mm512_maskz_cvtph_epi64 (__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvt_roundph_epi64(__m128h, int);
extern __m512i __cdecl _mm512_mask_cvt_roundph_epi64(__m512i, __mmask8, __m128h, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundph_epi64(__mmask8, __m128h, int);

// VCVTPH2UDQ
extern __m128i __cdecl _mm_cvtph_epu32(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_epu32(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_epu32(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvtph_epu32(__m128h);
extern __m256i __cdecl _mm256_mask_cvtph_epu32(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvtph_epu32(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvtph_epu32 (__m256h);
extern __m512i __cdecl _mm512_mask_cvtph_epu32 (__m512i, __mmask16, __m256h);
extern __m512i __cdecl _mm512_maskz_cvtph_epu32 (__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvt_roundph_epu32(__m256h, int);
extern __m512i __cdecl _mm512_mask_cvt_roundph_epu32(__m512i, __mmask16, __m256h, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundph_epu32(__mmask16, __m256h, int);

// VCVTPH2UQQ
extern __m128i __cdecl _mm_cvtph_epu64(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_epu64(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_epu64(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvtph_epu64(__m128h);
extern __m256i __cdecl _mm256_mask_cvtph_epu64(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvtph_epu64(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvtph_epu64 (__m128h);
extern __m512i __cdecl _mm512_mask_cvtph_epu64 (__m512i, __mmask8, __m128h);
extern __m512i __cdecl _mm512_maskz_cvtph_epu64 (__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvt_roundph_epu64(__m128h, int);
extern __m512i __cdecl _mm512_mask_cvt_roundph_epu64(__m512i, __mmask8, __m128h, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundph_epu64(__mmask8, __m128h, int);

// VCVTPH2UW
extern __m128i __cdecl _mm_cvtph_epu16(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_epu16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_epu16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvtph_epu16(__m256h);
extern __m256i __cdecl _mm256_mask_cvtph_epu16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_cvtph_epu16(__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvtph_epu16 (__m512h);
extern __m512i __cdecl _mm512_mask_cvtph_epu16 (__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_cvtph_epu16 (__mmask32, __m512h);
extern __m512i __cdecl _mm512_cvt_roundph_epu16(__m512h, int);
extern __m512i __cdecl _mm512_mask_cvt_roundph_epu16(__m512i, __mmask32, __m512h, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundph_epu16(__mmask32, __m512h, int);

// VCVTPH2W
extern __m128i __cdecl _mm_cvtph_epi16(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_epi16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_epi16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvtph_epi16(__m256h);
extern __m256i __cdecl _mm256_mask_cvtph_epi16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_cvtph_epi16(__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvtph_epi16 (__m512h);
extern __m512i __cdecl _mm512_mask_cvtph_epi16 (__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_cvtph_epi16 (__mmask32, __m512h);
extern __m512i __cdecl _mm512_cvt_roundph_epi16(__m512h, int);
extern __m512i __cdecl _mm512_mask_cvt_roundph_epi16(__m512i, __mmask32, __m512h, int);
extern __m512i __cdecl _mm512_maskz_cvt_roundph_epi16(__mmask32, __m512h, int);

// VCVTPS2PHX
extern __m128h __cdecl _mm_cvtxps_ph(__m128);
extern __m128h __cdecl _mm_mask_cvtxps_ph(__m128h, __mmask8, __m128);
extern __m128h __cdecl _mm_maskz_cvtxps_ph(__mmask8, __m128);
extern __m128h __cdecl _mm256_cvtxps_ph(__m256);
extern __m128h __cdecl _mm256_mask_cvtxps_ph(__m128h, __mmask8, __m256);
extern __m128h __cdecl _mm256_maskz_cvtxps_ph(__mmask8, __m256);
extern __m256h __cdecl _mm512_cvtxps_ph(__m512);
extern __m256h __cdecl _mm512_mask_cvtxps_ph(__m256h, __mmask16, __m512);
extern __m256h __cdecl _mm512_maskz_cvtxps_ph(__mmask16, __m512);
extern __m256h __cdecl _mm512_cvtx_roundps_ph(__m512, int);
extern __m256h __cdecl _mm512_mask_cvtx_roundps_ph(__m256h, __mmask16, __m512, int);
extern __m256h __cdecl _mm512_maskz_cvtx_roundps_ph(__mmask16, __m512, int);

// VCVTQQ2PH
extern __m128h __cdecl _mm_cvtepi64_ph(__m128i);
extern __m128h __cdecl _mm_mask_cvtepi64_ph(__m128h, __mmask8, __m128i);
extern __m128h __cdecl _mm_maskz_cvtepi64_ph(__mmask8, __m128i);
extern __m128h __cdecl _mm256_cvtepi64_ph(__m256i);
extern __m128h __cdecl _mm256_mask_cvtepi64_ph(__m128h, __mmask8, __m256i);
extern __m128h __cdecl _mm256_maskz_cvtepi64_ph(__mmask8, __m256i);
extern __m128h __cdecl _mm512_cvtepi64_ph (__m512i);
extern __m128h __cdecl _mm512_mask_cvtepi64_ph (__m128h, __mmask8, __m512i);
extern __m128h __cdecl _mm512_maskz_cvtepi64_ph (__mmask8, __m512i);
extern __m128h __cdecl _mm512_cvt_roundepi64_ph(__m512i, int);
extern __m128h __cdecl _mm512_mask_cvt_roundepi64_ph(__m128h, __mmask8, __m512i, int);
extern __m128h __cdecl _mm512_maskz_cvt_roundepi64_ph(__mmask8, __m512i, int);

// VCVTSD2SH
extern __m128h __cdecl _mm_cvtsd_sh(__m128h, __m128d);
extern __m128h __cdecl _mm_mask_cvtsd_sh(__m128h, __mmask8, __m128h, __m128d);
extern __m128h __cdecl _mm_maskz_cvtsd_sh(__mmask8, __m128h, __m128d);
extern __m128h __cdecl _mm_cvt_roundsd_sh(__m128h, __m128d, const int);
extern __m128h __cdecl _mm_mask_cvt_roundsd_sh(__m128h, __mmask8, __m128h, __m128d, const int);
extern __m128h __cdecl _mm_maskz_cvt_roundsd_sh(__mmask8, __m128h, __m128d, const int);

// VCVTSH2SD
extern __m128d __cdecl _mm_cvtsh_sd(__m128d, __m128h);
extern __m128d __cdecl _mm_mask_cvtsh_sd(__m128d, __mmask8, __m128d, __m128h);
extern __m128d __cdecl _mm_maskz_cvtsh_sd(__mmask8, __m128d, __m128h);
extern __m128d __cdecl _mm_cvt_roundsh_sd(__m128d, __m128h, const int);
extern __m128d __cdecl _mm_mask_cvt_roundsh_sd(__m128d, __mmask8, __m128d, __m128h, const int);
extern __m128d __cdecl _mm_maskz_cvt_roundsh_sd(__mmask8, __m128d, __m128h, const int);

// VCVTSH2SI
extern int __cdecl _mm_cvtsh_i32(__m128h);
extern __int64 __cdecl _mm_cvtsh_i64(__m128h);
extern int __cdecl _mm_cvt_roundsh_i32(__m128h, int);
extern __int64 __cdecl _mm_cvt_roundsh_i64(__m128h, int);

// VCVTSH2SS
extern __m128 __cdecl _mm_cvtsh_ss(__m128, __m128h);
extern __m128 __cdecl _mm_mask_cvtsh_ss(__m128, __mmask8, __m128, __m128h);
extern __m128 __cdecl _mm_maskz_cvtsh_ss(__mmask8, __m128, __m128h);
extern __m128 __cdecl _mm_cvt_roundsh_ss(__m128, __m128h, const int);
extern __m128 __cdecl _mm_mask_cvt_roundsh_ss(__m128, __mmask8, __m128, __m128h, const int);
extern __m128 __cdecl _mm_maskz_cvt_roundsh_ss(__mmask8, __m128, __m128h, const int);

// VCVTSH2USI
extern unsigned int __cdecl _mm_cvtsh_u32(__m128h);
extern unsigned __int64 __cdecl _mm_cvtsh_u64(__m128h);
extern unsigned int __cdecl _mm_cvt_roundsh_u32(__m128h, int);
extern unsigned __int64 __cdecl _mm_cvt_roundsh_u64(__m128h, int);

// VCVTSI2SH
extern __m128h __cdecl _mm_cvti32_sh(__m128h, int);
extern __m128h __cdecl _mm_cvti64_sh(__m128h, __int64);
extern __m128h __cdecl _mm_cvt_roundi32_sh(__m128h, int, int);
extern __m128h __cdecl _mm_cvt_roundi64_sh(__m128h, __int64, int);

// VCVTSS2SH
extern __m128h __cdecl _mm_cvtss_sh(__m128h, __m128);
extern __m128h __cdecl _mm_mask_cvtss_sh(__m128h, __mmask8, __m128h, __m128);
extern __m128h __cdecl _mm_maskz_cvtss_sh(__mmask8, __m128h, __m128);
extern __m128h __cdecl _mm_cvt_roundss_sh(__m128h, __m128, const int);
extern __m128h __cdecl _mm_mask_cvt_roundss_sh(__m128h, __mmask8, __m128h, __m128, const int);
extern __m128h __cdecl _mm_maskz_cvt_roundss_sh(__mmask8, __m128h, __m128, const int);

// VCVTTPH2DQ
extern __m128i __cdecl _mm_cvttph_epi32(__m128h);
extern __m128i __cdecl _mm_mask_cvttph_epi32(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvttph_epi32(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvttph_epi32(__m128h);
extern __m256i __cdecl _mm256_mask_cvttph_epi32(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvttph_epi32(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvttph_epi32 (__m256h);
extern __m512i __cdecl _mm512_mask_cvttph_epi32 (__m512i, __mmask16, __m256h);
extern __m512i __cdecl _mm512_maskz_cvttph_epi32 (__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvtt_roundph_epi32(__m256h, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundph_epi32(__m512i, __mmask16, __m256h, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundph_epi32(__mmask16, __m256h, int);

// VCVTTPH2QQ
extern __m128i __cdecl _mm_cvttph_epi64(__m128h);
extern __m128i __cdecl _mm_mask_cvttph_epi64(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvttph_epi64(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvttph_epi64(__m128h);
extern __m256i __cdecl _mm256_mask_cvttph_epi64(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvttph_epi64(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvttph_epi64 (__m128h);
extern __m512i __cdecl _mm512_mask_cvttph_epi64 (__m512i, __mmask8, __m128h);
extern __m512i __cdecl _mm512_maskz_cvttph_epi64 (__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvtt_roundph_epi64(__m128h, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundph_epi64(__m512i, __mmask8, __m128h, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundph_epi64(__mmask8, __m128h, int);

// VCVTTPH2UDQ
extern __m128i __cdecl _mm_cvttph_epu32(__m128h);
extern __m128i __cdecl _mm_mask_cvttph_epu32(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvttph_epu32(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvttph_epu32(__m128h);
extern __m256i __cdecl _mm256_mask_cvttph_epu32(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvttph_epu32(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvttph_epu32 (__m256h);
extern __m512i __cdecl _mm512_mask_cvttph_epu32 (__m512i, __mmask16, __m256h);
extern __m512i __cdecl _mm512_maskz_cvttph_epu32 (__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvtt_roundph_epu32(__m256h, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundph_epu32(__m512i, __mmask16, __m256h, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundph_epu32(__mmask16, __m256h, int);

// VCVTTPH2UQQ
extern __m128i __cdecl _mm_cvttph_epu64(__m128h);
extern __m128i __cdecl _mm_mask_cvttph_epu64(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvttph_epu64(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvttph_epu64(__m128h);
extern __m256i __cdecl _mm256_mask_cvttph_epu64(__m256i, __mmask8, __m128h);
extern __m256i __cdecl _mm256_maskz_cvttph_epu64(__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvttph_epu64 (__m128h);
extern __m512i __cdecl _mm512_mask_cvttph_epu64 (__m512i, __mmask8, __m128h);
extern __m512i __cdecl _mm512_maskz_cvttph_epu64 (__mmask8, __m128h);
extern __m512i __cdecl _mm512_cvtt_roundph_epu64(__m128h, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundph_epu64(__m512i, __mmask8, __m128h, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundph_epu64(__mmask8, __m128h, int);

// VCVTTPH2UW
extern __m128i __cdecl _mm_cvttph_epu16(__m128h);
extern __m128i __cdecl _mm_mask_cvttph_epu16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvttph_epu16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvttph_epu16(__m256h);
extern __m256i __cdecl _mm256_mask_cvttph_epu16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_cvttph_epu16(__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvttph_epu16 (__m512h);
extern __m512i __cdecl _mm512_mask_cvttph_epu16 (__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_cvttph_epu16 (__mmask32, __m512h);
extern __m512i __cdecl _mm512_cvtt_roundph_epu16(__m512h, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundph_epu16(__m512i, __mmask32, __m512h, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundph_epu16(__mmask32, __m512h, int);

// VCVTTPH2W
extern __m128i __cdecl _mm_cvttph_epi16(__m128h);
extern __m128i __cdecl _mm_mask_cvttph_epi16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvttph_epi16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_cvttph_epi16(__m256h);
extern __m256i __cdecl _mm256_mask_cvttph_epi16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_cvttph_epi16(__mmask16, __m256h);
extern __m512i __cdecl _mm512_cvttph_epi16 (__m512h);
extern __m512i __cdecl _mm512_mask_cvttph_epi16 (__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_cvttph_epi16 (__mmask32, __m512h);
extern __m512i __cdecl _mm512_cvtt_roundph_epi16(__m512h, int);
extern __m512i __cdecl _mm512_mask_cvtt_roundph_epi16(__m512i, __mmask32, __m512h, int);
extern __m512i __cdecl _mm512_maskz_cvtt_roundph_epi16(__mmask32, __m512h, int);

// VCVTTSH2SI
extern int __cdecl _mm_cvttsh_i32(__m128h);
extern __int64 __cdecl _mm_cvttsh_i64(__m128h);
extern int __cdecl _mm_cvtt_roundsh_i32(__m128h, int);
extern __int64 __cdecl _mm_cvtt_roundsh_i64(__m128h, int);

// VCVTTSH2USI
extern unsigned int __cdecl _mm_cvttsh_u32(__m128h);
extern unsigned __int64 __cdecl _mm_cvttsh_u64(__m128h);
extern unsigned int __cdecl _mm_cvtt_roundsh_u32(__m128h, int);
extern unsigned __int64 __cdecl _mm_cvtt_roundsh_u64(__m128h, int);

//VCVTUDQ2PH
extern __m128h __cdecl _mm_cvtepu32_ph(__m128i);
extern __m128h __cdecl _mm_mask_cvtepu32_ph(__m128h, __mmask8, __m128i);
extern __m128h __cdecl _mm_maskz_cvtepu32_ph(__mmask8, __m128i);
extern __m128h __cdecl _mm256_cvtepu32_ph(__m256i);
extern __m128h __cdecl _mm256_mask_cvtepu32_ph(__m128h, __mmask8, __m256i);
extern __m128h __cdecl _mm256_maskz_cvtepu32_ph(__mmask8, __m256i);
extern __m256h __cdecl _mm512_cvtepu32_ph (__m512i);
extern __m256h __cdecl _mm512_mask_cvtepu32_ph (__m256h, __mmask16, __m512i);
extern __m256h __cdecl _mm512_maskz_cvtepu32_ph (__mmask16, __m512i);
extern __m256h __cdecl _mm512_cvt_roundepu32_ph(__m512i, int);
extern __m256h __cdecl _mm512_mask_cvt_roundepu32_ph(__m256h, __mmask16, __m512i, int);
extern __m256h __cdecl _mm512_maskz_cvt_roundepu32_ph(__mmask16, __m512i, int);

// VCVTUQQ2PH
extern __m128h __cdecl _mm_cvtepu64_ph(__m128i);
extern __m128h __cdecl _mm_mask_cvtepu64_ph(__m128h, __mmask8, __m128i);
extern __m128h __cdecl _mm_maskz_cvtepu64_ph(__mmask8, __m128i);
extern __m128h __cdecl _mm256_cvtepu64_ph(__m256i);
extern __m128h __cdecl _mm256_mask_cvtepu64_ph(__m128h, __mmask8, __m256i);
extern __m128h __cdecl _mm256_maskz_cvtepu64_ph(__mmask8, __m256i);
extern __m128h __cdecl _mm512_cvtepu64_ph (__m512i);
extern __m128h __cdecl _mm512_mask_cvtepu64_ph (__m128h, __mmask8, __m512i);
extern __m128h __cdecl _mm512_maskz_cvtepu64_ph (__mmask8, __m512i);
extern __m128h __cdecl _mm512_cvt_roundepu64_ph(__m512i, int);
extern __m128h __cdecl _mm512_mask_cvt_roundepu64_ph(__m128h, __mmask8, __m512i, int);
extern __m128h __cdecl _mm512_maskz_cvt_roundepu64_ph(__mmask8, __m512i, int);

// VCVTUSI2SH
extern __m128h __cdecl _mm_cvtu32_sh(__m128h, unsigned int);
extern __m128h __cdecl _mm_cvtu64_sh(__m128h, unsigned __int64);
extern __m128h __cdecl _mm_cvt_roundu32_sh(__m128h, unsigned int, int);
extern __m128h __cdecl _mm_cvt_roundu64_sh(__m128h, unsigned __int64, int);

// VCVTUW2PH
extern __m128h __cdecl _mm_cvtepu16_ph(__m128i);
extern __m128h __cdecl _mm_mask_cvtepu16_ph(__m128h, __mmask8, __m128i);
extern __m128h __cdecl _mm_maskz_cvtepu16_ph(__mmask8, __m128i);
extern __m256h __cdecl _mm256_cvtepu16_ph(__m256i);
extern __m256h __cdecl _mm256_mask_cvtepu16_ph(__m256h, __mmask16, __m256i);
extern __m256h __cdecl _mm256_maskz_cvtepu16_ph(__mmask16, __m256i);
extern __m512h __cdecl _mm512_cvtepu16_ph (__m512i);
extern __m512h __cdecl _mm512_mask_cvtepu16_ph (__m512h, __mmask32, __m512i);
extern __m512h __cdecl _mm512_maskz_cvtepu16_ph (__mmask32, __m512i);
extern __m512h __cdecl _mm512_cvt_roundepu16_ph(__m512i, int);
extern __m512h __cdecl _mm512_mask_cvt_roundepu16_ph(__m512h, __mmask32, __m512i, int);
extern __m512h __cdecl _mm512_maskz_cvt_roundepu16_ph(__mmask32, __m512i, int);

// VCVTW2PH
extern __m128h __cdecl _mm_cvtepi16_ph(__m128i);
extern __m128h __cdecl _mm_mask_cvtepi16_ph(__m128h, __mmask8, __m128i);
extern __m128h __cdecl _mm_maskz_cvtepi16_ph(__mmask8, __m128i);
extern __m256h __cdecl _mm256_cvtepi16_ph(__m256i);
extern __m256h __cdecl _mm256_mask_cvtepi16_ph(__m256h, __mmask16, __m256i);
extern __m256h __cdecl _mm256_maskz_cvtepi16_ph(__mmask16, __m256i);
extern __m512h __cdecl _mm512_cvtepi16_ph (__m512i);
extern __m512h __cdecl _mm512_mask_cvtepi16_ph (__m512h, __mmask32, __m512i);
extern __m512h __cdecl _mm512_maskz_cvtepi16_ph (__mmask32, __m512i);
extern __m512h __cdecl _mm512_cvt_roundepi16_ph(__m512i, int);
extern __m512h __cdecl _mm512_mask_cvt_roundepi16_ph(__m512h, __mmask32, __m512i, int);
extern __m512h __cdecl _mm512_maskz_cvt_roundepi16_ph(__mmask32, __m512i, int);

// VDIVPH
extern __m128h __cdecl _mm_div_ph(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_div_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_div_ph(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_div_ph(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_div_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_div_ph(__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_div_ph(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_div_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_div_ph(__mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_div_round_ph(__m512h, __m512h, int);
extern __m512h __cdecl _mm512_mask_div_round_ph(__m512h, __mmask32, __m512h, __m512h, int);
extern __m512h __cdecl _mm512_maskz_div_round_ph(__mmask32, __m512h, __m512h, int);

// VDIVSH
extern __m128h __cdecl _mm_div_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_div_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_div_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_div_round_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_div_round_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_div_round_sh(__mmask8, __m128h, __m128h, int);

// VFMADDSUB[132,213,231]PH
extern __m128h __cdecl _mm_fmaddsub_ph(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmaddsub_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmaddsub_ph(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmaddsub_ph(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fmaddsub_ph(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fmaddsub_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fmaddsub_ph(__m256h, __m256h, __m256h, __mmask16);
extern __m256h __cdecl _mm256_maskz_fmaddsub_ph(__mmask16, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fmaddsub_ph(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fmaddsub_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fmaddsub_ph(__m512h, __m512h, __m512h, __mmask32);
extern __m512h __cdecl _mm512_maskz_fmaddsub_ph(__mmask32, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fmaddsub_round_ph(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fmaddsub_round_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fmaddsub_round_ph(__m512h, __m512h, __m512h, __mmask32, const int);
extern __m512h __cdecl _mm512_maskz_fmaddsub_round_ph(__mmask32, __m512h, __m512h, __m512h, const int);

// VFMSUBADD[132,213,231]PH
extern __m128h __cdecl _mm_fmsubadd_ph(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmsubadd_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmsubadd_ph(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmsubadd_ph(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fmsubadd_ph(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fmsubadd_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fmsubadd_ph(__m256h, __m256h, __m256h, __mmask16);
extern __m256h __cdecl _mm256_maskz_fmsubadd_ph(__mmask16, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fmsubadd_ph(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fmsubadd_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fmsubadd_ph(__m512h, __m512h, __m512h, __mmask32);
extern __m512h __cdecl _mm512_maskz_fmsubadd_ph(__mmask32, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fmsubadd_round_ph(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fmsubadd_round_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fmsubadd_round_ph(__m512h, __m512h, __m512h, __mmask32, const int);
extern __m512h __cdecl _mm512_maskz_fmsubadd_round_ph(__mmask32, __m512h, __m512h, __m512h, const int);

// VFPCLASSPH
extern __mmask8 __cdecl _mm_fpclass_ph_mask(__m128h, int);
extern __mmask8 __cdecl _mm_mask_fpclass_ph_mask(__mmask8, __m128h, int);
extern __mmask16 __cdecl _mm256_fpclass_ph_mask(__m256h, int);
extern __mmask16 __cdecl _mm256_mask_fpclass_ph_mask(__mmask16, __m256h, int);
extern __mmask32 __cdecl _mm512_fpclass_ph_mask(__m512h, int);
extern __mmask32 __cdecl _mm512_mask_fpclass_ph_mask(__mmask32, __m512h, int);

// VFPCLASSSH
extern __mmask8 __cdecl _mm_fpclass_sh_mask(__m128h, int);
extern __mmask8 __cdecl _mm_mask_fpclass_sh_mask(__mmask8, __m128h, int);

// VF[,C]MADDCPH
extern __m128h __cdecl _mm_fmadd_pch(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmadd_pch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmadd_pch(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmadd_pch(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fmadd_pch(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fmadd_pch(__m256h, __mmask8, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fmadd_pch(__m256h, __m256h, __m256h, __mmask8);
extern __m256h __cdecl _mm256_maskz_fmadd_pch(__mmask8, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fmadd_pch(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fmadd_pch(__m512h, __mmask16, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fmadd_pch(__m512h, __m512h, __m512h, __mmask16);
extern __m512h __cdecl _mm512_maskz_fmadd_pch(__mmask16, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fmadd_round_pch(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fmadd_round_pch(__m512h, __mmask16, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fmadd_round_pch(__m512h, __m512h, __m512h, __mmask16, const int);
extern __m512h __cdecl _mm512_maskz_fmadd_round_pch(__mmask16, __m512h, __m512h, __m512h, const int);
extern __m128h __cdecl _mm_fcmadd_pch(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fcmadd_pch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fcmadd_pch(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fcmadd_pch(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fcmadd_pch(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fcmadd_pch(__m256h, __mmask8, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fcmadd_pch(__m256h, __m256h, __m256h, __mmask8);
extern __m256h __cdecl _mm256_maskz_fcmadd_pch(__mmask8, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fcmadd_pch(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fcmadd_pch(__m512h, __mmask16, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fcmadd_pch(__m512h, __m512h, __m512h, __mmask16);
extern __m512h __cdecl _mm512_maskz_fcmadd_pch(__mmask16, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fcmadd_round_pch(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fcmadd_round_pch(__m512h, __mmask16, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fcmadd_round_pch(__m512h, __m512h, __m512h, __mmask16, const int);
extern __m512h __cdecl _mm512_maskz_fcmadd_round_pch(__mmask16, __m512h, __m512h, __m512h, const int);

// VF[,C]MADDCSH
extern __m128h __cdecl _mm_fcmadd_sch(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fcmadd_sch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fcmadd_sch(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fcmadd_sch(__mmask8, __m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_fcmadd_round_sch(__m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fcmadd_round_sch(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask3_fcmadd_round_sch(__m128h, __m128h, __m128h, __mmask8, const int);
extern __m128h __cdecl _mm_maskz_fcmadd_round_sch(__mmask8, __m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_fmadd_sch(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmadd_sch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmadd_sch(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmadd_sch(__mmask8, __m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_fmadd_round_sch(__m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fmadd_round_sch(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask3_fmadd_round_sch(__m128h, __m128h, __m128h, __mmask8, const int);
extern __m128h __cdecl _mm_maskz_fmadd_round_sch(__mmask8, __m128h, __m128h, __m128h, const int);

// VF[,C]MULCPH
extern __m128h __cdecl _mm_fcmul_pch(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_fcmul_pch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_fcmul_pch(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_fcmul_pch(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fcmul_pch(__m256h, __mmask8, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_fcmul_pch(__mmask8, __m256h, __m256h);
extern __m512h __cdecl _mm512_fcmul_pch(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fcmul_pch(__m512h, __mmask16, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_fcmul_pch(__mmask16, __m512h, __m512h);
extern __m512h __cdecl _mm512_fcmul_round_pch(__m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fcmul_round_pch(__m512h, __mmask16, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_maskz_fcmul_round_pch(__mmask16, __m512h, __m512h, const int);
extern __m128h __cdecl _mm_fmul_pch(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmul_pch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_fmul_pch(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_fmul_pch(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fmul_pch(__m256h, __mmask8, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_fmul_pch(__mmask8, __m256h, __m256h);
extern __m512h __cdecl _mm512_fmul_pch(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fmul_pch(__m512h, __mmask16, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_fmul_pch(__mmask16, __m512h, __m512h);
extern __m512h __cdecl _mm512_fmul_round_pch(__m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fmul_round_pch(__m512h, __mmask16, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_maskz_fmul_round_pch(__mmask16, __m512h, __m512h, const int);

// VF[,C]MULCSH
extern __m128h __cdecl _mm_fcmul_sch(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_fcmul_sch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_fcmul_sch(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_fcmul_round_sch(__m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fcmul_round_sch(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_maskz_fcmul_round_sch(__mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_fmul_sch(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmul_sch(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_fmul_sch(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_fmul_round_sch(__m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fmul_round_sch(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_maskz_fmul_round_sch(__mmask8, __m128h, __m128h, const int);

// VF[,N]MADD[132,213,231]PH
extern __m128h __cdecl _mm_fnmadd_ph(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fnmadd_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fnmadd_ph(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fnmadd_ph(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fnmadd_ph(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fnmadd_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fnmadd_ph(__m256h, __m256h, __m256h, __mmask16);
extern __m256h __cdecl _mm256_maskz_fnmadd_ph(__mmask16, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fnmadd_ph(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fnmadd_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fnmadd_ph(__m512h, __m512h, __m512h, __mmask32);
extern __m512h __cdecl _mm512_maskz_fnmadd_ph(__mmask32, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fnmadd_round_ph(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fnmadd_round_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fnmadd_round_ph(__m512h, __m512h, __m512h, __mmask32, const int);
extern __m512h __cdecl _mm512_maskz_fnmadd_round_ph(__mmask32, __m512h, __m512h, __m512h, const int);
extern __m128h __cdecl _mm_fmadd_ph(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmadd_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmadd_ph(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmadd_ph(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fmadd_ph(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fmadd_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fmadd_ph(__m256h, __m256h, __m256h, __mmask16);
extern __m256h __cdecl _mm256_maskz_fmadd_ph(__mmask16, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fmadd_ph(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fmadd_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fmadd_ph(__m512h, __m512h, __m512h, __mmask32);
extern __m512h __cdecl _mm512_maskz_fmadd_ph(__mmask32, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fmadd_round_ph(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fmadd_round_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fmadd_round_ph(__m512h, __m512h, __m512h, __mmask32, const int);
extern __m512h __cdecl _mm512_maskz_fmadd_round_ph(__mmask32, __m512h, __m512h, __m512h, const int);

// VF[,N]MADD[132,213,231]SH
extern __m128h __cdecl _mm_fnmadd_sh(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fnmadd_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fnmadd_sh(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fnmadd_sh(__mmask8, __m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_fnmadd_round_sh(__m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fnmadd_round_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask3_fnmadd_round_sh(__m128h, __m128h, __m128h, __mmask8, const int);
extern __m128h __cdecl _mm_maskz_fnmadd_round_sh(__mmask8, __m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_fmadd_sh(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmadd_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmadd_sh(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmadd_sh(__mmask8, __m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_fmadd_round_sh(__m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fmadd_round_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask3_fmadd_round_sh(__m128h, __m128h, __m128h, __mmask8, const int);
extern __m128h __cdecl _mm_maskz_fmadd_round_sh(__mmask8, __m128h, __m128h, __m128h, const int);

// VF[,N]MSUB[132,213,231]PH
extern __m128h __cdecl _mm_fnmsub_ph(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fnmsub_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fnmsub_ph(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fnmsub_ph(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fnmsub_ph(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fnmsub_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fnmsub_ph(__m256h, __m256h, __m256h, __mmask16);
extern __m256h __cdecl _mm256_maskz_fnmsub_ph(__mmask16, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fnmsub_ph(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fnmsub_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fnmsub_ph(__m512h, __m512h, __m512h, __mmask32);
extern __m512h __cdecl _mm512_maskz_fnmsub_ph(__mmask32, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fnmsub_round_ph(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fnmsub_round_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fnmsub_round_ph(__m512h, __m512h, __m512h, __mmask32, const int);
extern __m512h __cdecl _mm512_maskz_fnmsub_round_ph(__mmask32, __m512h, __m512h, __m512h, const int);
extern __m128h __cdecl _mm_fmsub_ph(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmsub_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmsub_ph(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmsub_ph(__mmask8, __m128h, __m128h, __m128h);
extern __m256h __cdecl _mm256_fmsub_ph(__m256h, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask_fmsub_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_mask3_fmsub_ph(__m256h, __m256h, __m256h, __mmask16);
extern __m256h __cdecl _mm256_maskz_fmsub_ph(__mmask16, __m256h, __m256h, __m256h);
extern __m512h __cdecl _mm512_fmsub_ph(__m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask_fmsub_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_mask3_fmsub_ph(__m512h, __m512h, __m512h, __mmask32);
extern __m512h __cdecl _mm512_maskz_fmsub_ph(__mmask32, __m512h, __m512h, __m512h);
extern __m512h __cdecl _mm512_fmsub_round_ph(__m512h, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_fmsub_round_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask3_fmsub_round_ph(__m512h, __m512h, __m512h, __mmask32, const int);
extern __m512h __cdecl _mm512_maskz_fmsub_round_ph(__mmask32, __m512h, __m512h, __m512h, const int);

// VF[,N]MSUB[132,213,231]SH
extern __m128h __cdecl _mm_fnmsub_sh(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fnmsub_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fnmsub_sh(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fnmsub_sh(__mmask8, __m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_fnmsub_round_sh(__m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fnmsub_round_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask3_fnmsub_round_sh(__m128h, __m128h, __m128h, __mmask8, const int);
extern __m128h __cdecl _mm_maskz_fnmsub_round_sh(__mmask8, __m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_fmsub_sh(__m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_mask_fmsub_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mask3_fmsub_sh(__m128h, __m128h, __m128h, __mmask8);
extern __m128h __cdecl _mm_maskz_fmsub_sh(__mmask8, __m128h, __m128h, __m128h);
extern __m128h __cdecl _mm_fmsub_round_sh(__m128h, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_fmsub_round_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask3_fmsub_round_sh(__m128h, __m128h, __m128h, __mmask8, const int);
extern __m128h __cdecl _mm_maskz_fmsub_round_sh(__mmask8, __m128h, __m128h, __m128h, const int);

// VGETEXPPH
extern __m128h __cdecl _mm_getexp_ph(__m128h);
extern __m128h __cdecl _mm_mask_getexp_ph(__m128h, __mmask8, __m128h);
extern __m128h __cdecl _mm_maskz_getexp_ph(__mmask8, __m128h);
extern __m256h __cdecl _mm256_getexp_ph(__m256h);
extern __m256h __cdecl _mm256_mask_getexp_ph(__m256h, __mmask16, __m256h);
extern __m256h __cdecl _mm256_maskz_getexp_ph(__mmask16, __m256h);
extern __m512h __cdecl _mm512_getexp_ph(__m512h);
extern __m512h __cdecl _mm512_mask_getexp_ph(__m512h, __mmask32, __m512h);
extern __m512h __cdecl _mm512_maskz_getexp_ph(__mmask32, __m512h);
extern __m512h __cdecl _mm512_getexp_round_ph(__m512h, const int);
extern __m512h __cdecl _mm512_mask_getexp_round_ph(__m512h, __mmask32, __m512h, const int);
extern __m512h __cdecl _mm512_maskz_getexp_round_ph(__mmask32, __m512h, const int);

// VGETEXPSH
extern __m128h __cdecl _mm_getexp_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_getexp_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_getexp_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_getexp_round_sh(__m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_getexp_round_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_maskz_getexp_round_sh(__mmask8, __m128h, __m128h, const int);

// VGETMANTPH
extern __m128h __cdecl _mm_getmant_ph(__m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128h __cdecl _mm_mask_getmant_ph(__m128h, __mmask8, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128h __cdecl _mm_maskz_getmant_ph(__mmask8, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m256h __cdecl _mm256_getmant_ph(__m256h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m256h __cdecl _mm256_mask_getmant_ph(__m256h, __mmask16, __m256h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m256h __cdecl _mm256_maskz_getmant_ph(__mmask16, __m256h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m512h __cdecl _mm512_getmant_ph(__m512h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m512h __cdecl _mm512_mask_getmant_ph(__m512h, __mmask32, __m512h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m512h __cdecl _mm512_maskz_getmant_ph(__mmask32, __m512h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m512h __cdecl _mm512_getmant_round_ph(__m512h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM, const int);
extern __m512h __cdecl _mm512_mask_getmant_round_ph(__m512h, __mmask32, __m512h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM, const int);
extern __m512h __cdecl _mm512_maskz_getmant_round_ph(__mmask32, __m512h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM, const int);

// VGETMANTSH
extern __m128h __cdecl _mm_getmant_sh(__m128h, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128h __cdecl _mm_mask_getmant_sh(__m128h, __mmask8, __m128h, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128h __cdecl _mm_maskz_getmant_sh(__mmask8, __m128h, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128h __cdecl _mm_getmant_round_sh(__m128h, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM, const int);
extern __m128h __cdecl _mm_mask_getmant_round_sh(__m128h, __mmask8, __m128h, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM, const int);
extern __m128h __cdecl _mm_maskz_getmant_round_sh(__mmask8, __m128h, __m128h, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM, const int);

// VMAXPH
extern __m128h __cdecl _mm_max_ph(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_max_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_max_ph(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_max_ph(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_max_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_max_ph(__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_max_ph(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_max_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_max_ph(__mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_max_round_ph(__m512h, __m512h, int);
extern __m512h __cdecl _mm512_mask_max_round_ph(__m512h, __mmask32, __m512h, __m512h, int);
extern __m512h __cdecl _mm512_maskz_max_round_ph(__mmask32, __m512h, __m512h, int);

// VMAXSH
extern __m128h __cdecl _mm_max_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_max_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_max_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_max_round_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_max_round_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_max_round_sh(__mmask8, __m128h, __m128h, int);

// VMINPH
extern __m128h __cdecl _mm_min_ph(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_min_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_min_ph(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_min_ph(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_min_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_min_ph(__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_min_ph(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_min_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_min_ph(__mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_min_round_ph(__m512h, __m512h, int);
extern __m512h __cdecl _mm512_mask_min_round_ph(__m512h, __mmask32, __m512h, __m512h, int);
extern __m512h __cdecl _mm512_maskz_min_round_ph(__mmask32, __m512h, __m512h, int);

// VMINSH
extern __m128h __cdecl _mm_min_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_min_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_min_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_min_round_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_min_round_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_min_round_sh(__mmask8, __m128h, __m128h, int);

// VMOVSH
extern __m128h __cdecl _mm_load_sh(void const*);
extern __m128h __cdecl _mm_mask_load_sh(__m128h, __mmask8, void const*);
extern __m128h __cdecl _mm_maskz_load_sh(__mmask8, void const*);
extern void __cdecl _mm_store_sh(void*, __m128h);
extern void __cdecl _mm_mask_store_sh(void*, __mmask8, __m128h);
extern __m128h __cdecl _mm_move_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_move_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_move_sh(__mmask8, __m128h, __m128h);

// VMOVW
extern __m128i __cdecl _mm_cvtsi16_si128(short);
extern short __cdecl _mm_cvtsi128_si16(__m128i);

// MULPH
extern __m128h __cdecl _mm_mul_ph(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_mul_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_mul_ph(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_mul_ph(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_mul_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_mul_ph(__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_mul_ph(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_mul_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_mul_ph(__mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_mul_round_ph(__m512h, __m512h, int);
extern __m512h __cdecl _mm512_mask_mul_round_ph(__m512h, __mmask32, __m512h, __m512h, int);
extern __m512h __cdecl _mm512_maskz_mul_round_ph(__mmask32, __m512h, __m512h, int);

// VMULSH
extern __m128h __cdecl _mm_mul_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_mul_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_mul_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_mul_round_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_mul_round_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_mul_round_sh(__mmask8, __m128h, __m128h, int);

// VRCPPH
extern __m128h __cdecl _mm_rcp_ph(__m128h);
extern __m128h __cdecl _mm_mask_rcp_ph(__m128h, __mmask8, __m128h);
extern __m128h __cdecl _mm_maskz_rcp_ph(__mmask8, __m128h);
extern __m256h __cdecl _mm256_rcp_ph(__m256h);
extern __m256h __cdecl _mm256_mask_rcp_ph(__m256h, __mmask16, __m256h);
extern __m256h __cdecl _mm256_maskz_rcp_ph(__mmask16, __m256h);
extern __m512h __cdecl _mm512_rcp_ph(__m512h);
extern __m512h __cdecl _mm512_mask_rcp_ph(__m512h, __mmask32, __m512h);
extern __m512h __cdecl _mm512_maskz_rcp_ph(__mmask32, __m512h);

// VRCPSH
extern __m128h __cdecl _mm_rcp_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_rcp_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_rcp_sh(__mmask8, __m128h, __m128h);

// VREDUCEPH
extern __m128h __cdecl _mm_reduce_ph(__m128h, int);
extern __m128h __cdecl _mm_mask_reduce_ph(__m128h, __mmask8, __m128h, int);
extern __m128h __cdecl _mm_maskz_reduce_ph(__mmask8, __m128h, int);
extern __m256h __cdecl _mm256_reduce_ph(__m256h, int);
extern __m256h __cdecl _mm256_mask_reduce_ph(__m256h, __mmask16, __m256h, int);
extern __m256h __cdecl _mm256_maskz_reduce_ph(__mmask16, __m256h, int);
extern __m512h __cdecl _mm512_reduce_ph(__m512h, int);
extern __m512h __cdecl _mm512_mask_reduce_ph(__m512h, __mmask32, __m512h, int);
extern __m512h __cdecl _mm512_maskz_reduce_ph(__mmask32, __m512h, int);
extern __m512h __cdecl _mm512_reduce_round_ph(__m512h, int, const int);
extern __m512h __cdecl _mm512_mask_reduce_round_ph(__m512h, __mmask32, __m512h, int, const int);
extern __m512h __cdecl _mm512_maskz_reduce_round_ph(__mmask32, __m512h, int, const int);

// VREDUCESH
extern __m128h __cdecl _mm_reduce_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_reduce_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_reduce_sh(__mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_reduce_round_sh(__m128h, __m128h, int, const int);
extern __m128h __cdecl _mm_mask_reduce_round_sh(__m128h, __mmask8, __m128h, __m128h, int, const int);
extern __m128h __cdecl _mm_maskz_reduce_round_sh(__mmask8, __m128h, __m128h, int, const int);

// VRNDSCALEPH
extern __m128h __cdecl _mm_roundscale_ph(__m128h, int);
extern __m128h __cdecl _mm_mask_roundscale_ph(__m128h, __mmask8, __m128h, int);
extern __m128h __cdecl _mm_maskz_roundscale_ph(__mmask8, __m128h, int);
extern __m256h __cdecl _mm256_roundscale_ph(__m256h, int);
extern __m256h __cdecl _mm256_mask_roundscale_ph(__m256h, __mmask16, __m256h, int);
extern __m256h __cdecl _mm256_maskz_roundscale_ph(__mmask16, __m256h, int);
extern __m512h __cdecl _mm512_roundscale_ph(__m512h, int);
extern __m512h __cdecl _mm512_mask_roundscale_ph(__m512h, __mmask32, __m512h, int);
extern __m512h __cdecl _mm512_maskz_roundscale_ph(__mmask32, __m512h, int);
extern __m512h __cdecl _mm512_roundscale_round_ph(__m512h, int, const int);
extern __m512h __cdecl _mm512_mask_roundscale_round_ph(__m512h, __mmask32, __m512h, int, const int);
extern __m512h __cdecl _mm512_maskz_roundscale_round_ph(__mmask32, __m512h, int, const int);

// VRNDSCALESH
extern __m128h __cdecl _mm_roundscale_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_roundscale_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_roundscale_sh(__mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_roundscale_round_sh(__m128h, __m128h, int, const int);
extern __m128h __cdecl _mm_mask_roundscale_round_sh(__m128h, __mmask8, __m128h, __m128h, int, const int);
extern __m128h __cdecl _mm_maskz_roundscale_round_sh(__mmask8, __m128h, __m128h, int, const int);

// VRSQRTPH
extern __m128h __cdecl _mm_rsqrt_ph(__m128h);
extern __m128h __cdecl _mm_mask_rsqrt_ph(__m128h, __mmask8, __m128h);
extern __m128h __cdecl _mm_maskz_rsqrt_ph(__mmask8, __m128h);
extern __m256h __cdecl _mm256_rsqrt_ph(__m256h);
extern __m256h __cdecl _mm256_mask_rsqrt_ph(__m256h, __mmask16, __m256h);
extern __m256h __cdecl _mm256_maskz_rsqrt_ph(__mmask16, __m256h);
extern __m512h __cdecl _mm512_rsqrt_ph(__m512h);
extern __m512h __cdecl _mm512_mask_rsqrt_ph(__m512h, __mmask32, __m512h);
extern __m512h __cdecl _mm512_maskz_rsqrt_ph(__mmask32, __m512h);

// VRSQRTSH
extern __m128h __cdecl _mm_rsqrt_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_rsqrt_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_rsqrt_sh(__mmask8, __m128h, __m128h);

// VSCALEFPH
extern __m128h __cdecl _mm_scalef_ph(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_scalef_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_scalef_ph(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_scalef_ph(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_scalef_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_scalef_ph(__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_scalef_ph(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_scalef_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_scalef_ph(__mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_scalef_round_ph(__m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_scalef_round_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_maskz_scalef_round_ph(__mmask32, __m512h, __m512h, const int);

// VSCALEFSH
extern __m128h __cdecl _mm_scalef_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_scalef_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_scalef_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_scalef_round_sh(__m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_scalef_round_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_maskz_scalef_round_sh(__mmask8, __m128h, __m128h, const int);

// VSQRTPH
extern __m128h __cdecl _mm_sqrt_ph(__m128h);
extern __m128h __cdecl _mm_mask_sqrt_ph(__m128h, __mmask8, __m128h);
extern __m128h __cdecl _mm_maskz_sqrt_ph(__mmask8, __m128h);
extern __m256h __cdecl _mm256_sqrt_ph(__m256h);
extern __m256h __cdecl _mm256_mask_sqrt_ph(__m256h, __mmask16, __m256h);
extern __m256h __cdecl _mm256_maskz_sqrt_ph(__mmask16, __m256h);
extern __m512h __cdecl _mm512_sqrt_ph(__m512h);
extern __m512h __cdecl _mm512_mask_sqrt_ph(__m512h, __mmask32, __m512h);
extern __m512h __cdecl _mm512_maskz_sqrt_ph(__mmask32, __m512h);
extern __m512h __cdecl _mm512_sqrt_round_ph(__m512h, const int);
extern __m512h __cdecl _mm512_mask_sqrt_round_ph(__m512h, __mmask32, __m512h, const int);
extern __m512h __cdecl _mm512_maskz_sqrt_round_ph(__mmask32, __m512h, const int);

// VSQRTSH
extern __m128h __cdecl _mm_sqrt_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_sqrt_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_sqrt_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_sqrt_round_sh(__m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_sqrt_round_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_maskz_sqrt_round_sh(__mmask8, __m128h, __m128h, const int);

// VSUBPH
extern __m128h __cdecl _mm_sub_ph(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_sub_ph(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_sub_ph(__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_sub_ph(__m256h, __m256h);
extern __m256h __cdecl _mm256_mask_sub_ph(__m256h, __mmask16, __m256h, __m256h);
extern __m256h __cdecl _mm256_maskz_sub_ph(__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_sub_ph(__m512h, __m512h);
extern __m512h __cdecl _mm512_mask_sub_ph(__m512h, __mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_maskz_sub_ph(__mmask32, __m512h, __m512h);
extern __m512h __cdecl _mm512_sub_round_ph(__m512h, __m512h, int);
extern __m512h __cdecl _mm512_mask_sub_round_ph(__m512h, __mmask32, __m512h, __m512h, int);
extern __m512h __cdecl _mm512_maskz_sub_round_ph(__mmask32, __m512h, __m512h, int);

// VSUBSH
extern __m128h __cdecl _mm_sub_sh(__m128h, __m128h);
extern __m128h __cdecl _mm_mask_sub_sh(__m128h, __mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_maskz_sub_sh(__mmask8, __m128h, __m128h);
extern __m128h __cdecl _mm_sub_round_sh(__m128h, __m128h, int);
extern __m128h __cdecl _mm_mask_sub_round_sh(__m128h, __mmask8, __m128h, __m128h, int);
extern __m128h __cdecl _mm_maskz_sub_round_sh(__mmask8, __m128h, __m128h, int);

extern __m128h __cdecl _mm_mask_blend_ph (__mmask8, __m128h, __m128h);
extern __m256h __cdecl _mm256_mask_blend_ph (__mmask16, __m256h, __m256h);
extern __m512h __cdecl _mm512_mask_blend_ph (__mmask32, __m512h, __m512h);

// AVX512VL RSQRT14 intrinsics
extern __m256  __cdecl _mm256_rsqrt14_ps(__m256);
extern __m256d __cdecl _mm256_rsqrt14_pd(__m256d);
extern __m128  __cdecl _mm_rsqrt14_ps(__m128);
extern __m128d __cdecl _mm_rsqrt14_pd(__m128d);

// Scalar FP intrinsics (with double/float arguments)
extern float  __cdecl __getexp_ss(float);
extern double __cdecl __getexp_sd(double);
extern float  __cdecl __getmant_ss(float, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern double __cdecl __getmant_sd(double, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern float  __cdecl __scalef_ss(float, float);
extern double __cdecl __scalef_sd(double, double);

// EVEX SM4
extern __m512i __cdecl _mm512_sm4key4_epi32(__m512i, __m512i);
extern __m512i __cdecl _mm512_sm4rnds4_epi32(__m512i, __m512i);

// Macros for fmul_*ch intrinsics
#define _mm_mul_pch(A, B) _mm_fmul_pch(A, B)
#define _mm_mask_mul_pch(W, U, A, B) _mm_mask_fmul_pch(W, U, A, B)
#define _mm_maskz_mul_pch(U, A, B) _mm_maskz_fmul_pch(U, A, B)

#define _mm256_mul_pch(A, B) _mm256_fmul_pch(A, B)
#define _mm256_mask_mul_pch(W, U, A, B) _mm256_mask_fmul_pch(W, U, A, B)
#define _mm256_maskz_mul_pch(U, A, B) _mm256_maskz_fmul_pch(U, A, B)

#define _mm512_mul_pch(A, B) _mm512_fmul_pch(A, B)
#define _mm512_mask_mul_pch(W, U, A, B) _mm512_mask_fmul_pch(W, U, A, B)
#define _mm512_maskz_mul_pch(U, A, B) _mm512_maskz_fmul_pch(U, A, B)

#define _mm512_mul_round_pch(A, B, R) _mm512_fmul_round_pch(A, B, R)
#define _mm512_mask_mul_round_pch(W, U, A, B, R) \
                                _mm512_mask_fmul_round_pch(W, U, A, B, R)
#define _mm512_maskz_mul_round_pch(U, A, B, R) \
                                _mm512_maskz_fmul_round_pch(U, A, B, R)

#define _mm_mul_sch(A, B) _mm_fmul_sch(A, B)
#define _mm_mask_mul_sch(W, U, A, B) _mm_mask_fmul_sch(W, U, A, B)
#define _mm_maskz_mul_sch(U, A, B) _mm_maskz_fmul_sch(U, A, B)

#define _mm_mul_round_sch(A, B, R) _mm_fmul_round_sch(A, B, R)
#define _mm_mask_mul_round_sch(W, U, A, B, R) \
                                    _mm_mask_fmul_round_sch(W, U, A, B, R)
#define _mm_maskz_mul_round_sch(U, A, B, R) _mm_maskz_fmul_round_sch(U, A, B, R)

// Macros for fcmul_*ch intrinsics
#define _mm_cmul_pch(A, B) _mm_fcmul_pch(A, B)
#define _mm_mask_cmul_pch(W, U, A, B) _mm_mask_fcmul_pch(W, U, A, B)
#define _mm_maskz_cmul_pch(U, A, B) _mm_maskz_fcmul_pch(U, A, B)

#define _mm256_cmul_pch(A, B) _mm256_fcmul_pch(A, B)
#define _mm256_mask_cmul_pch(W, U, A, B) _mm256_mask_fcmul_pch(W, U, A, B)
#define _mm256_maskz_cmul_pch(U, A, B) _mm256_maskz_fcmul_pch(U, A, B)

#define _mm512_cmul_pch(A, B) _mm512_fcmul_pch(A, B)
#define _mm512_mask_cmul_pch(W, U, A, B) _mm512_mask_fcmul_pch(W, U, A, B)
#define _mm512_maskz_cmul_pch(U, A, B) _mm512_maskz_fcmul_pch(U, A, B)

#define _mm512_cmul_round_pch(A, B, R) _mm512_fcmul_round_pch(A, B, R)
#define _mm512_mask_cmul_round_pch(W, U, A, B, R) \
                                _mm512_mask_fcmul_round_pch(W, U, A, B, R)
#define _mm512_maskz_cmul_round_pch(U, A, B, R) \
                                _mm512_maskz_fcmul_round_pch(U, A, B, R)

#define _mm_cmul_sch(A, B) _mm_fcmul_sch(A, B)
#define _mm_mask_cmul_sch(W, U, A, B) _mm_mask_fcmul_sch(W, U, A, B)
#define _mm_maskz_cmul_sch(U, A, B) _mm_maskz_fcmul_sch(U, A, B)

#define _mm_cmul_round_sch(A, B, R) _mm_fcmul_round_sch(A, B, R)
#define _mm_mask_cmul_round_sch(W, U, A, B, R) \
                                _mm_mask_fcmul_round_sch(W, U, A, B, R)
#define _mm_maskz_cmul_round_sch(U, A, B, R) \
                                _mm_maskz_fcmul_round_sch(U, A, B, R)

// FP16 type conversion macros
#define _mm_castph_ps(x)            _mm_castsi128_ps(x)
#define _mm256_castph_ps(x)         _mm256_castsi256_ps(x)
#define _mm512_castph_ps(x)         _mm512_castsi512_ps(x)
#define _mm_castph_pd(x)            _mm_castsi128_pd(x)
#define _mm256_castph_pd(x)         _mm256_castsi256_pd(x)
#define _mm512_castph_pd(x)         _mm512_castsi512_pd(x)
#define _mm_castph_si128(x)         (x)
#define _mm256_castph_si256(x)      (x)
#define _mm512_castph_si512(x)      (x)
#define _mm_castps_ph(x)            _mm_castps_si128(x)
#define _mm256_castps_ph(x)         _mm256_castps_si256(x)
#define _mm512_castps_ph(x)         _mm512_castps_si512(x)
#define _mm_castpd_ph(x)            _mm_castpd_si128(x)
#define _mm256_castpd_ph(x)         _mm256_castpd_si256(x)
#define _mm512_castpd_ph(x)         _mm512_castpd_si512(x)
#define _mm_castsi128_ph(x)         (x)
#define _mm256_castsi256_ph(x)      (x)
#define _mm512_castsi512_ph(x)      (x)
#define _mm256_castph256_ph128(x)   _mm256_castsi256_si128(x)
#define _mm512_castph512_ph128(x)   _mm512_castsi512_si128(x)
#define _mm512_castph512_ph256(x)   _mm512_castsi512_si256(x)
#define _mm256_castph128_ph256(x)   _mm256_castsi128_si256(x)
#define _mm512_castph128_ph512(x)   _mm512_castsi128_si512(x)
#define _mm512_castph256_ph512(x)   _mm512_castsi256_si512(x)
#define _mm256_zextph128_ph256(x)   _mm256_zextsi128_si256(x)
#define _mm512_zextph128_ph512(x)   _mm512_zextsi128_si512(x)
#define _mm512_zextph256_ph512(x)   _mm512_zextsi256_si512(x)

#define _mm_comieq_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_EQ_OQ)
#define _mm_comilt_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_LT_OQ)
#define _mm_comile_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_LE_OQ)
#define _mm_comigt_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_GT_OQ)
#define _mm_comige_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_GE_OQ)
#define _mm_comineq_sh(v1, v2) _mm_comi_sh((v1), (v2), _CMP_NEQ_OQ)

#define _mm_ucomieq_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_EQ_OS)
#define _mm_ucomilt_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_LT_OS)
#define _mm_ucomile_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_LE_OS)
#define _mm_ucomigt_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_GT_OS)
#define _mm_ucomige_sh(v1, v2)  _mm_comi_sh((v1), (v2), _CMP_GE_OS)
#define _mm_ucomineq_sh(v1, v2) _mm_comi_sh((v1), (v2), _CMP_NEQ_OS)

// FP16 intrinsic macros
#define _mm_abs_ph(a)                    _mm_and_epi32(_mm_set1_epi32(0x7FFF7FFF), (a))
#define _mm256_abs_ph(a)                 _mm256_and_epi32(_mm256_set1_epi32(0x7FFF7FFF), (a))
#define _mm512_abs_ph(a)                 _mm512_and_epi32(_mm512_set1_epi32(0x7FFF7FFF), (a))

#define _mm_conj_pch(a)                  _mm_xor_epi32((a), _mm_set1_epi32(0x80000000))
#define _mm_mask_conj_pch(src, k, a)     _mm_mask_xor_epi32((src), (k), (a), _mm_set1_epi32(0x80000000))
#define _mm_maskz_conj_pch(k, a)         _mm_maskz_xor_epi32((k), (a), _mm_set1_epi32(0x80000000))

#define _mm256_conj_pch(a)               _mm256_xor_epi32((a), _mm256_set1_epi32(0x80000000))
#define _mm256_mask_conj_pch(src, k, a)  _mm256_mask_xor_epi32((src), (k), (a), _mm256_set1_epi32(0x80000000))
#define _mm256_maskz_conj_pch(k, a)      _mm256_maskz_xor_epi32((k), (a), _mm256_set1_epi32(0x80000000))

#define _mm512_conj_pch(a)               _mm512_xor_epi32((a), _mm512_set1_epi32(0x80000000))
#define _mm512_mask_conj_pch(src, k, a)  _mm512_mask_xor_epi32((src), (k), (a), _mm512_set1_epi32(0x80000000))
#define _mm512_maskz_conj_pch(k, a)      _mm512_maskz_xor_epi32((k), (a), _mm512_set1_epi32(0x80000000))

#define _mm_load_ph(x)                   _mm_castps_ph(_mm_load_ps((float *)(x)))
#define _mm256_load_ph(x)                _mm256_castps_ph(_mm256_load_ps((float *)(x)))
#define _mm512_load_ph(x)                _mm512_castps_ph(_mm512_load_ps(x))

#define _mm_loadu_ph(x)                  _mm_castps_ph(_mm_loadu_ps((float *)(x)))
#define _mm256_loadu_ph(x)               _mm256_castps_ph(_mm256_loadu_ps((float *)(x)))
#define _mm512_loadu_ph(x)               _mm512_castps_ph(_mm512_loadu_ps(x))

#define _mm_store_ph(x, a)               _mm_store_ps((float *)(x), _mm_castph_ps(a))
#define _mm256_store_ph(x, a)            _mm256_store_ps((float *)(x), _mm256_castph_ps(a))
#define _mm512_store_ph(x, a)            _mm512_store_ps((x), _mm512_castph_ps(a))

#define _mm_storeu_ph(x, a)              _mm_storeu_ps((float *)(x), _mm_castph_ps(a))
#define _mm256_storeu_ph(x, a)           _mm256_storeu_ps((float *)(x), _mm256_castph_ps(a))
#define _mm512_storeu_ph(x, a)           _mm512_storeu_ps((x), _mm512_castph_ps(a))

#define _mm_setzero_ph()                 _mm_castps_ph(_mm_setzero_ps())
#define _mm256_setzero_ph()              _mm256_castps_ph(_mm256_setzero_ps())
#define _mm512_setzero_ph()              _mm512_castps_ph(_mm512_setzero_ps())

#define _mm_undefined_ph()               _mm_undefined_si128()
#define _mm256_undefined_ph()            _mm256_undefined_si256()
#define _mm512_undefined_ph()            _mm512_undefined_epi32()

#define _mm_permutex2var_ph(a, i, b)     _mm_permutex2var_epi16((a), (i), (b))
#define _mm256_permutex2var_ph(a, i, b)  _mm256_permutex2var_epi16((a), (i), (b))
#define _mm512_permutex2var_ph(a, i, b)  _mm512_permutex2var_epi16((a), (i), (b))

#define _mm_permutexvar_ph(i, a)         _mm_permutexvar_epi16((i), (a))
#define _mm256_permutexvar_ph(i, a)      _mm256_permutexvar_epi16((i), (a))
#define _mm512_permutexvar_ph(i, a)      _mm512_permutexvar_epi16((i), (a))

// MOVRS instructions
#if defined (_M_X64)
extern __m128i __cdecl _mm_loadrs_epi8(void const *);
extern __m128i __cdecl _mm_mask_loadrs_epi8(__m128i, __mmask16, void const *);
extern __m128i __cdecl _mm_maskz_loadrs_epi8(__mmask16, void const *);

extern __m256i __cdecl _mm256_loadrs_epi8(void const *);
extern __m256i __cdecl _mm256_mask_loadrs_epi8(__m256i, __mmask32, void const *);
extern __m256i __cdecl _mm256_maskz_loadrs_epi8(__mmask32, void const *);

extern __m512i __cdecl _mm512_loadrs_epi8(void const *);
extern __m512i __cdecl _mm512_mask_loadrs_epi8(__m512i, __mmask64, void const *);
extern __m512i __cdecl _mm512_maskz_loadrs_epi8(__mmask64, void const *);

extern __m128i __cdecl _mm_loadrs_epi16(void const *);
extern __m128i __cdecl _mm_mask_loadrs_epi16(__m128i, __mmask8, void const *);
extern __m128i __cdecl _mm_maskz_loadrs_epi16(__mmask8, void const *);

extern __m256i __cdecl _mm256_loadrs_epi16(void const *);
extern __m256i __cdecl _mm256_mask_loadrs_epi16(__m256i, __mmask16, void const *);
extern __m256i __cdecl _mm256_maskz_loadrs_epi16(__mmask16, void const *);

extern __m512i __cdecl _mm512_loadrs_epi16(void const *);
extern __m512i __cdecl _mm512_mask_loadrs_epi16(__m512i, __mmask32, void const *);
extern __m512i __cdecl _mm512_maskz_loadrs_epi16(__mmask32, void const *);

extern __m128i __cdecl _mm_loadrs_epi32(void const *);
extern __m128i __cdecl _mm_mask_loadrs_epi32(__m128i, __mmask8, void const *);
extern __m128i __cdecl _mm_maskz_loadrs_epi32(__mmask8, void const *);

extern __m256i __cdecl _mm256_loadrs_epi32(void const *);
extern __m256i __cdecl _mm256_mask_loadrs_epi32(__m256i, __mmask8, void const *);
extern __m256i __cdecl _mm256_maskz_loadrs_epi32(__mmask8, void const *);

extern __m512i __cdecl _mm512_loadrs_epi32(void const *);
extern __m512i __cdecl _mm512_mask_loadrs_epi32(__m512i, __mmask16, void const *);
extern __m512i __cdecl _mm512_maskz_loadrs_epi32(__mmask16, void const *);

extern __m128i __cdecl _mm_loadrs_epi64(void const *);
extern __m128i __cdecl _mm_mask_loadrs_epi64(__m128i, __mmask8, void const *);
extern __m128i __cdecl _mm_maskz_loadrs_epi64(__mmask8, void const *);

extern __m256i __cdecl _mm256_loadrs_epi64(void const *);
extern __m256i __cdecl _mm256_mask_loadrs_epi64(__m256i, __mmask8, void const *);
extern __m256i __cdecl _mm256_maskz_loadrs_epi64(__mmask8, void const *);

extern __m512i __cdecl _mm512_loadrs_epi64(void const *);
extern __m512i __cdecl _mm512_mask_loadrs_epi64(__m512i, __mmask8, void const *);
extern __m512i __cdecl _mm512_maskz_loadrs_epi64(__mmask8, void const *);
#endif // defined (_M_X64)

// AVX10-BF16
extern __m128bh  __cdecl _mm_add_pbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_add_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_maskz_add_pbh(__mmask8, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_add_pbh(__m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_add_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_add_pbh(__mmask16, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_add_pbh(__m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_add_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_add_pbh(__mmask32, __m512bh, __m512bh);
extern __mmask8  __cdecl _mm_cmp_pbh_mask(__m128bh, __m128bh, const int);
extern __mmask8  __cdecl _mm_mask_cmp_pbh_mask(__mmask8, __m128bh, __m128bh, const int);
extern __mmask16 __cdecl _mm256_cmp_pbh_mask(__m256bh, __m256bh, const int);
extern __mmask16 __cdecl _mm256_mask_cmp_pbh_mask(__mmask16, __m256bh, __m256bh, const int);
extern __mmask32 __cdecl _mm512_cmp_pbh_mask(__m512bh, __m512bh, const int);
extern __mmask32 __cdecl _mm512_mask_cmp_pbh_mask(__mmask32, __m512bh, __m512bh, const int);
extern int       __cdecl _mm_comeq_sbh(__m128bh, __m128bh);
extern int       __cdecl _mm_comlt_sbh(__m128bh, __m128bh);
extern int       __cdecl _mm_comle_sbh(__m128bh, __m128bh);
extern int       __cdecl _mm_comgt_sbh(__m128bh, __m128bh);
extern int       __cdecl _mm_comge_sbh(__m128bh, __m128bh);
extern int       __cdecl _mm_comneq_sbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_div_pbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_div_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_maskz_div_pbh(__mmask8, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_div_pbh(__m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_div_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_div_pbh(__mmask16, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_div_pbh(__m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_div_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_div_pbh(__mmask32, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_fmadd_pbh(__m128bh, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_fmadd_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask3_fmadd_pbh(__m128bh, __m128bh, __m128bh, __mmask8);
extern __m128bh  __cdecl _mm_maskz_fmadd_pbh(__mmask8, __m128bh, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_fmadd_pbh(__m256bh, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_fmadd_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask3_fmadd_pbh(__m256bh, __m256bh, __m256bh, __mmask16);
extern __m256bh  __cdecl _mm256_maskz_fmadd_pbh(__mmask16, __m256bh, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_fmadd_pbh(__m512bh, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_fmadd_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask3_fmadd_pbh(__m512bh, __m512bh, __m512bh, __mmask32);
extern __m512bh  __cdecl _mm512_maskz_fmadd_pbh(__mmask32, __m512bh, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_fmsub_pbh(__m128bh, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_fmsub_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask3_fmsub_pbh(__m128bh, __m128bh, __m128bh, __mmask8);
extern __m128bh  __cdecl _mm_maskz_fmsub_pbh(__mmask8, __m128bh, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_fmsub_pbh(__m256bh, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_fmsub_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask3_fmsub_pbh(__m256bh, __m256bh, __m256bh, __mmask16);
extern __m256bh  __cdecl _mm256_maskz_fmsub_pbh(__mmask16, __m256bh, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_fmsub_pbh(__m512bh, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_fmsub_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask3_fmsub_pbh(__m512bh, __m512bh, __m512bh, __mmask32);
extern __m512bh  __cdecl _mm512_maskz_fmsub_pbh(__mmask32, __m512bh, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_fnmadd_pbh(__m128bh, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_fnmadd_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask3_fnmadd_pbh(__m128bh, __m128bh, __m128bh, __mmask8);
extern __m128bh  __cdecl _mm_maskz_fnmadd_pbh(__mmask8, __m128bh, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_fnmadd_pbh(__m256bh, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_fnmadd_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask3_fnmadd_pbh(__m256bh, __m256bh, __m256bh, __mmask16);
extern __m256bh  __cdecl _mm256_maskz_fnmadd_pbh(__mmask16, __m256bh, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_fnmadd_pbh(__m512bh, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_fnmadd_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask3_fnmadd_pbh(__m512bh, __m512bh, __m512bh, __mmask32);
extern __m512bh  __cdecl _mm512_maskz_fnmadd_pbh(__mmask32, __m512bh, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_fnmsub_pbh(__m128bh, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_fnmsub_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask3_fnmsub_pbh(__m128bh, __m128bh, __m128bh, __mmask8);
extern __m128bh  __cdecl _mm_maskz_fnmsub_pbh(__mmask8, __m128bh, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_fnmsub_pbh(__m256bh, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_fnmsub_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask3_fnmsub_pbh(__m256bh, __m256bh, __m256bh, __mmask16);
extern __m256bh  __cdecl _mm256_maskz_fnmsub_pbh(__mmask16, __m256bh, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_fnmsub_pbh(__m512bh, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_fnmsub_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask3_fnmsub_pbh(__m512bh, __m512bh, __m512bh, __mmask32);
extern __m512bh  __cdecl _mm512_maskz_fnmsub_pbh(__mmask32, __m512bh, __m512bh, __m512bh);
extern __mmask8  __cdecl _mm_fpclass_pbh_mask(__m128bh, const int);
extern __mmask8  __cdecl _mm_mask_fpclass_pbh_mask(__mmask8, __m128bh, const int);
extern __mmask16 __cdecl _mm256_fpclass_pbh_mask(__m256bh, const int);
extern __mmask16 __cdecl _mm256_mask_fpclass_pbh_mask(__mmask16, __m256bh, const int);
extern __mmask32 __cdecl _mm512_fpclass_pbh_mask(__m512bh, const int);
extern __mmask32 __cdecl _mm512_mask_fpclass_pbh_mask(__mmask32, __m512bh, const int);
extern __m128bh  __cdecl _mm_getexp_pbh(__m128bh);
extern __m128bh  __cdecl _mm_mask_getexp_pbh(__m128bh, __mmask8, __m128bh);
extern __m128bh  __cdecl _mm_maskz_getexp_pbh(__mmask8, __m128bh);
extern __m256bh  __cdecl _mm256_getexp_pbh(__m256bh);
extern __m256bh  __cdecl _mm256_mask_getexp_pbh(__m256bh, __mmask16, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_getexp_pbh(__mmask16, __m256bh);
extern __m512bh  __cdecl _mm512_getexp_pbh(__m512bh);
extern __m512bh  __cdecl _mm512_mask_getexp_pbh(__m512bh, __mmask32, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_getexp_pbh(__mmask32, __m512bh);
extern __m128bh  __cdecl _mm_getmant_pbh(__m128bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128bh  __cdecl _mm_mask_getmant_pbh(__m128bh, __mmask8, __m128bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128bh  __cdecl _mm_maskz_getmant_pbh(__mmask8, __m128bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m256bh  __cdecl _mm256_getmant_pbh(__m256bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m256bh  __cdecl _mm256_mask_getmant_pbh(__m256bh, __mmask16, __m256bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m256bh  __cdecl _mm256_maskz_getmant_pbh(__mmask16, __m256bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m512bh  __cdecl _mm512_getmant_pbh(__m512bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m512bh  __cdecl _mm512_mask_getmant_pbh(__m512bh, __mmask32, __m512bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m512bh  __cdecl _mm512_maskz_getmant_pbh(__mmask32, __m512bh, _MM_MANTISSA_NORM_ENUM, _MM_MANTISSA_SIGN_ENUM);
extern __m128bh  __cdecl _mm_max_pbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_max_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_maskz_max_pbh(__mmask8, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_max_pbh(__m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_max_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_max_pbh(__mmask16, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_max_pbh(__m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_max_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_max_pbh(__mmask32, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_min_pbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_min_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_maskz_min_pbh(__mmask8, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_min_pbh(__m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_min_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_min_pbh(__mmask16, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_min_pbh(__m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_min_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_min_pbh(__mmask32, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_mul_pbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_mul_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_maskz_mul_pbh(__mmask8, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_mul_pbh(__m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_mul_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_mul_pbh(__mmask16, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_mul_pbh(__m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_mul_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_mul_pbh(__mmask32, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_rcp_pbh(__m128bh);
extern __m128bh  __cdecl _mm_mask_rcp_pbh(__m128bh, __mmask8, __m128bh);
extern __m128bh  __cdecl _mm_maskz_rcp_pbh(__mmask8, __m128bh);
extern __m256bh  __cdecl _mm256_rcp_pbh(__m256bh);
extern __m256bh  __cdecl _mm256_mask_rcp_pbh(__m256bh, __mmask16, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_rcp_pbh(__mmask16, __m256bh);
extern __m512bh  __cdecl _mm512_rcp_pbh(__m512bh);
extern __m512bh  __cdecl _mm512_mask_rcp_pbh(__m512bh, __mmask32, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_rcp_pbh(__mmask32, __m512bh);
extern __m128bh  __cdecl _mm_reduce_pbh(__m128bh, int);
extern __m128bh  __cdecl _mm_mask_reduce_pbh(__m128bh, __mmask8, __m128bh, int);
extern __m128bh  __cdecl _mm_maskz_reduce_pbh(__mmask8, __m128bh, int);
extern __m256bh  __cdecl _mm256_reduce_pbh(__m256bh, int);
extern __m256bh  __cdecl _mm256_mask_reduce_pbh(__m256bh, __mmask16, __m256bh, int);
extern __m256bh  __cdecl _mm256_maskz_reduce_pbh(__mmask16, __m256bh, int);
extern __m512bh  __cdecl _mm512_reduce_pbh(__m512bh, int);
extern __m512bh  __cdecl _mm512_mask_reduce_pbh(__m512bh, __mmask32, __m512bh, int);
extern __m512bh  __cdecl _mm512_maskz_reduce_pbh(__mmask32, __m512bh, int);
extern __m128bh  __cdecl _mm_rndscale_pbh(__m128bh, int);
extern __m128bh  __cdecl _mm_mask_rndscale_pbh(__m128bh, __mmask8, __m128bh, int);
extern __m128bh  __cdecl _mm_maskz_rndscale_pbh(__mmask8, __m128bh, int);
extern __m256bh  __cdecl _mm256_rndscale_pbh(__m256bh, int);
extern __m256bh  __cdecl _mm256_mask_rndscale_pbh(__m256bh, __mmask16, __m256bh, int);
extern __m256bh  __cdecl _mm256_maskz_rndscale_pbh(__mmask16, __m256bh, int);
extern __m512bh  __cdecl _mm512_rndscale_pbh(__m512bh, int);
extern __m512bh  __cdecl _mm512_mask_rndscale_pbh(__m512bh, __mmask32, __m512bh, int);
extern __m512bh  __cdecl _mm512_maskz_rndscale_pbh(__mmask32, __m512bh, int);
extern __m128bh  __cdecl _mm_rsqrt_pbh(__m128bh);
extern __m128bh  __cdecl _mm_mask_rsqrt_pbh(__m128bh, __mmask8, __m128bh);
extern __m128bh  __cdecl _mm_maskz_rsqrt_pbh(__mmask8, __m128bh);
extern __m256bh  __cdecl _mm256_rsqrt_pbh(__m256bh);
extern __m256bh  __cdecl _mm256_mask_rsqrt_pbh(__m256bh, __mmask16, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_rsqrt_pbh(__mmask16, __m256bh);
extern __m512bh  __cdecl _mm512_rsqrt_pbh(__m512bh);
extern __m512bh  __cdecl _mm512_mask_rsqrt_pbh(__m512bh, __mmask32, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_rsqrt_pbh(__mmask32, __m512bh);
extern __m128bh  __cdecl _mm_scalef_pbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_scalef_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_maskz_scalef_pbh(__mmask8, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_scalef_pbh(__m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_scalef_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_scalef_pbh(__mmask16, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_scalef_pbh(__m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_scalef_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_scalef_pbh(__mmask32, __m512bh, __m512bh);
extern __m128bh  __cdecl _mm_sqrt_pbh(__m128bh);
extern __m128bh  __cdecl _mm_mask_sqrt_pbh(__m128bh, __mmask8, __m128bh);
extern __m128bh  __cdecl _mm_maskz_sqrt_pbh(__mmask8, __m128bh);
extern __m256bh  __cdecl _mm256_sqrt_pbh(__m256bh);
extern __m256bh  __cdecl _mm256_mask_sqrt_pbh(__m256bh, __mmask16, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_sqrt_pbh(__mmask16, __m256bh);
extern __m512bh  __cdecl _mm512_sqrt_pbh(__m512bh);
extern __m512bh  __cdecl _mm512_mask_sqrt_pbh(__m512bh, __mmask32, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_sqrt_pbh(__mmask32, __m512bh);
extern __m128bh  __cdecl _mm_sub_pbh(__m128bh, __m128bh);
extern __m128bh  __cdecl _mm_mask_sub_pbh(__m128bh, __mmask8, __m128bh, __m128bh);
extern __m128bh  __cdecl _mm_maskz_sub_pbh(__mmask8, __m128bh, __m128bh);
extern __m256bh  __cdecl _mm256_sub_pbh(__m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_mask_sub_pbh(__m256bh, __mmask16, __m256bh, __m256bh);
extern __m256bh  __cdecl _mm256_maskz_sub_pbh(__mmask16, __m256bh, __m256bh);
extern __m512bh  __cdecl _mm512_sub_pbh(__m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_mask_sub_pbh(__m512bh, __mmask32, __m512bh, __m512bh);
extern __m512bh  __cdecl _mm512_maskz_sub_pbh(__mmask32, __m512bh, __m512bh);

// AVX10-CONVERT
extern __m128h __cdecl _mm_cvtx2ps_ph(__m128, __m128);
extern __m128h __cdecl _mm_mask_cvtx2ps_ph(__m128h, __mmask8, __m128, __m128);
extern __m128h __cdecl _mm_maskz_cvtx2ps_ph(__mmask8, __m128, __m128);
extern __m256h __cdecl _mm256_cvtx2ps_ph(__m256, __m256);
extern __m256h __cdecl _mm256_mask_cvtx2ps_ph(__m256h, __mmask16, __m256, __m256);
extern __m256h __cdecl _mm256_maskz_cvtx2ps_ph(__mmask16, __m256, __m256);
extern __m512h __cdecl _mm512_cvtx2ps_ph(__m512, __m512);
extern __m512h __cdecl _mm512_mask_cvtx2ps_ph(__m512h, __mmask32, __m512, __m512);
extern __m512h __cdecl _mm512_maskz_cvtx2ps_ph(__mmask32, __m512, __m512);
extern __m512h __cdecl _mm512_cvtx_round2ps_ph(__m512, __m512, const int);
extern __m512h __cdecl _mm512_mask_cvtx_round2ps_ph(__m512h, __mmask32, __m512, __m512, const int);
extern __m512h __cdecl _mm512_maskz_cvtx_round2ps_ph(__mmask32, __m512, __m512, const int);
extern __m128i __cdecl _mm_cvtbiasph_pbf8(__m128i, __m128h);
extern __m128i __cdecl _mm_mask_cvtbiasph_pbf8(__m128i, __mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm_maskz_cvtbiasph_pbf8(__mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm256_cvtbiasph_pbf8(__m256i, __m256h);
extern __m128i __cdecl _mm256_mask_cvtbiasph_pbf8(__m128i, __mmask16, __m256i, __m256h);
extern __m128i __cdecl _mm256_maskz_cvtbiasph_pbf8(__mmask16, __m256i, __m256h);
extern __m256i __cdecl _mm512_cvtbiasph_pbf8(__m512i, __m512h);
extern __m256i __cdecl _mm512_mask_cvtbiasph_pbf8(__m256i, __mmask32, __m512i, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtbiasph_pbf8(__mmask32, __m512i, __m512h);
extern __m128i __cdecl _mm_cvtbiassph_pbf8(__m128i, __m128h);
extern __m128i __cdecl _mm_mask_cvtbiassph_pbf8(__m128i, __mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm_maskz_cvtbiassph_pbf8(__mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm256_cvtbiassph_pbf8(__m256i, __m256h);
extern __m128i __cdecl _mm256_mask_cvtbiassph_pbf8(__m128i, __mmask16, __m256i, __m256h);
extern __m128i __cdecl _mm256_maskz_cvtbiassph_pbf8(__mmask16, __m256i, __m256h);
extern __m256i __cdecl _mm512_cvtbiassph_pbf8(__m512i, __m512h);
extern __m256i __cdecl _mm512_mask_cvtbiassph_pbf8(__m256i, __mmask32, __m512i, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtbiassph_pbf8(__mmask32, __m512i, __m512h);
extern __m128i __cdecl _mm_cvtbiasph_phf8(__m128i, __m128h);
extern __m128i __cdecl _mm_mask_cvtbiasph_phf8(__m128i, __mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm_maskz_cvtbiasph_phf8(__mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm256_cvtbiasph_phf8(__m256i, __m256h);
extern __m128i __cdecl _mm256_mask_cvtbiasph_phf8(__m128i, __mmask16, __m256i, __m256h);
extern __m128i __cdecl _mm256_maskz_cvtbiasph_phf8(__mmask16, __m256i, __m256h);
extern __m256i __cdecl _mm512_cvtbiasph_phf8(__m512i, __m512h);
extern __m256i __cdecl _mm512_mask_cvtbiasph_phf8(__m256i, __mmask32, __m512i, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtbiasph_phf8(__mmask32, __m512i, __m512h);
extern __m128i __cdecl _mm_cvtbiassph_phf8(__m128i, __m128h);
extern __m128i __cdecl _mm_mask_cvtbiassph_phf8(__m128i, __mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm_maskz_cvtbiassph_phf8(__mmask8, __m128i, __m128h);
extern __m128i __cdecl _mm256_cvtbiassph_phf8(__m256i, __m256h);
extern __m128i __cdecl _mm256_mask_cvtbiassph_phf8(__m128i, __mmask16, __m256i, __m256h);
extern __m128i __cdecl _mm256_maskz_cvtbiassph_phf8(__mmask16, __m256i, __m256h);
extern __m256i __cdecl _mm512_cvtbiassph_phf8(__m512i, __m512h);
extern __m256i __cdecl _mm512_mask_cvtbiassph_phf8(__m256i, __mmask32, __m512i, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtbiassph_phf8(__mmask32, __m512i, __m512h);
extern __m128h __cdecl _mm_cvthf8_ph(__m128i);
extern __m128h __cdecl _mm_mask_cvthf8_ph(__m128h, __mmask8, __m128i);
extern __m128h __cdecl _mm_maskz_cvthf8_ph(__mmask8, __m128i);
extern __m256h __cdecl _mm256_cvthf8_ph(__m128i);
extern __m256h __cdecl _mm256_mask_cvthf8_ph(__m256h, __mmask16, __m128i);
extern __m256h __cdecl _mm256_maskz_cvthf8_ph(__mmask16, __m128i);
extern __m512h __cdecl _mm512_cvthf8_ph(__m256i);
extern __m512h __cdecl _mm512_mask_cvthf8_ph(__m512h, __mmask32, __m256i);
extern __m512h __cdecl _mm512_maskz_cvthf8_ph(__mmask32, __m256i);
extern __m128i __cdecl _mm_cvt2ph_bf8(__m128h, __m128h);
extern __m128i __cdecl _mm_mask_cvt2ph_bf8(__m128i, __mmask16, __m128h, __m128h);
extern __m128i __cdecl _mm_maskz_cvt2ph_bf8(__mmask16, __m128h, __m128h);
extern __m256i __cdecl _mm256_cvt2ph_bf8(__m256h, __m256h);
extern __m256i __cdecl _mm256_mask_cvt2ph_bf8(__m256i, __mmask32, __m256h, __m256h);
extern __m256i __cdecl _mm256_maskz_cvt2ph_bf8(__mmask32, __m256h, __m256h);
extern __m512i __cdecl _mm512_cvt2ph_bf8(__m512h, __m512h);
extern __m512i __cdecl _mm512_mask_cvt2ph_bf8(__m512i, __mmask64, __m512h, __m512h);
extern __m512i __cdecl _mm512_maskz_cvt2ph_bf8(__mmask64, __m512h, __m512h);
extern __m128i __cdecl _mm_cvts2ph_bf8(__m128h, __m128h);
extern __m128i __cdecl _mm_mask_cvts2ph_bf8(__m128i, __mmask16, __m128h, __m128h);
extern __m128i __cdecl _mm_maskz_cvts2ph_bf8(__mmask16, __m128h, __m128h);
extern __m256i __cdecl _mm256_cvts2ph_bf8(__m256h, __m256h);
extern __m256i __cdecl _mm256_mask_cvts2ph_bf8(__m256i, __mmask32, __m256h, __m256h);
extern __m256i __cdecl _mm256_maskz_cvts2ph_bf8(__mmask32, __m256h, __m256h);
extern __m512i __cdecl _mm512_cvts2ph_bf8(__m512h, __m512h);
extern __m512i __cdecl _mm512_mask_cvts2ph_bf8(__m512i, __mmask64, __m512h, __m512h);
extern __m512i __cdecl _mm512_maskz_cvts2ph_bf8(__mmask64, __m512h, __m512h);
extern __m128i __cdecl _mm_cvt2ph_hf8(__m128h, __m128h);
extern __m128i __cdecl _mm_mask_cvt2ph_hf8(__m128i, __mmask16, __m128h, __m128h);
extern __m128i __cdecl _mm_maskz_cvt2ph_hf8(__mmask16, __m128h, __m128h);
extern __m256i __cdecl _mm256_cvt2ph_hf8(__m256h, __m256h);
extern __m256i __cdecl _mm256_mask_cvt2ph_hf8(__m256i, __mmask32, __m256h, __m256h);
extern __m256i __cdecl _mm256_maskz_cvt2ph_hf8(__mmask32, __m256h, __m256h);
extern __m512i __cdecl _mm512_cvt2ph_hf8(__m512h, __m512h);
extern __m512i __cdecl _mm512_mask_cvt2ph_hf8(__m512i, __mmask64, __m512h, __m512h);
extern __m512i __cdecl _mm512_maskz_cvt2ph_hf8(__mmask64, __m512h, __m512h);
extern __m128i __cdecl _mm_cvts2ph_hf8(__m128h, __m128h);
extern __m128i __cdecl _mm_mask_cvts2ph_hf8(__m128i, __mmask16, __m128h, __m128h);
extern __m128i __cdecl _mm_maskz_cvts2ph_hf8(__mmask16, __m128h, __m128h);
extern __m256i __cdecl _mm256_cvts2ph_hf8(__m256h, __m256h);
extern __m256i __cdecl _mm256_mask_cvts2ph_hf8(__m256i, __mmask32, __m256h, __m256h);
extern __m256i __cdecl _mm256_maskz_cvts2ph_hf8(__mmask32, __m256h, __m256h);
extern __m512i __cdecl _mm512_cvts2ph_hf8(__m512h, __m512h);
extern __m512i __cdecl _mm512_mask_cvts2ph_hf8(__m512i, __mmask64, __m512h, __m512h);
extern __m512i __cdecl _mm512_maskz_cvts2ph_hf8(__mmask64, __m512h, __m512h);
extern __m128i __cdecl _mm_cvtph_bf8(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_bf8(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_bf8(__mmask8, __m128h);
extern __m128i __cdecl _mm256_cvtph_bf8(__m256h);
extern __m128i __cdecl _mm256_mask_cvtph_bf8(__m128i, __mmask16,  __m256h);
extern __m128i __cdecl _mm256_maskz_cvtph_bf8(__mmask16, __m256h);
extern __m256i __cdecl _mm512_cvtph_bf8(__m512h);
extern __m256i __cdecl _mm512_mask_cvtph_bf8(__m256i, __mmask32, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtph_bf8(__mmask32, __m512h);
extern __m128i __cdecl _mm_cvtsph_bf8(__m128h);
extern __m128i __cdecl _mm_mask_cvtsph_bf8(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtsph_bf8(__mmask8, __m128h);
extern __m128i __cdecl _mm256_cvtsph_bf8(__m256h);
extern __m128i __cdecl _mm256_mask_cvtsph_bf8(__m128i, __mmask16,  __m256h);
extern __m128i __cdecl _mm256_maskz_cvtsph_bf8(__mmask16, __m256h);
extern __m256i __cdecl _mm512_cvtsph_bf8(__m512h);
extern __m256i __cdecl _mm512_mask_cvtsph_bf8(__m256i, __mmask32, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtsph_bf8(__mmask32, __m512h);
extern __m128i __cdecl _mm_cvtph_hf8(__m128h);
extern __m128i __cdecl _mm_mask_cvtph_hf8(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtph_hf8(__mmask8, __m128h);
extern __m128i __cdecl _mm256_cvtph_hf8(__m256h);
extern __m128i __cdecl _mm256_mask_cvtph_hf8(__m128i, __mmask16,  __m256h);
extern __m128i __cdecl _mm256_maskz_cvtph_hf8(__mmask16, __m256h);
extern __m256i __cdecl _mm512_cvtph_hf8(__m512h);
extern __m256i __cdecl _mm512_mask_cvtph_hf8(__m256i, __mmask32, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtph_hf8(__mmask32, __m512h);
extern __m128i __cdecl _mm_cvtsph_hf8(__m128h);
extern __m128i __cdecl _mm_mask_cvtsph_hf8(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_cvtsph_hf8(__mmask8, __m128h);
extern __m128i __cdecl _mm256_cvtsph_hf8(__m256h);
extern __m128i __cdecl _mm256_mask_cvtsph_hf8(__m128i, __mmask16,  __m256h);
extern __m128i __cdecl _mm256_maskz_cvtsph_hf8(__mmask16, __m256h);
extern __m256i __cdecl _mm512_cvtsph_hf8(__m512h);
extern __m256i __cdecl _mm512_mask_cvtsph_hf8(__m256i, __mmask32, __m512h);
extern __m256i __cdecl _mm512_maskz_cvtsph_hf8(__mmask32, __m512h);

// AVX10.2 Saturating convert instructions
// VCVTTSS2USIS
extern unsigned int __cdecl _mm_cvttsss_u32(__m128);
extern unsigned int __cdecl _mm_cvtts_roundss_u32(__m128, const int);
#if defined (_M_X64)
extern unsigned long long __cdecl _mm_cvttsss_u64(__m128);
extern unsigned long long __cdecl _mm_cvtts_roundss_u64(__m128, const int); 
#endif 

// VCVTTSS2SIS
extern int       __cdecl _mm_cvttsss_i32(__m128);
extern int       __cdecl _mm_cvtts_roundss_i32(__m128, const int);
#if defined (_M_X64)
extern long long   __cdecl _mm_cvtts_roundss_i64(__m128, const int); 
extern long long   __cdecl _mm_cvttsss_i64(__m128);
#endif  

// VCVTTSD2USIS
extern unsigned int __cdecl _mm_cvttsds_u32(__m128d);
extern unsigned int __cdecl _mm_cvtts_roundsd_u32(__m128d, const int);
#if defined (_M_X64)
extern unsigned long long __cdecl _mm_cvttsds_u64(__m128d);
extern unsigned long long __cdecl _mm_cvtts_roundsd_u64(__m128d, const int); 
#endif  

// VCVTTSD2SIS
extern int       __cdecl _mm_cvttsds_i32(__m128d);
extern int       __cdecl _mm_cvtts_roundsd_i32(__m128d, const int);
#if defined (_M_X64)
extern long long   __cdecl _mm_cvtts_roundsd_i64(__m128d, const int);
extern long long   __cdecl _mm_cvttsds_i64(__m128d);
#endif 

// VCVTTPS2UQQS
extern __m128i __cdecl _mm_cvttsps_epu64(__m128);
extern __m128i __cdecl _mm_mask_cvttsps_epu64(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_cvttsps_epu64(__mmask8, __m128);
extern __m256i __cdecl _mm256_cvttsps_epu64(__m128);
extern __m256i __cdecl _mm256_mask_cvttsps_epu64(__m256i, __mmask8, __m128);
extern __m256i __cdecl _mm256_maskz_cvttsps_epu64(__mmask8, __m128);
extern __m512i __cdecl _mm512_cvttsps_epu64(__m256);
extern __m512i __cdecl _mm512_mask_cvttsps_epu64(__m512i, __mmask8, __m256);
extern __m512i __cdecl _mm512_maskz_cvttsps_epu64(__mmask8, __m256);
extern __m512i __cdecl _mm512_cvtts_roundps_epu64(__m256, const int);
extern __m512i __cdecl _mm512_mask_cvtts_roundps_epu64(__m512i, __mmask8, __m256, const int);
extern __m512i __cdecl _mm512_maskz_cvtts_roundps_epu64(__mmask8, __m256, const int);

// VCVTTPS2UDQS
extern __m128i __cdecl _mm_cvttsps_epu32(__m128);
extern __m128i __cdecl _mm_mask_cvttsps_epu32(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_cvttsps_epu32(__mmask8, __m128);
extern __m256i __cdecl _mm256_cvttsps_epu32(__m256);
extern __m256i __cdecl _mm256_mask_cvttsps_epu32(__m256i, __mmask8, __m256);
extern __m256i __cdecl _mm256_maskz_cvttsps_epu32(__mmask8, __m256);
extern __m512i __cdecl _mm512_cvttsps_epu32(__m512); 
extern __m512i __cdecl _mm512_mask_cvttsps_epu32(__m512i, __mmask16, __m512); 
extern __m512i __cdecl _mm512_maskz_cvttsps_epu32(__mmask16, __m512); 
extern __m512i __cdecl _mm512_cvtts_roundps_epu32(__m512, const int);
extern __m512i __cdecl _mm512_mask_cvtts_roundps_epu32(__m512i, __mmask16, __m512, const int);
extern __m512i __cdecl _mm512_maskz_cvtts_roundps_epu32(__mmask16, __m512, const int);

// VCVTTPS2QQS
extern __m128i __cdecl _mm_cvttsps_epi64(__m128);
extern __m128i __cdecl _mm_mask_cvttsps_epi64(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_cvttsps_epi64(__mmask8, __m128);
extern __m256i __cdecl _mm256_cvttsps_epi64(__m128);
extern __m256i __cdecl _mm256_mask_cvttsps_epi64(__m256i, __mmask8, __m128);
extern __m256i __cdecl _mm256_maskz_cvttsps_epi64(__mmask8, __m128);
extern __m512i __cdecl _mm512_cvttsps_epi64(__m256); 
extern __m512i __cdecl _mm512_mask_cvttsps_epi64(__m512i, __mmask8, __m256); 
extern __m512i __cdecl _mm512_maskz_cvttsps_epi64(__mmask8, __m256); 
extern __m512i __cdecl _mm512_cvtts_roundps_epi64(__m256, const int);
extern __m512i __cdecl _mm512_mask_cvtts_roundps_epi64(__m512i, __mmask8, __m256, const int);
extern __m512i __cdecl _mm512_maskz_cvtts_roundps_epi64(__mmask8, __m256, const int);

// VCVTTPS2DQS
extern __m128i __cdecl _mm_cvttsps_epi32(__m128);
extern __m128i __cdecl _mm_mask_cvttsps_epi32(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_cvttsps_epi32(__mmask8, __m128);
extern __m256i __cdecl _mm256_cvttsps_epi32(__m256);
extern __m256i __cdecl _mm256_mask_cvttsps_epi32(__m256i, __mmask8, __m256);
extern __m256i __cdecl _mm256_maskz_cvttsps_epi32(__mmask8, __m256);
extern __m512i __cdecl _mm512_cvttsps_epi32(__m512);
extern __m512i __cdecl _mm512_mask_cvttsps_epi32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_cvttsps_epi32(__mmask16, __m512);
extern __m512i __cdecl _mm512_cvtts_roundps_epi32(__m512, const int);
extern __m512i __cdecl _mm512_mask_cvtts_roundps_epi32(__m512i, __mmask16, __m512, const int);
extern __m512i __cdecl _mm512_maskz_cvtts_roundps_epi32(__mmask16, __m512, const int);

// VCVTTPD2UQQS
extern __m128i __cdecl _mm_cvttspd_epu64(__m128d);
extern __m128i __cdecl _mm_mask_cvttspd_epu64(__m128i, __mmask8, __m128d);
extern __m128i __cdecl _mm_maskz_cvttspd_epu64(__mmask8, __m128d);
extern __m256i __cdecl _mm256_cvttspd_epu64(__m256d);
extern __m256i __cdecl _mm256_mask_cvttspd_epu64(__m256i, __mmask8, __m256d);
extern __m256i __cdecl _mm256_maskz_cvttspd_epu64(__mmask8, __m256d);
extern __m512i __cdecl _mm512_cvttspd_epu64(__m512d); 
extern __m512i __cdecl _mm512_mask_cvttspd_epu64(__m512i, __mmask8, __m512d); 
extern __m512i __cdecl _mm512_maskz_cvttspd_epu64(__mmask8, __m512d);
extern __m512i __cdecl _mm512_cvtts_roundpd_epu64(__m512d, const int);
extern __m512i __cdecl _mm512_mask_cvtts_roundpd_epu64(__m512i, __mmask8, __m512d, const int);
extern __m512i __cdecl _mm512_maskz_cvtts_roundpd_epu64(__mmask8, __m512d, const int);

// VCVTTPD2UDQS
extern __m128i __cdecl _mm_cvttspd_epu32(__m128d);
extern __m128i __cdecl _mm_mask_cvttspd_epu32(__m128i, __mmask8, __m128d);
extern __m128i __cdecl _mm_maskz_cvttspd_epu32(__mmask8, __m128d);
extern __m128i __cdecl _mm256_cvttspd_epu32(__m256d);
extern __m128i __cdecl _mm256_mask_cvttspd_epu32(__m128i, __mmask8, __m256d);
extern __m128i __cdecl _mm256_maskz_cvttspd_epu32(__mmask8, __m256d);
extern __m256i __cdecl _mm512_cvttspd_epu32(__m512d); 
extern __m256i __cdecl _mm512_mask_cvttspd_epu32(__m256i, __mmask8, __m512d); 
extern __m256i __cdecl _mm512_maskz_cvttspd_epu32(__mmask8, __m512d); 
extern __m256i __cdecl _mm512_cvtts_roundpd_epu32(__m512d, const int);
extern __m256i __cdecl _mm512_mask_cvtts_roundpd_epu32(__m256i, __mmask8, __m512d, const int);
extern __m256i __cdecl _mm512_maskz_cvtts_roundpd_epu32(__mmask8, __m512d, const int);

// VCVTTPD2QQS
extern __m128i __cdecl _mm_cvttspd_epi64(__m128d);
extern __m128i __cdecl _mm_mask_cvttspd_epi64(__m128i, __mmask8, __m128d);
extern __m128i __cdecl _mm_maskz_cvttspd_epi64(__mmask8, __m128d);
extern __m256i __cdecl _mm256_cvttspd_epi64(__m256d);
extern __m256i __cdecl _mm256_mask_cvttspd_epi64(__m256i, __mmask8, __m256d);
extern __m256i __cdecl _mm256_maskz_cvttspd_epi64(__mmask8, __m256d);
extern __m512i __cdecl _mm512_cvttspd_epi64(__m512d); 
extern __m512i __cdecl _mm512_mask_cvttspd_epi64(__m512i, __mmask8, __m512d);
extern __m512i __cdecl _mm512_maskz_cvttspd_epi64(__mmask8, __m512d);
extern __m512i __cdecl _mm512_cvtts_roundpd_epi64(__m512d, const int);
extern __m512i __cdecl _mm512_mask_cvtts_roundpd_epi64(__m512i, __mmask8, __m512d, const int);
extern __m512i __cdecl _mm512_maskz_cvtts_roundpd_epi64(__mmask8, __m512d, const int);

// VCVTTPD2DQS
extern __m128i __cdecl _mm_cvttspd_epi32(__m128d);
extern __m128i __cdecl _mm_mask_cvttspd_epi32(__m128i, __mmask8, __m128d);
extern __m128i __cdecl _mm_maskz_cvttspd_epi32(__mmask8, __m128d);
extern __m128i __cdecl _mm256_cvttspd_epi32(__m256d);
extern __m128i __cdecl _mm256_mask_cvttspd_epi32(__m128i, __mmask8, __m256d);
extern __m128i __cdecl _mm256_maskz_cvttspd_epi32(__mmask8, __m256d); 
extern __m256i __cdecl _mm512_cvttspd_epi32(__m512d);
extern __m256i __cdecl _mm512_mask_cvttspd_epi32(__m256i, __mmask8, __m512d); 
extern __m256i __cdecl _mm512_maskz_cvttspd_epi32(__mmask8, __m512d);
extern __m256i __cdecl _mm512_cvtts_roundpd_epi32(__m512d, const int);
extern __m256i __cdecl _mm512_mask_cvtts_roundpd_epi32(__m256i, __mmask8, __m512d, const int);
extern __m256i __cdecl _mm512_maskz_cvtts_roundpd_epi32(__mmask8, __m512d, const int);

// VCVTBF162IBS
extern __m128i __cdecl _mm_ipcvtbf16_epi16(__m128bh);
extern __m128i __cdecl _mm_mask_ipcvtbf16_epi16(__m128i, __mmask8, __m128bh);
extern __m128i __cdecl _mm_maskz_ipcvtbf16_epi16(__mmask8, __m128bh);
extern __m256i __cdecl _mm256_ipcvtbf16_epi16(__m256bh);
extern __m256i __cdecl _mm256_mask_ipcvtbf16_epi16(__m256i, __mmask16, __m256bh);
extern __m256i __cdecl _mm256_maskz_ipcvtbf16_epi16(__mmask16, __m256bh);
extern __m512i __cdecl _mm512_ipcvtbf16_epi16(__m512bh);
extern __m512i __cdecl _mm512_mask_ipcvtbf16_epi16(__m512i, __mmask32, __m512bh);
extern __m512i __cdecl _mm512_maskz_ipcvtbf16_epi16(__mmask32, __m512bh);

// VCVTTBF162IBS
extern __m128i __cdecl _mm_ipcvttbf16_epi16(__m128bh);
extern __m128i __cdecl _mm_mask_ipcvttbf16_epi16(__m128i, __mmask8, __m128bh);
extern __m128i __cdecl _mm_maskz_ipcvttbf16_epi16(__mmask8, __m128bh);
extern __m256i __cdecl _mm256_ipcvttbf16_epi16(__m256bh);
extern __m256i __cdecl _mm256_mask_ipcvttbf16_epi16(__m256i, __mmask16, __m256bh);
extern __m256i __cdecl _mm256_maskz_ipcvttbf16_epi16(__mmask16, __m256bh);
extern __m512i __cdecl _mm512_ipcvttbf16_epi16(__m512bh);
extern __m512i __cdecl _mm512_mask_ipcvttbf16_epi16(__m512i, __mmask32, __m512bh);
extern __m512i __cdecl _mm512_maskz_ipcvttbf16_epi16(__mmask32, __m512bh);

// VCVTBF162IUBS
extern __m128i __cdecl _mm_ipcvtbf16_epu16(__m128bh);
extern __m128i __cdecl _mm_mask_ipcvtbf16_epu16(__m128i, __mmask8, __m128bh);
extern __m128i __cdecl _mm_maskz_ipcvtbf16_epu16(__mmask8, __m128bh);
extern __m256i __cdecl _mm256_ipcvtbf16_epu16(__m256bh);
extern __m256i __cdecl _mm256_mask_ipcvtbf16_epu16(__m256i, __mmask16, __m256bh);
extern __m256i __cdecl _mm256_maskz_ipcvtbf16_epu16(__mmask16, __m256bh);
extern __m512i __cdecl _mm512_ipcvtbf16_epu16(__m512bh);
extern __m512i __cdecl _mm512_mask_ipcvtbf16_epu16(__m512i, __mmask32, __m512bh);
extern __m512i __cdecl _mm512_maskz_ipcvtbf16_epu16(__mmask32, __m512bh);

// VCVTTBF162IUBS
extern __m128i __cdecl _mm_ipcvttbf16_epu16(__m128bh);
extern __m128i __cdecl _mm_mask_ipcvttbf16_epu16(__m128i, __mmask8, __m128bh);
extern __m128i __cdecl _mm_maskz_ipcvttbf16_epu16(__mmask8, __m128bh);
extern __m256i __cdecl _mm256_ipcvttbf16_epu16(__m256bh);
extern __m256i __cdecl _mm256_mask_ipcvttbf16_epu16(__m256i, __mmask16, __m256bh);
extern __m256i __cdecl _mm256_maskz_ipcvttbf16_epu16(__mmask16, __m256bh);
extern __m512i __cdecl _mm512_ipcvttbf16_epu16(__m512bh);
extern __m512i __cdecl _mm512_mask_ipcvttbf16_epu16(__m512i, __mmask32, __m512bh);
extern __m512i __cdecl _mm512_maskz_ipcvttbf16_epu16(__mmask32, __m512bh);

// VCVTPH2IBS
extern __m128i __cdecl _mm_ipcvtph_epi16(__m128h);
extern __m128i __cdecl _mm_mask_ipcvtph_epi16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_ipcvtph_epi16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_ipcvtph_epi16(__m256h);
extern __m256i __cdecl _mm256_mask_ipcvtph_epi16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_ipcvtph_epi16( __mmask16, __m256h);
extern __m512i __cdecl _mm512_ipcvtph_epi16(__m512h);
extern __m512i __cdecl _mm512_mask_ipcvtph_epi16(__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_ipcvtph_epi16( __mmask32, __m512h);
extern __m512i __cdecl _mm512_ipcvt_roundph_epi16(__m512h, const int);
extern __m512i __cdecl _mm512_mask_ipcvt_roundph_epi16(__m512i, __mmask32, __m512h, const int);
extern __m512i __cdecl _mm512_maskz_ipcvt_roundph_epi16(__mmask32, __m512h, const int);

// VCVTTPH2IBS
extern __m128i __cdecl _mm_ipcvttph_epi16(__m128h);
extern __m128i __cdecl _mm_mask_ipcvttph_epi16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_ipcvttph_epi16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_ipcvttph_epi16(__m256h);
extern __m256i __cdecl _mm256_mask_ipcvttph_epi16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_ipcvttph_epi16( __mmask16, __m256h);
extern __m512i __cdecl _mm512_ipcvttph_epi16(__m512h);
extern __m512i __cdecl _mm512_mask_ipcvttph_epi16(__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_ipcvttph_epi16( __mmask32, __m512h);
extern __m512i __cdecl _mm512_ipcvtt_roundph_epi16(__m512h, const int);
extern __m512i __cdecl _mm512_mask_ipcvtt_roundph_epi16(__m512i, __mmask32, __m512h, const int);
extern __m512i __cdecl _mm512_maskz_ipcvtt_roundph_epi16(__mmask32, __m512h, const int);

// VCVTPH2IUBS
extern __m128i __cdecl _mm_ipcvtph_epu16(__m128h);
extern __m128i __cdecl _mm_mask_ipcvtph_epu16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_ipcvtph_epu16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_ipcvtph_epu16(__m256h);
extern __m256i __cdecl _mm256_mask_ipcvtph_epu16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_ipcvtph_epu16( __mmask16, __m256h);
extern __m512i __cdecl _mm512_ipcvtph_epu16(__m512h);
extern __m512i __cdecl _mm512_mask_ipcvtph_epu16(__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_ipcvtph_epu16( __mmask32, __m512h);
extern __m512i __cdecl _mm512_ipcvt_roundph_epu16(__m512h, const int);
extern __m512i __cdecl _mm512_mask_ipcvt_roundph_epu16(__m512i, __mmask32, __m512h, const int);
extern __m512i __cdecl _mm512_maskz_ipcvt_roundph_epu16(__mmask32, __m512h, const int);

// VCVTTPH2IUBS
extern __m128i __cdecl _mm_ipcvttph_epu16(__m128h);
extern __m128i __cdecl _mm_mask_ipcvttph_epu16(__m128i, __mmask8, __m128h);
extern __m128i __cdecl _mm_maskz_ipcvttph_epu16(__mmask8, __m128h);
extern __m256i __cdecl _mm256_ipcvttph_epu16(__m256h);
extern __m256i __cdecl _mm256_mask_ipcvttph_epu16(__m256i, __mmask16, __m256h);
extern __m256i __cdecl _mm256_maskz_ipcvttph_epu16( __mmask16, __m256h);
extern __m512i __cdecl _mm512_ipcvttph_epu16(__m512h);
extern __m512i __cdecl _mm512_mask_ipcvttph_epu16(__m512i, __mmask32, __m512h);
extern __m512i __cdecl _mm512_maskz_ipcvttph_epu16( __mmask32, __m512h);
extern __m512i __cdecl _mm512_ipcvtt_roundph_epu16(__m512h, const int);
extern __m512i __cdecl _mm512_mask_ipcvtt_roundph_epu16(__m512i, __mmask32, __m512h, const int);
extern __m512i __cdecl _mm512_maskz_ipcvtt_roundph_epu16(__mmask32, __m512h, const int);

// VCVTPS2IBS
extern __m128i __cdecl _mm_ipcvtps_epi32(__m128);
extern __m128i __cdecl _mm_mask_ipcvtps_epi32(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_ipcvtps_epi32(__mmask8, __m128);
extern __m256i __cdecl _mm256_ipcvtps_epi32(__m256);
extern __m256i __cdecl _mm256_mask_ipcvtps_epi32(__m256i, __mmask8, __m256);
extern __m256i __cdecl _mm256_maskz_ipcvtps_epi32(__mmask8, __m256);
extern __m512i __cdecl _mm512_ipcvtps_epi32(__m512);
extern __m512i __cdecl _mm512_mask_ipcvtps_epi32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_ipcvtps_epi32(__mmask16, __m512);
extern __m512i __cdecl _mm512_ipcvt_roundps_epi32(__m512, const int);
extern __m512i __cdecl _mm512_mask_ipcvt_roundps_epi32(__m512i, __mmask16, __m512, const int);
extern __m512i __cdecl _mm512_maskz_ipcvt_roundps_epi32(__mmask16, __m512, const int);

// VCVTTPS2IBS
extern __m128i __cdecl _mm_ipcvttps_epi32(__m128);
extern __m128i __cdecl _mm_mask_ipcvttps_epi32(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_ipcvttps_epi32(__mmask8, __m128);
extern __m256i __cdecl _mm256_ipcvttps_epi32(__m256);
extern __m256i __cdecl _mm256_mask_ipcvttps_epi32(__m256i, __mmask8, __m256);
extern __m256i __cdecl _mm256_maskz_ipcvttps_epi32(__mmask8, __m256);
extern __m512i __cdecl _mm512_ipcvttps_epi32(__m512);
extern __m512i __cdecl _mm512_mask_ipcvttps_epi32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_ipcvttps_epi32(__mmask16, __m512);
extern __m512i __cdecl _mm512_ipcvtt_roundps_epi32(__m512, const int);
extern __m512i __cdecl _mm512_mask_ipcvtt_roundps_epi32(__m512i, __mmask16, __m512, const int);
extern __m512i __cdecl _mm512_maskz_ipcvtt_roundps_epi32(__mmask16, __m512, const int);

// VCVTPS2IUBS
extern __m128i __cdecl _mm_ipcvtps_epu32(__m128);
extern __m128i __cdecl _mm_mask_ipcvtps_epu32(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_ipcvtps_epu32(__mmask8, __m128);
extern __m256i __cdecl _mm256_ipcvtps_epu32(__m256);
extern __m256i __cdecl _mm256_mask_ipcvtps_epu32(__m256i, __mmask8, __m256);
extern __m256i __cdecl _mm256_maskz_ipcvtps_epu32(__mmask8, __m256);
extern __m512i __cdecl _mm512_ipcvtps_epu32(__m512);
extern __m512i __cdecl _mm512_mask_ipcvtps_epu32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_ipcvtps_epu32(__mmask16, __m512);
extern __m512i __cdecl _mm512_ipcvt_roundps_epu32(__m512, const int);
extern __m512i __cdecl _mm512_mask_ipcvt_roundps_epu32(__m512i, __mmask16, __m512, const int);
extern __m512i __cdecl _mm512_maskz_ipcvt_roundps_epu32(__mmask16, __m512, const int);

// VCVTTPS2IUBS
extern __m128i __cdecl _mm_ipcvttps_epu32(__m128);
extern __m128i __cdecl _mm_mask_ipcvttps_epu32(__m128i, __mmask8, __m128);
extern __m128i __cdecl _mm_maskz_ipcvttps_epu32(__mmask8, __m128);
extern __m256i __cdecl _mm256_ipcvttps_epu32(__m256);
extern __m256i __cdecl _mm256_mask_ipcvttps_epu32(__m256i, __mmask8, __m256);
extern __m256i __cdecl _mm256_maskz_ipcvttps_epu32(__mmask8, __m256);
extern __m512i __cdecl _mm512_ipcvttps_epu32(__m512);
extern __m512i __cdecl _mm512_mask_ipcvttps_epu32(__m512i, __mmask16, __m512);
extern __m512i __cdecl _mm512_maskz_ipcvttps_epu32(__mmask16, __m512);
extern __m512i __cdecl _mm512_ipcvtt_roundps_epu32(__m512, const int);
extern __m512i __cdecl _mm512_mask_ipcvtt_roundps_epu32(__m512i, __mmask16, __m512, const int);
extern __m512i __cdecl _mm512_maskz_ipcvtt_roundps_epu32(__mmask16, __m512, const int);

// AVX10.2 MinMax instructions 
// VMINMAXPBF16
extern __m128bh __cdecl _mm_minmax_pbh(__m128bh, __m128bh, const int);
extern __m128bh __cdecl _mm_mask_minmax_pbh(__m128bh, __mmask8, __m128bh, __m128bh, const int);
extern __m128bh __cdecl _mm_maskz_minmax_pbh(__mmask8, __m128bh, __m128bh, const int);
extern __m256bh __cdecl _mm256_minmax_pbh(__m256bh, __m256bh, const int);
extern __m256bh __cdecl _mm256_mask_minmax_pbh(__m256bh, __mmask16, __m256bh, __m256bh, const int);
extern __m256bh __cdecl _mm256_maskz_minmax_pbh(__mmask16, __m256bh, __m256bh, const int);
extern __m512bh __cdecl _mm512_minmax_pbh(__m512bh, __m512bh, const int);
extern __m512bh __cdecl _mm512_mask_minmax_pbh(__m512bh, __mmask32, __m512bh, __m512bh, const int);
extern __m512bh __cdecl _mm512_maskz_minmax_pbh(__mmask32, __m512bh, __m512bh, const int);

// VMINMAXPD
extern __m128d __cdecl _mm_minmax_pd(__m128d, __m128d, const int);
extern __m128d __cdecl _mm_mask_minmax_pd(__m128d, __mmask8, __m128d, __m128d, const int);
extern __m128d __cdecl _mm_maskz_minmax_pd(__mmask8, __m128d, __m128d, const int);
extern __m256d __cdecl _mm256_minmax_pd(__m256d, __m256d, const int);
extern __m256d __cdecl _mm256_mask_minmax_pd(__m256d, __mmask8, __m256d, __m256d, const int);
extern __m256d __cdecl _mm256_maskz_minmax_pd(__mmask8, __m256d, __m256d, const int);
extern __m512d __cdecl _mm512_minmax_pd(__m512d, __m512d, const int);
extern __m512d __cdecl _mm512_mask_minmax_pd(__m512d, __mmask8, __m512d, __m512d, const int);
extern __m512d __cdecl _mm512_maskz_minmax_pd(__mmask8, __m512d, __m512d, const int);
extern __m512d __cdecl _mm512_minmax_round_pd(__m512d, __m512d, const int, const int);
extern __m512d __cdecl _mm512_mask_minmax_round_pd(__m512d, __mmask8, __m512d, __m512d, const int, const int);
extern __m512d __cdecl _mm512_maskz_minmax_round_pd(__mmask8, __m512d, __m512d, const int, const int);

// VMINMAXPS
extern __m128 __cdecl _mm_minmax_ps(__m128, __m128, const int);
extern __m128 __cdecl _mm_mask_minmax_ps(__m128, __mmask8, __m128, __m128, const int);
extern __m128 __cdecl _mm_maskz_minmax_ps(__mmask8, __m128, __m128, const int);
extern __m256 __cdecl _mm256_minmax_ps(__m256, __m256, const int);
extern __m256 __cdecl _mm256_mask_minmax_ps(__m256, __mmask8, __m256, __m256, const int);
extern __m256 __cdecl _mm256_maskz_minmax_ps(__mmask8, __m256, __m256, const int);
extern __m512 __cdecl _mm512_minmax_ps(__m512, __m512, const int);
extern __m512 __cdecl _mm512_mask_minmax_ps(__m512, __mmask16, __m512, __m512, const int);
extern __m512 __cdecl _mm512_maskz_minmax_ps(__mmask16, __m512, __m512, const int);
extern __m512 __cdecl _mm512_minmax_round_ps(__m512, __m512, const int, const int);
extern __m512 __cdecl _mm512_mask_minmax_round_ps(__m512, __mmask16, __m512, __m512, const int, const int);
extern __m512 __cdecl _mm512_maskz_minmax_round_ps(__mmask16, __m512, __m512, const int, const int);

// VMINMAXPH
extern __m128h __cdecl _mm_minmax_ph(__m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_minmax_ph(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_maskz_minmax_ph(__mmask8, __m128h, __m128h, const int);
extern __m256h __cdecl _mm256_minmax_ph(__m256h, __m256h, const int);
extern __m256h __cdecl _mm256_mask_minmax_ph(__m256h, __mmask16, __m256h, __m256h, const int);
extern __m256h __cdecl _mm256_maskz_minmax_ph(__mmask16, __m256h, __m256h, const int);
extern __m512h __cdecl _mm512_minmax_ph(__m512h, __m512h, const int);
extern __m512h __cdecl _mm512_mask_minmax_ph(__m512h, __mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_maskz_minmax_ph(__mmask32, __m512h, __m512h, const int);
extern __m512h __cdecl _mm512_minmax_round_ph(__m512h, __m512h, const int, const int);
extern __m512h __cdecl _mm512_mask_minmax_round_ph(__m512h, __mmask32, __m512h, __m512h, const int, const int);
extern __m512h __cdecl _mm512_maskz_minmax_round_ph(__mmask32, __m512h, __m512h, const int, const int);

// VMINMAX[SH,SS,SD]
extern __m128d __cdecl _mm_minmax_sd(__m128d, __m128d, const int);
extern __m128d __cdecl _mm_mask_minmax_sd(__m128d, __mmask8, __m128d, __m128d, const int);
extern __m128d __cdecl _mm_maskz_minmax_sd(__mmask8, __m128d, __m128d, const int);
extern __m128d __cdecl _mm_minmax_round_sd(__m128d, __m128d, const int, const int);
extern __m128d __cdecl _mm_mask_minmax_round_sd(__m128d, __mmask8, __m128d, __m128d, const int, const int);
extern __m128d __cdecl _mm_maskz_minmax_round_sd(__mmask8, __m128d, __m128d, const int, const int);
extern __m128h __cdecl _mm_minmax_sh(__m128h, __m128h, const int);
extern __m128h __cdecl _mm_mask_minmax_sh(__m128h, __mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_maskz_minmax_sh(__mmask8, __m128h, __m128h, const int);
extern __m128h __cdecl _mm_minmax_round_sh(__m128h, __m128h, const int, const int);
extern __m128h __cdecl _mm_mask_minmax_round_sh(__m128h, __mmask8, __m128h, __m128h, const int, const int);
extern __m128h __cdecl _mm_maskz_minmax_round_sh(__mmask8, __m128h, __m128h, const int, const int);
extern __m128 __cdecl _mm_minmax_ss(__m128, __m128, const int);
extern __m128 __cdecl _mm_mask_minmax_ss(__m128, __mmask8, __m128, __m128, const int);
extern __m128 __cdecl _mm_maskz_minmax_ss(__mmask8, __m128, __m128, const int);
extern __m128 __cdecl _mm_minmax_round_ss(__m128, __m128, const int, const int);
extern __m128 __cdecl _mm_mask_minmax_round_ss(__m128, __mmask8, __m128, __m128, const int, const int);
extern __m128 __cdecl _mm_maskz_minmax_round_ss(__mmask8, __m128, __m128, const int, const int);

// AVX10.2 Zero-extending partial vector copy instructions 
// VMOV[D,W]
extern __m128i __cdecl _mm_move_epi32(__m128i);
extern __m128i __cdecl _mm_move_epi16(__m128i);

// AVX10.2 Compare Scalar FP with Enhanced Eflags instructions
// V[U,]COMXS[H,S,D]
extern int __cdecl _mm_comx_sd(__m128d, __m128d, const int);
extern int __cdecl _mm_comx_round_sd(__m128d, __m128d, const int, const int);
extern int __cdecl _mm_comx_sh(__m128h, __m128h, const int);
extern int __cdecl _mm_comx_round_sh(__m128h, __m128h, const int, const int);
extern int __cdecl _mm_comx_ss(__m128, __m128, const int);
extern int __cdecl _mm_comx_round_ss(__m128, __m128, const int, const int);
extern int __cdecl _mm_ucomx_sd(__m128d, __m128d, const int);
extern int __cdecl _mm_ucomx_round_sd(__m128d, __m128d, const int, const int);
extern int __cdecl _mm_ucomx_sh(__m128h, __m128h, const int);
extern int __cdecl _mm_ucomx_round_sh(__m128h, __m128h, const int, const int);
extern int __cdecl _mm_ucomx_ss(__m128, __m128, const int);
extern int __cdecl _mm_ucomx_round_ss(__m128, __m128, const int, const int);

#define _mm_comxeq_sh(v1, v2)  _mm_comx_sh((v1), (v2), _CMP_EQ_OQ)
#define _mm_comxlt_sh(v1, v2)  _mm_comx_sh((v1), (v2), _CMP_LT_OQ)
#define _mm_comxle_sh(v1, v2)  _mm_comx_sh((v1), (v2), _CMP_LE_OQ)
#define _mm_comxgt_sh(v1, v2)  _mm_comx_sh((v1), (v2), _CMP_GT_OQ)
#define _mm_comxge_sh(v1, v2)  _mm_comx_sh((v1), (v2), _CMP_GE_OQ)
#define _mm_comxneq_sh(v1, v2) _mm_comx_sh((v1), (v2), _CMP_NEQ_OQ)

#define _mm_ucomxeq_sh(v1, v2)  _mm_ucomx_sh((v1), (v2), _CMP_EQ_OS)
#define _mm_ucomxlt_sh(v1, v2)  _mm_ucomx_sh((v1), (v2), _CMP_LT_OS)
#define _mm_ucomxle_sh(v1, v2)  _mm_ucomx_sh((v1), (v2), _CMP_LE_OS)
#define _mm_ucomxgt_sh(v1, v2)  _mm_ucomx_sh((v1), (v2), _CMP_GT_OS)
#define _mm_ucomxge_sh(v1, v2)  _mm_ucomx_sh((v1), (v2), _CMP_GE_OS)
#define _mm_ucomxneq_sh(v1, v2) _mm_ucomx_sh((v1), (v2), _CMP_NEQ_OS)

#define _mm_comxeq_sd(v1, v2)  _mm_comx_sd((v1), (v2), _CMP_EQ_OQ)
#define _mm_comxlt_sd(v1, v2)  _mm_comx_sd((v1), (v2), _CMP_LT_OQ)
#define _mm_comxle_sd(v1, v2)  _mm_comx_sd((v1), (v2), _CMP_LE_OQ)
#define _mm_comxgt_sd(v1, v2)  _mm_comx_sd((v1), (v2), _CMP_GT_OQ)
#define _mm_comxge_sd(v1, v2)  _mm_comx_sd((v1), (v2), _CMP_GE_OQ)
#define _mm_comxneq_sd(v1, v2) _mm_comx_sd((v1), (v2), _CMP_NEQ_OQ)

#define _mm_ucomxeq_sd(v1, v2)  _mm_ucomx_sd((v1), (v2), _CMP_EQ_OS)
#define _mm_ucomxlt_sd(v1, v2)  _mm_ucomx_sd((v1), (v2), _CMP_LT_OS)
#define _mm_ucomxle_sd(v1, v2)  _mm_ucomx_sd((v1), (v2), _CMP_LE_OS)
#define _mm_ucomxgt_sd(v1, v2)  _mm_ucomx_sd((v1), (v2), _CMP_GT_OS)
#define _mm_ucomxge_sd(v1, v2)  _mm_ucomx_sd((v1), (v2), _CMP_GE_OS)
#define _mm_ucomxneq_sd(v1, v2) _mm_ucomx_sd((v1), (v2), _CMP_NEQ_OS)

#define _mm_comxeq_ss(v1, v2)  _mm_comx_ss((v1), (v2), _CMP_EQ_OQ)
#define _mm_comxlt_ss(v1, v2)  _mm_comx_ss((v1), (v2), _CMP_LT_OQ)
#define _mm_comxle_ss(v1, v2)  _mm_comx_ss((v1), (v2), _CMP_LE_OQ)
#define _mm_comxgt_ss(v1, v2)  _mm_comx_ss((v1), (v2), _CMP_GT_OQ)
#define _mm_comxge_ss(v1, v2)  _mm_comx_ss((v1), (v2), _CMP_GE_OQ)
#define _mm_comxneq_ss(v1, v2) _mm_comx_ss((v1), (v2), _CMP_NEQ_OQ)

#define _mm_ucomxeq_ss(v1, v2)  _mm_ucomx_ss((v1), (v2), _CMP_EQ_OS)
#define _mm_ucomxlt_ss(v1, v2)  _mm_ucomx_ss((v1), (v2), _CMP_LT_OS)
#define _mm_ucomxle_ss(v1, v2)  _mm_ucomx_ss((v1), (v2), _CMP_LE_OS)
#define _mm_ucomxgt_ss(v1, v2)  _mm_ucomx_ss((v1), (v2), _CMP_GT_OS)
#define _mm_ucomxge_ss(v1, v2)  _mm_ucomx_ss((v1), (v2), _CMP_GE_OS)
#define _mm_ucomxneq_ss(v1, v2) _mm_ucomx_ss((v1), (v2), _CMP_NEQ_OS)

#ifdef __cplusplus
}
#endif /* __cplusplus */


#endif /* _ZMMINTRIN_H_INCLUDED */
