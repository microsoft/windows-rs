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
 * Helpers required by the DEFINE_ENUM_FLAG_OPERATORS expansion (matching the
 * definitions from the real Windows SDK <winnt.h>).
 * ---------------------------------------------------------------------- */
#ifndef WIN_NOEXCEPT
#  ifdef __cplusplus
#    define WIN_NOEXCEPT noexcept
#  else
#    define WIN_NOEXCEPT
#  endif
#endif

#ifndef _ENUM_FLAG_CONSTEXPR
#  define _ENUM_FLAG_CONSTEXPR constexpr
#endif

#ifdef __cplusplus
template <__SIZE_TYPE__ S>
struct _ENUM_FLAG_INTEGER_FOR_SIZE;
template <>
struct _ENUM_FLAG_INTEGER_FOR_SIZE<1> { typedef unsigned char type; };
template <>
struct _ENUM_FLAG_INTEGER_FOR_SIZE<2> { typedef unsigned short type; };
template <>
struct _ENUM_FLAG_INTEGER_FOR_SIZE<4> { typedef unsigned int type; };
template <>
struct _ENUM_FLAG_INTEGER_FOR_SIZE<8> { typedef unsigned long long type; };
template <typename T>
struct _ENUM_FLAG_SIZED_INTEGER {
    typedef typename _ENUM_FLAG_INTEGER_FOR_SIZE<sizeof(T)>::type type;
};
#endif

/* -------------------------------------------------------------------------
 * DEFINE_ENUM_FLAG_OPERATORS — bitwise operators for scoped enum types.
 * The expansion produces inline functions with bodies; the collector skips
 * those since they cannot be represented in RDL.
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
