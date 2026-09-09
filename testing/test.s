	.globl WinMain
WinMain:
	call _main
	ret
	.globl _main
_main:
	pushq %rbp
	movq %rsp, %rbp
	subq $4, %rsp
	movl $0, %eax
	movl %eax, -4(%rbp)
	movl -4(%rbp), %eax
	jmp return0
return0:
	movq %rbp, %rsp
	popq %rbp
	ret
