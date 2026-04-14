// xfilesystem_abi.h internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XFILESYSTEM_ABI_H
#define _XFILESYSTEM_ABI_H
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#include <cstdint>
#include <type_traits>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C" {
inline constexpr size_t __std_fs_max_path      = 260; // #define MAX_PATH          260
inline constexpr size_t __std_fs_temp_path_max = __std_fs_max_path + 1;

enum class __std_win_error : unsigned long {
    _Success                   = 0, // #define ERROR_SUCCESS                    0L
    _Invalid_function          = 1, // #define ERROR_INVALID_FUNCTION           1L
    _File_not_found            = 2, // #define ERROR_FILE_NOT_FOUND             2L
    _Path_not_found            = 3, // #define ERROR_PATH_NOT_FOUND             3L
    _Access_denied             = 5, // #define ERROR_ACCESS_DENIED              5L
    _Not_enough_memory         = 8, // #define ERROR_NOT_ENOUGH_MEMORY          8L
    _No_more_files             = 18, // #define ERROR_NO_MORE_FILES              18L
    _Sharing_violation         = 32, // #define ERROR_SHARING_VIOLATION          32L
    _Not_supported             = 50, // #define ERROR_NOT_SUPPORTED              50L
    _Error_bad_netpath         = 53, // #define ERROR_BAD_NETPATH                53L
    _Error_netname_deleted     = 64, // #define ERROR_NETNAME_DELETED            64L
    _File_exists               = 80, // #define ERROR_FILE_EXISTS                80L
    _Invalid_parameter         = 87, // #define ERROR_INVALID_PARAMETER          87L
    _Insufficient_buffer       = 122, // #define ERROR_INSUFFICIENT_BUFFER        122L
    _Invalid_name              = 123, // #define ERROR_INVALID_NAME               123L
    _Directory_not_empty       = 145, // #define ERROR_DIR_NOT_EMPTY              145L
    _Already_exists            = 183, // #define ERROR_ALREADY_EXISTS             183L
    _Filename_exceeds_range    = 206, // #define ERROR_FILENAME_EXCED_RANGE       206L
    _Directory_name_is_invalid = 267, // #define ERROR_DIRECTORY                  267L
    _Reparse_tag_invalid       = 4393L, // #define ERROR_REPARSE_TAG_INVALID        4393L
    _Max                       = ~0UL // sentinel not used by Win32
};

#pragma warning(push)
#pragma warning(disable : 4061) // enumerator not explicitly handled by switch label
_NODISCARD inline bool __std_is_file_not_found(const __std_win_error _Error) noexcept {
    switch (_Error) {
    case __std_win_error::_File_not_found:
    case __std_win_error::_Path_not_found:
    case __std_win_error::_Error_bad_netpath:
    case __std_win_error::_Invalid_name:
    case __std_win_error::_Directory_name_is_invalid: // Windows 11 24H2
    case __std_win_error::_Error_netname_deleted: // Windows 11 24H2
        return true;
    default:
        return false;
    }
}
#pragma warning(pop)

enum class __std_fs_dir_handle : intptr_t { _Invalid = -1 };

enum class __std_fs_file_attr : unsigned long {
    _Readonly      = 0x00000001, // #define FILE_ATTRIBUTE_READONLY             0x00000001
    _Hidden        = 0x00000002, // #define FILE_ATTRIBUTE_HIDDEN               0x00000002
    _System        = 0x00000004, // #define FILE_ATTRIBUTE_SYSTEM               0x00000004
    _Directory     = 0x00000010, // #define FILE_ATTRIBUTE_DIRECTORY            0x00000010
    _Archive       = 0x00000020, // #define FILE_ATTRIBUTE_ARCHIVE              0x00000020
    _Device        = 0x00000040, // #define FILE_ATTRIBUTE_DEVICE               0x00000040
    _Normal        = 0x00000080, // #define FILE_ATTRIBUTE_NORMAL               0x00000080
    _Temporary     = 0x00000100, // #define FILE_ATTRIBUTE_TEMPORARY            0x00000100
    _Sparse_file   = 0x00000200, // #define FILE_ATTRIBUTE_SPARSE_FILE          0x00000200
    _Reparse_point = 0x00000400, // #define FILE_ATTRIBUTE_REPARSE_POINT        0x00000400

    _Invalid = 0xFFFFFFFF, // #define INVALID_FILE_ATTRIBUTES ((DWORD)-1)
};
} // extern "C"

_EXTERN_CXX_WORKAROUND
_BITMASK_OPS(_EMPTY_ARGUMENT, __std_fs_file_attr)
_END_EXTERN_CXX_WORKAROUND

extern "C" {
enum class __std_fs_reparse_tag : unsigned long {
    _None        = 0,
    _Mount_point = (0xA0000003L), // #define IO_REPARSE_TAG_MOUNT_POINT              (0xA0000003L)
    _Symlink     = (0xA000000CL), // #define IO_REPARSE_TAG_SYMLINK                  (0xA000000CL)
};

struct __std_fs_filetime { // typedef struct _FILETIME {
    unsigned long _Low; //     DWORD dwLowDateTime;
    unsigned long _High; //     DWORD dwHighDateTime;
}; // } FILETIME, *PFILETIME, *LPFILETIME;

struct __std_fs_find_data { // typedef struct _WIN32_FIND_DATAW {
    __std_fs_file_attr _Attributes; //     DWORD dwFileAttributes;
    __std_fs_filetime _Creation_time; //     FILETIME ftCreationTime;
    __std_fs_filetime _Last_access_time; //     FILETIME ftLastAccessTime;
    __std_fs_filetime _Last_write_time; //     FILETIME ftLastWriteTime;
    unsigned long _File_size_high; //     DWORD nFileSizeHigh;
    unsigned long _File_size_low; //     DWORD nFileSizeLow;

    // MSDN: dwReserved0 specifies the reparse point tag if
    // MSDN:  (dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT) != 0

    __std_fs_reparse_tag _Reparse_point_tag; //     DWORD dwReserved0;
    unsigned long _Reserved1; //     DWORD dwReserved1;
    wchar_t _File_name[__std_fs_max_path]; //     _Field_z_ WCHAR  cFileName[ MAX_PATH ];
    wchar_t _Short_file_name[14]; //     _Field_z_ WCHAR  cAlternateFileName[ 14 ];
}; // } WIN32_FIND_DATAW, ... ;

enum class __std_fs_stats_flags : unsigned long {
    _None = 0,

    _Follow_symlinks = 0x01, // resolve symlink
    _Attributes      = 0x02, // read/has attributes
    _Reparse_tag     = 0x04, // read/has reparse_tag; may not be combined with _Follow_symlinks
    _File_size       = 0x08, // read/has file size
    _Link_count      = 0x10, // read/has link count
    _Last_write_time = 0x20, // read/has last write time

    _All_data = _Attributes | _Reparse_tag | _File_size | _Link_count | _Last_write_time
};
} // extern "C"

_EXTERN_CXX_WORKAROUND
_BITMASK_OPS(_EMPTY_ARGUMENT, __std_fs_stats_flags)
_END_EXTERN_CXX_WORKAROUND

extern "C" {
struct __std_fs_stats {
    long long _Last_write_time;
    unsigned long long _File_size;
    __std_fs_file_attr _Attributes;
    __std_fs_reparse_tag _Reparse_point_tag;
    unsigned long _Link_count;
    __std_fs_stats_flags _Available; // which fields are available

    _NODISCARD __std_fs_file_attr _Symlink_hint_attributes() const noexcept {
        if (_STD _Bitmask_includes_any(_Available, __std_fs_stats_flags::_Attributes)) {
            return _Attributes;
        }

        return __std_fs_file_attr::_Invalid;
    }
};

struct __std_fs_reparse_data_buffer { // typedef struct _REPARSE_DATA_BUFFER
    unsigned long _Reparse_tag;
    unsigned short _Reparse_data_length;
    unsigned short _Reserved;
    union {
        struct {
            unsigned short _Substitute_name_offset;
            unsigned short _Substitute_name_length;
            unsigned short _Print_name_offset;
            unsigned short _Print_name_length;
            unsigned long _Flags;
            wchar_t _Path_buffer[1];
        } _Symbolic_link_reparse_buffer;
        struct {
            unsigned short _Substitute_name_offset;
            unsigned short _Substitute_name_length;
            unsigned short _Print_name_offset;
            unsigned short _Print_name_length;
            wchar_t _Path_buffer[1];
        } _Mount_point_reparse_buffer;
        struct {
            unsigned char _Data_buffer[1];
        } _Generic_reparse_buffer;
    };
};

struct __std_ulong_and_error {
    unsigned long _Size;
    __std_win_error _Error;
};

enum class __std_fs_volume_name_kind : unsigned long {
    _Dos  = 0, // #define VOLUME_NAME_DOS  0x0
    _Guid = 1, // #define VOLUME_NAME_GUID 0x1
    _Nt   = 2, // #define VOLUME_NAME_NT   0x2
    _None = 4 // #define VOLUME_NAME_NONE 0x4
};

enum class __std_access_rights : unsigned long {
    _Delete                = 0x00010000, // #define DELETE                           (0x00010000L)
    _File_read_attributes  = 0x0080, // #define FILE_READ_ATTRIBUTES      ( 0x0080 )
    _File_write_attributes = 0x0100, // #define FILE_WRITE_ATTRIBUTES     ( 0x0100 )

    // #define READ_CONTROL          (0x00020000L)
    // #define STANDARD_RIGHTS_WRITE (READ_CONTROL)
    // #define FILE_WRITE_DATA       (0x0002)
    // #define FILE_WRITE_ATTRIBUTES (0x0100)
    // #define FILE_WRITE_EA         (0x0010)
    // #define FILE_APPEND_DATA      (0x0004)
    // #define SYNCHRONIZE           (0x00100000L)
    // #define FILE_GENERIC_WRITE    (STANDARD_RIGHTS_WRITE | FILE_WRITE_DATA | FILE_WRITE_ATTRIBUTES
    //                                   | FILE_WRITE_EA | FILE_APPEND_DATA | SYNCHRONIZE)
    _File_generic_write = 0x00120116,
};
} // extern "C"

_EXTERN_CXX_WORKAROUND
_BITMASK_OPS(_EMPTY_ARGUMENT, __std_access_rights)
_END_EXTERN_CXX_WORKAROUND

extern "C" {
enum class __std_fs_file_flags : unsigned long {
    _None               = 0,
    _Backup_semantics   = 0x02000000, // #define FILE_FLAG_BACKUP_SEMANTICS      0x02000000
    _Open_reparse_point = 0x00200000, // #define FILE_FLAG_OPEN_REPARSE_POINT    0x00200000
};
} // extern "C"

_EXTERN_CXX_WORKAROUND
_BITMASK_OPS(_EMPTY_ARGUMENT, __std_fs_file_flags)
_END_EXTERN_CXX_WORKAROUND

extern "C" {
enum class __std_fs_file_handle : intptr_t { _Invalid = -1 };

enum class __std_code_page : unsigned int { _Acp = 0, _Utf8 = 65001 };

struct __std_fs_convert_result {
    int _Len;
    __std_win_error _Err;
};

enum class __std_fs_copy_options {
    _None = 0x0,

    _Existing_mask      = 0xF,
    _Skip_existing      = 0x1,
    _Overwrite_existing = 0x2,
    _Update_existing    = 0x4,
};
} // extern "C"

_EXTERN_CXX_WORKAROUND
_BITMASK_OPS(_EMPTY_ARGUMENT, __std_fs_copy_options)
_END_EXTERN_CXX_WORKAROUND

extern "C" {
_NODISCARD __std_ulong_and_error __stdcall __std_fs_get_full_path_name(_In_z_ const wchar_t* _Source,
    _In_ unsigned long _Target_size, _Out_writes_z_(_Target_size) wchar_t* _Target) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_open_handle(_Out_ __std_fs_file_handle* _Handle,
    _In_z_ const wchar_t* _File_name, _In_ __std_access_rights _Desired_access,
    _In_ __std_fs_file_flags _Flags) noexcept;

void __stdcall __std_fs_close_handle(__std_fs_file_handle _Handle) noexcept;

_NODISCARD _Success_(return == __std_win_error::_Success) __std_win_error
    __stdcall __std_fs_get_file_attributes_by_handle(
        _In_ __std_fs_file_handle _Handle, _Out_ unsigned long* _File_attributes) noexcept;

_NODISCARD __std_ulong_and_error __stdcall __std_fs_get_final_path_name_by_handle(_In_ __std_fs_file_handle _Handle,
    _Out_writes_z_(_Target_size) wchar_t* _Target, _In_ unsigned long _Target_size,
    _In_ __std_fs_volume_name_kind _Flags) noexcept;

struct __std_fs_copy_file_result {
    bool _Copied;
    __std_win_error _Error;
};

_NODISCARD __std_fs_copy_file_result __stdcall __std_fs_copy_file(
    _In_z_ const wchar_t* _Source, _In_z_ const wchar_t* _Target, _In_ __std_fs_copy_options _Options) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_directory_iterator_open(_In_z_ const wchar_t* _Path_spec,
    _Inout_ __std_fs_dir_handle* _Handle, _Out_ __std_fs_find_data* _Results) noexcept;

void __stdcall __std_fs_directory_iterator_close(_In_ __std_fs_dir_handle _Handle) noexcept;

_NODISCARD _Success_(return == __std_win_error::_Success) __std_win_error
    __stdcall __std_fs_get_stats(_In_z_ const wchar_t* _Path, __std_fs_stats* _Stats, _In_ __std_fs_stats_flags _Flags,
        _In_ __std_fs_file_attr _Symlink_attribute_hint = __std_fs_file_attr::_Invalid) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_directory_iterator_advance(
    _In_ __std_fs_dir_handle _Handle, _Out_ __std_fs_find_data* _Results) noexcept;

_NODISCARD __std_code_page __stdcall __std_fs_code_page() noexcept;

_NODISCARD __std_fs_convert_result __stdcall __std_fs_convert_narrow_to_wide(_In_ __std_code_page _Code_page,
    _In_reads_(_Input_len) const char* _Input_str, _In_ int _Input_len,
    _Out_writes_opt_(_Output_len) wchar_t* _Output_str, _In_ int _Output_len) noexcept;

_NODISCARD __std_fs_convert_result __stdcall __std_fs_convert_wide_to_narrow(_In_ __std_code_page _Code_page,
    _In_reads_(_Input_len) const wchar_t* _Input_str, _In_ int _Input_len,
    _Out_writes_opt_(_Output_len) char* _Output_str, _In_ int _Output_len) noexcept;

_NODISCARD __std_fs_convert_result __stdcall __std_fs_convert_wide_to_narrow_replace_chars(
    _In_ __std_code_page _Code_page, _In_reads_(_Input_len) const wchar_t* _Input_str, _In_ int _Input_len,
    _Out_writes_opt_(_Output_len) char* _Output_str, _In_ int _Output_len) noexcept;

struct __std_fs_equivalent_result {
    bool _Equivalent;
    __std_win_error _Error;
};

_NODISCARD __std_fs_equivalent_result __stdcall __std_fs_equivalent(
    _In_z_ const wchar_t* _Path1, _In_z_ const wchar_t* _Path2) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_set_last_write_time(
    _In_ long long _Last_write_filetime, _In_z_ const wchar_t* _Path) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_change_permissions(
    _In_z_ const wchar_t* _Path, _In_ bool _Follow_symlinks, _In_ bool _Readonly) noexcept;

_NODISCARD _Success_(return._Error == __std_win_error::_Success) __std_ulong_and_error
    __stdcall __std_fs_get_temp_path(_Out_writes_z_(__std_fs_temp_path_max) wchar_t* _Target) noexcept;

_NODISCARD _Success_(return._Error == __std_win_error::_Success) __std_ulong_and_error
    __stdcall __std_fs_get_current_path(
        _In_ unsigned long _Target_size, _Out_writes_z_(_Target_size) wchar_t* _Target) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_set_current_path(_In_z_ const wchar_t* _Target) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_create_directory_symbolic_link(
    _In_z_ const wchar_t* _Symlink_file_name, _In_z_ const wchar_t* _Target_file_name) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_create_hard_link(
    _In_z_ const wchar_t* _File_name, _In_z_ const wchar_t* _Existing_file_name) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_create_symbolic_link(
    _In_z_ const wchar_t* _Symlink_file_name, _In_z_ const wchar_t* _Target_file_name) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_read_reparse_data_buffer(_In_ __std_fs_file_handle _Handle,
    _Out_writes_bytes_(_Buffer_size) void* _Buffer, _In_ unsigned long _Buffer_size) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_write_reparse_data_buffer(
    _In_ __std_fs_file_handle _Handle, _In_ const __std_fs_reparse_data_buffer* _Buffer) noexcept;

_NODISCARD bool __stdcall __std_fs_is_junction_from_reparse_data_buffer(
    _In_ const __std_fs_reparse_data_buffer* _Buffer) noexcept;

_NODISCARD _Success_(return == __std_win_error::_Success) __std_win_error
    __stdcall __std_fs_read_name_from_reparse_data_buffer(
        _In_ __std_fs_reparse_data_buffer* _Handle, _Out_ wchar_t** _Offset, _Out_ unsigned short* _Length) noexcept;

struct __std_fs_create_directory_result {
    bool _Created;
    __std_win_error _Error;
};

_NODISCARD __std_fs_create_directory_result __stdcall __std_fs_create_directory(
    _In_z_ const wchar_t* _New_directory) noexcept;

struct __std_fs_remove_result {
    bool _Removed;
    __std_win_error _Error;
};

_NODISCARD __std_fs_remove_result __stdcall __std_fs_remove(_In_z_ const wchar_t* _Target) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_rename(
    _In_z_ const wchar_t* _Source, _In_z_ const wchar_t* _Target) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_resize_file(_In_z_ const wchar_t* _Target, uintmax_t _New_size) noexcept;

_NODISCARD __std_win_error __stdcall __std_fs_space(_In_z_ const wchar_t* _Target, _Out_ uintmax_t* _Available,
    _Out_ uintmax_t* _Total_bytes, _Out_ uintmax_t* _Free_bytes) noexcept;
} // extern "C"

_STD_BEGIN
struct _Fs_file {
    __std_fs_file_handle _Raw;

    explicit _Fs_file(void* const _Handle) : _Raw{reinterpret_cast<intptr_t>(_Handle)} {}

    _Fs_file(const wchar_t* const _File_name, const __std_access_rights _Desired_access,
        const __std_fs_file_flags _Flags, __std_win_error* const _Err) {
        *_Err = __std_fs_open_handle(&_Raw, _File_name, _Desired_access, _Flags);
    }

    _Fs_file(const _Fs_file&)            = delete;
    _Fs_file& operator=(const _Fs_file&) = delete;

    ~_Fs_file() {
        __std_fs_close_handle(_Raw);
    }

    _NODISCARD void* _Get() const {
        return reinterpret_cast<void*>(_Raw);
    }
};

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XFILESYSTEM_ABI_H
