//
// excpt.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// The declarations of the compiler-dependent intrinsics, support functions, and
// keywords which implement the structured exception handling extensions.
//
#pragma once
#define _INC_EXCPT

#include <vcruntime.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

_CRT_BEGIN_C_HEADER



// Exception disposition return values
typedef enum _EXCEPTION_DISPOSITION
{
    ExceptionContinueExecution,
    ExceptionContinueSearch,
    ExceptionNestedException,
    ExceptionCollidedUnwind
} EXCEPTION_DISPOSITION;



// SEH handler
#if defined(_M_IX86) && !defined(_CHPE_X86_ARM64_EH_)

    struct _EXCEPTION_RECORD;
    struct _CONTEXT;

    EXCEPTION_DISPOSITION __cdecl _except_handler(
        _In_ struct _EXCEPTION_RECORD* _ExceptionRecord,
        _In_ void*                     _EstablisherFrame,
        _Inout_ struct _CONTEXT*       _ContextRecord,
        _Inout_ void*                  _DispatcherContext
        );

#elif defined _M_X64 || defined _M_ARM || defined _M_ARM64 || defined _CHPE_X86_ARM64_EH_
    #ifndef _M_CEE_PURE

        struct _EXCEPTION_RECORD;
        struct _CONTEXT;
        struct _DISPATCHER_CONTEXT;

        _VCRTIMP EXCEPTION_DISPOSITION __cdecl __C_specific_handler(
            _In_    struct _EXCEPTION_RECORD*   ExceptionRecord,
            _In_    void*                       EstablisherFrame,
            _Inout_ struct _CONTEXT*            ContextRecord,
            _Inout_ struct _DISPATCHER_CONTEXT* DispatcherContext
            );

    #endif
#endif



// SEH intrinsics
#define GetExceptionCode            _exception_code
#define exception_code              _exception_code
#define GetExceptionInformation()   ((struct _EXCEPTION_POINTERS *)_exception_info())
#define exception_info()            ((struct _EXCEPTION_POINTERS *)_exception_info())
#define AbnormalTermination         _abnormal_termination
#define abnormal_termination        _abnormal_termination

unsigned long __cdecl _exception_code(void);
void *        __cdecl _exception_info(void);
int           __cdecl _abnormal_termination(void);



// Defined values for the exception filter expression
#define EXCEPTION_EXECUTE_HANDLER      1
#define EXCEPTION_CONTINUE_SEARCH      0
#define EXCEPTION_CONTINUE_EXECUTION (-1)



_CRT_END_C_HEADER

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
