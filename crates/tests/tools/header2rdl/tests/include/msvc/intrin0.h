/***
*   intrin0.h - declarations of compiler intrinsics used by the C++ Standard Library.
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*   This header file declares compiler intrinsics that are used by the
*   C++ Standard Library, especially <atomic>. Compiler throughput is
*   the only reason that intrin0.h is separate from intrin.h.
*
****/

#pragma once
#define __INTRIN0_H_

#ifdef _M_ARM64EC
#include <intrin.h>
#else  // _M_ARM64EC
#include <intrin0.inl.h>
#endif // _M_ARM64EC
