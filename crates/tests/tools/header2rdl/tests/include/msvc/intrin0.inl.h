/***
*   intrin0.inl.h - declarations of compiler intrinsics used by the C++
*                   Standard Library.
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*   This header file declares compiler intrinsics that are used by the
*   C++ Standard Library, especially <atomic>. Compiler throughput is
*   the only reason that intrin0.inl.h is separate from intrin.h.
*
****/

#pragma once
#define __INTRIN0_INL_H_
#include <vcruntime.h>

#if _VCRT_COMPILER_PREPROCESSOR && !defined(__midl)

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)
// Suppress C28251: Inconsistent annotation for prior declaration.
// Depending on the include order the definition may not exist so
// _Use_decl_annotations_ can not be used.
#pragma warning(disable: 28251)

#if defined(__cplusplus)
extern "C" {
#endif

/*
** __MACHINE              : everything
** __MACHINEX86           : x86 only
** __MACHINEX64           : x64 only
** __MACHINEX86_X64       : x86 and x64 only
** __MACHINEX86_X64_ARM64 : x86, x64, and ARM64 only
** __MACHINEARM           : ARM only
** __MACHINEARM64         : ARM64 only
** __MACHINEARM_ARM64     : ARM and ARM64 only
** __MACHINEARM_ARM64_X64 : ARM and 64-bit Arch only
** __MACHINEARM64_X64     : ARM64 and x64 only
** __MACHINECHPEX86ARM64  : CHPE x86 on arm64 only
** __MACHINEWVMPURE       : /clr:pure only
** __MACHINEZ             : nothing
*/

#define __MACHINEX86           __MACHINE
#define __MACHINEX64           __MACHINE
#define __MACHINEX86_X64       __MACHINE
#define __MACHINEX86_X64_ARM64 __MACHINE
#define __MACHINEARM           __MACHINE
#define __MACHINEARM64         __MACHINE
#define __MACHINEARM_ARM64     __MACHINE
#define __MACHINEARM_ARM64_X64 __MACHINE
#define __MACHINEARM64_X64     __MACHINE
#define __MACHINECHPEX86ARM64  __MACHINE
#define __MACHINEARM64EC       __MACHINE

/* Most intrinsics not available to pure managed code */
#if defined(_M_CEE_PURE)
#define __MACHINE(X)        __MACHINEZ(X)
#define __MACHINEWVMPURE(X) X;
#else
#define __MACHINE(X)        X;
#define __MACHINEWVMPURE(X) __MACHINEZ(X)
#endif

#define __MACHINEZ(X)       /* NOTHING */

#if !defined(_M_IX86) || defined(_CHPE_ONLY_)
#undef __MACHINEX86
#define __MACHINEX86        __MACHINEZ
#endif

#if !defined(_M_X64) || defined(_M_ARM64EC)
#undef __MACHINEX64
#define __MACHINEX64        __MACHINEZ
#endif

#if !(defined(_M_IX86) || defined(_M_X64)) || defined(_CHPE_ONLY_) || defined(_M_ARM64EC)
#undef __MACHINEX86_X64
#define __MACHINEX86_X64    __MACHINEZ
#endif

#if !defined(_M_ARM)
#undef  __MACHINEARM
#define __MACHINEARM        __MACHINEZ
#endif

/* For compatibility with <winnt.h>, some intrinsics are __cdecl except on x64 */
#if defined(_M_X64)
#define __MACHINECALL_CDECL_OR_DEFAULT
#else
#define __MACHINECALL_CDECL_OR_DEFAULT __cdecl
#endif

#if !defined(_M_ARM64) && !defined(_M_HYBRID_X86_ARM64) && !defined(_M_ARM64EC)
#undef  __MACHINEARM64
#define __MACHINEARM64      __MACHINEZ
#endif

#if !(defined(_M_ARM) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC))
#undef __MACHINEARM_ARM64
#define __MACHINEARM_ARM64  __MACHINEZ
#endif

#if !(defined(_M_ARM) || defined(_M_X64) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC))
#undef __MACHINEARM_ARM64_X64
#define __MACHINEARM_ARM64_X64     __MACHINEZ
#endif

#if !(defined(_M_X64) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC))
#undef __MACHINEARM64_X64
#define __MACHINEARM64_X64     __MACHINEZ
#endif

#if !(defined(_M_IX86) || defined(_M_X64) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)) || defined(_CHPE_ONLY_) 
#undef __MACHINEX86_ARM64_X64
#define __MACHINEX86_ARM64_X64     __MACHINEZ
#endif

#if !defined(_M_HYBRID_X86_ARM64)
#undef __MACHINECHPEX86ARM64
#define __MACHINECHPEX86ARM64 __MACHINEZ
#endif

#if !defined(_M_ARM64EC)
#undef __MACHINEARM64EC
#define __MACHINEARM64EC __MACHINEZ
#endif

/*******************************************************************
* Note: New intrinsics should be added here IF AND ONLY IF they're *
* being used by the C++ Standard Library.                          *
* OTHERWISE, new intrinsics should be added to intrin.h.           *
*******************************************************************/

__MACHINEARM_ARM64(void __dmb(unsigned int _Type))

__MACHINE(unsigned char _BitScanForward(unsigned long * _Index, unsigned long _Mask))
__MACHINEX64(unsigned char _BitScanForward64(unsigned long * _Index, unsigned __int64 _Mask))
__MACHINEARM64(unsigned char _BitScanForward64(unsigned long * _Index, unsigned __int64 _Mask))
__MACHINE(unsigned char _BitScanReverse(unsigned long * _Index, unsigned long _Mask))
__MACHINEX64(unsigned char _BitScanReverse64(unsigned long * _Index, unsigned __int64 _Mask))
__MACHINEARM64(unsigned char _BitScanReverse64(unsigned long * _Index, unsigned __int64 _Mask))
__MACHINE(unsigned char _bittest(long const *, long))
__MACHINEARM_ARM64(unsigned int _CountLeadingZeros(unsigned long))
__MACHINEARM_ARM64(unsigned int _CountLeadingZeros64(unsigned __int64))
__MACHINEARM_ARM64(unsigned int _CountTrailingZeros(unsigned long))
__MACHINEARM_ARM64(unsigned int _CountTrailingZeros64(unsigned __int64))
__MACHINE(long _InterlockedAnd(long volatile * _Value, long _Mask))
__MACHINE(short _InterlockedAnd16(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedAnd16_acq(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedAnd16_nf(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedAnd16_rel(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64_X64(__int64 _InterlockedAnd64(__int64 volatile * _Value, __int64 _Mask))
__MACHINE(__int64 _interlockedand64(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedAnd64_acq(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedAnd64_nf(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedAnd64_rel(__int64 volatile * _Value, __int64 _Mask))
__MACHINE(char _InterlockedAnd8(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedAnd8_acq(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedAnd8_nf(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedAnd8_rel(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(long _InterlockedAnd_acq(long volatile * _Value, long _Mask))
__MACHINEARM_ARM64(long _InterlockedAnd_nf(long volatile * _Value, long _Mask))
__MACHINEARM_ARM64(long _InterlockedAnd_rel(long volatile * _Value, long _Mask))
__MACHINE(long __MACHINECALL_CDECL_OR_DEFAULT _InterlockedCompareExchange(long volatile * _Destination, long _Exchange, long _Comparand))
__MACHINEWVMPURE(long _InterlockedCompareExchange(long volatile * _Destination, long _Exchange, long _Comparand))
__MACHINE(short _InterlockedCompareExchange16(short volatile * _Destination, short _Exchange, short _Comparand))
__MACHINEARM_ARM64(short _InterlockedCompareExchange16_acq(short volatile * _Destination, short _Exchange, short _Comparand))
__MACHINEARM_ARM64(short _InterlockedCompareExchange16_nf(short volatile * _Destination, short _Exchange, short _Comparand))
__MACHINEARM_ARM64(short _InterlockedCompareExchange16_rel(short volatile * _Destination, short _Exchange, short _Comparand))
__MACHINE(__int64 _InterlockedCompareExchange64(__int64 volatile * _Destination, __int64 _Exchange, __int64 _Comparand))
__MACHINEARM_ARM64(__int64 _InterlockedCompareExchange64_acq(__int64 volatile * _Destination, __int64 _Exchange, __int64 _Comparand))
__MACHINEARM_ARM64(__int64 _InterlockedCompareExchange64_nf(__int64 volatile * _Destination, __int64 _Exchange, __int64 _Comparand))
__MACHINEARM_ARM64(__int64 _InterlockedCompareExchange64_rel(__int64 volatile * _Destination, __int64 _Exchange, __int64 _Comparand))
__MACHINE(char _InterlockedCompareExchange8(char volatile * _Destination, char _Exchange, char _Comparand))
__MACHINEARM_ARM64(char _InterlockedCompareExchange8_acq(char volatile * _Destination, char _Exchange, char _Comparand))
__MACHINEARM_ARM64(char _InterlockedCompareExchange8_nf(char volatile * _Destination, char _Exchange, char _Comparand))
__MACHINEARM_ARM64(char _InterlockedCompareExchange8_rel(char volatile * _Destination, char _Exchange, char _Comparand))
__MACHINEARM_ARM64(long _InterlockedCompareExchange_acq(long volatile * _Destination, long _Exchange, long _Comparand))
__MACHINEARM_ARM64(long _InterlockedCompareExchange_nf(long volatile * _Destination, long _Exchange, long _Comparand))
__MACHINEARM_ARM64(long _InterlockedCompareExchange_rel(long volatile * _Destination, long _Exchange, long _Comparand))
__MACHINEARM64_X64(unsigned char _InterlockedCompareExchange128(__int64 volatile * _Destination, __int64 _ExchangeHigh, __int64 _ExchangeLow, __int64 * _ComparandResult))
__MACHINEARM64(unsigned char _InterlockedCompareExchange128_acq(__int64 volatile * _Destination, __int64 _ExchangeHigh, __int64 _ExchangeLow, __int64 * _ComparandResult))
__MACHINEARM64(unsigned char _InterlockedCompareExchange128_nf(__int64 volatile * _Destination, __int64 _ExchangeHigh, __int64 _ExchangeLow, __int64 * _ComparandResult))
__MACHINEARM64(unsigned char _InterlockedCompareExchange128_rel(__int64 volatile * _Destination, __int64 _ExchangeHigh, __int64 _ExchangeLow, __int64 * _ComparandResult))
__MACHINE(long __MACHINECALL_CDECL_OR_DEFAULT _InterlockedDecrement(long volatile * _Addend))
__MACHINEWVMPURE(long _InterlockedDecrement(long volatile * _Addend))
__MACHINE(short _InterlockedDecrement16(short volatile * _Addend))
__MACHINEARM_ARM64_X64(__int64 _InterlockedDecrement64(__int64 volatile * _Addend))
__MACHINE(__int64 _interlockeddecrement64(__int64 volatile * _Addend))
__MACHINE(long __MACHINECALL_CDECL_OR_DEFAULT _InterlockedExchange(long volatile * _Target, long _Value))
__MACHINEWVMPURE(long __MACHINECALL_CDECL_OR_DEFAULT _InterlockedExchange(long volatile * _Target, long _Value))
__MACHINE(short _InterlockedExchange16(short volatile * _Target, short _Value))
__MACHINEARM_ARM64(short _InterlockedExchange16_acq(short volatile * _Target, short _Value))
__MACHINEARM_ARM64(short _InterlockedExchange16_nf(short volatile * _Target, short _Value))
__MACHINEARM_ARM64(short _InterlockedExchange16_rel(short volatile * _Target, short _Value))
__MACHINEARM_ARM64_X64(__int64 _InterlockedExchange64(__int64 volatile * _Target, __int64 _Value))
__MACHINE(__int64 _interlockedexchange64(__int64 volatile * _Target, __int64 _Value))
__MACHINEARM_ARM64(__int64 _InterlockedExchange64_acq(__int64 volatile * _Target, __int64 _Value))
__MACHINEARM_ARM64(__int64 _InterlockedExchange64_nf(__int64 volatile * _Target, __int64 _Value))
__MACHINEARM_ARM64(__int64 _InterlockedExchange64_rel(__int64 volatile * _Target, __int64 _Value))
__MACHINE(char _InterlockedExchange8(char volatile * _Target, char _Value))
__MACHINEARM_ARM64(char _InterlockedExchange8_acq(char volatile * _Target, char _Value))
__MACHINEARM_ARM64(char _InterlockedExchange8_nf(char volatile * _Target, char _Value))
__MACHINEARM_ARM64(char _InterlockedExchange8_rel(char volatile * _Target, char _Value))
__MACHINE(long __MACHINECALL_CDECL_OR_DEFAULT _InterlockedExchangeAdd(long volatile * _Addend, long _Value))
__MACHINE(short _InterlockedExchangeAdd16(short volatile * _Addend, short _Value))
__MACHINEARM_ARM64(short _InterlockedExchangeAdd16_acq(short volatile * _Addend, short _Value))
__MACHINEARM_ARM64(short _InterlockedExchangeAdd16_nf(short volatile * _Addend, short _Value))
__MACHINEARM_ARM64(short _InterlockedExchangeAdd16_rel(short volatile * _Addend, short _Value))
__MACHINEARM_ARM64_X64(__int64 _InterlockedExchangeAdd64(__int64 volatile * _Addend, __int64 _Value))
__MACHINE(__int64 _interlockedexchangeadd64(__int64 volatile * _Addend, __int64 _Value))
__MACHINEARM_ARM64(__int64 _InterlockedExchangeAdd64_acq(__int64 volatile * _Addend, __int64 _Value))
__MACHINEARM_ARM64(__int64 _InterlockedExchangeAdd64_nf(__int64 volatile * _Addend, __int64 _Value))
__MACHINEARM_ARM64(__int64 _InterlockedExchangeAdd64_rel(__int64 volatile * _Addend, __int64 _Value))
__MACHINE(char _InterlockedExchangeAdd8(char volatile * _Addend, char _Value))
__MACHINEARM_ARM64(char _InterlockedExchangeAdd8_acq(char volatile * _Addend, char _Value))
__MACHINEARM_ARM64(char _InterlockedExchangeAdd8_nf(char volatile * _Addend, char _Value))
__MACHINEARM_ARM64(char _InterlockedExchangeAdd8_rel(char volatile * _Addend, char _Value))
__MACHINEARM_ARM64(long _InterlockedExchangeAdd_acq(long volatile * _Addend, long _Value))
__MACHINEARM_ARM64(long _InterlockedExchangeAdd_nf(long volatile * _Addend, long _Value))
__MACHINEARM_ARM64(long _InterlockedExchangeAdd_rel(long volatile * _Addend, long _Value))
__MACHINEARM_ARM64(long _InterlockedExchange_acq(long volatile * _Target, long _Value))
__MACHINEARM_ARM64(long _InterlockedExchange_nf(long volatile * _Target, long _Value))
__MACHINEARM_ARM64(long _InterlockedExchange_rel(long volatile * _Target, long _Value))
__MACHINE(long __MACHINECALL_CDECL_OR_DEFAULT _InterlockedIncrement(long volatile * _Addend))
__MACHINEWVMPURE(long _InterlockedIncrement(long volatile * _Addend))
__MACHINE(short _InterlockedIncrement16(short volatile * _Addend))
__MACHINEARM_ARM64_X64(__int64 _InterlockedIncrement64(__int64 volatile * _Addend))
__MACHINE(__int64 _interlockedincrement64(__int64 volatile * _Addend))
__MACHINEARM_ARM64(long _InterlockedIncrement_nf(long volatile * _Addend))
__MACHINE(long _InterlockedOr(long volatile * _Value, long _Mask))
__MACHINE(short _InterlockedOr16(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedOr16_acq(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedOr16_nf(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedOr16_rel(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64_X64(__int64 _InterlockedOr64(__int64 volatile * _Value, __int64 _Mask))
__MACHINE(__int64 _interlockedor64(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedOr64_acq(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedOr64_nf(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedOr64_rel(__int64 volatile * _Value, __int64 _Mask))
__MACHINE(char _InterlockedOr8(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedOr8_acq(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedOr8_nf(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedOr8_rel(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(long _InterlockedOr_acq(long volatile * _Value, long _Mask))
__MACHINEARM_ARM64(long _InterlockedOr_nf(long volatile * _Value, long _Mask))
__MACHINEARM_ARM64(long _InterlockedOr_rel(long volatile * _Value, long _Mask))
__MACHINE(long _InterlockedXor(long volatile * _Value, long _Mask))
__MACHINE(short _InterlockedXor16(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedXor16_acq(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedXor16_nf(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64(short _InterlockedXor16_rel(short volatile * _Value, short _Mask))
__MACHINEARM_ARM64_X64(__int64 _InterlockedXor64(__int64 volatile * _Value, __int64 _Mask))
__MACHINE(__int64 _interlockedxor64(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedXor64_acq(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedXor64_nf(__int64 volatile * _Value, __int64 _Mask))
__MACHINEARM_ARM64(__int64 _InterlockedXor64_rel(__int64 volatile * _Value, __int64 _Mask))
__MACHINE(char _InterlockedXor8(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedXor8_acq(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedXor8_nf(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(char _InterlockedXor8_rel(char volatile * _Value, char _Mask))
__MACHINEARM_ARM64(long _InterlockedXor_acq(long volatile * _Value, long _Mask))
__MACHINEARM_ARM64(long _InterlockedXor_nf(long volatile * _Value, long _Mask))
__MACHINEARM_ARM64(long _InterlockedXor_rel(long volatile * _Value, long _Mask))
__MACHINE(void _ReadWriteBarrier(void))
__MACHINE(__int16 __iso_volatile_load16(const volatile __int16 *))
__MACHINE(__int32 __iso_volatile_load32(const volatile __int32 *))
__MACHINE(__int64 __iso_volatile_load64(const volatile __int64 *))
__MACHINE(__int8 __iso_volatile_load8(const volatile __int8 *))
__MACHINE(void __iso_volatile_store16(volatile __int16 *, __int16))
__MACHINE(void __iso_volatile_store32(volatile __int32 *, __int32))
__MACHINE(void __iso_volatile_store64(volatile __int64 *, __int64))
__MACHINE(void __iso_volatile_store8(volatile __int8 *, __int8))
__MACHINEARM(__int64 __ldrexd(const volatile __int64 *))
__MACHINEARM_ARM64(void __yield(void))
__MACHINEX86_X64(void _mm_pause(void))
__MACHINEX86_X64(unsigned int __lzcnt(unsigned int))
__MACHINEX86_X64(unsigned short __lzcnt16(unsigned short))
__MACHINEX64(unsigned __int64 __lzcnt64(unsigned __int64))
__MACHINEX86_X64_ARM64(unsigned int __popcnt(unsigned int))
__MACHINEX86_X64_ARM64(unsigned short __popcnt16(unsigned short))
__MACHINEARM64_X64(unsigned __int64 __popcnt64(unsigned __int64))
__MACHINE(unsigned int __cdecl _rotl(_In_ unsigned int _Value, _In_ int _Shift))
__MACHINE(unsigned short __cdecl _rotl16(unsigned short _Value, unsigned char _Shift))
__MACHINE(unsigned __int64 __cdecl _rotl64(_In_ unsigned __int64 _Value, _In_ int _Shift))
__MACHINE(unsigned char __cdecl _rotl8(unsigned char _Value, unsigned char _Shift))
__MACHINE(unsigned int __cdecl _rotr(_In_ unsigned int _Value, _In_ int _Shift))
__MACHINE(unsigned short __cdecl _rotr16(unsigned short _Value, unsigned char _Shift))
__MACHINE(unsigned __int64 __cdecl _rotr64(_In_ unsigned __int64 _Value, _In_ int _Shift))
__MACHINE(unsigned char __cdecl _rotr8(unsigned char _Value, unsigned char _Shift))
__MACHINEX64(unsigned __int64 __shiftleft128(unsigned __int64 _LowPart, unsigned __int64 _HighPart, unsigned char _Shift))
__MACHINEX64(unsigned __int64 __shiftright128(unsigned __int64 _LowPart, unsigned __int64 _HighPart, unsigned char _Shift))
#ifndef __clang__
__MACHINEX86_X64(unsigned int _tzcnt_u32(unsigned int))
__MACHINEX64(unsigned __int64 _tzcnt_u64(unsigned __int64))
#endif // __clang__
__MACHINEX64(unsigned __int64 _umul128(unsigned __int64 _Multiplier, unsigned __int64 _Multiplicand, unsigned __int64 * _HighProduct))
__MACHINEARM64_X64(unsigned __int64 __umulh(unsigned __int64, unsigned __int64))
// _udiv128 is also declared in <immintrin.h>
__MACHINEX64(unsigned __int64 __cdecl _udiv128(unsigned __int64 _HighDividend, unsigned __int64 _LowDividend, unsigned __int64 _Divisor, unsigned __int64* _Remainder))
__MACHINEX64(unsigned char __cdecl _addcarry_u64(unsigned char, unsigned __int64, unsigned __int64, unsigned __int64 *))
__MACHINEX64(unsigned char __cdecl _subborrow_u64(unsigned char, unsigned __int64, unsigned __int64, unsigned __int64 *))
__MACHINE(double __ceil(double))
__MACHINE(float __ceilf(float))
__MACHINE(double __floor(double))
__MACHINE(float __floorf(float))
__MACHINE(double __round(double))
__MACHINE(float __roundf(float))
__MACHINE(double __trunc(double))
__MACHINE(float __truncf(float))
__MACHINE(double __copysign(double, double))
__MACHINE(float __copysignf(float, float))
__MACHINE(unsigned __signbitvalue(double))
__MACHINE(unsigned __signbitvaluef(float))
__MACHINEARM64(unsigned __int8 __ldar8(const volatile unsigned __int8 * _Target))
__MACHINEARM64(unsigned __int16 __ldar16(const volatile unsigned __int16 * _Target))
__MACHINEARM64(unsigned __int32 __ldar32(const volatile unsigned __int32 * _Target))
__MACHINEARM64(unsigned __int64 __ldar64(const volatile unsigned __int64 * _Target))
__MACHINEARM64(unsigned __int8 __load_acquire8(const volatile unsigned __int8 * _Target))
__MACHINEARM64(unsigned __int16 __load_acquire16(const volatile unsigned __int16 * _Target))
__MACHINEARM64(unsigned __int32 __load_acquire32(const volatile unsigned __int32 * _Target))
__MACHINEARM64(unsigned __int64 __load_acquire64(const volatile unsigned __int64 * _Target))
__MACHINEARM64(void __stlr8(volatile unsigned __int8 * _Target, unsigned __int8 _Value))
__MACHINEARM64(void __stlr16(volatile unsigned __int16 * _Target, unsigned __int16 _Value))
__MACHINEARM64(void __stlr32(volatile unsigned __int32 * _Target, unsigned __int32 _Value))
__MACHINEARM64(void __stlr64(volatile unsigned __int64 * _Target, unsigned __int64 _Value))

#if defined(__cplusplus)
#if !defined(__clang__) && !defined(__EDG__) && !defined(__CUDACC__)
__MACHINE(constexpr void * __cdecl __builtin_assume_aligned(const void *, size_t, ...) noexcept)
#else
__MACHINE(void * __cdecl __builtin_assume_aligned(const void *, size_t, ...) noexcept)
#endif // !defined(__clang__) && !defined(__EDG__) && !defined(__CUDACC__)
#endif // defined(__cplusplus)

/*******************************************************************
* Note: New intrinsics should be added here IF AND ONLY IF they're *
* being used by the C++ Standard Library.                          *
* OTHERWISE, new intrinsics should be added to intrin.h.           *
*******************************************************************/

#if defined(__cplusplus)
}
#endif
#pragma warning(pop) // disable: _VCRUNTIME_DISABLED_WARNINGS, 28251
#endif /* _VCRT_COMPILER_PREPROCESSOR && !defined(__midl) */
