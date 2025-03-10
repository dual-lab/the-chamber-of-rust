;;; Reverse a  fixed string stored in memeory
;;;
%include "common.asm"

    section .data
      SYS_WRITE equ 1
      STD_OUT   equ 1
      SYS_EXIT  equ 60
      EXIT_CODE equ 0

      NEW_LINE  db 0xa
      INPUT     db "Hello world!"

    section .bss
      OUTPUT resb 12

    section .text
    global _start

_start:
    mov rsi, INPUT
    xor rcx,rcx
    cld
    mov rdi, $ + 15
    call calculate_str_len
    xor rax, rax
    xor rdi, rdi
    jmp _reverse_Str

calculate_str_len:
    cmp byte [rsi], 0
    je _exit_from_routine
    lodsb
    push rax
    inc rcx
    jmp calculate_str_len

_exit_from_routine:
    ; Need to push back on the stack the next instruction pointer
    push rdi
    ret

_reverse_Str:
    cmp rcx, 0
    je _print_result
    pop rax
    mov [OUTPUT + rdi], rax
    dec rcx
    inc rdi
    jmp _reverse_Str

_print_result:
    mov rdx, rdi
    mov rax, SYS_WRITE
    mov rdi, STD_OUT
    mov rsi, OUTPUT
    syscall
    jmp _print_new_line

_print_new_line:
    PRINT 0xa
    jmp _exit

_exit:
    EXIT EXIT_CODE

