; Calculate a circle radius using floating point
%include "common.asm"

    extern printResult

    section .data
       radius  dq 1.7
       result dq 0

    global _start
    section .text

_start:
    fld qword [radius]
    fld qword [radius]
    fmul

    fldpi
    fmul
    fstp qword [result]

    mov rax,0
    movq xmm0, [result]
    call printResult

    EXIT 0

