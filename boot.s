.extern main
.extern __RAM_START

.section .text.init
.global _start
_start:
    call _set_sp
    call _clear_bss

    call loader_main
    call __RAM_PROGRAM_START
    j .

.global _set_sp
_set_sp:
    la sp, __RAM_START
    addi sp,sp,1024
    mv a0,tp
.L_bump_sp:
    beqz a0, .L_end_set_sp
    addi sp,sp,1024
    addi a0,a0,-1
    j .L_bump_sp
.L_end_set_sp:
    ret

.global _clear_bss
_clear_bss:
.L_clear_init_done:
    la a0,init_done
    li a1,0
    sb a1,0(a0)
    bnez tp, .L_wait_for_init
.L_init_bss:
    la a0,__BSS_START
    la a1,__BSS_END
    li a2,0
.L_clear_next:
    sd a2,0(a0)
    bge a0,a1,.L_end_init_bss
    addi a0,a0,8
    j .L_clear_next
.L_end_init_bss:
    la a0,init_done
    li a1,1
    sb a1,0(a0)
    j .L_end_clear_bss
.L_wait_for_init:
    li a2,100
.L_wait_inner:
    beqz a2, .L_check_init
    addi a2,a2,-1
    j .L_wait_inner
.L_check_init:
    lb a1,0(a0)
    bnez a1, .L_end_clear_bss
    j .L_wait_for_init
.L_end_clear_bss:
    li a0,0
    li a1,0
    li a2,0
    ret

.section .bss
init_done: .byte 0
