	.globl WinMain
WinMain:
	call _main
	ret
	.globl _main
_main:
	pushq %rbp
	movq %rsp, %rbp
	subq $20, %rsp
	movl $250, %eax
	movl %eax, -8(%rbp)
	movl $5, %eax
	movl %eax, -12(%rbp)
	movl $2, -20(%rbp)
	movl $5, %eax
	cdq
	idiv -20(%rbp)
	movl %eax, -16(%rbp)
	movl -12(%rbp), %eax
	addl -16(%rbp), %eax
	movl %eax, -12(%rbp)
	movl -12(%rbp), %eax
	jmp block_end0
block_end0:
	movl %eax, -4(%rbp)
	movl $2, %eax
	movl %eax, -12(%rbp)
	movl -12(%rbp), %ecx
	movl %ecx, -16(%rbp)
	movl -4(%rbp), %eax
	cdq
	idiv -16(%rbp)
	movl %eax, -4(%rbp)
	movl $2, -20(%rbp)
	movl $35, %eax
	subl -20(%rbp), %eax
	movl %eax, -16(%rbp)
	movl -12(%rbp), %eax
	addl -16(%rbp), %eax
	movl %eax, -12(%rbp)
	movl -12(%rbp), %ecx
	movl %ecx, -16(%rbp)
	movl -4(%rbp), %eax
	imull -16(%rbp), %eax
	movl %eax, -4(%rbp)
	jmp block_end2
	movl $1, -16(%rbp)
	movl -4(%rbp), %eax
	addl -16(%rbp), %eax
	movl %eax, -4(%rbp)
block_end2:
	jmp block_end1
	movl $1, -16(%rbp)
	movl -4(%rbp), %eax
	addl -16(%rbp), %eax
	movl %eax, -4(%rbp)
block_end1:
	movl -4(%rbp), %eax
	jmp return0
return0:
	movq %rbp, %rsp
	popq %rbp
	ret
