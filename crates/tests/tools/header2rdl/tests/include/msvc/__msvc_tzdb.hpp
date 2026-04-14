// __msvc_tzdb.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_TZDB_HPP
#define __MSVC_TZDB_HPP
#include <yvals.h>
#if _STL_COMPILER_PREPROCESSOR
#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <xutility>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C" {

using __std_tzdb_epoch_milli = double;

struct __std_tzdb_leap_info {
    uint16_t _Year;
    uint16_t _Month;
    uint16_t _Day;
    uint16_t _Hour;
    uint16_t _Negative;
    uint16_t _Reserved;
};

enum class __std_tzdb_error {
    _Success   = 0,
    _Win_error = 1,
    _Icu_error = 2,
};

struct __std_tzdb_time_zones_info {
    __std_tzdb_error _Err;
    // timezone data version currently being used
    const char* _Version;
    size_t _Num_time_zones;
    // ordered list of null-terminated time_zone/time_zone_link names
    const char** _Names;
    // contains corresponding entry for every name, if:
    //    (_Links[i] == nullptr) then _Names[i] is a time_zone
    //    (_Links[i] != nullptr) then _Names[i] is a time_zone_link to time_zone with name _Links[i]
    const char** _Links;
};

struct __std_tzdb_current_zone_info {
    __std_tzdb_error _Err;
    const char* _Tz_name;
};

struct __std_tzdb_sys_info {
    __std_tzdb_error _Err;
    __std_tzdb_epoch_milli _Begin;
    __std_tzdb_epoch_milli _End;
    int32_t _Offset;
    int32_t _Save;
    const char* _Abbrev;
};

enum class __std_tzdb_sys_info_type : char {
    // TRANSITION, ABI: In order to be compatible with existing object files which do not know about
    // `__std_tzdb_sys_info_type`, the type is passed in the after-end byte of a string passed with its length to
    // `__std_tzdb_get_sys_info`. Since older object files always pass the `.c_str()` of a `std::string`
    // to that function, the after-end byte will always be '\0'.
    _Full = '\0',
    _Offset_only,
    _Offset_and_range,
};

_NODISCARD __std_tzdb_time_zones_info* __stdcall __std_tzdb_get_time_zones() noexcept;
void __stdcall __std_tzdb_delete_time_zones(__std_tzdb_time_zones_info* _Info) noexcept;

_NODISCARD __std_tzdb_current_zone_info* __stdcall __std_tzdb_get_current_zone() noexcept;
void __stdcall __std_tzdb_delete_current_zone(__std_tzdb_current_zone_info* _Info) noexcept;

_NODISCARD __std_tzdb_sys_info* __stdcall __std_tzdb_get_sys_info(
    const char* _Tz, size_t _Tz_len, __std_tzdb_epoch_milli _Local) noexcept;
void __stdcall __std_tzdb_delete_sys_info(__std_tzdb_sys_info* _Info) noexcept;

_NODISCARD __std_tzdb_leap_info* __stdcall __std_tzdb_get_leap_seconds(
    size_t _Prev_ls_size, size_t* _Current_ls_size) noexcept;
void __stdcall __std_tzdb_delete_leap_seconds(__std_tzdb_leap_info* _Info) noexcept;

_NODISCARD void* __stdcall __std_calloc_crt(size_t _Count, size_t _Size) noexcept;
void __stdcall __std_free_crt(void* _Ptr) noexcept;

} // extern "C"

_STD_BEGIN

template <class _Ty>
struct _Tzdb_deleter;

template <>
struct _Tzdb_deleter<__std_tzdb_time_zones_info> {
    _STATIC_CALL_OPERATOR void operator()(__std_tzdb_time_zones_info* _Info) _CONST_CALL_OPERATOR noexcept {
        __std_tzdb_delete_time_zones(_Info);
    }
};

template <>
struct _Tzdb_deleter<__std_tzdb_current_zone_info> {
    _STATIC_CALL_OPERATOR void operator()(__std_tzdb_current_zone_info* _Info) _CONST_CALL_OPERATOR noexcept {
        __std_tzdb_delete_current_zone(_Info);
    }
};

template <>
struct _Tzdb_deleter<__std_tzdb_sys_info> {
    _STATIC_CALL_OPERATOR void operator()(__std_tzdb_sys_info* _Info) _CONST_CALL_OPERATOR noexcept {
        __std_tzdb_delete_sys_info(_Info);
    }
};

template <>
struct _Tzdb_deleter<__std_tzdb_leap_info[]> {
    _STATIC_CALL_OPERATOR void operator()(__std_tzdb_leap_info* _Info) _CONST_CALL_OPERATOR noexcept {
        __std_tzdb_delete_leap_seconds(_Info);
    }
};

template <class _Ty>
class _Crt_allocator {
public:
    using value_type                             = _Ty;
    using propagate_on_container_move_assignment = true_type;
    using is_always_equal                        = true_type;

    constexpr _Crt_allocator() noexcept = default;

    constexpr _Crt_allocator(const _Crt_allocator&) noexcept = default;
    template <class _Other>
    constexpr _Crt_allocator(const _Crt_allocator<_Other>&) noexcept {}

    _NODISCARD __declspec(allocator) _Ty* allocate(_CRT_GUARDOVERFLOW const size_t _Count) {
        const auto _Ptr = __std_calloc_crt(_Count, sizeof(_Ty));
        if (!_Ptr) {
            _Xbad_alloc();
        }
        return static_cast<_Ty*>(_Ptr);
    }

    void deallocate(_Ty* const _Ptr, size_t) noexcept {
        __std_free_crt(_Ptr);
    }

    template <class _Other>
    _NODISCARD bool operator==(const _Crt_allocator<_Other>&) const noexcept {
        return true;
    }
};

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_TZDB_HPP
