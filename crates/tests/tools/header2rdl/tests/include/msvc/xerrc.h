// xerrc.h internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XERRC_H
#define _XERRC_H
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
_EXPORT_STD enum class errc { // names for generic error codes
    address_family_not_supported                       = 102, // EAFNOSUPPORT
    address_in_use                                     = 100, // EADDRINUSE
    address_not_available                              = 101, // EADDRNOTAVAIL
    already_connected                                  = 113, // EISCONN
    argument_list_too_long                             = 7, // E2BIG
    argument_out_of_domain                             = 33, // EDOM
    bad_address                                        = 14, // EFAULT
    bad_file_descriptor                                = 9, // EBADF
    bad_message                                        = 104, // EBADMSG
    broken_pipe                                        = 32, // EPIPE
    connection_aborted                                 = 106, // ECONNABORTED
    connection_already_in_progress                     = 103, // EALREADY
    connection_refused                                 = 107, // ECONNREFUSED
    connection_reset                                   = 108, // ECONNRESET
    cross_device_link                                  = 18, // EXDEV
    destination_address_required                       = 109, // EDESTADDRREQ
    device_or_resource_busy                            = 16, // EBUSY
    directory_not_empty                                = 41, // ENOTEMPTY
    executable_format_error                            = 8, // ENOEXEC
    file_exists                                        = 17, // EEXIST
    file_too_large                                     = 27, // EFBIG
    filename_too_long                                  = 38, // ENAMETOOLONG
    function_not_supported                             = 40, // ENOSYS
    host_unreachable                                   = 110, // EHOSTUNREACH
    identifier_removed                                 = 111, // EIDRM
    illegal_byte_sequence                              = 42, // EILSEQ
    inappropriate_io_control_operation                 = 25, // ENOTTY
    interrupted                                        = 4, // EINTR
    invalid_argument                                   = 22, // EINVAL
    invalid_seek                                       = 29, // ESPIPE
    io_error                                           = 5, // EIO
    is_a_directory                                     = 21, // EISDIR
    message_size                                       = 115, // EMSGSIZE
    network_down                                       = 116, // ENETDOWN
    network_reset                                      = 117, // ENETRESET
    network_unreachable                                = 118, // ENETUNREACH
    no_buffer_space                                    = 119, // ENOBUFS
    no_child_process                                   = 10, // ECHILD
    no_link                                            = 121, // ENOLINK
    no_lock_available                                  = 39, // ENOLCK
    no_message_available _CXX23_DEPRECATE_UNIX_STREAMS = 120, // ENODATA
    no_message                                         = 122, // ENOMSG
    no_protocol_option                                 = 123, // ENOPROTOOPT
    no_space_on_device                                 = 28, // ENOSPC
    no_stream_resources _CXX23_DEPRECATE_UNIX_STREAMS  = 124, // ENOSR
    no_such_device_or_address                          = 6, // ENXIO
    no_such_device                                     = 19, // ENODEV
    no_such_file_or_directory                          = 2, // ENOENT
    no_such_process                                    = 3, // ESRCH
    not_a_directory                                    = 20, // ENOTDIR
    not_a_socket                                       = 128, // ENOTSOCK
    not_a_stream _CXX23_DEPRECATE_UNIX_STREAMS         = 125, // ENOSTR
    not_connected                                      = 126, // ENOTCONN
    not_enough_memory                                  = 12, // ENOMEM
    not_supported                                      = 129, // ENOTSUP
    operation_canceled                                 = 105, // ECANCELED
    operation_in_progress                              = 112, // EINPROGRESS
    operation_not_permitted                            = 1, // EPERM
    operation_not_supported                            = 130, // EOPNOTSUPP
    operation_would_block                              = 140, // EWOULDBLOCK
    owner_dead                                         = 133, // EOWNERDEAD
    permission_denied                                  = 13, // EACCES
    protocol_error                                     = 134, // EPROTO
    protocol_not_supported                             = 135, // EPROTONOSUPPORT
    read_only_file_system                              = 30, // EROFS
    resource_deadlock_would_occur                      = 36, // EDEADLK
    resource_unavailable_try_again                     = 11, // EAGAIN
    result_out_of_range                                = 34, // ERANGE
    state_not_recoverable                              = 127, // ENOTRECOVERABLE
    stream_timeout _CXX23_DEPRECATE_UNIX_STREAMS       = 137, // ETIME
    text_file_busy                                     = 139, // ETXTBSY
    timed_out                                          = 138, // ETIMEDOUT
    too_many_files_open_in_system                      = 23, // ENFILE
    too_many_files_open                                = 24, // EMFILE
    too_many_links                                     = 31, // EMLINK
    too_many_symbolic_link_levels                      = 114, // ELOOP
    value_too_large                                    = 132, // EOVERFLOW
    wrong_protocol_type                                = 136 // EPROTOTYPE
};

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XERRC_H
