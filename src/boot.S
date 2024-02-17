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
    la sp, __HEAP_END
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
    la a0, m_trap_vector
    csrw mtvec, a0

    mv a0, tp
    call init_rt
    call __RAM_PROGRAM_START
    j .


.altmacro
.set NUM_GP_REGS, 32  # Number of registers per context
.set NUM_FP_REGS, 32
.set REG_SIZE, 8   # Register size (in bytes)
.set MAX_CPUS, 8   # Maximum number of CPUs

# Use macros for saving and restoring multiple registers
.macro save_gp i, basereg=t6
	sd	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro load_gp i, basereg=t6
	ld	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro save_fp i, basereg=t6
	fsd	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm
.macro load_fp i, basereg=t6
	fld	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm

.align 4
.option norvc
m_trap_vector:
# All registers are volatile here, we need to save them
# before we do anything.
csrrw	t6, mscratch, t6
# csrrw will atomically swap t6 into mscratch and the old
# value of mscratch into t6. This is nice because we just
# switched values and didn't destroy anything -- all atomically!
# in cpu.rs we have a structure of:
#  32 gp regs		0
#  32 fp regs		256
#  SATP register	512
#  Trap stack       520
#  CPU HARTID		528
# We use t6 as the temporary register because it is the very
# bottom register (x31)
.set 	i, 1
.rept	30
	save_gp	%i
	.set	i, i+1
.endr

# Save the actual t6 register, which we swapped into
# mscratch
mv		t5, t6
csrr	t6, mscratch
save_gp 31, t5

# Restore the kernel trap frame into mscratch
csrw	mscratch, t5

# Get ready to go into Rust (trap.rs)
# We don't want to write into the user's stack or whomever
# messed with us here.
csrr	a0, mepc
csrr	a1, mtval
csrr	a2, mcause
csrr	a3, mhartid
csrr	a4, mstatus
mv		a5, t5
ld		sp, 520(a5)
call	m_trap

# When we get here, we've returned from m_trap, restore registers
# and return.
# m_trap will return the return address via a0.

csrw	mepc, a0

# Now load the trap frame back into t6
csrr	t6, mscratch

# Restore all GP registers
.set	i, 1
.rept	31
	load_gp %i
	.set	i, i+1
.endr

# Since we ran this loop 31 times starting with i = 1,
# the last one loaded t6 back to its original value.

mret
    
.section .data

.section .bss
init_done: .byte
THREAD_COUNT: .double
