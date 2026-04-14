;; LISTING.INC
;;
;; This file contains assembler macros and is included by the files created
;; with the -FA compiler switch to be assembled by MASM (Microsoft Macro
;; Assembler).
;;
;; Copyright (c) 1993-2015, Microsoft Corporation. All rights reserved.

;; Generates VS2015 'multi-byte' nops.  If desired, uncomment
;; the following EQU to generate 'legacy' nops

;_ARCHIA32 EQU 1

ifndef _ARCHIA32

;; non destructive 'multi-byte' nops
npad macro size
if size eq 1
  DB 90H
else
 if size eq 2
   DB 66H, 90H
 else
  if size eq 3
    DB 0FH, 1FH, 00H
  else
   if size eq 4
     DB 0FH, 1FH, 40H, 00H
   else
    if size eq 5
      DB 0FH, 1FH, 44H, 00H, 00H
    else
     if size eq 6
       DB 66H, 0FH, 1FH, 44H, 00H, 00H
     else
      if size eq 7
        DB 0FH, 1FH, 80H, 00H, 00H, 00H, 00H
      else
       if size eq 8
         DB 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
       else
        if size eq 9
          DB 66H, 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
        else
         if size eq 10
           DB 66H, 66H, 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
         else
          if size eq 11
           DB 66H, 66H, 66H, 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
          else
           if size eq 12
             DB 0FH, 1FH, 40H, 00H, 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
           else
            if size eq 13
              DB 0FH, 1FH, 40H, 00H, 66H, 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
            else
             if size eq 14
               DB 0FH, 1FH, 40H, 00H, 66H, 66H, 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
             else
              if size eq 15
                DB 0FH, 1FH, 40H, 00H, 66H, 66H, 66H, 0FH, 1FH, 84H, 00H, 00H, 00H, 00H, 00H
              else
	       %out error: unsupported npad size
               .err
              endif
             endif
            endif
           endif
          endif
         endif
        endif
       endif
      endif
     endif
    endif
   endif
  endif
 endif
endif
endm
;; end non destructive 'multi-byte' nops

else

;; non destructive 'legacy' nops
npad macro size
if size eq 1
  nop
else
 if size eq 2
   mov edi, edi
 else
  if size eq 3
    ; lea ecx, [ecx+00]
    DB 8DH, 49H, 00H
  else
   if size eq 4
     ; lea esp, [esp+00]
     DB 8DH, 64H, 24H, 00H
   else
    if size eq 5
      ; jmp .+5; .npad 3
      DB 0EBH, 03H, 8DH, 49H, 00H
    else
     if size eq 6
       ; lea ebx, [ebx+00000000]
       DB 8DH, 9BH, 00H, 00H, 00H, 00H
     else
      if size eq 7
	; lea esp, [esp+00000000]
	DB 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H
      else
       if size eq 8
        ; jmp .+8; .npad 6
	DB 0EBH, 06H, 8DH, 9BH, 00H, 00H, 00H, 00H
       else
        if size eq 9
         ; jmp .+9; .npad 7
         DB 0EBH, 07H, 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H
        else
         if size eq 10
          ; jmp .+A; .npad 7; .npad 1
          DB 0EBH, 08H, 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H, 90H
         else
          if size eq 11
           ; jmp .+B; .npad 7; .npad 2
           DB 0EBH, 09H, 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H, 8BH, 0FFH
          else
           if size eq 12
            ; jmp .+C; .npad 7; .npad 3
            DB 0EBH, 0AH, 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H, 8DH, 49H, 00H
           else
            if size eq 13
             ; jmp .+D; .npad 7; .npad 4
             DB 0EBH, 0BH, 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H, 8DH, 64H, 24H, 00H
            else
             if size eq 14
              ; jmp .+E; .npad 7; .npad 5
              DB 0EBH, 0CH, 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H, 0EBH, 03H, 8DH, 49H, 00H
             else
              if size eq 15
               ; jmp .+F; .npad 7; .npad 6
               DB 0EBH, 0DH, 8DH, 0A4H, 24H, 00H, 00H, 00H, 00H, 8DH, 9BH, 00H, 00H, 00H, 00H
              else
	       %out error: unsupported npad size
               .err
              endif
             endif
            endif
           endif
          endif
         endif
        endif
       endif
      endif
     endif
    endif
   endif
  endif
 endif
endif
endm
;; end non destructive 'legacy' nops

endif

;; destructive nops
dpad macro size, reg
if size eq 1
  inc reg
else
  %out error: unsupported dpad size
  .err
endif
endm
