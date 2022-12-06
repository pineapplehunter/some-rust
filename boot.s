.extern main
.extern __RAM_START

.section .text.init
.global _start
_start:
    beq tp,zero,TOMAIN
    j .
TOMAIN:
    la sp, __RAM_START
    addi sp, sp,1024

    call main
