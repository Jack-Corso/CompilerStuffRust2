	.globl WinMain
WinMain:
	call _main
	ret
	.globl _main
_main:
	pushq %rbp
	movq %rsp, %rbp
	subq $8, %rsp
	movl $25, -8(%rbp)
	movl $0, %eax
	addl -8(%rbp), %eax
	jmp block_end0
block_end0:
	movl %eax, -4(%rbp)
	movl -4(%rbp), %eax
	jmp return0
return0:
	movq %rbp, %rsp
	popq %rbp
	ret
