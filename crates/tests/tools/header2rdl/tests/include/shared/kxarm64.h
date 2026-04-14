;++
;
; Copyright (c) Microsoft Corporation.  All rights reserved.
;
;
; Module:
;
;   kxarm64.w
;
; Abstract:
;
;   Contains ARM architecture constants and assembly macros.
;
;--

;
; The ARM assembler uses a baroque syntax that is documented as part
; of the online Windows CE documentation.  The syntax derives from
; ARM's own assembler and was chosen to allow the migration of
; specific assembly code bases, namely ARM's floating point runtime.
; While this compatibility is no longer strictly necessary, the
; syntax lives on....
;
; Highlights:
;      * Assembler is white space sensitive.  Symbols are defined by putting
;        them in the first column
;      * The macro definition mechanism is very primitive
;
; To augment the assembler, assembly files are run through CPP (as they are
; on IA64).  This works well for constants but not structural components due
; to the white space sensitivity.
;
; For now, we use a mix of native assembler and CPP macros.
;

#undef TRUE
#undef FALSE

#define TRUE 1
#define FALSE 0

#include "kxarm64unw.h"


;
; For assembly files that are built for both ARM64 and ARM64EC (discouraged
; since the files might not have been ported to use X64 behavior in the ARM64EC
; paths), use this macro to wrap all references to function names (ASM and C).
;
; For ARM64, this does nothing.
;
; For ARM64EC, this changes FuncName to |#FuncName|.
;

#ifndef A64NAME
#define A64NAME_HASHLIT #
#define A64NAME_HASH() A64NAME_HASHLIT
#ifndef _M_ARM64EC
#define A64NAME(FuncName) FuncName
#else
#define A64NAME_DECORATE(FuncName) A64NAME_HASH() ## FuncName
#define A64NAME(FuncName) |A64NAME_DECORATE(FuncName)|  
#endif /*_M_ARM64EC*/
#endif /*A64NAME*/

        ;
        ; Global variables
        ;

        ; Current function names and labels
        GBLS    __FuncNameNoBars
        GBLS    __FuncStartLabel
        GBLS    __FuncEpilog1StartLabel
        GBLS    __FuncEpilog2StartLabel
        GBLS    __FuncEpilog3StartLabel
        GBLS    __FuncEpilog4StartLabel
        GBLS    __FuncPDataLabel
        GBLS    __FuncXDataLabel
        GBLS    __FuncXDataPrologLabel
        GBLS    __FuncXDataEpilog1Label
        GBLS    __FuncXDataEpilog2Label
        GBLS    __FuncXDataEpilog3Label
        GBLS    __FuncXDataEpilog4Label
        GBLS    __FuncXDataEndLabel
        GBLS    __FuncEndLabel
        GBLS    __FuncEntryThunkLabel
        GBLS    __FuncExitThunkLabel

        ; other globals relating to the current function
        GBLS    __FuncComDat
        GBLS    __FuncArea
        GBLS    __FuncPDataArea
        GBLS    __FuncXDataArea
        GBLA    __FuncAlignment
__FuncAlignment SETA 4

        ;
        ; Helper macro: generate the various labels we will use internally
        ; for a function
        ;
        ; Output is placed in the various __Func*Label globals
        ;

        MACRO
        __DeriveFunctionLabels $FuncName, $AreaName

__FuncNameNoBars        SETS "$FuncName"
        IF ("$FuncName":LEFT:1 == "|") && ("$FuncName":RIGHT:1 == "|")
__FuncNameNoBars        SETS ("$FuncName":LEFT:(:LEN:"$FuncName" - 1):RIGHT:(:LEN:"$FuncName" - 2))
        ENDIF
__FuncStartLabel        SETS "|$__FuncNameNoBars|"
__FuncEndLabel          SETS "|$__FuncNameNoBars._end|"
__FuncEpilog1StartLabel SETS "|$__FuncNameNoBars._epilog1_start|"
__FuncEpilog2StartLabel SETS "|$__FuncNameNoBars._epilog2_start|"
__FuncEpilog3StartLabel SETS "|$__FuncNameNoBars._epilog3_start|"
__FuncEpilog4StartLabel SETS "|$__FuncNameNoBars._epilog4_start|"
__FuncPDataLabel        SETS "|$__FuncNameNoBars._pdata|"
__FuncXDataLabel        SETS "|$__FuncNameNoBars._xdata|"
__FuncXDataPrologLabel  SETS "|$__FuncNameNoBars._xdata_prolog|"
__FuncXDataEpilog1Label SETS "|$__FuncNameNoBars._xdata_epilog1|"
__FuncXDataEpilog2Label SETS "|$__FuncNameNoBars._xdata_epilog2|"
__FuncXDataEpilog3Label SETS "|$__FuncNameNoBars._xdata_epilog3|"
__FuncXDataEpilog4Label SETS "|$__FuncNameNoBars._xdata_epilog4|"
__FuncXDataEndLabel     SETS "|$__FuncNameNoBars._xdata_end|"
__FuncEntryThunkLabel   SETS "|$__FuncNameNoBars._entry_thunk|"
__FuncArea              SETS "|.text|"
__FuncPDataArea         SETS "|.pdata|"
__FuncXDataArea         SETS "|.xdata|"
        IF "$AreaName" != ""
__FuncArea              SETS "$AreaName"
        ENDIF
        IF __FuncComDat != ""
__FuncArea              SETS __FuncArea:CC:"{|$__FuncNameNoBars|}"
__FuncPDataArea         SETS __FuncPDataArea:CC:"{$__FuncPDataLabel}"
__FuncXDataArea         SETS __FuncXDataArea:CC:"{$__FuncXDataLabel}"
        ENDIF

        MEND


        ;
        ; Helper macro: create a global label for the given name,
        ; decorate it, and export it for external consumption.
        ;

        MACRO
        __ExportName $FuncName

        LCLS    Name

        IF ("$FuncName":LEFT:1 == "|") && ("$FuncName":RIGHT:1 == "|")
Name    SETS    "$FuncName"
        ELSE
Name    SETS    "|$FuncName|"
        ENDIF

        ALIGN   4
        EXPORT  $Name
$Name
        MEND

        MACRO
        __ExportProc $FuncName

        LCLS    Name
Name    SETS    "|$FuncName|"
        ALIGN   4
        EXPORT  $Name
$Name   PROC
        MEND


        ;
        ; Helper macro to set the AREA to the correct answer
        ; for the current function, and configure the alignment.
        ;

        MACRO
        __SetFunctionAreaAndAlign $Alignment

        LCLS    AreaAlign
        LCLS    AlignStmt

        ;
        ; "NOALIGN" is supported to just set the area
        ;

        IF "$Alignment" == "NOALIGN"
        AREA    $__FuncArea

        ;
        ; COMDAT functions must set alignment in the AREA
        ; statement
        ;

        ELIF __FuncComDat != ""

AreaAlign SETS "4"
        IF "$Alignment" != ""
        IF $Alignment > 4
AreaAlign SETS "$Alignment"
        ENDIF
        ENDIF

        AREA    $__FuncArea,CODE,READONLY,ALIGN=$AreaAlign

        ELSE

AlignStmt SETS ""
        IF "$Alignment" != ""
AlignStmt SETS "ALIGN 0x":CC: :STR:(1 << $Alignment)
        ENDIF

        AREA    $__FuncArea,CODE,READONLY
        $AlignStmt

        ENDIF

        MEND


        ;
        ; Helper macro to emit the special DWORD prior to the start of a
        ; function which contains an offset to the ARM64EC entry thunk.
        ; The entry thunk must have been previously defined or else this
        ; macro is a no-op.
        ;

        MACRO
        __AddEntryThunkPointer $Alignment

        IF :DEF:$__FuncEntryThunkLabel
        ASSERT(__FuncComDat == "")
        ALIGN   (1 << $Alignment),(1 << $Alignment) - 4
        dcd     ($__FuncEntryThunkLabel - ({PC} + 4)) + 1
        ENDIF

        MEND


        ;
        ; Declare that all following code/data is to be put in the .text segment
        ;
        ; N.B. The ALIGN attribute here specifies an exponent of base 2; not a
        ;      direct byte count. Thus ALIGN=4 specifies a 16 byte alignment.
        ;

        MACRO
        TEXTAREA
        AREA    |.text|,ALIGN=4,CODE,READONLY
        MEND


        ;
        ; Declare that all following code/data is to be put in the .data segment
        ;

        MACRO
        DATAAREA
        AREA    |.data|,DATA
        MEND


        ;
        ; Declare that all following code/data is to be put in the .rdata segment
        ;

        MACRO
        RODATAAREA
        AREA    |.rdata|,DATA,READONLY
        MEND


        ;
        ; Set/reset the alignment for COMDAT functions that follow.
        ;
        MACRO
        SET_COMDAT_ALIGNMENT $Alignment

__FuncAlignment SETA $Alignment
        IF __FuncAlignment < 4
__FuncAlignment SETA 4
        ENDIF

        MEND


        MACRO
        RESET_COMDAT_ALIGNMENT

__FuncAlignment SETA 4

        MEND


        ;
        ; Macro for indicating the start of a nested function. Nested functions
        ; imply a prolog, epilog, and unwind codes. This macro presumes that
        ; __DeriveFunctionLabels and __SetFunctionAreaAndAlign have already been
        ; called as appropriate
        ;

        MACRO
        NESTED_ENTRY $FuncName, $AreaName, $ExceptHandler, $Alignment

__FuncComDat SETS ""
        __DeriveFunctionLabels $FuncName,$AreaName
        __SetFunctionAreaAndAlign $Alignment

        IF ("$Alignment" != "")
__FuncAlignment SETA $Alignment
        ELSE
__FuncAlignment SETA 4
        ENDIF

        __AddEntryThunkPointer $__FuncAlignment
        __ResetUnwindState $ExceptHandler
        __ExportProc $__FuncNameNoBars
        ROUT

        MEND


        MACRO
        NESTED_ENTRY_COMDAT $FuncName, $AreaName, $ExceptHandler, $Alignment

        IF ("$Alignment" != "")
        SET_COMDAT_ALIGNMENT $Alignment
        ENDIF

__FuncComDat SETS "COMDAT"
        __DeriveFunctionLabels $FuncName,$AreaName
        __SetFunctionAreaAndAlign $__FuncAlignment
        ASSERT (!:DEF:$__FuncEntryThunkLabel)
        __ResetUnwindState $ExceptHandler
        __ExportProc $__FuncNameNoBars
        ROUT

        MEND


        ;
        ; Generate an ARM64EC entry thunk for an upcoming function.
        ; Note that only COMDAT functions are supported.
        ;

#if defined(_M_ARM64EC)
        IMPORT __os_arm64x_dispatch_ret

        MACRO
        ARM64EC_ENTRY_THUNK $FuncName, $Parameters, $SaveQCount, $AreaName, $Alignment

        LCLS    OriginalFunc
        LCLS    OriginalArea
        LCLA    Params
        LCLA    QCount

Params  SETA    $Parameters
        IF "$SaveQCount" == ""
QCount  SETA    10
        ELSE
QCount  SETA    (($SaveQCount + 1)/2)*2
        ENDIF

        EXPORT $FuncName

        ; first derive labels for the original function
        ; and set that as the target area
__FuncComDat SETS ""
        __DeriveFunctionLabels $FuncName,$AreaName
        __SetFunctionAreaAndAlign $Alignment
OriginalFunc SETS __FuncStartLabel
OriginalArea SETS __FuncArea

        ; then derive labels for the thunk
        __DeriveFunctionLabels $__FuncEntryThunkLabel,$AreaName
__FuncArea SETS OriginalArea
        __ResetUnwindState
        __ExportProc $__FuncNameNoBars

        PROLOG_SAVE_REG_PAIR fp, lr, #-16!

        ; load parameters into x4-x8 as appropriate
        ASSERT(Params <= 8)
        IF (Params >= 6)
        PROLOG_NOP ldp x5, x6, [sp, #48]
        ELIF (Params >= 5)
        PROLOG_NOP ldr x5, [sp, #48]
        ENDIF
        IF (Params >= 8)
        PROLOG_NOP ldp x7, x8, [sp, #64]
        ELIF (Params >= 7)
        PROLOG_NOP ldr x7, [sp, #64]
        ENDIF

        ; optionally save XMMs
        IF (QCount >= 10)
        PROLOG_SAVE_REG_PAIR q6, q7, #-160!
        PROLOG_SAVE_REG_PAIR q8, q9, #32
        PROLOG_SAVE_REG_PAIR q10, q11, #64
        PROLOG_SAVE_REG_PAIR q12, q13, #96
        PROLOG_SAVE_REG_PAIR q14, q15, #128
        ELIF (QCount >= 8)
        PROLOG_SAVE_REG_PAIR q6, q7, #-128!
        PROLOG_SAVE_REG_PAIR q8, q9, #32
        PROLOG_SAVE_REG_PAIR q10, q11, #64
        PROLOG_SAVE_REG_PAIR q12, q13, #96
        ELIF (QCount >= 6)
        PROLOG_SAVE_REG_PAIR q6, q7, #-96!
        PROLOG_SAVE_REG_PAIR q8, q9, #32
        PROLOG_SAVE_REG_PAIR q10, q11, #64
        ELIF (QCount >= 4)
        PROLOG_SAVE_REG_PAIR q6, q7, #-64!
        PROLOG_SAVE_REG_PAIR q8, q9, #32
        ELIF (QCount >= 2)
        PROLOG_SAVE_REG_PAIR q6, q7, #-32!
        ENDIF

        ; call through and copy return result to x8
        bl      $OriginalFunc
        mov     x8,x0

        ; optionally restore XMMs
        IF (QCount >= 10)
        EPILOG_RESTORE_REG_PAIR q14, q15, #128
        EPILOG_RESTORE_REG_PAIR q12, q13, #96
        EPILOG_RESTORE_REG_PAIR q10, q11, #64
        EPILOG_RESTORE_REG_PAIR q8, q9, #32
        EPILOG_RESTORE_REG_PAIR q6, q7, #160!
        ELIF (QCount >= 8)
        EPILOG_RESTORE_REG_PAIR q12, q13, #96
        EPILOG_RESTORE_REG_PAIR q10, q11, #64
        EPILOG_RESTORE_REG_PAIR q8, q9, #32
        EPILOG_RESTORE_REG_PAIR q6, q7, #128!
        ELIF (QCount >= 6)
        EPILOG_RESTORE_REG_PAIR q10, q11, #64
        EPILOG_RESTORE_REG_PAIR q8, q9, #32
        EPILOG_RESTORE_REG_PAIR q6, q7, #96!
        ELIF (QCount >= 4)
        EPILOG_RESTORE_REG_PAIR q8, q9, #32
        EPILOG_RESTORE_REG_PAIR q6, q7, #64!
        ELIF (QCount >= 2)
        EPILOG_RESTORE_REG_PAIR q6, q7, #32!
        ENDIF

        EPILOG_RESTORE_REG_PAIR fp, lr, #16!

        EPILOG_NOP adrp    x16, __os_arm64x_dispatch_ret
        EPILOG_NOP ldr     x16, [x16, __os_arm64x_dispatch_ret]
        EPILOG_END br      x16

        NESTED_END

        MEND

        MACRO
        ARM64EC_CUSTOM_ENTRY_THUNK $FuncName, $AreaName, $Alignment

        LCLS    OriginalFunc
        LCLS    OriginalArea

        EXPORT $FuncName

        ; first derive labels for the original function
        ; and set that as the target area
__FuncComDat SETS ""
        __DeriveFunctionLabels $FuncName,$AreaName
        __SetFunctionAreaAndAlign $Alignment
OriginalFunc SETS __FuncStartLabel
OriginalArea SETS __FuncArea

        ; then derive labels for the thunk
        __DeriveFunctionLabels $__FuncEntryThunkLabel,$AreaName
__FuncArea SETS OriginalArea
        __ResetUnwindState
        __ExportProc $__FuncNameNoBars

        MEND

        MACRO
        ARM64EC_CUSTOM_ENTRY_THUNK_END

        EPILOG_NOP adrp    x16, __os_arm64x_dispatch_ret
        EPILOG_NOP ldr     x16, [x16, __os_arm64x_dispatch_ret]
        EPILOG_END br      x16

        NESTED_END

        MEND

#else
        MACRO
        ARM64EC_ENTRY_THUNK $FuncName, $Parameters, $SaveQCount, $AreaName, $Alignment
        MEND

        MACRO
        ARM64EC_CUSTOM_ENTRY_THUNK $FuncName, $AreaName, $Alignment
        MEND
#endif


        ;
        ; Macro for indicating the end of a nested function. We generate the
        ; .pdata and .xdata records here as necessary.
        ;
        ; Note that the $FuncName parameter is vestigial and not consumed.
        ;

        MACRO
        NESTED_END $FuncName

        ; mark the end of the function
$__FuncEndLabel
        LTORG
        ENDP

        ; generate .pdata

        IF __FuncComDat != ""
        AREA    $__FuncPDataArea,ALIGN=2,READONLY,ASSOC=$__FuncArea
        ELSE
        AREA    $__FuncPDataArea,ALIGN=2,READONLY
        ENDIF

        DCD     $__FuncStartLabel
        RELOC   2                                       ; make this relative to image base

        DCD     $__FuncXDataLabel
        RELOC   2                                       ; make this relative to image base

        ; generate .xdata
        __EmitUnwindXData

        ; back to the original area
        __SetFunctionAreaAndAlign NOALIGN

        ; reset the labels
__FuncStartLabel SETS    ""
__FuncEndLabel  SETS    ""

        MEND


        ;
        ; Macro for indicating the start of a leaf function.
        ;

        MACRO
        LEAF_ENTRY $FuncName, $AreaName, $Alignment

        NESTED_ENTRY $FuncName, $AreaName, "", $Alignment

        MEND


        MACRO
        LEAF_ENTRY_COMDAT $FuncName, $AreaName, $Alignment

        NESTED_ENTRY_COMDAT $FuncName, $AreaName, "", $Alignment

        MEND


        ;
        ; Macro for indicating the end of a leaf function.
        ;

        MACRO
        LEAF_END $FuncName

        NESTED_END $FuncName

        MEND


#if defined(_CHPE_X86_ARM64_) || defined(ARM64_ASM_IN_COMDAT) || (defined(_M_ARM64EC) && defined (ARM64EC_ASM_IN_COMDAT))
#define NESTED_ENTRY NESTED_ENTRY_COMDAT
#define LEAF_ENTRY LEAF_ENTRY_COMDAT
#endif


#if !defined(_CHPE_X86_ARM64_)

        ;
        ; Macro for indicating the start of a leaf function.
        ;

        MACRO
        LEAF_ENTRY_NO_PDATA $FuncName, $AreaName

        ; compute the function's labels
        __DeriveFunctionLabels $FuncName,$AreaName
        __SetFunctionAreaAndAlign

        ; export the function name
        __ExportProc $__FuncNameNoBars

        ; flush any pending literal pool stuff
        ROUT

        MEND


        ;
        ; Macro for indicating the end of a leaf function.
        ;

        MACRO
        LEAF_END_NO_PDATA $FuncName

        ; mark the end of the function
$__FuncEndLabel
        LTORG
        ENDP

        ; reset the labels
__FuncStartLabel SETS    ""
__FuncEndLabel  SETS    ""

        MEND

#endif


        ;
        ; Macro for indicating an alternate entry point into a function.
        ;

        MACRO
        ALTERNATE_ENTRY $FuncName

        ; export the entry point's name
        __ExportName $FuncName

        ; flush any pending literal pool stuff
        ROUT

        MEND


        ;
        ; Macro for getting the address of a data item.
        ;
        
        MACRO
        ADDROF $Reg, $Variable
        
        adrp    $Reg, $Variable                 ; get the page address first
        add     $Reg, $Reg, $Variable           ; add in the low bits
        
        MEND


        ;
        ; Macro for loading a 32-bit constant.
        ;
        
        MACRO
        MOVL32 $Reg, $Variable
        
        IF ((($Variable):SHR:16):AND:0xffff) == 0
        movz    $Reg, #$Variable
        ELIF ((($Variable):SHR:0):AND:0xffff) == 0
        movz    $Reg, #((($Variable):SHR:16):AND:0xffff), lsl #16
        ELSE
        movz    $Reg, #(($Variable):AND:0xffff)
        movk    $Reg, #((($Variable):SHR:16):AND:0xffff), lsl #16
        ENDIF
        
        MEND


#if defined(_CAPKERN)

        ;
        ; Icecap entrypoints
        ;

        IMPORT _CAP_Start_Profiling
        IMPORT _CAP_End_Profiling


        ;
        ; Macro to record a call record
        ;

        MACRO
        CAPSTART $arg1, $arg2

        stp x0, x1, [sp, #-16]!

        ldr x0, =$arg1
        ldr x1, =$arg2
        bl _CAP_Start_Profiling

        ldp x0, x1, [sp], #16

        MEND


        ;
        ; Macro to record a return record
        ;

        MACRO
        CAPEND $arg1

        sub sp, sp, #16
        str x0, [sp, #0]

        ldr x0, =$arg1
        bl _CAP_End_Profiling

        ldr x0, [sp, #0]
        add sp, sp, #16

        MEND

#else

        MACRO
        CAPSTART $arg1, $arg2
        MEND

        MACRO
        CAPEND $arg1
        MEND

#endif


        ;
        ; Macro to align a Control Flow Guard valid call target.
        ; Not necessary to use this before functions anymore as
        ; it is the default for NESTED_ENTRY/LEAF_ENTRY macros.
        ;

        MACRO
        CFG_ALIGN
        ALIGN 16
        MEND


        ;
        ; Macro to perform a Control Flow Guard check on a live call target.
        ;
        ; $TargetReg - Target address register
        ; x16 - Bitmap address
        ; $FailLabel - Label to jump to in the event of a failure
        ; 
        ; N.B. x16-x17 are free, other registers should be treated as live.
        ;
        ; N.B. ValidTarget should only specify a label for the ARM64EC checkers.
        ;      The function has logic below which depends on this.
        ;

        MACRO
        CFG_ICALL_CHECK_BITMAP $TargetReg, $ValidTarget, $FailLabel

        LCLS    ValidLabel

ValidLabel SETS ""

        IF ("$ValidTarget":LEFT:1 != "x" && "$ValidTarget":LEFT:1 != "w" && "$ValidTarget" != "lr")

ValidLabel SETS "$ValidTarget"

        ENDIF

;
; Bitmap is an array of 2-bit values. Each 2-bit value represents 16 bytes, with the low
; bit set if jumps are permitted, and the upper bit set if misaligned jumps are permitted.
;
; The bit index is (address >> 4).
; Each byte holds 4 entries, so the byte index is (address >> 6).
; The shift amount is computed as 2*((address >> 4) & 3), or (address >> 3) & 6
; If address is aligned, (address >> 3) & 7 is equivalent, and ubfx can be used to extract.
;

        lsr     x17, $TargetReg, #6             ; compute bitmap byte index
        tst     $TargetReg, #15                 ; misaligned address?
        ldrb    w17, [x16, x17]                 ; load byte from bitmap
        ubfx    x16, $TargetReg, #3, #3         ; compute bit index*2
        bne     %F2                             ; if misaligned, account for extra bits
        lsr     x17, x17, x16                   ; shift bitmap chunk over to valid align bit
        tbz     x17, #0, %F3                    ; if low bit not set, verify the upper bit to
                                                ; allow an export suppressed target
1 ; Valid

;
; "ret lr" and "br lr" have different encodings and use of "ret lr" is
; preferred to hint that it is returning to the caller.
;

        IF "$ValidLabel" != ""
            b       $ValidLabel                 ; jump to valid target
        ELIF "$ValidTarget" != "lr"
            br      $ValidTarget                ; jump to valid target
        ELSE
            ret     lr                          ; return
        ENDIF

2 ; Misaligned
        ;
        ; Code on ARM64 should always be 16 byte aligned if address taken.
        ;
        ; TODO:  Compress bitmap format to 1 bit per address on ARM64?
        ; TODO:  We are seeing 8-byte aligned code now. (1/22/14)
        ;
        and     x16, x16, #0xfffffffffffffffe   ; force low bit of shift to 0
        lsr     x17, x17, x16                   ; shift bitmap chunk down
        tbz     x17, #0, %F4                    ; invalid if the low bit was clear
3 ; FailOpen
        tbnz    x17, #1, %B1                    ; valid if upper bit was set as well
4 ; Failure
        IF "$ValidLabel" != ""
            ;
            ; Don't need to explicitly set mode for ARM64EC.
            ;
        ELIF "$ValidTarget" != "lr"
            mov     x16, #1                     ; CFG dispatch mode
            IF ("$TargetReg" != "x15")
                mov     x15, $TargetReg         ; move CFG target to x15 for failure function
            ENDIF
        ELSE
            mov     x16, #0                     ; CFG check mode
            IF ("$TargetReg" != "x15")
                mov     x15, $TargetReg         ; move CFG target to x15 for failure function
            ENDIF
        ENDIF

        b       $FailLabel                      ; jump to failure function
        MEND


        ;
        ; Macro to perform a Control Flow Guard check on a live call target with export suppression.
        ;
        ; $TargetReg - Target address register
        ; x16 - Bitmap address
        ; $FailLabel - Label to jump to in the event of a failure
        ; 
        ; N.B. x16-x17 are free, other registers should be treated as live.
        ;
        ; N.B. ValidTarget should only specify a label for the ARM64EC checkers.
        ;      The function has logic below which depends on this.
        ;

        MACRO
        CFG_ICALL_CHECK_BITMAP_ES $TargetReg, $ValidTarget, $FailLabel

        LCLS    ValidLabel

ValidLabel SETS ""

        IF ("$ValidTarget":LEFT:1 != "x" && "$ValidTarget":LEFT:1 != "w" && "$ValidTarget" != "lr")

ValidLabel SETS "$ValidTarget"

        ENDIF

;
; Bitmap is an array of 2-bit values. Each 2-bit value represents 16 bytes, with the low
; bit set if jumps are permitted, and the upper bit set if misaligned jumps are permitted.
;
; The bit index is (address >> 4).
; Each byte holds 4 entries, so the byte index is (address >> 6).
; The shift amount is computed as 2*((address >> 4) & 3), or (address >> 3) & 6
; If address is aligned, (address >> 3) & 7 is equivalent, and ubfx can be used to extract.
;

        lsr     x17, $TargetReg, #6             ; compute bitmap byte index
        tst     $TargetReg, #15                 ; misaligned address?
        ldrb    w17, [x16, x17]                 ; load byte from bitmap
        ubfx    x16, $TargetReg, #3, #3         ; compute bit index*2
        bne     %F2                             ; if misaligned, account for extra bits
        lsr     x17, x17, x16                   ; shift bitmap chunk over to valid align bit
        tbz     x17, #0, %F3                    ; if low bit not set, either invalid or export
                                                ; suppressed target
1 ; Valid

;
; "ret lr" and "br lr" have different encodings and use of "ret lr" is
; preferred to hint that it is returning to the caller.
;

        IF "$ValidLabel" != ""
            b       $ValidLabel                 ; jump to valid target
        ELIF "$ValidTarget" != "lr"
            br      $ValidTarget                ; jump to valid target
        ELSE
            ret     lr                          ; return
        ENDIF

2 ; Misaligned
        ;
        ; Code on ARM64 should always be 16 byte aligned if address taken.
        ;
        ; TODO:  Compress bitmap format to 1 bit per address on ARM64?
        ; TODO:  We are seeing 8-byte aligned code now. (1/22/14)
        ;
        and     x16, x16, #0xfffffffffffffffe   ; force low bit of shift to 0
        lsr     x17, x17, x16                   ; shift bitmap chunk down
        tbz     x17, #0, %F3                    ; invalid if the low bit was clear
        tbnz    x17, #1, %B1                    ; valid if upper bit was set as well

3 ; Failure
        IF "$ValidLabel" != ""
            ;
            ; Don't need to explicitly set mode for ARM64EC.
            ;
        ELIF "$ValidTarget" != "lr"
            mov     x16, #1                     ; CFG dispatch mode
            IF ("$TargetReg" != "x15")
                mov     x15, $TargetReg         ; move CFG target to x15 for failure function
            ENDIF
        ELSE
            mov     x16, #0                     ; CFG check mode
            IF ("$TargetReg" != "x15")
                mov     x15, $TargetReg         ; move CFG target to x15 for failure function
            ENDIF
        ENDIF

        b       $FailLabel                      ; jump to failure function
        MEND


        ;
        ; Macro to acquire a spin lock at address $Reg + $Offset. Clobbers {r0-r2}
        ;

; ARM64_WORKITEM : should we use acquire/release semantics instead of DMB?

;
; TODO: Today this routine is not used. If it is used in the future, consider
;       whether the yield should be switched to enlightened yield.
;

        MACRO
        ACQUIRE_SPIN_LOCK $Reg, $Offset

        mov     x0, #1                                  ; we want to exchange with a 1
        dmb                                             ; memory barrier ahead of the loop
1
        ldxr    x1, [$Reg, $Offset]                     ; load the new value
        stxr    x2, x0, [$Reg, $Offset]                 ; attempt to store the 1
        cbnz    x2, %B1                                 ; did we succeed before someone else did?
        cbz     x1, %F3                                 ; was the lock previously owned? if not, we're done
        yield                                           ; yield execution
        b       %B1                                     ; and try again
3
        dmb

        MEND


        ;
        ; Macro to release a spin lock at address $Reg + $Offset.
        ;

; ARM64_WORKITEM : should we use acquire/release semantics instead of DMB?

        MACRO
        RELEASE_SPIN_LOCK $Reg, $Offset

        dmb
        str     xzr, [$Reg, $Offset]                    ; store 0

        MEND


        ;
        ; Macro to increment a 64-bit statistic.
        ;

        MACRO
        INCREMENT_STAT $AddrReg, $Temp1, $Temp2

1       ldxr    $Temp1, [$AddrReg]                      ; load current value
        add     $Temp1, $Temp1, #1                      ; increment
        stxr    $Temp2, $Temp1, [$AddrReg]              ; attempt to store
        cbnz    $Temp2, %B1                             ; loop until it works?

        MEND


        ;
        ; Macros to enable/disable interrupts.
        ;
        
        MACRO
        ENABLE_INTERRUPTS
        msr     DAIFClr, #2                             ; enable interrupts
        MEND

        MACRO
        DISABLE_INTERRUPTS
        msr     DAIFSet, #2                             ; disable interrupts
        MEND


        ;
        ; Macros to read/write the current IRQL
        ;
        ; N.B. These macros do not do hardware and software IRQL processing.
        ;

        MACRO
        GET_IRQL $Irql
        ldrb    $Irql, [x18, #PcCurrentIrql]            ; read IRQL
        MEND

        MACRO
        RAISE_IRQL $Reg, $NewIrql

#if DBG

        GET_IRQL $Reg                                   ; get old IRQL
        cmp      $Reg, #$NewIrql                        ; check if old IRQL is lower or same
        bls      %F1                                    ; if LS, valid transition

        EMIT_BREAKPOINT                                 ; break
1
#endif

        mov     $Reg, #$NewIrql                         ; get new IRQL
        strb    $Reg, [x18, #PcCurrentIrql]             ; update IRQL
        MEND


        ;
        ; Macros to output special undefined opcodes that indicate breakpoints
        ; and debug services.
        ;

        MACRO
        EMIT_BREAKPOINT
        brk     #ARM64_BREAKPOINT
        MEND


        MACRO
        EMIT_DEBUG_SERVICE
        brk     #ARM64_DEBUG_SERVICE
        MEND

        MACRO
        FASTFAIL $FastFailCode
        mov     x0, $FastFailCode
        brk     #ARM64_FASTFAIL
        MEND


        ;
        ; Macro to generate an exception frame; this is intended to
        ; be used within the prolog of a function.
        ;

        MACRO
        GENERATE_EXCEPTION_FRAME
        PROLOG_SAVE_REG_PAIR x19, x20, #-96!
        PROLOG_SAVE_REG_PAIR x21, x22, #16
#if !defined(_M_ARM64EC)
        PROLOG_SAVE_REG_PAIR x23, x24, #32
#else
        PROLOG_NOP stp       xzr, xzr, [sp, #32]
#endif
        PROLOG_SAVE_REG_PAIR x25, x26, #48
#if !defined(_M_ARM64EC)
        PROLOG_SAVE_REG_PAIR x27, x28, #64
#else
        PROLOG_SAVE_REG      x27, #64
        PROLOG_NOP str       xzr, [sp, #72]
#endif
        PROLOG_SAVE_REG_PAIR fp, lr, #80
        MEND


        ;
        ; Macro to restore from an exception frame; this is intended to
        ; be used within the epilog of a function.
        ;

        MACRO
        RESTORE_EXCEPTION_STATE
        EPILOG_RESTORE_REG_PAIR fp, lr, #80
#if !defined(_M_ARM64EC)
        EPILOG_RESTORE_REG_PAIR x27, x28, #64
#else
        EPILOG_RESTORE_REG      x27 , #64
#endif
        EPILOG_RESTORE_REG_PAIR x25, x26, #48
#if !defined(_M_ARM64EC)
        EPILOG_RESTORE_REG_PAIR x23, x24, #32
#endif
        EPILOG_RESTORE_REG_PAIR x21, x22, #16
        EPILOG_RESTORE_REG_PAIR x19, x20, #96!
        MEND


        ;
        ; Macro to ensure that any eret is followed by barriers to
        ; prevent speculation
        ;
        MACRO
        ERET_FIX
        eret
        dsb sy
        isb sy
        DCD 0xD50330FF ; sb
        MEND

#define eret ERET_FIX

        ;
        ; Given an address, obtains a pointer to the EC code bitmap from the
        ; specified global (typically located in the .mrdata section) and
        ; performs a bitmap lookup to determine if the address is EC code.
        ;
        ; Sets the Zero Flag for X64 targets, clears the Zero Flag for EC targets.
        ;
        ; xAddress      - On input, the code address to be tested.
        ;                 This value is preserved.
        ;
        ; T1            - On input, the address of the EC Bitmap.
        ;                 This register is then used as scratch.
        ;
        ; T2            - On input, the max user-land address.
        ;                 This register is then used as scratch.
        ;
        ; xResult       - Returns true(1) if the target is EC code
        ;                 and false(0) otherwise. This can be 'xzr'
        ;                 if a boolen result is not required. It can
        ;                 also overlap with xAddress, T1 or T2.
        ;
        ; SkipBoundsChecking - If set to "SkipBoundsChecking", no EC
        ;                      Bitmap bounds checks are performed (and
        ;                      T2 doesn't need to provide the max user-land
        ;                      address). T2 is still a scratch reg.
        ;
        ; Zero Flag     - Z=0 for EC code and Z=1 otherwise.
        ;

        MACRO
        EC_BITMAP_LOOKUP $xAddress, $T1, $T2, $xResult, $SkipBoundsChecking

        IF "$SkipBoundsChecking" != "SkipBoundsChecking"

        cmp     $xAddress, x$T2             ; Check if the address is above user space range
        bhi     %F1

        cmp     $xAddress, #(MM_LOWEST_USER_ADDRESS / 4096), lsl #12 ; Check if address < MM_LOWEST_USER_ADDRESS (64KiB)
        blo     %F1                         ; if so, take the fast path

        ENDIF

        lsr     x$T2, $xAddress, #15        ; each byte of bitmap indexes 8*4K = 2^15 byte span
        ldrb    w$T2, [x$T1, x$T2]          ; load the bitmap byte for the 8*4K span
                                            ;
                                            ; * IF THIS INSTRUCTION EVER CHANGES, SO MUST
                                            ; KiOpPreprocessAccessViolation *
                                            ;
        ubfx    x$T1, $xAddress, #12, #3    ; index to the 4K page within the 8*4K span
        lsr     x$T1, x$T2, x$T1

        IF "$SkipBoundsChecking" != "SkipBoundsChecking"

        b       %F2
1
        mov     x$T1, xzr

        ENDIF

2
        ands    $xResult, x$T1, #1          ; test the specific page

        MEND
