;++
;
; Copyright (c) Microsoft Corporation.  All rights reserved.
;
;
; Module:
;
;   kxarm64unw.w
;
; Abstract:
;
;   Contains ARM64 unwind code helper macros
;
;   This file is not really useful on its own without the support from
;   kxarm64.h.
;
;--

;
; The following macros are defined here:
;
;   PROLOG_STACK_ALLOC <amount>
;   PROLOG_SAVE_REG
;   PROLOG_SAVE_REG_PAIR
;   PROLOG_SAVE_REG_PAIR_NO_FP
;   PROLOG_NOP <operation>
;   PROLOG_SAVE_NEXT_PAIR <operation>
;   PROLOG_PUSH_TRAP_FRAME
;   PROLOG_PUSH_MACHINE_FRAME
;   PROLOG_PUSH_CONTEXT
;   PROLOG_PUSH_EC_CONTEXT
;   PROLOG_SIGN_RETURN_ADDRESS
;
;   EPILOG_STACK_FREE <amount>
;   EPILOG_RECOVER_SP <offset>
;   EPILOG_RESTORE_REG
;   EPILOG_RESTORE_REG_PAIR
;   EPILOG_NOP <operation>
;   EPILOG_RESTORE_NEXT_PAIR <operation>
;   EPILOG_AUTHENTICATE_RETURN_ADDRESS
;   EPILOG_RETURN
;

        ;
        ; Global variables
        ;

        ; result from __ParseRegisterNumber
        GBLA __ParsedRegNumber

        ; result from __ParseOffset
        GBLA __ParsedOffsetAbs
        GBLA __ParsedOffsetShifted3
        GBLA __ParsedOffsetShifted4
        GBLA __ParsedOffsetPreinc
        GBLS __ParsedOffsetRawString
        GBLS __ParsedOffsetString

        ; results from __ComputeCodes[...]
        GBLS __ComputedCodes
        GBLL __RegPairWasFpLr

        ; global state and accumulators
        GBLS __PrologUnwindString
        GBLS __PrologLastLabel
        GBLA __EpilogUnwindCount
        GBLS __Epilog1UnwindString
        GBLS __Epilog2UnwindString
        GBLS __Epilog3UnwindString
        GBLS __Epilog4UnwindString
        GBLL __EpilogStartNotDefined
        GBLA __RunningIndex
        GBLS __RunningLabel
        GBLS __FuncExceptionHandler


        ;
        ; Helper macro: emit an opcode with a generated label
        ;
        ; Output: Name of label is in $__RunningLabel
        ;

        MACRO
        __EmitRunningLabelAndOpcode $O1,$O2,$O3,$O4,$O5,$O6

__RunningLabel SETS "|Temp.$__RunningIndex|"
__RunningIndex SETA __RunningIndex + 1

        IF "$O6" != ""
$__RunningLabel $O1,$O2,$O3,$O4,$O5,$O6
        ELIF "$O5" != ""
$__RunningLabel $O1,$O2,$O3,$O4,$O5
        ELIF "$O4" != ""
$__RunningLabel $O1,$O2,$O3,$O4
        ELIF "$O3" != ""
$__RunningLabel $O1,$O2,$O3
        ELIF "$O2" != ""
$__RunningLabel $O1,$O2
        ELIF "$O1" != ""
$__RunningLabel $O1
        ELSE
$__RunningLabel
        ENDIF

        MEND


        ;
        ; Helper macro: append unwind codes to the prolog string
        ;
        ; Input is in __ComputedCodes
        ;

        MACRO
        __AppendPrologCodes

__PrologUnwindString SETS "$__ComputedCodes,$__PrologUnwindString"

        MEND


        ;
        ; Helper macro: append unwind codes to the epilog string
        ;
        ; Input is in __ComputedCodes
        ;

        MACRO
        __AppendEpilogCodes

        IF __EpilogUnwindCount == 1
__Epilog1UnwindString SETS "$__Epilog1UnwindString,$__ComputedCodes"
        ELIF __EpilogUnwindCount == 2
__Epilog2UnwindString SETS "$__Epilog2UnwindString,$__ComputedCodes"
        ELIF __EpilogUnwindCount == 3
__Epilog3UnwindString SETS "$__Epilog3UnwindString,$__ComputedCodes"
        ELIF __EpilogUnwindCount == 4
__Epilog4UnwindString SETS "$__Epilog4UnwindString,$__ComputedCodes"
        ENDIF

        MEND


        ;
        ; Helper macro: detect prolog end
        ;

        MACRO
        __DeclarePrologEnd

__PrologLastLabel SETS "$__RunningLabel"

        MEND


        ;
        ; Helper macro: detect epilog start
        ;

        MACRO
        __DeclareEpilogStart

        IF __EpilogStartNotDefined
__EpilogStartNotDefined SETL {false}
__EpilogUnwindCount SETA __EpilogUnwindCount + 1
        IF __EpilogUnwindCount == 1
$__FuncEpilog1StartLabel
        ELIF __EpilogUnwindCount == 2
$__FuncEpilog2StartLabel
        ELIF __EpilogUnwindCount == 3
$__FuncEpilog3StartLabel
        ELIF __EpilogUnwindCount == 4
$__FuncEpilog4StartLabel
        ELSE
        INFO    1, "Too many epilogues!"
        ENDIF
        ENDIF

        MEND


        ;
        ; Helper macro: specify epilog end
        ;

        MACRO
        __DeclareEpilogEnd

__EpilogStartNotDefined SETL {true}

        MEND


        ;
        ; Parse a register number
        ;
        ; Calling macro name is in $Name
        ; Input is in $Reg
        ; Output is placed in __ParsedRegNumber
        ;

        MACRO
        __ParseRegisterNumber $Name, $Reg

        LCLS    RString

RString SETS    "$Reg"

        IF (RString:LEFT:1 == "q") || (RString:LEFT:1 == "v")

RString  SETS    RString:RIGHT:(:LEN:RString - 1)
__ParsedRegNumber SETA  64 + $RString

        ELIF RString:LEFT:1 == "d"

RString  SETS    RString:RIGHT:(:LEN:RString - 1)
__ParsedRegNumber SETA  32 + $RString

        ELSE

__ParsedRegNumber SETA  :RCONST:$RString

        ENDIF

        MEND


        ;
        ; Parse a stack offset
        ;
        ; Calling macro name is in $Name
        ; Input is in $Offset
        ; Which is "Prolog" or "Epilog"
        ; Output is placed in __ParsedOffsetAbs, __ParsedOffsetPreinc, __ParsedOffsetString
        ;

        MACRO
        __ParseOffset $Name, $Offset, $Which

        ; copy to local string
        LCLS    OffsStr
OffsStr SETS    "$Offset"

        ; strip opening # if present
        IF OffsStr:LEFT:1 == "#"
OffsStr SETS    OffsStr:RIGHT:(:LEN:OffsStr - 1)
        ENDIF

        ; look for pre/postincrement forms
        IF OffsStr:RIGHT:1 == "!"

        ; prolog must be preincrement with a negative offset
        IF "$Which" == "Prolog"

        IF OffsStr:LEFT:1 != "-"
        INFO    1, "$Name: Preincrement offsets must be negative"
        MEXIT
        ENDIF

OffsStr SETS    OffsStr:LEFT:(:LEN:OffsStr - 1)
__ParsedOffsetAbs SETA $OffsStr
__ParsedOffsetAbs SETA -__ParsedOffsetAbs
__ParsedOffsetPreinc SETA 1
__ParsedOffsetRawString SETS "#":CC:OffsStr
__ParsedOffsetString SETS "[sp, #":CC:OffsStr:CC:"]!"

        ; epilog must be postincrement with a positive offset
        ELSE

        IF OffsStr:LEFT:1 == "-"
        INFO    1, "$Name: Postincrement offsets must not be negative"
        MEXIT
        ENDIF

OffsStr SETS    OffsStr:LEFT:(:LEN:OffsStr - 1)
__ParsedOffsetAbs SETA $OffsStr
__ParsedOffsetPreinc SETA 1
__ParsedOffsetRawString SETS "#":CC:OffsStr
__ParsedOffsetString SETS "[sp], #":CC:OffsStr

        ENDIF

        ; standard form
        ELSE

        IF OffsStr:LEFT:1 == "-"
        INFO    1, "$Name: Stack offsets must not be negative"
        MEXIT
        ENDIF

__ParsedOffsetAbs SETA $OffsStr
__ParsedOffsetPreinc SETA 0
__ParsedOffsetRawString SETS "#":CC:OffsStr
__ParsedOffsetString SETS "[sp, #":CC:OffsStr:CC:"]"

        ENDIF

__ParsedOffsetShifted3 SETA __ParsedOffsetAbs:SHR:3 - __ParsedOffsetPreinc
__ParsedOffsetShifted4 SETA __ParsedOffsetAbs:SHR:4 - __ParsedOffsetPreinc

        IF __ParsedOffsetAbs != ((__ParsedOffsetShifted3 + __ParsedOffsetPreinc):SHL:3) || __ParsedOffsetShifted3 >= 0x40
        INFO    1, "$Name: invalid offset $Offset"
        MEXIT
        ENDIF


        MEND


        ;
        ; Compute unwind codes for a register save operation
        ;
        ; Calling macro name is in $Name
        ; Input is in $Reg1, $Offset
        ; Which specifies "Prolog" or "Epilog"
        ; Output is placed in __ComputedCodes
        ;

        MACRO
        __ComputeSaveRegCodes $Name, $Reg1, $Offset, $Which

        LCLA    ByteVal
        LCLA    ByteVal2
        LCLA    ByteVal3
        LCLA    RegNum

        __ParseRegisterNumber $Name, $Reg1
RegNum  SETA    __ParsedRegNumber

        __ParseOffset $Name, $Offset, $Which

        IF (RegNum >= 19) && (RegNum <= 30)
ByteVal SETA 0xd0:OR:(__ParsedOffsetPreinc:SHL:2):OR:((RegNum - 19):SHR:2)
ByteVal2 SETA (((RegNum - 19):AND:3):SHL:6):OR:__ParsedOffsetShifted3
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2):CC:",0x":CC:((:STR:ByteVal2):RIGHT:2)

        ELIF (RegNum >= 40) && (RegNum <= 47)
ByteVal SETA 0xdc:OR:(__ParsedOffsetPreinc:SHL:1):OR:((RegNum - 40):SHR:2)
ByteVal2 SETA (((RegNum - 40):AND:3):SHL:6):OR:__ParsedOffsetShifted3
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2):CC:",0x":CC:((:STR:ByteVal2):RIGHT:2)

#if defined(_M_ARM64EC) || ENABLE_SAVE_ANY_REG
        ELIF (RegNum < 96)
        ; 11100111 ' 0pxrrrrr ' ffoooooo
        ; p: 0/1 - single/pair
        ; x: 0/1 - positive offset / negative offset with writeback
        ; r: register number
        ; f: 00/01/10 - X / D / Q
        ; o: offset * 16 for x=1 or p=1 or f=Q / else offset * 8
ByteVal SETA 0xe7
ByteVal2 SETA 0x00:OR:(__ParsedOffsetPreinc:SHL:5):OR:(RegNum:AND:31)
ByteVal3 SETA (RegNum:AND:0x60):SHL:1
        IF (RegNum >= 64) || (__ParsedOffsetPreinc != 0)
ByteVal3 SETA ByteVal3:OR:__ParsedOffsetShifted4
        ELSE
ByteVal3 SETA ByteVal3:OR:__ParsedOffsetShifted3
        ENDIF
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2):CC:",0x":CC:((:STR:ByteVal2):RIGHT:2):CC:",0x":CC:((:STR:ByteVal3):RIGHT:2)
#endif

        ELSE
        INFO    1, "$Name: Unsupported register: $Reg1"
        ENDIF

        MEND


        ;
        ; Compute unwind codes for a register pair save operation
        ;
        ; Calling macro name is in $Name
        ; Input is in $Reg1, $Reg2, $Offset
        ; Which specifies "Prolog" or "Epilog"
        ; Output is placed in __ComputedCodes
        ;

        MACRO
        __ComputeSaveRegPairCodes $Name, $Reg1, $Reg2, $Offset, $Which

        LCLA    ByteVal
        LCLA    ByteVal2
        LCLA    ByteVal3
        LCLA    RegNum1
        LCLA    RegNum2

        __ParseRegisterNumber $Name, $Reg1
RegNum1 SETA    __ParsedRegNumber

        __ParseRegisterNumber $Name, $Reg2
RegNum2 SETA    __ParsedRegNumber

        __ParseOffset $Name, $Offset, $Which

__RegPairWasFpLr SETL {false}

        IF (RegNum1 == 29) && (RegNum2 == 30)
ByteVal SETA    (0x40+(__ParsedOffsetPreinc*0x40)):OR:__ParsedOffsetShifted3
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2)
__RegPairWasFpLr SETL {true}

        ELIF (RegNum1 == 19) && (RegNum2 == 20) && (__ParsedOffsetPreinc != 0)
ByteVal SETA    0x20:OR:(__ParsedOffsetShifted3 + 1)
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2)

        ELIF (RegNum1 >= 19) && (RegNum1 <= 30) && (((RegNum1 - 19):AND:1) == 0) && (RegNum2 == 30) && (__ParsedOffsetPreinc == 0)
ByteVal SETA 0xd6:OR:((RegNum1 - 19):SHR:3)
ByteVal2 SETA ((((RegNum1 - 19):SHR:1):AND:3):SHL:6):OR:__ParsedOffsetShifted3
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2):CC:",0x":CC:((:STR:ByteVal2):RIGHT:2)

        ELIF (RegNum1 >= 19) && (RegNum1 <= 30) && (RegNum2 == (RegNum1 + 1))
ByteVal SETA 0xc8:OR:(__ParsedOffsetPreinc:SHL:2):OR:((RegNum1 - 19):SHR:2)
ByteVal2 SETA (((RegNum1 - 19):AND:3):SHL:6):OR:__ParsedOffsetShifted3
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2):CC:",0x":CC:((:STR:ByteVal2):RIGHT:2)

        ELIF (RegNum1 >= 40) && (RegNum1 <= 47) && (RegNum2 == (RegNum1 + 1))
ByteVal SETA 0xd8:OR:(__ParsedOffsetPreinc:SHL:1):OR:((RegNum1 - 40):SHR:2)
ByteVal2 SETA (((RegNum1 - 40):AND:3):SHL:6):OR:__ParsedOffsetShifted3
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2):CC:",0x":CC:((:STR:ByteVal2):RIGHT:2)

#if defined(_M_ARM64EC) || ENABLE_SAVE_ANY_REG
        ELIF (RegNum2 == (RegNum1 + 1))
        ; 11100111 ' 0pxrrrrr ' ffoooooo
        ; p: 0/1 - single/pair
        ; x: 0/1 - positive offset / negative offset with writeback
        ; r: register number
        ; f: 00/01/10 - X / D / Q
        ; o: offset * 16 for x=1 or p=1 or f=Q / else offset * 8
ByteVal SETA 0xe7
ByteVal2 SETA 0x40:OR:(__ParsedOffsetPreinc:SHL:5):OR:(RegNum1:AND:31)
ByteVal3 SETA ((RegNum1:AND:0x60):SHL:1):OR:(__ParsedOffsetAbs:SHR:4 - __ParsedOffsetPreinc)
__ComputedCodes SETS "0x":CC:((:STR:ByteVal):RIGHT:2):CC:",0x":CC:((:STR:ByteVal2):RIGHT:2):CC:",0x":CC:((:STR:ByteVal3):RIGHT:2)
#endif

        ELSE
        INFO    1, "$Name: Unsupported register pair: $Reg1, $Reg2"
        ENDIF

        MEND


        ;
        ; Compute unwind codes for a stack alloc/dealloc operation
        ;
        ; Output is placed in __ComputedCodes
        ;

        MACRO
        __ComputeStackAllocCodes $Name, $Amount

        LCLA    Shifted
        LCLA    Byte1
        LCLA    Byte2
        LCLA    Byte3

Shifted SETA  ($Amount):SHR:4

        IF Shifted < 0x20
__ComputedCodes SETS "0x":CC:((:STR:Shifted):RIGHT:2)

        ELIF Shifted < 0x800
Byte1   SETA  0xC0:OR:((Shifted:SHR:8):AND:0x7)
Byte2   SETA  Shifted:AND:0xFF
__ComputedCodes SETS "0x":CC:((:STR:Byte1):RIGHT:2):CC:",0x":CC:((:STR:Byte2):RIGHT:2)

        ELIF Shifted < 0x1000000
Byte1   SETA  ((Shifted:SHR:16):AND:0xFF)
Byte2   SETA  ((Shifted:SHR:8):AND:0xFF)
Byte3   SETA  (Shifted:AND:0xFF)
__ComputedCodes SETS "0xE0,0x":CC:((:STR:Byte1):RIGHT:2):CC:",0x":CC:((:STR:Byte2):RIGHT:2):CC:",0x":CC:((:STR:Byte3):RIGHT:2)

        ELSE
        INFO    1, "$Name too large for unwind code encoding"
        ENDIF

        MEND


        ;
        ; Macro for allocating space on the stack in the prolog
        ;

        MACRO
        PROLOG_STACK_ALLOC $Amount

        __ComputeStackAllocCodes "PROLOG_STACK_ALLOC", $Amount

        __EmitRunningLabelAndOpcode sub sp, sp, #$Amount
        __DeclarePrologEnd
        __AppendPrologCodes

        MEND


        ;
        ; Macro for a single register save operation in a prologue
        ;

        MACRO
        PROLOG_SAVE_REG $Reg1, $Offset

        IF "$Offset" == ""
        INFO    1, "Must specify offset in PROLOG_SAVE_REG"
        MEXIT
        ENDIF

        __ComputeSaveRegCodes "PROLOG_SAVE_REG", $Reg1, $Offset, "Prolog"

        __EmitRunningLabelAndOpcode str $Reg1, $__ParsedOffsetString
        __DeclarePrologEnd
        __AppendPrologCodes

        MEND


        ;
        ; Macro for an register pair save operation in a prologue
        ;

        MACRO
        PROLOG_SAVE_REG_PAIR $Reg1, $Reg2, $Offset

        IF "$Offset" == ""
        INFO    1, "Must specify offset in PROLOG_SAVE_REG_PAIR"
        MEXIT
        ENDIF

        __ComputeSaveRegPairCodes "PROLOG_SAVE_REG_PAIR", $Reg1, $Reg2, $Offset, "Prolog"

        IF __RegPairWasFpLr

#if defined(_CHPE_X86_ARM64_)

        __EmitRunningLabelAndOpcode stp wfp, wlr, $__ParsedOffsetString

#else

        __EmitRunningLabelAndOpcode stp fp, lr, $__ParsedOffsetString

#endif

        IF (__ParsedOffsetAbs != 0) && (__ParsedOffsetPreinc == 0)
        __EmitRunningLabelAndOpcode add fp, sp, $__ParsedOffsetRawString
__ComputedCodes SETS "0xe2,0x":CC:((:STR:__ParsedOffsetShifted3):RIGHT:2):CC:",":CC:__ComputedCodes
        ELSE

        __EmitRunningLabelAndOpcode mov fp, sp
__ComputedCodes SETS "0xe1,":CC:__ComputedCodes
        ENDIF

        ELSE

        __EmitRunningLabelAndOpcode stp $Reg1, $Reg2, $__ParsedOffsetString

        ENDIF

        __DeclarePrologEnd
        __AppendPrologCodes

        MEND


        ;
        ; Same as above but don't treat FP specially
        ;

        MACRO
        PROLOG_SAVE_REG_PAIR_NO_FP $Reg1, $Reg2, $Offset

        IF "$Offset" == ""
        INFO    1, "Must specify offset in PROLOG_SAVE_REG_PAIR"
        MEXIT
        ENDIF

        __ComputeSaveRegPairCodes "PROLOG_SAVE_REG_PAIR", $Reg1, $Reg2, $Offset, "Prolog"

        IF __RegPairWasFpLr

#if defined(_CHPE_X86_ARM64_)

        __EmitRunningLabelAndOpcode stp wfp, wlr, $__ParsedOffsetString

#else

        __EmitRunningLabelAndOpcode stp fp, lr, $__ParsedOffsetString

#endif

        ELSE

        __EmitRunningLabelAndOpcode stp $Reg1, $Reg2, $__ParsedOffsetString

        ENDIF

        __DeclarePrologEnd
        __AppendPrologCodes

        MEND


        ;
        ; Macro for including an arbitrary operation in the prolog
        ;

        MACRO
        PROLOG_NOP $O1,$O2,$O3,$O4

__ComputedCodes SETS "0xE3"

        __EmitRunningLabelAndOpcode $O1,$O2,$O3,$O4
        __DeclarePrologEnd
        __AppendPrologCodes

        MEND


        ;
        ; Macro for saving the next pair of registers
        ;

        MACRO
        PROLOG_SAVE_NEXT_PAIR $O1,$O2,$O3,$O4

__ComputedCodes SETS "0xE6"

        __EmitRunningLabelAndOpcode $O1,$O2,$O3,$O4
        __DeclarePrologEnd
        __AppendPrologCodes

        MEND


        ;
        ; Macro for indicating a trap frame lives above us
        ;

        MACRO
        PROLOG_PUSH_TRAP_FRAME
        __DeclarePrologEnd

__ComputedCodes SETS "0xE8"
        __AppendPrologCodes

        MEND


        ;
        ; Macro for indicating a machine frame lives above us
        ;

        MACRO
        PROLOG_PUSH_MACHINE_FRAME
        __DeclarePrologEnd

__ComputedCodes SETS "0xE9"
        __AppendPrologCodes

        MEND


        ;
        ; Macro for indicating a ARM64_NT_CONTEXT lives above us
        ;

        MACRO
        PROLOG_PUSH_CONTEXT
        __DeclarePrologEnd

__ComputedCodes SETS "0xEA"
        __AppendPrologCodes

        MEND


        ;
        ; Macro for indicating a ARM64EC_NT_CONTEXT lives above us
        ;

        MACRO
        PROLOG_PUSH_EC_CONTEXT
        __DeclarePrologEnd

__ComputedCodes SETS "0xEB"
        __AppendPrologCodes

        MEND


        ;
        ; Macro for signing the return address in the prolog
        ;

        MACRO
        PROLOG_SIGN_RETURN_ADDRESS

__ComputedCodes SETS "0xFC"

        __EmitRunningLabelAndOpcode pacibsp
        __DeclarePrologEnd
        __AppendPrologCodes

        MEND


        ;
        ; Macro for restoring the stack pointer.
        ;

        MACRO
        EPILOG_STACK_RESTORE

__ComputedCodes SETS "0xE1"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode mov sp, fp
        __AppendEpilogCodes

        MEND


        ;
        ; Macro for deallocating space on the stack in the prolog
        ;

        MACRO
        EPILOG_STACK_FREE $Amount

        __ComputeStackAllocCodes "EPILOG_STACK_FREE", $Amount

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode add sp, sp, #$Amount
        __AppendEpilogCodes

        MEND


        ;
        ; Macro for a single integer register restore operation in an epilogue
        ;

        MACRO
        EPILOG_RESTORE_REG $Reg1, $Offset

        IF "$Offset" == ""
        INFO    1, "Must specify offset in EPILOG_RESTORE_REG"
        MEXIT
        ENDIF

        __ComputeSaveRegCodes "EPILOG_RESTORE_REG", $Reg1, $Offset, "Epilog"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode ldr $Reg1, $__ParsedOffsetString
        __AppendEpilogCodes

        MEND


        ;
        ; Macro for an integer register pair restore operation in an epilogue
        ;

        MACRO
        EPILOG_RESTORE_REG_PAIR $Reg1, $Reg2, $Offset

        IF "$Offset" == ""
        INFO    1, "Must specify offset in EPILOG_RESTORE_REG_PAIR"
        MEXIT
        ENDIF

        __ComputeSaveRegPairCodes "EPILOG_RESTORE_REG_PAIR", $Reg1, $Reg2, $Offset, "Epilog"

        __DeclareEpilogStart

        IF __RegPairWasFpLr

#if defined(_CHPE_X86_ARM64_)

        __EmitRunningLabelAndOpcode ldp wfp, wlr, $__ParsedOffsetString

#else

        __EmitRunningLabelAndOpcode ldp fp, lr, $__ParsedOffsetString

#endif


        ELSE

        __EmitRunningLabelAndOpcode ldp $Reg1, $Reg2, $__ParsedOffsetString

        ENDIF

        __AppendEpilogCodes

        MEND


        ;
        ; Macro for including an arbitrary operation in the epilog
        ;

        MACRO
        EPILOG_NOP $O1,$O2,$O3,$O4

__ComputedCodes SETS "0xE3"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode $O1,$O2,$O3,$O4
        __AppendEpilogCodes

        MEND


        ;
        ; Macro for restoring the next pair of registers
        ;

        MACRO
        EPILOG_RESTORE_NEXT_PAIR $O1,$O2,$O3,$O4

__ComputedCodes SETS "0xE6"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode $O1,$O2,$O3,$O4
        __AppendEpilogCodes

        MEND


        ;
        ; Macro for authenticating the return address in the epilog
        ;

        MACRO
        EPILOG_AUTHENTICATE_RETURN_ADDRESS

__ComputedCodes SETS "0xFC"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode autibsp
        __AppendEpilogCodes

        MEND


        ;
        ; Macro for a bx lr-style return in the epilog
        ;

        MACRO
        EPILOG_RETURN

__ComputedCodes SETS "0xE4"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode ret x30
        __AppendEpilogCodes
        __DeclareEpilogEnd

        MEND


        ;
        ; Macro for generic end of epilog such as direct
        ; (b) or indirect (br) tail calls.
        ; 

        MACRO
        EPILOG_END $O1,$O2,$O3,$O4

__ComputedCodes SETS "0xE4"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode $O1,$O2,$O3,$O4
        __AppendEpilogCodes
        __DeclareEpilogEnd

        MEND


        ;
        ; Emit an opcode indicating that the unwind address of this function
        ; will be at the next instruction after a call rather than the call
        ; instruction. An example where this is necessary is the security cookie
        ; pop code, which modifies the caller's frame.
        ;

        MACRO
        EPILOG_RETURN_CLEAR_UNWIND_TO_CALLER

__ComputedCodes SETS "0xEC, 0XE4"

        __DeclareEpilogStart
        __EmitRunningLabelAndOpcode ret x30
        __EmitRunningLabelAndOpcode nop
        __AppendEpilogCodes
        __DeclareEpilogEnd

        MEND

        ;
        ; Macro to reset the internal uwninding states
        ;

        MACRO
        __ResetUnwindState $ExceptHandler
__PrologUnwindString SETS ""
__EpilogUnwindCount SETA 0
__Epilog1UnwindString SETS ""
__Epilog2UnwindString SETS ""
__Epilog3UnwindString SETS ""
__Epilog4UnwindString SETS ""
__EpilogStartNotDefined SETL {true}
__FuncExceptionHandler SETS ""
        IF "$ExceptHandler" != ""
__FuncExceptionHandler SETS "$ExceptHandler"

        ;
        ; Add bars to the exception handler name only when the name does not have bars
        ;

        IF ("$ExceptHandler":LEFT:1 != "|") && ("$ExceptHandler":RIGHT:1 != "|")
__FuncExceptionHandler SETS "|$ExceptHandler|"
        ENDIF

        ENDIF
        MEND


        ;
        ; Macro to emit the xdata for unwinding
        ;

        MACRO
        __EmitUnwindXData

        LCLA    XBit

XBit    SETA    0
        IF "$__FuncExceptionHandler" != ""
XBit    SETA    1:SHL:20
        ENDIF

        ;
        ; Append terminators where necessary
        ;
        IF __EpilogUnwindCount >= 1
__Epilog1UnwindString SETS __Epilog1UnwindString:RIGHT:(:LEN:__Epilog1UnwindString - 1)
        IF (:LEN:__Epilog1UnwindString) >= 5
        IF __Epilog1UnwindString:RIGHT:4 < "0xE4"
__Epilog1UnwindString SETS __Epilog1UnwindString:CC:",0xE4"
        ENDIF
        ENDIF
        ENDIF

        IF __EpilogUnwindCount >= 2
__Epilog2UnwindString SETS __Epilog2UnwindString:RIGHT:(:LEN:__Epilog2UnwindString - 1)
        IF (:LEN:__Epilog2UnwindString) >= 5
        IF __Epilog2UnwindString:RIGHT:4 < "0xE4"
__Epilog2UnwindString SETS __Epilog2UnwindString:CC:",0xE4"
        ENDIF
        ENDIF
        ENDIF

        IF __EpilogUnwindCount >= 3
__Epilog3UnwindString SETS __Epilog3UnwindString:RIGHT:(:LEN:__Epilog3UnwindString - 1)
        IF (:LEN:__Epilog3UnwindString) >= 5
        IF __Epilog3UnwindString:RIGHT:4 < "0xE4"
__Epilog3UnwindString SETS __Epilog3UnwindString:CC:",0xE4"
        ENDIF
        ENDIF
        ENDIF

        IF __EpilogUnwindCount >= 4
__Epilog4UnwindString SETS __Epilog4UnwindString:RIGHT:(:LEN:__Epilog4UnwindString - 1)
        IF (:LEN:__Epilog4UnwindString) >= 5
        IF __Epilog4UnwindString:RIGHT:4 < "0xE4"
__Epilog4UnwindString SETS __Epilog4UnwindString:CC:",0xE4"
        ENDIF
        ENDIF
        ENDIF

        IF "$__PrologUnwindString" != ""
__PrologUnwindString SETS __PrologUnwindString:CC:"0xE4"
        ELSE
__PrologUnwindString SETS "0xE4"
        ENDIF

        ; optimize out the prolog string if it matches
;        IF (:LEN:__Epilog1UnwindString) >= 6
;        IF __Epilog1UnwindString:LEFT:(:LEN:__Epilog1UnwindString - 4) == __PrologUnwindString:LEFT:(:LEN:__PrologUnwindString - 4)
;__PrologUnwindString SETS ""
;        ENDIF
;        ENDIF

        ;
        ; Switch to the .xdata section, aligned to a DWORD
        ;

        IF __FuncComDat != ""
        AREA    $__FuncXDataArea,ALIGN=2,READONLY,ASSOC=$__FuncArea
        ELSE
        AREA    $__FuncXDataArea,ALIGN=2,READONLY
        ENDIF

        ALIGN   4

        ; declare the xdata header with unwind code size, epilog count,
        ; exception bit, and function length
$__FuncXDataLabel
        DCD     ((($__FuncXDataEndLabel - $__FuncXDataPrologLabel)/4):SHL:27) :OR: (__EpilogUnwindCount:SHL:22) :OR: XBit :OR: (($__FuncEndLabel - $__FuncStartLabel)/4)

        ; if we have an epilogue, output a single scope record
        IF __EpilogUnwindCount >= 1
        DCD     (($__FuncXDataEpilog1Label - $__FuncXDataPrologLabel):SHL:22) :OR: (($__FuncEpilog1StartLabel - $__FuncStartLabel)/4)
        ENDIF
        IF __EpilogUnwindCount >= 2
        DCD     (($__FuncXDataEpilog2Label - $__FuncXDataPrologLabel):SHL:22) :OR: (($__FuncEpilog2StartLabel - $__FuncStartLabel)/4)
        ENDIF
        IF __EpilogUnwindCount >= 3
        DCD     (($__FuncXDataEpilog3Label - $__FuncXDataPrologLabel):SHL:22) :OR: (($__FuncEpilog3StartLabel - $__FuncStartLabel)/4)
        ENDIF
        IF __EpilogUnwindCount >= 4
        DCD     (($__FuncXDataEpilog4Label - $__FuncXDataPrologLabel):SHL:22) :OR: (($__FuncEpilog4StartLabel - $__FuncStartLabel)/4)
        ENDIF

        ; output the prolog unwind string
$__FuncXDataPrologLabel
        DCB     $__PrologUnwindString

        ; if we have an epilogue, output the epilog unwind codes
        IF __EpilogUnwindCount >= 1
$__FuncXDataEpilog1Label
        DCB     $__Epilog1UnwindString
        ENDIF
        IF __EpilogUnwindCount >= 2
$__FuncXDataEpilog2Label
        DCB     $__Epilog2UnwindString
        ENDIF
        IF __EpilogUnwindCount >= 3
$__FuncXDataEpilog3Label
        DCB     $__Epilog3UnwindString
        ENDIF
        IF __EpilogUnwindCount >= 4
$__FuncXDataEpilog4Label
        DCB     $__Epilog4UnwindString
        ENDIF

        ALIGN   4
$__FuncXDataEndLabel

        ; output the exception handler information
        IF "$__FuncExceptionHandler" != ""
        DCD     $__FuncExceptionHandler
        RELOC   2                                       ; make this relative to image base
        DCD     0                                       ; append a 0 for the data (keeps Vulcan happy)
        ENDIF

        ; switch back to the original area
        AREA    $__FuncArea,CODE,READONLY

        MEND
