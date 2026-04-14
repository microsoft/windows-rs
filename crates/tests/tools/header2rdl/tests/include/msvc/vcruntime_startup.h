//
// vcruntime_startup.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// Declarations of the VCRuntime startup functionality
//
#pragma once

#include <vcruntime.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

_CRT_BEGIN_C_HEADER



typedef enum _crt_argv_mode
{
    _crt_argv_no_arguments,
    _crt_argv_unexpanded_arguments,
    _crt_argv_expanded_arguments,
} _crt_argv_mode;

typedef enum _crt_exit_return_mode
{
    _crt_exit_terminate_process,
    _crt_exit_return_to_caller
} _crt_exit_return_mode;

typedef enum _crt_exit_cleanup_mode
{
    _crt_exit_full_cleanup,
    _crt_exit_quick_cleanup,
    _crt_exit_no_cleanup
} _crt_exit_cleanup_mode;

extern _crt_exit_return_mode __current_exit_return_mode;



__vcrt_bool __cdecl __vcrt_initialize(void);
__vcrt_bool __cdecl __vcrt_uninitialize(_In_ __vcrt_bool _Terminating);
__vcrt_bool __cdecl __vcrt_uninitialize_critical(void);
__vcrt_bool __cdecl __vcrt_thread_attach(void);
__vcrt_bool __cdecl __vcrt_thread_detach(void);

int __cdecl __isa_available_init(void);
_crt_argv_mode __CRTDECL _get_startup_argv_mode(void);



_CRT_END_C_HEADER

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
