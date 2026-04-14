// __msvc_minmax.hpp internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_MINMAX_HPP
#define __MSVC_MINMAX_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR
#include <cstdint>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C" {
struct _Min_max_element_t {
    const void* _Min;
    const void* _Max;
};

struct _Min_max_1i {
    int8_t _Min;
    int8_t _Max;
};

struct _Min_max_1u {
    uint8_t _Min;
    uint8_t _Max;
};

struct _Min_max_2i {
    int16_t _Min;
    int16_t _Max;
};

struct _Min_max_2u {
    uint16_t _Min;
    uint16_t _Max;
};

struct _Min_max_4i {
    int32_t _Min;
    int32_t _Max;
};

struct _Min_max_4u {
    uint32_t _Min;
    uint32_t _Max;
};

struct _Min_max_8i {
    int64_t _Min;
    int64_t _Max;
};

struct _Min_max_8u {
    uint64_t _Min;
    uint64_t _Max;
};

struct _Min_max_f {
    float _Min;
    float _Max;
};

struct _Min_max_d {
    double _Min;
    double _Max;
};

struct _Min_max_p {
    void* _Min;
    void* _Max;
};
} // extern "C"

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_MINMAX_HPP
