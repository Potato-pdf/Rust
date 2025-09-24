; calc.asm -- Calculadora básica en NASM (x86-64, Linux)
; Ensamblar y ejecutar:
;   nasm -felf64 calc.asm -o calc.o
;   ld calc.o -o calc
;   ./calc

SECTION .data
prompt1     db  "Primer numero: ", 0
p1_len      equ $-prompt1
prompt2     db  "Operador (+ - * /): ", 0
p2_len      equ $-prompt2
prompt3     db  "Segundo numero: ", 0
p3_len      equ $-prompt3
err_div0    db  "Error: Division por cero", 10, 0
err_op      db  "Operador invalido", 10, 0
out_fmt     db  "Resultado: ", 0
newline     db  10, 0

SECTION .bss
buf1    resb 64
buf2    resb 4
buf3    resb 64
resbuf  resb 64

SECTION .text
global _start

%define SYS_read   0
%define SYS_write  1
%define SYS_exit   60

_start:
    ; ----- Primer número -----
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, prompt1
    mov rdx, p1_len
    syscall

    mov rax, SYS_read
    mov rdi, 0
    mov rsi, buf1
    mov rdx, 63
    syscall

    lea rdi, [buf1]
    call parse_int
    mov rbx, rax        ; n1

    ; ----- Operador -----
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, prompt2
    mov rdx, p2_len
    syscall

    mov rax, SYS_read
    mov rdi, 0
    mov rsi, buf2
    mov rdx, 3
    syscall

    movzx r13, byte [buf2] ; operador

    ; ----- Segundo número -----
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, prompt3
    mov rdx, p3_len
    syscall

    mov rax, SYS_read
    mov rdi, 0
    mov rsi, buf3
    mov rdx, 63
    syscall

    lea rdi, [buf3]
    call parse_int
    mov rcx, rax        ; n2

    ; ----- Operaciones -----
    cmp r13, '+'
    je .do_add
    cmp r13, '-'
    je .do_sub
    cmp r13, '*'
    je .do_mul
    cmp r13, '/'
    je .do_div

    ; operador inválido
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, err_op
    mov rdx, 18
    syscall
    jmp .exit

.do_add:
    mov rax, rbx
    add rax, rcx
    jmp .print_result

.do_sub:
    mov rax, rbx
    sub rax, rcx
    jmp .print_result

.do_mul:
    mov rax, rbx
    imul rax, rcx
    jmp .print_result

.do_div:
    cmp rcx, 0
    je .div_zero
    mov rax, rbx
    cqo
    idiv rcx
    jmp .print_result

.div_zero:
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, err_div0
    mov rdx, 26
    syscall
    jmp .exit

.print_result:
    ; mensaje "Resultado: "
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, out_fmt
    mov rdx, 11
    syscall

    ; convertir número en string
    mov rsi, rax        ; rax contiene el resultado de la operación
    mov rdi, resbuf
    mov rsi, rax        ; aseguramos que rsi tiene el resultado
    call int_to_str

    ; imprimir número
    mov rdx, rax        ; rax = longitud del string
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, resbuf
    syscall

    ; salto de línea
    mov rax, SYS_write
    mov rdi, 1
    mov rsi, newline
    mov rdx, 1
    syscall

.exit:
    mov rax, SYS_exit
    xor rdi, rdi
    syscall

; ----------------------
; FUNCIONES AUXILIARES
; ----------------------

; parse_int: convierte string a número
; Entrada: RDI -> buffer
; Salida:  RAX = entero
parse_int:
    xor rax, rax
    xor rcx, rcx        ; signo
    mov rsi, rdi

.skip:
    mov bl, [rsi]
    cmp bl, ' '
    je .next
    cmp bl, 9
    je .next
    jmp .sign
.next:
    inc rsi
    jmp .skip

.sign:
    mov bl, [rsi]
    cmp bl, '-'
    jne .check_plus
    mov rcx, 1
    inc rsi
    jmp .parse
.check_plus:
    cmp bl, '+'
    jne .parse
    inc rsi

.parse:
    xor rax, rax
.loop:
    mov bl, [rsi]
    cmp bl, 10
    je .done
    cmp bl, 0
    je .done
    cmp bl, '0'
    jb .done
    cmp bl, '9'
    ja .done
    imul rax, 10
    mov rdx, rbx
    sub rdx, '0'
    add rax, rdx
    inc rsi
    jmp .loop

.done:
    cmp rcx, 1
    jne .ret
    neg rax
.ret:
    ret

; int_to_str: convierte entero a string
; Entrada: RSI = valor, RDI = buffer
; Salida:  RAX = longitud
int_to_str:
    mov rax, rsi
    mov rbx, rdi
    mov rcx, 0
    cmp rax, 0
    jne .not_zero
    mov byte [rbx], '0'
    mov rax, 1
    ret

.not_zero:
    mov r8, 0
    cmp rax, 0
    jge .abs_ok
    neg rax
    mov r8, 1
.abs_ok:
    lea rdx, [rbx + 63]
    mov r9, rdx

.conv:
    xor rdx, rdx
    mov rcx, 10
    div rcx
    add dl, '0'
    mov [r9], dl
    dec r9
    inc rcx
    cmp rax, 0
    jne .conv

    cmp r8, 1
    jne .copy
    mov byte [r9], '-'
    dec r9
    inc rcx

.copy:
    inc r9
    mov rsi, r9
    mov rdi, rbx
    mov rdx, rcx
.copy_loop:
    cmp rdx, 0
    je .done_copy
    mov al, [rsi]
    mov [rdi], al
    inc rsi
    inc rdi
    dec rdx
    jmp .copy_loop

.done_copy:
    mov rax, rcx
    ret
