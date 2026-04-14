//
// setjmp.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// The C Standard Library <setjmp.h> header.
//
#pragma once
#define _INC_SETJMP

#include <vcruntime.h>

#ifdef _M_CEE
    // The reason why simple setjmp won't work here is that there may
    // be case when CLR stubs are on the stack e.g. function call just
    // after jitting, and not unwinding CLR will result in bad state of
    // CLR which then can AV or do something very bad.
    #include <setjmpex.h>
#endif

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

_CRT_BEGIN_C_HEADER



// Definitions specific to particular setjmp implementations.
#if defined _M_IX86

    #define _JBLEN  16
    #define _JBTYPE int

    typedef struct __JUMP_BUFFER
    {
        unsigned long Ebp;
        unsigned long Ebx;
        unsigned long Edi;
        unsigned long Esi;
        unsigned long Esp;
        unsigned long Eip;
        unsigned long Registration;
        unsigned long TryLevel;
        unsigned long Cookie;
        unsigned long UnwindFunc;
        unsigned long UnwindData[6];
    } _JUMP_BUFFER;

#elif defined _M_X64

    typedef struct _VCRT_ALIGN(16) _SETJMP_FLOAT128
    {
        unsigned __int64 Part[2];
    } SETJMP_FLOAT128;

    #define _JBLEN  16
    typedef SETJMP_FLOAT128 _JBTYPE;

    typedef struct _JUMP_BUFFER
    {
        unsigned __int64 Frame;
        unsigned __int64 Rbx;
        unsigned __int64 Rsp;
        unsigned __int64 Rbp;
        unsigned __int64 Rsi;
        unsigned __int64 Rdi;
        unsigned __int64 R12;
        unsigned __int64 R13;
        unsigned __int64 R14;
        unsigned __int64 R15;
        unsigned __int64 Rip;
        unsigned long MxCsr;
        unsigned short FpCsr;
        unsigned short Spare;

        SETJMP_FLOAT128 Xmm6;
        SETJMP_FLOAT128 Xmm7;
        SETJMP_FLOAT128 Xmm8;
        SETJMP_FLOAT128 Xmm9;
        SETJMP_FLOAT128 Xmm10;
        SETJMP_FLOAT128 Xmm11;
        SETJMP_FLOAT128 Xmm12;
        SETJMP_FLOAT128 Xmm13;
        SETJMP_FLOAT128 Xmm14;
        SETJMP_FLOAT128 Xmm15;
    } _JUMP_BUFFER;

#elif defined _M_ARM

    #define _JBLEN  28
    #define _JBTYPE int

    typedef struct _JUMP_BUFFER
    {
        unsigned long Frame;

        unsigned long R4;
        unsigned long R5;
        unsigned long R6;
        unsigned long R7;
        unsigned long R8;
        unsigned long R9;
        unsigned long R10;
        unsigned long R11;

        unsigned long Sp;
        unsigned long Pc;
        unsigned long Fpscr;
        unsigned long long D[8]; // D8-D15 VFP/NEON regs
    } _JUMP_BUFFER;

#elif defined _M_ARM64

    #define _JBLEN  24
    #define _JBTYPE unsigned __int64

    typedef struct _JUMP_BUFFER {
        unsigned __int64 Frame;
        unsigned __int64 Reserved;
        unsigned __int64 X19;   // x19 -- x28: callee saved registers
        unsigned __int64 X20;
        unsigned __int64 X21;
        unsigned __int64 X22;
        unsigned __int64 X23;
        unsigned __int64 X24;
        unsigned __int64 X25;
        unsigned __int64 X26;
        unsigned __int64 X27;
        unsigned __int64 X28;
        unsigned __int64 Fp;    // x29 frame pointer
        unsigned __int64 Lr;    // x30 link register
        unsigned __int64 Sp;    // x31 stack pointer
        unsigned __int32 Fpcr;  // fp control register
        unsigned __int32 Fpsr;  // fp status register

        double D[8]; // D8-D15 FP regs
    } _JUMP_BUFFER;



#endif



// Define the buffer type for holding the state information
#ifndef _JMP_BUF_DEFINED
    #define _JMP_BUF_DEFINED
    typedef _JBTYPE jmp_buf[_JBLEN];
#endif



#ifndef _INC_SETJMPEX
    #define setjmp _setjmp
#endif



// Function prototypes
int __cdecl setjmp(
    _Out_ jmp_buf _Buf
    );

#ifdef __cplusplus
    __declspec(noreturn) void __cdecl longjmp(
        _In_reads_(_JBLEN) jmp_buf _Buf,
        _In_ int     _Value
        ) noexcept(false);
#else
    __declspec(noreturn) void __cdecl longjmp(
        _In_reads_(_JBLEN) jmp_buf _Buf,
        _In_ int     _Value
        );
#endif


_CRT_END_C_HEADER

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
