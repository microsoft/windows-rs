
//
//  WIDEMATH.H
//
//  Portable (x86, x64, arm, arm64) 64-bit and 128-bit arithmetic math functions.
//
//  These are broken up into two main categories:
//
//    - wide integer operations, e.g. 128-bit add, sub, mul, div, shifts, rotates
//    - wide packed vector operations, e.g. 64-bit and 128-bit SIMD primitives
//
//  These functions are all implemented using portable 64-bit integer arithmetic.
//  Portability is achieved by using no CPU-specific intrinsics, types, or ASM.
//

#pragma once

#ifdef __cplusplus

extern "C"
{

#endif

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef int int32_t;
typedef unsigned int uint32_t;
typedef __int64 int64_t;
typedef unsigned __int64 uint64_t;

typedef union uint128_t
{
    uint64_t   Q[2];
    uint32_t   D[4];
    uint16_t   W[8];
    uint8_t    B[16];
    float      F32[4];
    double     F64[2];

} uint128_t;

#define _BITS_PER_BYTE (8)
#define _BITS_PER_WORD (16)
#define _BITS_PER_LONG (32)
#define _BITS_PER_QUAD (64)

#define _WIDEMASK_BYTE (0x0101010101010101ull)
#define _WIDEMASK_WORD (0x0001000100010001ull)
#define _WIDEMASK_LONG (0x0000000100000001ull)
#define _WIDEMASK_QUAD (0x0000000000000001ull)

#ifndef WIDEMATHAPI
#define WIDEMATHAPI __stdcall
#endif

// Note that this is bit-reversed from the polynomial in the Intel manual
#define _CRC32_POLYNOMIAL (0x105EC76F1ull)

uint64_t WIDEMATHAPI POPCNT64(uint64_t X);

int32_t WIDEMATHAPI _SDIV64(int64_t Dividend, int32_t Divisor, int32_t *Remainder);

uint32_t WIDEMATHAPI _UDIV64(uint64_t Dividend, uint32_t Divisor, uint32_t *Remainder);

int64_t WIDEMATHAPI SMUL128(int64_t X, int64_t Y, int64_t *ResultHiOut);

uint64_t WIDEMATHAPI UMUL128(uint64_t X, uint64_t Y, uint64_t *ResultHiOut);

int64_t WIDEMATHAPI SDIV128(int64_t DividendHi, int64_t DividendLo, int64_t Divisor, int64_t *Remainder);

uint64_t WIDEMATHAPI UDIV128(uint64_t DividendHi, uint64_t DividendLo, uint64_t Divisor, uint64_t *Remainder);

uint64_t WIDEMATHAPI PABS64(uint64_t Y, uint64_t Mask, unsigned DataWidth);

#if 0

uint64_t WIDEMATHAPI PADC64(uint64_t X, uint64_t Y, uint64_t C, uint64_t Mask);

uint64_t WIDEMATHAPI PADD64(uint64_t X, uint64_t Y, uint64_t Mask);

uint64_t WIDEMATHAPI PSUB64(uint64_t X, uint64_t Y, uint64_t Mask);

#else

__forceinline
uint64_t WIDEMATHAPI PADC64(uint64_t X, uint64_t Y, uint64_t C, uint64_t Mask)
{
    const uint64_t HighBitMask = _rotr64(Mask, 1);          // 80...80...
    const uint64_t HighSum = (X ^ Y) & HighBitMask;
    const uint64_t LowSum = (X & ~HighBitMask) + (Y & ~HighBitMask) + C;

    return HighSum ^ LowSum;
}

__forceinline
uint64_t WIDEMATHAPI PADD64(uint64_t X, uint64_t Y, uint64_t Mask)
{
    return PADC64(X, Y, 0ull, Mask);
}

__forceinline
uint64_t WIDEMATHAPI PSUB64(uint64_t X, uint64_t Y, uint64_t Mask)
{
    return PADC64(X, ~Y, Mask, Mask);
}

#endif

uint64_t WIDEMATHAPI PADDSI64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PADDSU64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PALIGNR64(uint64_t X, uint64_t Y, uint64_t ShiftCount);

__forceinline
uint64_t WIDEMATHAPI PAND64(uint64_t X, uint64_t Y)
{
    return X & Y;
}

__forceinline
uint64_t WIDEMATHAPI PANDN64(uint64_t X, uint64_t Y)
{
    return (~X) & Y;
}

uint64_t WIDEMATHAPI PAVG64(uint64_t X, uint64_t Y, uint64_t Mask);

uint64_t WIDEMATHAPI PCMPEQ64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PCMPGT64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

int WIDEMATHAPI PEXTRW64(uint64_t X, unsigned Imm);

uint64_t WIDEMATHAPI PHADDSW64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PHADDW64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PHADDD64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PHSUBSW64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PHSUBW64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PHSUBD64(uint64_t X, uint64_t Y);

uint64_t WIDEMATHAPI PINSRW64(uint64_t X, uint64_t Y, unsigned Imm);

uint64_t WIDEMATHAPI PMADDDD64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PMADDUBSW64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PMADDWD64(uint64_t X, uint64_t Y);

uint64_t WIDEMATHAPI PMAXI64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PMAXU64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PMINI64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PMINU64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

int WIDEMATHAPI PMOVMSKB64(uint64_t X);

uint64_t WIDEMATHAPI PMULHRSW64(uint64_t X, uint64_t Y);

uint64_t WIDEMATHAPI PMULHW64(uint64_t X, uint64_t Y);

uint64_t WIDEMATHAPI PMULLW64(uint64_t X, uint64_t Y);

uint64_t WIDEMATHAPI PMULHUW64(uint64_t X, uint64_t Y);

uint64_t WIDEMATHAPI PMULUDQ64(uint64_t X, uint64_t Y);

__forceinline
uint64_t WIDEMATHAPI POR64(uint64_t X, uint64_t Y)
{
    return X | Y;
}

uint64_t WIDEMATHAPI PSADBW64(uint64_t X, uint64_t Y);

uint64_t WIDEMATHAPI PSIGN64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PSUBSI64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);
uint64_t WIDEMATHAPI PSUBSU64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PSLL64(uint64_t X, uint64_t ShiftCount, uint64_t Mask, unsigned DataWidth);
uint64_t WIDEMATHAPI PSRA64(uint64_t X, uint64_t ShiftCount, uint64_t Mask, unsigned DataWidth);
uint64_t WIDEMATHAPI PSRL64(uint64_t X, uint64_t ShiftCount, uint64_t Mask, unsigned DataWidth);

uint64_t WIDEMATHAPI PSHUFB64(uint64_t X, uint64_t Y);
uint64_t WIDEMATHAPI PSHUFW64(uint64_t Y, int Imm);

uint64_t WIDEMATHAPI PUNPCK64(uint64_t X, uint64_t Y, uint64_t SwizzleBytes);

__forceinline
uint64_t WIDEMATHAPI PXOR64(uint64_t X, uint64_t Y)
{
    return X ^ Y;
}

uint64_t WIDEMATHAPI PACKSSWB64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);
uint64_t WIDEMATHAPI PACKUSWB64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);
uint64_t WIDEMATHAPI PACKSSDW64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);
uint64_t WIDEMATHAPI PACKUSDW64(uint64_t X, uint64_t Y, uint64_t Mask, unsigned DataWidth);

//
//  128-bit versions mainly just do pairs of the 64-bit versions.
//

uint128_t WIDEMATHAPI PACKSSWB128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PACKUSWB128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PACKSSDW128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PACKUSDW128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);

uint128_t WIDEMATHAPI PABS128(uint128_t Y, uint64_t Mask, unsigned DataWidth);

__forceinline
uint128_t WIDEMATHAPI PADD128(uint128_t X, uint128_t Y, uint64_t Mask)
{
    uint128_t Result;

    Result.Q[0] = PADD64(X.Q[0], Y.Q[0], Mask);
    Result.Q[1] = PADD64(X.Q[1], Y.Q[1], Mask);

    return Result;
}

uint128_t WIDEMATHAPI PADDSI128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PADDSU128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);

__forceinline
uint128_t WIDEMATHAPI PAND128(uint128_t X, uint128_t Y)
{
    uint128_t Result;

    Result.Q[0] = PAND64(X.Q[0], Y.Q[0]);
    Result.Q[1] = PAND64(X.Q[1], Y.Q[1]);

    return Result;
}

__forceinline
uint128_t WIDEMATHAPI PANDN128(uint128_t X, uint128_t Y)
{
    uint128_t Result;

    Result.Q[0] = PANDN64(X.Q[0], Y.Q[0]);
    Result.Q[1] = PANDN64(X.Q[1], Y.Q[1]);

    return Result;
}

uint128_t WIDEMATHAPI PALIGNR128(uint128_t X, uint128_t Y, uint64_t ShiftCount);

uint128_t WIDEMATHAPI PAVG128(uint128_t X, uint128_t Y, uint64_t Mask);

uint128_t WIDEMATHAPI PBLENDVB128(uint128_t X, uint128_t Y, uint128_t Mask);
uint128_t WIDEMATHAPI PBLENDW128(uint128_t X, uint128_t Y, unsigned Mask);

uint128_t WIDEMATHAPI PCLMULQDQ128(uint128_t X, uint128_t Y, unsigned Imm);

uint128_t WIDEMATHAPI PCMPEQ128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PCMPGT128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);

uint32_t WIDEMATHAPI PCMPESTRF128(uint128_t X, int LenX, uint128_t Y, int LenY, int Imm);
uint32_t WIDEMATHAPI PCMPESTRI128(uint128_t X, int LenX, uint128_t Y, int LenY, int Imm, uint32_t *Eflags);
uint128_t WIDEMATHAPI PCMPESTRM128(uint128_t X, int LenX, uint128_t Y, int LenY, int Imm, uint32_t *Eflags);
uint32_t WIDEMATHAPI PCMPISTRF128(uint128_t X, uint128_t Y, unsigned Imm);
uint32_t WIDEMATHAPI PCMPISTRI128(uint128_t X, uint128_t Y, unsigned Imm, uint32_t *Eflags);
uint128_t WIDEMATHAPI PCMPISTRM128(uint128_t X, uint128_t Y, unsigned Imm, uint32_t *Eflags);

int WIDEMATHAPI PEXTRB128(uint128_t X, unsigned Imm);
int WIDEMATHAPI PEXTRW128(uint128_t X, unsigned Imm);
int WIDEMATHAPI PEXTRD128(uint128_t X, unsigned Imm);
int64_t WIDEMATHAPI PEXTRQ128(uint128_t X, unsigned Imm);

uint128_t WIDEMATHAPI PMADDWD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PHADDPW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PHADDSW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PHADDW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PHADDD128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI PHMINPOSUW128(uint128_t Y);

uint128_t WIDEMATHAPI PHSUBSW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PHSUBW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PHSUBD128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI PINSRB128(uint128_t X, uint64_t Y, unsigned Imm);
uint128_t WIDEMATHAPI PINSRD128(uint128_t X, uint64_t Y, unsigned Imm);
uint128_t WIDEMATHAPI PINSRQ128(uint128_t X, uint64_t Y, unsigned Imm);
uint128_t WIDEMATHAPI PINSRW128(uint128_t X, uint64_t Y, unsigned Imm);

uint128_t WIDEMATHAPI PMADDDD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMADDUBSW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMADDWD128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI PMAXI128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PMAXU128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);

uint128_t WIDEMATHAPI PMINI128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PMINU128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);

int WIDEMATHAPI PMOVMSKB128(uint128_t X);

uint128_t WIDEMATHAPI PMOVSXBW128(uint64_t X);
uint128_t WIDEMATHAPI PMOVSXBD128(uint64_t X);
uint128_t WIDEMATHAPI PMOVSXBQ128(uint64_t X);
uint128_t WIDEMATHAPI PMOVSXWD128(uint64_t X);
uint128_t WIDEMATHAPI PMOVSXWQ128(uint64_t X);
uint128_t WIDEMATHAPI PMOVSXDQ128(uint64_t X);

uint128_t WIDEMATHAPI PMOVZXBW128(uint64_t X);
uint128_t WIDEMATHAPI PMOVZXBD128(uint64_t X);
uint128_t WIDEMATHAPI PMOVZXBQ128(uint64_t X);
uint128_t WIDEMATHAPI PMOVZXWD128(uint64_t X);
uint128_t WIDEMATHAPI PMOVZXWQ128(uint64_t X);
uint128_t WIDEMATHAPI PMOVZXDQ128(uint64_t X);

uint128_t WIDEMATHAPI PMULDQ128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMULHRSW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMULHUW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMULHW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMULLD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMULLW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PMULUDQ128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI POR128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI PSADBW128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI PSIGN128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);

uint128_t WIDEMATHAPI PSLL128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PSLLDQ(uint128_t X, uint64_t ShiftCount);
uint128_t WIDEMATHAPI PSRA128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PSRL128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PSRLDQ(uint128_t X, uint64_t ShiftCount);

uint128_t WIDEMATHAPI PSUB128(uint128_t X, uint128_t Y, uint64_t Mask);
uint128_t WIDEMATHAPI PSUBSI128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);
uint128_t WIDEMATHAPI PSUBSU128(uint128_t X, uint128_t Y, uint64_t Mask, unsigned DataWidth);

uint128_t WIDEMATHAPI PSHUFB128(uint128_t X, uint128_t Y);

__forceinline
uint128_t WIDEMATHAPI PSHUFD128(uint128_t Y, int Imm)
{
    uint64_t InputLo = Y.Q[0];
    uint64_t InputHi = Y.Q[1];
    uint128_t OutputVector;

    OutputVector.Q[0] = (uint32_t)((((Imm & 0x02) ? InputHi : InputLo) >> ((Imm & 0x01) << 5)))
                      | ((((Imm & 0x08) ? InputHi : InputLo) >> ((Imm & 0x04) << 3)) << 32);
    OutputVector.Q[1] = (uint32_t)((((Imm & 0x20) ? InputHi : InputLo) >> ((Imm & 0x10) << 1)))
                      | ((((Imm & 0x80) ? InputHi : InputLo) >> ((Imm & 0x40) >> 1)) << 32);

    return OutputVector;
}

uint32_t WIDEMATHAPI PTESTC128(uint128_t X, uint128_t Y);
uint32_t WIDEMATHAPI PTESTZ128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI PUNPCK128(uint128_t X, uint128_t Y, uint64_t SwizzleBytes);
uint128_t WIDEMATHAPI PUNPCKH128(uint128_t X, uint128_t Y, uint64_t SwizzleBytes);
uint128_t WIDEMATHAPI PUNPCKHBW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PUNPCKHWD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PUNPCKHDQ128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PUNPCKLBW128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PUNPCKLWD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PUNPCKLDQ128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PUNPCKQ128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI PUNPCKHQ128(uint128_t X, uint128_t Y);

__forceinline
uint128_t WIDEMATHAPI PXOR128(uint128_t X, uint128_t Y)
{
    uint128_t Result;

    Result.Q[0] = PXOR64(X.Q[0], Y.Q[0]);
    Result.Q[1] = PXOR64(X.Q[1], Y.Q[1]);

    return Result;
}

//
//  SSE floating point related operations
//

uint128_t WIDEMATHAPI MOVDDUP128(uint128_t X);

uint128_t WIDEMATHAPI MOVSLDUP128(uint128_t X);
uint128_t WIDEMATHAPI MOVSHDUP128(uint128_t X);

int WIDEMATHAPI MOVMSKPD128(uint128_t X);
int WIDEMATHAPI MOVMSKPS128(uint128_t X);

uint128_t WIDEMATHAPI MPSADBW128(uint128_t X, uint128_t Y, unsigned Imm);

unsigned WIDEMATHAPI UCOMIS64(uint64_t X, uint64_t Y, unsigned IsDouble);
unsigned WIDEMATHAPI UCOMISS128(uint128_t X, uint128_t Y);
unsigned WIDEMATHAPI UCOMISD128(uint128_t X, uint128_t Y) ;

uint128_t WIDEMATHAPI ADDPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI ADDPS128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI ADDSD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI ADDSS128(uint128_t X, uint128_t Y);

uint64_t WIDEMATHAPI ADDSD64(uint64_t X, uint64_t Y);
unsigned WIDEMATHAPI ADDSS64(uint64_t X, uint64_t Y);

uint128_t WIDEMATHAPI ADDSUBPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI ADDSUBPS128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI AESDEC128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI AESDECLAST128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI AESENC128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI AESENCLAST128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI AESIMC128(uint128_t Y);
uint128_t WIDEMATHAPI AESKEYGENASSIST128(uint128_t Y, unsigned Imm);

uint128_t WIDEMATHAPI BLENDPD128(uint128_t X, uint128_t Y, unsigned Mask);
uint128_t WIDEMATHAPI BLENDPS128(uint128_t X, uint128_t Y, unsigned Mask);
uint128_t WIDEMATHAPI BLENDVPD128(uint128_t X, uint128_t Y, uint128_t Mask);
uint128_t WIDEMATHAPI BLENDVPS128(uint128_t X, uint128_t Y, uint128_t Mask);

uint64_t WIDEMATHAPI CMPSD64(uint64_t X, uint64_t Y, int Operation);
unsigned WIDEMATHAPI CMPSS64(uint64_t X, uint64_t Y, int Operation);
uint128_t WIDEMATHAPI CMPPD128(uint128_t X, uint128_t Y, int Operation);
uint128_t WIDEMATHAPI CMPPS128(uint128_t X, uint128_t Y, int Operation);

uint128_t WIDEMATHAPI CLMULQDQ64(uint64_t X, uint64_t Y);

uint32_t WIDEMATHAPI _CRC64(uint32_t X, uint64_t Y, uint64_t Polynomial, unsigned Datawidth);

uint32_t WIDEMATHAPI _CVTSI2SS32(int32_t I32);
uint32_t WIDEMATHAPI CVTSI2SS64S(int64_t I64, uint16_t *FPStatus);
uint32_t WIDEMATHAPI CVTSI2SS64(int64_t I64);

uint128_t WIDEMATHAPI CVTDQ2PS128(uint128_t X);

uint64_t WIDEMATHAPI CVTSI2SD64S(int64_t I64, uint16_t *FPStatus);
uint64_t WIDEMATHAPI CVTSI2SD64(int64_t I64);

uint128_t WIDEMATHAPI CVTSI2SD80(int64_t I64);
uint128_t WIDEMATHAPI CVTSD2SD80(int64_t F64);

uint128_t WIDEMATHAPI CVTDQ2PD128(uint64_t X);

uint128_t WIDEMATHAPI CVTPI2PD128(uint64_t X);

uint64_t WIDEMATHAPI CVTPI2PS128(uint64_t X);

uint32_t WIDEMATHAPI CVTSD2SS64S(uint64_t X, uint16_t * FPStatus);
uint32_t WIDEMATHAPI CVTSD2SS64(uint64_t X);
uint128_t WIDEMATHAPI CVTSD2SS128(uint128_t X, uint128_t Y);

uint64_t WIDEMATHAPI CVTSS2SD64(uint64_t X);
uint128_t WIDEMATHAPI CVTSS2SD128(uint128_t X, uint128_t Y);

uint32_t WIDEMATHAPI _CVTSD2SI32(uint64_t X, unsigned RoundingMode);
uint64_t WIDEMATHAPI _CVTSD2SI64(uint64_t X, unsigned RoundingMode);
uint32_t WIDEMATHAPI _CVTSS2SI32(uint32_t X, unsigned RoundingMode);

uint128_t WIDEMATHAPI CVTPD2DQ128(uint128_t X, unsigned RoundingMode);
uint128_t WIDEMATHAPI CVTTPD2DQ128(uint128_t X);

uint128_t WIDEMATHAPI CVTPS2DQ128(uint128_t X, unsigned RoundingMode);
uint128_t WIDEMATHAPI CVTTPS2DQ128(uint128_t X);

uint64_t WIDEMATHAPI CVTPD2PI128(uint128_t X, unsigned RoundingMode);
uint64_t WIDEMATHAPI CVTTPD2PI128(uint128_t X);

uint64_t WIDEMATHAPI CVTPS2PI128(uint64_t X, unsigned RoundingMode);
uint64_t WIDEMATHAPI CVTTPS2PI128(uint64_t X);

uint128_t WIDEMATHAPI CVTSI2SD128(uint128_t X, int64_t Y);
uint128_t WIDEMATHAPI CVTSI2SS128(uint128_t X, int64_t Y);

uint128_t WIDEMATHAPI DIVPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI DIVPS128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI DIVSD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI DIVSS128(uint128_t X, uint128_t Y);

uint64_t WIDEMATHAPI DIVSD64(uint64_t X, uint64_t Y);
unsigned WIDEMATHAPI DIVSS64(uint64_t X, uint64_t Y);

uint128_t WIDEMATHAPI DPPD128(uint128_t X, uint128_t Y, unsigned Imm);
uint128_t WIDEMATHAPI DPPS128(uint128_t X, uint128_t Y, unsigned Imm);

uint128_t WIDEMATHAPI HADDPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI HADDPS128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI HSUBPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI HSUBPS128(uint128_t X, uint128_t Y);

uint128_t WIDEMATHAPI INSERTPS128(uint128_t X, uint128_t Y, unsigned Imm);

uint128_t WIDEMATHAPI MAXPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MAXPS128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MAXSD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MAXSS128(uint128_t X, uint128_t Y);

uint64_t WIDEMATHAPI MAXSD64(uint64_t X, uint64_t Y);
unsigned WIDEMATHAPI MAXSS64(uint64_t X, uint64_t Y);

uint128_t WIDEMATHAPI MINPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MINPS128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MINSD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MINSS128(uint128_t X, uint128_t Y);

uint64_t WIDEMATHAPI MINSD64(uint64_t X, uint64_t Y);
unsigned WIDEMATHAPI MINSS64(uint64_t X, uint64_t Y);

uint128_t WIDEMATHAPI MULPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MULPS128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MULSD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI MULSS128(uint128_t X, uint128_t Y);

uint64_t WIDEMATHAPI MULSD64(uint64_t X, uint64_t Y);
unsigned WIDEMATHAPI MULSS64(uint64_t X, uint64_t Y);

uint128_t WIDEMATHAPI ROUNDSD128(uint128_t X, uint128_t Y, unsigned RoundingMode);
uint128_t WIDEMATHAPI ROUNDSS128(uint128_t X, uint128_t Y, unsigned RoundingMode);
uint128_t WIDEMATHAPI ROUNDPD128(uint128_t Y, unsigned RoundingMode);
uint128_t WIDEMATHAPI ROUNDPS128(uint128_t Y, unsigned RoundingMode);

unsigned WIDEMATHAPI RCPSS64(uint64_t Y);
uint128_t WIDEMATHAPI RCPSS128(uint128_t Y);
uint128_t WIDEMATHAPI RCPPS128(uint128_t Y);

unsigned WIDEMATHAPI RSQRTSS64(uint64_t Y);
uint128_t WIDEMATHAPI RSQRTSS128(uint128_t Y);
uint128_t WIDEMATHAPI RSQRTPS128(uint128_t Y);

uint128_t WIDEMATHAPI SQRTPD128(uint128_t Y);
uint128_t WIDEMATHAPI SQRTPS128(uint128_t Y);
uint128_t WIDEMATHAPI SQRTSD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI SQRTSS128(uint128_t Y);

unsigned WIDEMATHAPI SQRTSS64(uint64_t X);
uint64_t WIDEMATHAPI SQRTSD64(uint64_t X);

uint128_t WIDEMATHAPI SHUFPD128(uint128_t X, uint128_t Y, unsigned Imm);
uint128_t WIDEMATHAPI SHUFPS128(uint128_t X, uint128_t Y, unsigned Imm);

uint128_t WIDEMATHAPI SUBPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI SUBPS128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI SUBSD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI SUBSS128(uint128_t X, uint128_t Y);

uint64_t WIDEMATHAPI SUBSD64(uint64_t X, uint64_t Y);
unsigned WIDEMATHAPI SUBSS64(uint64_t X, uint64_t Y);

uint128_t WIDEMATHAPI UNPCKHPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI UNPCKHPS128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI UNPCKLPD128(uint128_t X, uint128_t Y);
uint128_t WIDEMATHAPI UNPCKLPS128(uint128_t X, uint128_t Y);

//
// Misc.
//

uint128_t WIDEMATHAPI SETZERO(void);

uint128_t WIDEMATHAPI BROADCAST128(uint64_t X, unsigned DataWidth);

void WIDEMATHAPI CPUIDEX64(int CPUInfo[4], int Function, int Subfunction);

uint64_t WIDEMATHAPI RDTSC64(void);
uint64_t WIDEMATHAPI RDTSCP64(unsigned int *p);

void WIDEMATHAPI _FXRSTOR(void const *);
void WIDEMATHAPI _FXRSTOR64(void const *);
void WIDEMATHAPI _FXSAVE(void *);
void WIDEMATHAPI _FXSAVE64(void *);

#ifdef __cplusplus

}  // extern "C"

#endif

