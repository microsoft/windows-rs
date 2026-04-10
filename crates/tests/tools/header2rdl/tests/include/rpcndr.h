#pragma once
/* Minimal shim for <rpcndr.h> — satisfies the version check in MIDL-generated headers. */

/* Satisfy the __RPCNDR_H_VERSION__ version check. */
#ifndef __RPCNDR_H_VERSION__
#define __RPCNDR_H_VERSION__ 475
#endif

/* -------------------------------------------------------------------------
 * EXTERN_C — expands to extern "C" in C++ mode, nothing in C.
 * ---------------------------------------------------------------------- */
#ifndef EXTERN_C
#  ifdef __cplusplus
#    define EXTERN_C extern "C"
#  else
#    define EXTERN_C extern
#  endif
#endif

/* -------------------------------------------------------------------------
 * DEFINE_ENUM_FLAG_OPERATORS — bitwise operators for scoped enum types.
 * In C++ mode this expands to inline operator overloads; we just define it
 * as a no-op so it doesn't produce any AST nodes that confuse the collector.
 * ---------------------------------------------------------------------- */
#ifndef DEFINE_ENUM_FLAG_OPERATORS
#define DEFINE_ENUM_FLAG_OPERATORS(ENUMTYPE) \
extern "C++" { \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator | (ENUMTYPE a, ENUMTYPE b) WIN_NOEXCEPT { return ENUMTYPE(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a) | ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline ENUMTYPE &operator |= (ENUMTYPE &a, ENUMTYPE b) WIN_NOEXCEPT { return (ENUMTYPE &)(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type &)a) |= ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator & (ENUMTYPE a, ENUMTYPE b) WIN_NOEXCEPT { return ENUMTYPE(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a) & ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline ENUMTYPE &operator &= (ENUMTYPE &a, ENUMTYPE b) WIN_NOEXCEPT { return (ENUMTYPE &)(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type &)a) &= ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator ~ (ENUMTYPE a) WIN_NOEXCEPT { return ENUMTYPE(~((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a)); } \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator ^ (ENUMTYPE a, ENUMTYPE b) WIN_NOEXCEPT { return ENUMTYPE(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a) ^ ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline ENUMTYPE &operator ^= (ENUMTYPE &a, ENUMTYPE b) WIN_NOEXCEPT { return (ENUMTYPE &)(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type &)a) ^= ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
}
#endif
