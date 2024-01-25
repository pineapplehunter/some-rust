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


.macro set_sp
    la sp, __PROGRAM_END
    li s0, 1024*64
    add sp, sp, s0
    csrr a0, mhartid
1:
    beqz a0, 2f
    add sp,sp,s0
    addi a0,a0,-1
    j 1b
2:
.endm

.section .text.init
.global _start
_start:
    init_all_regs

    set_sp

    csrr tp, mhartid

    li a0, '0'
    csrr a1, mhartid
    add a0,a0,a1
    la a1, __UART_START
    sw a0, 0(a1)
    la a0, _trap
    csrw mtvec, a0

    call zero_bss
    mv a0, tp
    call init_rt
    call __RAM_PROGRAM_START
    j .

.macro get_value_offset_and_print_to_uart offset
    mv a2,a0
    srli a2, a2, \offset*4
    andi a2, a2, 0x0F
    li a3,10
    bge a2, a3, 1f
    addi a2,a2,'0'
    j 2f
1:
    addi a2, a2, -10
    addi a2, a2, 'A'
2:
    sw a2, 0(a1)
.endm

.macro print_64_bit_value
    la a1, __UART_START
    get_value_offset_and_print_to_uart 15
    get_value_offset_and_print_to_uart 14
    get_value_offset_and_print_to_uart 13
    get_value_offset_and_print_to_uart 12
    get_value_offset_and_print_to_uart 11
    get_value_offset_and_print_to_uart 10
    get_value_offset_and_print_to_uart 9
    get_value_offset_and_print_to_uart 8
    get_value_offset_and_print_to_uart 7
    get_value_offset_and_print_to_uart 6
    get_value_offset_and_print_to_uart 5
    get_value_offset_and_print_to_uart 4
    get_value_offset_and_print_to_uart 3
    get_value_offset_and_print_to_uart 2
    get_value_offset_and_print_to_uart 1
    get_value_offset_and_print_to_uart 0
    li a2, '\n'
    sw a2, 0(a1)
.endm

.macro print_64_bit_value_a0
    la a1, __UART_START
    li a2, 15
1:
    mv a3, a0
    mv a4, a2
    slli a4, a4, 2
    srl a3, a3, a4
    andi a3, a3, 0xF
    li a4, 10
    bge a3, a4, 3f
5:
    addi a3, a3, '0'
    j 4f
3:
    addi a3, a3, -10
    addi a3, a3, 'A'
4:
    sw a3, 0(a1)
    beqz a2, 2f
    addi a2,a2,-1
    j 1b
2:
    li a2, '\n'
    sw a2, 0(a1)
.endm

.macro print_char_lit char_reg
    mv a0, \char_lit
    la a1, __UART_START
    sw a0, 0(a1)
.endm

.macro print_str_ptr str_ptr_reg
    la a1, __UART_START
    mv a2, \str_ptr_reg
1:
    lbu a0, 0(a2)
    beqz a0, 2f
    sw a0, 0(a1)
    addi a2, a2, 1
    j 1b
2:
.endm

zero_bss:
    la a0, __BSS_START
    la a1, __BSS_END
    li a2, 0
0:
    blt a1, a0, 1f
    sd a2, 0(a0)
    addi a0,a0,8
    j 0b
1:
    ret

.align 4
.globl _trap
_trap:
    la a1, __UART_START
    li a2, '\n'
    sw a2, 0(a1)
    sw a2, 0(a1)
    sw a2, 0(a1)

    mv tp, a0

    la a0, mhartid_str
    print_str_ptr a0
    csrr a0, mhartid
    print_64_bit_value_a0
    csrr a0, mepc 
    print_64_bit_value_a0
    csrr a0, mcause
    print_64_bit_value_a0
    mv a0, sp
    print_64_bit_value_a0
    mv a0, ra
    print_64_bit_value_a0
    mv a0, tp
    print_64_bit_value_a0
    j .

.section .data
mhartid_str: .string "mhartid: \t"

.section .bss
init_done: .byte
THREAD_COUNT: .double
