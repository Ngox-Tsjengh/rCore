.altmacro
.set	REG_SIZE, 8
.set	CONTEXT_SIZE, 34

.macro	SAVE reg,offset
	sd	\reg, \offset*8(sp)
.endm

.macro	SAVE_N n
	SAVE x\n, \n
.endm

.macro	LOAD reg,offset
	ld	\reg, \offset*8(sp)
.endm

.macro	LOAD_N n
	LOAD x\n, \n
.endm

	
	.section .text
	.globl	__interrupt
__interrupt:

	addi	sp,sp,-34*8

	SAVE	x1,1
#save the original sp(x2)
	addi	x1,sp,34*8
	SAVE	x1,2

	.set	n,3
	.rept	29
		SAVE_N %n
		.set	n,n+1
	.endr

	csrr	s1,sstatus
	csrr	s2,sepc
	SAVE	s1,32	
	SAVE	s2,33

#call handle_interrupt
	mv		a0,sp
	csrr	a1,scause
	csrr	a2,stval

	.globl __restore
# 离开中断
# 从 Context 中恢复所有寄存器，并跳转至 Context 中 sepc 的位置
	__restore:
# 恢复 CSR
	LOAD    s1, 32
	LOAD    s2, 33
	csrw    sstatus, s1
	csrw    sepc, s2

# 恢复通用寄存器
	LOAD    x1, 1
# 恢复 x3 至 x31
	.set    n, 3
	.rept   29
	LOAD_N  %n
	.set    n, n + 1
	.endr

# 恢复 sp（又名 x2）这里最后恢复是为了上面可以正常使用 LOAD 宏
	LOAD    x2, 2
	sret

