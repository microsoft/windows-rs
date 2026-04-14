/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    vdmctxt.h

Abstract:

    This include file defines the vdm context structure so we can inspect it from 64-bit

Author:

--*/

/* XLATOFF */

#ifndef _VDMCTXT_H_
#define _VDMCTXT_H_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <pshpack4.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#endif

#pragma warning(disable:4214) // bit field types other than int
#pragma warning(disable:4121) // structure is sensitive to alignment

//
// The following flags control the contents of the CONTEXT structure.
//

#define VDMCONTEXT_i386    0x00010000    // this assumes that i386 and
#define VDMCONTEXT_i486    0x00010000    // i486 have identical context records

#define VDMCONTEXT_CONTROL         (VDMCONTEXT_i386 | 0x00000001L) // SS:SP, CS:IP, FLAGS, BP
#define VDMCONTEXT_INTEGER         (VDMCONTEXT_i386 | 0x00000002L) // AX, BX, CX, DX, SI, DI
#define VDMCONTEXT_SEGMENTS        (VDMCONTEXT_i386 | 0x00000004L) // DS, ES, FS, GS
#define VDMCONTEXT_FLOATING_POINT  (VDMCONTEXT_i386 | 0x00000008L) // 387 state
#define VDMCONTEXT_DEBUG_REGISTERS (VDMCONTEXT_i386 | 0x00000010L) // DB 0-3,6,7
#define VDMCONTEXT_EXTENDED_REGISTERS  (VDMCONTEXT_i386 | 0x00000020L) // cpu specific extensions

#define VDMCONTEXT_FULL (VDMCONTEXT_CONTROL | VDMCONTEXT_INTEGER |\
                      VDMCONTEXT_SEGMENTS)


#ifdef _X86_

#define VDM_KGDT_R3_CODE KGDT_R3_CODE

// On x86 machines, just copy the definition of the CONTEXT and LDT_ENTRY
// structures.
typedef struct _CONTEXT VDMCONTEXT;
typedef struct _LDT_ENTRY VDMLDT_ENTRY;

#else // _X86_

#define VDM_KGDT_R3_CODE 24

//
//  Define the size of the 80387 save area, which is in the context frame.
//

#define SIZE_OF_80387_REGISTERS      80

typedef struct _FLOATING_SAVE_AREA {
    ULONG   ControlWord;
    ULONG   StatusWord;
    ULONG   TagWord;
    ULONG   ErrorOffset;
    ULONG   ErrorSelector;
    ULONG   DataOffset;
    ULONG   DataSelector;
    UCHAR   RegisterArea[SIZE_OF_80387_REGISTERS];
    ULONG   Cr0NpxState;
} FLOATING_SAVE_AREA;

//
// Simulated context structure for the 16-bit environment
//

#define VDM_MAXIMUM_SUPPORTED_EXTENSION     512

typedef struct _VDMCONTEXT {

    //
    // The flags values within this flag control the contents of
    // a CONTEXT record.
    //
    // If the context record is used as an input parameter, then
    // for each portion of the context record controlled by a flag
    // whose value is set, it is assumed that that portion of the
    // context record contains valid context. If the context record
    // is being used to modify a threads context, then only that
    // portion of the threads context will be modified.
    //
    // If the context record is used as an IN OUT parameter to capture
    // the context of a thread, then only those portions of the thread's
    // context corresponding to set flags will be returned.
    //
    // The context record is never used as an OUT only parameter.
    //
    // CONTEXT_FULL on some systems (MIPS namely) does not contain the
    // CONTEXT_SEGMENTS definition.  VDMDBG assumes that CONTEXT_INTEGER also
    // includes CONTEXT_SEGMENTS to account for this.
    //

    ULONG ContextFlags;

    //
    // This section is specified/returned if CONTEXT_DEBUG_REGISTERS is
    // set in ContextFlags.  Note that CONTEXT_DEBUG_REGISTERS is NOT
    // included in CONTEXT_FULL.
    //

    ULONG   Dr0;
    ULONG   Dr1;
    ULONG   Dr2;
    ULONG   Dr3;
    ULONG   Dr6;
    ULONG   Dr7;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_FLOATING_POINT.
    //

    FLOATING_SAVE_AREA FloatSave;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_SEGMENTS.
    //

    ULONG   SegGs;
    ULONG   SegFs;
    ULONG   SegEs;
    ULONG   SegDs;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_INTEGER.
    //

    ULONG   Edi;
    ULONG   Esi;
    ULONG   Ebx;
    ULONG   Edx;
    ULONG   Ecx;
    ULONG   Eax;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_CONTROL.
    //

    ULONG   Ebp;
    ULONG   Eip;
    ULONG   SegCs;              // MUST BE SANITIZED
    ULONG   EFlags;             // MUST BE SANITIZED
    ULONG   Esp;
    ULONG   SegSs;

    UCHAR   ExtendedRegisters[VDM_MAXIMUM_SUPPORTED_EXTENSION];

} VDMCONTEXT;

//
//  LDT descriptor entry
//

typedef struct _VDMLDT_ENTRY {
    USHORT  LimitLow;
    USHORT  BaseLow;
    union {
        struct {
            UCHAR   BaseMid;
            UCHAR   Flags1;     // Declare as bytes to avoid alignment
            UCHAR   Flags2;     // Problems.
            UCHAR   BaseHi;
        } Bytes;
        struct {
            ULONG   BaseMid : 8;
            ULONG   Type : 5;
            ULONG   Dpl : 2;
            ULONG   Pres : 1;
            ULONG   LimitHi : 4;
            ULONG   Sys : 1;
            ULONG   Reserved_0 : 1;
            ULONG   Default_Big : 1;
            ULONG   Granularity : 1;
            ULONG   BaseHi : 8;
        } Bits;
    } HighWord;
} VDMLDT_ENTRY;

#endif // _X86_

//
// Legacy context before XSAVE was added to WinXP
//
typedef struct _VDMCONTEXT_WITHOUT_XSAVE {

    //
    // The flags values within this flag control the contents of
    // a CONTEXT record.
    //
    // If the context record is used as an input parameter, then
    // for each portion of the context record controlled by a flag
    // whose value is set, it is assumed that that portion of the
    // context record contains valid context. If the context record
    // is being used to modify a threads context, then only that
    // portion of the threads context will be modified.
    //
    // If the context record is used as an IN OUT parameter to capture
    // the context of a thread, then only those portions of the thread's
    // context corresponding to set flags will be returned.
    //
    // The context record is never used as an OUT only parameter.
    //
    // CONTEXT_FULL on some systems (MIPS namely) does not contain the
    // CONTEXT_SEGMENTS definition.  VDMDBG assumes that CONTEXT_INTEGER also
    // includes CONTEXT_SEGMENTS to account for this.
    //

    ULONG ContextFlags;

    //
    // This section is specified/returned if CONTEXT_DEBUG_REGISTERS is
    // set in ContextFlags.  Note that CONTEXT_DEBUG_REGISTERS is NOT
    // included in CONTEXT_FULL.
    //

    ULONG   Dr0;
    ULONG   Dr1;
    ULONG   Dr2;
    ULONG   Dr3;
    ULONG   Dr6;
    ULONG   Dr7;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_FLOATING_POINT.
    //

    FLOATING_SAVE_AREA FloatSave;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_SEGMENTS.
    //

    ULONG   SegGs;
    ULONG   SegFs;
    ULONG   SegEs;
    ULONG   SegDs;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_INTEGER.
    //

    ULONG   Edi;
    ULONG   Esi;
    ULONG   Ebx;
    ULONG   Edx;
    ULONG   Ecx;
    ULONG   Eax;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_CONTROL.
    //

    ULONG   Ebp;
    ULONG   Eip;
    ULONG   SegCs;              // MUST BE SANITIZED
    ULONG   EFlags;             // MUST BE SANITIZED
    ULONG   Esp;
    ULONG   SegSs;

} VDMCONTEXT_WITHOUT_XSAVE;

typedef VDMCONTEXT    *LPVDMCONTEXT;
typedef VDMLDT_ENTRY  *LPVDMLDT_ENTRY; // Keep since published.
typedef VDMLDT_ENTRY  *PVDMLDT_ENTRY;
typedef PVDMLDT_ENTRY *PPVDMLDT_ENTRY;

#define VDMCONTEXT_TO_PROGRAM_COUNTER(Context) (PVOID)((Context)->Eip)

#define VDMCONTEXT_LENGTH  (sizeof(VDMCONTEXT))
#define VDMCONTEXT_ALIGN   (sizeof(ULONG))
#define VDMCONTEXT_ROUND   (VDMCONTEXT_ALIGN - 1)

#define V86FLAGS_CARRY      0x00001
#define V86FLAGS_PARITY     0x00004
#define V86FLAGS_AUXCARRY   0x00010
#define V86FLAGS_ZERO       0x00040
#define V86FLAGS_SIGN       0x00080
#define V86FLAGS_TRACE      0x00100
#define V86FLAGS_INTERRUPT  0x00200
#define V86FLAGS_DIRECTION  0x00400
#define V86FLAGS_OVERFLOW   0x00800
#define V86FLAGS_IOPL       0x03000
#define V86FLAGS_IOPL_BITS  0x12
#define V86FLAGS_RESUME     0x10000
#define V86FLAGS_V86        0x20000     // Used to detect RealMode v. ProtMode
#define V86FLAGS_ALIGNMENT  0x40000

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#include <poppack.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif   // _VDMCTXT_H_
