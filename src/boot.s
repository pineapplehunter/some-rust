.extern main
.extern __RAM_START

.macro init_all_regs
li x1,0
li x2,0
li x3,0
li x4,0
li x5,0
li x6,0
li x7,0
li x8,0
li x9,0
li x10,0
li x11,0
li x12,0
li x13,0
li x14,0
li x15,0
li x16,0
li x17,0
li x18,0
li x19,0
li x20,0
li x21,0
li x22,0
li x23,0
li x24,0
li x25,0
li x26,0
li x27,0
li x28,0
li x29,0
li x30,0
li x31,0
.endm

.section .text.init
.global _start
_start:
    init_all_regs
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
