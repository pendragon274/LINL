.code64
.section .kernel.text, "ax", @progbits

.align 4096

.globl KernelMain
KernelMain:
    endbr64

    movq $0x2f592f412f4b2f4f, %rax
    movq %rax, (0xb8000)

    hlt
