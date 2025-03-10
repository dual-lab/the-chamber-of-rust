; Example of commmon macro definition
; Safe check to prevent double include
%ifndef COMMON_MAC
    %define COMMON_MAC

    ; Prepare for print static input string
    ; TODO preserve old register in x64 pusha is not supported
    %macro PRINT 1
      ;pusha
      pushf
      jmp %%astr
    %%str db %1,0
    %%slen equ $-%%str
    %%astr:
      _sys_call %%str, %%slen
      popf
      ;popa
    %endmacro

    ; Wrap system call to print on stdout
    %macro _sys_call 2
      mov rax, 1
      mov rdi, 1
      mov rsi, %1
      mov rdx, %2
      syscall
    %endmacro

    ; Warap systm call exit with code
    %macro EXIT 1
      mov rax, 60
      mov rdi, %1
      syscall
    %endmacro

%endif
