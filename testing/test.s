	.globl WinMain
WinMain:
	call _main
	ret
	.globl _main
_main:
	pushq %rbp
	movq %rsp, %rbp
	subq $16, %rsp
	movl $5, %eax
	movl %eax, -8(%rbp)
	movl $2, -16(%rbp)
	movl $5, %eax
	cdq
	idiv -16(%rbp)
	movl %eax, -12(%rbp)
	movl -8(%rbp), %eax
	addl -12(%rbp), %eax
	movl %eax, -8(%rbp)
	movl -8(%rbp), %eax
	jmp block_end0
block_end0:
	movl %eax, -4(%rbp)
	movl $2, %eax
	movl %eax, -8(%rbp)
	movl -8(%rbp), %ecx
	movl %ecx, -12(%rbp)
	movl -4(%rbp), %eax
	cdq
	idiv -12(%rbp)
	movl %eax, -4(%rbp)
	movl $2, -16(%rbp)
	movl $35, %eax
	movl %eax, -12(%rbp)
	subl -16(%rbp), -12(%rbp)
	movl -8(%rbp), %eax
	addl -12(%rbp), %eax
	movl %eax, -8(%rbp)
	movl -8(%rbp), %ecx
	movl %ecx, -12(%rbp)
	movl -4(%rbp), %eax
	imull -12(%rbp), %eax
	movl %eax, -4(%rbp)
	jmp block_end2
	movl $1, -12(%rbp)
	movl -4(%rbp), %eax
	addl -12(%rbp), %eax
	movl %eax, -4(%rbp)
block_end2:
	jmp block_end1
	movl $1, -12(%rbp)
	movl -4(%rbp), %eax
	addl -12(%rbp), %eax
	movl %eax, -4(%rbp)
block_end1:
	movl -4(%rbp), %eax
	jmp return0
return0:
	movq %rbp, %rsp
	popq %rbp
	ret
