/***
*   arm64intr.h - definitions and declarations for ARM64 specific intrinsics
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*       This file contains constant definitions and external subroutine
*       declarations for the ARM64 specific intrinsics
*
****/

#pragma once


#if !defined(_M_ARM64) && !defined(_M_HYBRID_X86_ARM64) && !defined(_M_ARM64EC)
#error This header is specific to ARM64 targets
#endif

#define ARM64_SYSREG(op0, op1, crn, crm, op2) \
        ( ((op0 & 1) << 14) | \
          ((op1 & 7) << 11) | \
          ((crn & 15) << 7) | \
          ((crm & 15) << 3) | \
          ((op2 & 7) << 0) )

#define ARM64_FPCR              ARM64_SYSREG(3, 3, 4, 4, 0)   // Floating point control register (EL0)
#define ARM64_FPSR              ARM64_SYSREG(3, 3, 4, 4, 1)   // Floating point status register (EL0)

typedef struct {
    unsigned __int64 _val[8];
} data512_t;

#if defined(__cplusplus)
extern "C" {
#endif

// ARM64-TRANSITION: define more. Only need barrier so far.

typedef enum _tag_ARM64INTR_BARRIER_TYPE
{
    _ARM64_BARRIER_SY    = 0xF,
    _ARM64_BARRIER_ST    = 0xE,
    _ARM64_BARRIER_LD    = 0xD,
    _ARM64_BARRIER_ISH   = 0xB,
    _ARM64_BARRIER_ISHST = 0xA,
    _ARM64_BARRIER_ISHLD = 0x9,
    _ARM64_BARRIER_NSH   = 0x7,
    _ARM64_BARRIER_NSHST = 0x6,
    _ARM64_BARRIER_NSHLD = 0x5,
    _ARM64_BARRIER_OSH   = 0x3,
    _ARM64_BARRIER_OSHST = 0x2,
    _ARM64_BARRIER_OSHLD = 0x1
}
_ARM64INTR_BARRIER_TYPE;

void __dmb(unsigned int _Type);
void __dsb(unsigned int _Type);
void __isb(unsigned int _Type);
void __sb(void);

void __ld64b(const void *_addr, unsigned __int64 _value[8]);
void __st64b(void *_addr, unsigned __int64 _value[8]);
unsigned __int64 __st64bv(void *_addr, unsigned __int64 _value[8]);
unsigned __int64 __st64bv0(void *_addr, unsigned __int64 _value[8]);

static __forceinline data512_t __arm_ld64b(const void *_addr) 
{ 
    data512_t _value;
    __ld64b(_addr, _value._val);
    return _value;
}

static __forceinline void __arm_st64b(void *_addr, data512_t _value)  
{
    __st64b(_addr, _value._val);
}

static __forceinline unsigned __int64 __arm_st64bv(void *_addr, data512_t _value)  
{
    return __st64bv(_addr, _value._val);
}

static __forceinline unsigned __int64 __arm_st64bv0(void *_addr, data512_t _value) 
{
    return __st64bv0(_addr, _value._val);
}

unsigned __int8 __ldar8(const volatile unsigned __int8 * _Target);
unsigned __int16 __ldar16(const volatile unsigned __int16 * _Target);
unsigned __int32 __ldar32(const volatile unsigned __int32 * _Target);
unsigned __int64 __ldar64(const volatile unsigned __int64 * _Target);

unsigned __int8 __ldapr8(const volatile unsigned __int8 * _Target);
unsigned __int16 __ldapr16(const volatile unsigned __int16 * _Target);
unsigned __int32 __ldapr32(const volatile unsigned __int32 * _Target);
unsigned __int64 __ldapr64(const volatile unsigned __int64 * _Target);

unsigned __int8 __load_acquire8(const volatile unsigned __int8 * _Target);
unsigned __int16 __load_acquire16(const volatile unsigned __int16 * _Target);
unsigned __int32 __load_acquire32(const volatile unsigned __int32 * _Target);
unsigned __int64 __load_acquire64(const volatile unsigned __int64 * _Target);

unsigned __int8 __ldxr8(const volatile unsigned __int8 * _Target);
unsigned __int16 __ldxr16(const volatile unsigned __int16 * _Target);
unsigned __int32 __ldxr32(const volatile unsigned __int32 * _Target);
unsigned __int64 __ldxr64(const volatile unsigned __int64 * _Target);
unsigned __int8 __ldaxr8(const volatile unsigned __int8 * _Target);
unsigned __int16 __ldaxr16(const volatile unsigned __int16 * _Target);
unsigned __int32 __ldaxr32(const volatile unsigned __int32 * _Target);
unsigned __int64 __ldaxr64(const volatile unsigned __int64 * _Target);

unsigned __int8 __stxr8(volatile unsigned __int8 * _Target, unsigned __int8 _Value);
unsigned __int8 __stxr16(volatile unsigned __int16 * _Target, unsigned __int16 _Value);
unsigned __int8 __stxr32(volatile unsigned __int32 * _Target, unsigned __int32 _Value);
unsigned __int8 __stxr64(volatile unsigned __int64 * _Target, unsigned __int64 _Value);
unsigned __int8 __stlxr8(volatile unsigned __int8 * _Target, unsigned __int8 _Value);
unsigned __int8 __stlxr16(volatile unsigned __int16 * _Target, unsigned __int16 _Value);
unsigned __int8 __stlxr32(volatile unsigned __int32 * _Target, unsigned __int32 _Value);
unsigned __int8 __stlxr64(volatile unsigned __int64 * _Target, unsigned __int64 _Value);

void __clrex(unsigned __int8 crm);

void __stlr8(volatile unsigned __int8 * _Target, unsigned __int8 _Value);
void __stlr16(volatile unsigned __int16 * _Target, unsigned __int16 _Value);
void __stlr32(volatile unsigned __int32 * _Target, unsigned __int32 _Value);
void __stlr64(volatile unsigned __int64 * _Target, unsigned __int64 _Value);

// load/store unprivileged
unsigned __int8 __ldtr8(const volatile unsigned __int8 * _Target);
unsigned __int16 __ldtr16(const volatile unsigned __int16 * _Target);
unsigned __int32 __ldtr32(const volatile unsigned __int32 * _Target);
unsigned __int64 __ldtr64(const volatile unsigned __int64 * _Target);

signed __int8 __ldtrs8(const volatile __int8 * _Target);
signed __int16 __ldtrs16(const volatile __int16 * _Target);
signed __int32 __ldtrs32(const volatile __int32 * _Target);

void __sttr8(volatile unsigned __int8 * _Target, unsigned __int8 _Value);
void __sttr16(volatile unsigned __int16 * _Target, unsigned __int16 _Value);
void __sttr32(volatile unsigned __int32 * _Target, unsigned __int32 _Value);
void __sttr64(volatile unsigned __int64 * _Target, unsigned __int64 _Value);

unsigned __int8 __swp8(unsigned __int8 volatile * _Target, unsigned __int8 _Value);
unsigned __int16 __swp16(unsigned __int16 volatile * _Target, unsigned __int16 _Value);
unsigned __int32 __swp32(unsigned __int32 volatile * _Target, unsigned __int32 _Value);
unsigned __int64 __swp64(unsigned __int64 volatile * _Target, unsigned __int64 _Value);
unsigned __int8 __swpa8(unsigned __int8 volatile * _Target, unsigned __int8 _Value);
unsigned __int16 __swpa16(unsigned __int16 volatile * _Target, unsigned __int16 _Value);
unsigned __int32 __swpa32(unsigned __int32 volatile * _Target, unsigned __int32 _Value);
unsigned __int64 __swpa64(unsigned __int64 volatile * _Target, unsigned __int64 _Value);
unsigned __int8 __swpl8(unsigned __int8 volatile * _Target, unsigned __int8 _Value);
unsigned __int16 __swpl16(unsigned __int16 volatile * _Target, unsigned __int16 _Value);
unsigned __int32 __swpl32(unsigned __int32 volatile * _Target, unsigned __int32 _Value);
unsigned __int64 __swpl64(unsigned __int64 volatile * _Target, unsigned __int64 _Value);
unsigned __int8 __swpal8(unsigned __int8 volatile * _Target, unsigned __int8 _Value);
unsigned __int16 __swpal16(unsigned __int16 volatile * _Target, unsigned __int16 _Value);
unsigned __int32 __swpal32(unsigned __int32 volatile * _Target, unsigned __int32 _Value);
unsigned __int64 __swpal64(unsigned __int64 volatile * _Target, unsigned __int64 _Value);

unsigned __int8 __cas8(unsigned __int8 volatile * _Target, unsigned __int8 _Comp, unsigned __int8 _Value);
unsigned __int16 __cas16(unsigned __int16 volatile * _Target, unsigned __int16 _Comp, unsigned __int16 _Value);
unsigned __int32 __cas32(unsigned __int32 volatile * _Target, unsigned __int32 _Comp, unsigned __int32 _Value);
unsigned __int64 __cas64(unsigned __int64 volatile * _Target, unsigned __int64 _Comp, unsigned __int64 _Value);
unsigned __int8 __casa8(unsigned __int8 volatile * _Target, unsigned __int8 _Comp, unsigned __int8 _Value);
unsigned __int16 __casa16(unsigned __int16 volatile * _Target, unsigned __int16 _Comp, unsigned __int16 _Value);
unsigned __int32 __casa32(unsigned __int32 volatile * _Target, unsigned __int32 _Comp, unsigned __int32 _Value);
unsigned __int64 __casa64(unsigned __int64 volatile * _Target, unsigned __int64 _Comp, unsigned __int64 _Value);
unsigned __int8 __casl8(unsigned __int8 volatile * _Target, unsigned __int8 _Comp, unsigned __int8 _Value);
unsigned __int16 __casl16(unsigned __int16 volatile * _Target, unsigned __int16 _Comp, unsigned __int16 _Value);
unsigned __int32 __casl32(unsigned __int32 volatile * _Target, unsigned __int32 _Comp, unsigned __int32 _Value);
unsigned __int64 __casl64(unsigned __int64 volatile * _Target, unsigned __int64 _Comp, unsigned __int64 _Value);
unsigned __int8 __casal8(unsigned __int8 volatile * _Target, unsigned __int8 _Comp, unsigned __int8 _Value);
unsigned __int16 __casal16(unsigned __int16 volatile * _Target, unsigned __int16 _Comp, unsigned __int16 _Value);
unsigned __int32 __casal32(unsigned __int32 volatile * _Target, unsigned __int32 _Comp, unsigned __int32 _Value);
unsigned __int64 __casal64(unsigned __int64 volatile * _Target, unsigned __int64 _Comp, unsigned __int64 _Value);

void * __xpaci(void * _Pointer);

// Reverses the bits in _Value.
unsigned __int32 __rbit(unsigned __int32 _Value);
unsigned long __rbitl(unsigned long _Value);
unsigned __int64 __rbitll(unsigned __int64 _Value);

#ifdef __cplusplus
}
#endif
