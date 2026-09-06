.code32
.section .pre_long_mode_kernel

# PreserveCore(void) -> CoreDump
.globl PreserveCore
PreserveCore:
    endbr32

    sub $CoreDump_sizeof, %esp                  # Allocate space for CoreDump struct.

    movl %eax, CoreDump_eax+4(%esp)             # Preserve eax into eax spot shifted up 4 to move ret pointer down.

    movl CoreDump_sizeof(%esp), %eax
    movl %eax, (%esp)                           # Move ret pointer down to stack top.

    lea CoreDump_sizeof+4(%esp), %eax
    movl %eax, CoreDump_esp+4(%esp)             # Move original esp into esp section of CoreDump struct.

    lea 4(%esp), %eax                           # Move pointer to new CoreDump struct into eax.

    movl %ebx, CoreDump_ebx(%eax)

    pop %ebx
    movl %ebx, CoreDump_eip(%eax)
    push %ebx

    movl %ecx, CoreDump_ecx(%eax)
    movl %edx, CoreDump_edx(%eax)
    movl %esi, CoreDump_esi(%eax)
    movl %edi, CoreDump_edi(%eax)
    movl %ebp, CoreDump_ebp(%eax)
    pushfl
    pop %ecx
    movl %ecx, CoreDump_efl(%eax)

    ret

# WriteCoreVGA(core: CoreDump*) -> void
.globl WriteCoreVGA
WriteCoreVGA:
    endbr32
    push %ebx
    mov %edi, %ebx

    mov $eax_txt, %edi
    call WriteStrVGA

    mov CoreDump_eax(%ebx), %edi
    call WriteIntAsHexVGA

    mov $spacer_txt, %edi
    call WriteStrVGA

    mov $ebx_txt, %edi
    call WriteStrVGA

    mov CoreDump_ebx(%ebx), %edi
    call WriteIntAsHexVGA

    mov $spacer_txt, %edi
    call WriteStrVGA

    mov $ecx_txt, %edi
    call WriteStrVGA

    mov CoreDump_ecx(%ebx), %edi
    call WriteIntAsHexVGA

    mov $spacer_txt, %edi
    call WriteStrVGA

    mov $edx_txt, %edi
    call WriteStrVGA

    mov CoreDump_edx(%ebx), %edi
    call WriteIntAsHexVGA

    mov $'\n', %edx
    call WriteCharVGA

    mov $esi_txt, %edi
    call WriteStrVGA

    mov CoreDump_esi(%ebx), %edi
    call WriteIntAsHexVGA

    mov $spacer_txt, %edi
    call WriteStrVGA

    mov $edi_txt, %edi
    call WriteStrVGA

    mov CoreDump_edi(%ebx), %edi
    call WriteIntAsHexVGA

    mov $spacer_txt, %edi
    call WriteStrVGA

    mov $ebp_txt, %edi
    call WriteStrVGA

    mov CoreDump_ebp(%ebx), %edi
    call WriteIntAsHexVGA

    mov $spacer_txt, %edi
    call WriteStrVGA

    mov $esp_txt, %edi
    call WriteStrVGA

    mov CoreDump_esp(%ebx), %edi
    call WriteIntAsHexVGA

    mov $'\n', %edx
    call WriteCharVGA

    mov $eip_txt, %edi
    call WriteStrVGA

    mov CoreDump_eip(%ebx), %edi
    call WriteIntAsHexVGA

    mov $spacer_txt, %edi
    call WriteStrVGA

    mov $efl_txt, %edi
    call WriteStrVGA

    mov CoreDump_efl(%ebx), %edi
    call WriteIntAsHexVGA

    call FlushBufferVGA

    pop %ebx
    ret

# DumpCore(core: CoreDump*) -> !
.globl DumpCore
DumpCore:
    call WriteCoreVGA
    hlt

# CoreDump struct
.struct 0
    CoreDump_eax:
.struct 4
    CoreDump_ebx:
.struct 8
    CoreDump_ecx:
.struct 12
    CoreDump_edx:
.struct 16
    CoreDump_esi:
.struct 20
    CoreDump_edi:
.struct 24
    CoreDump_ebp:
.struct 28
    CoreDump_esp:
.struct 32
    CoreDump_eip:
.struct 36
    CoreDump_efl:
.struct 40
    CoreDump_sizeof:

.section .pre_long_rodata
spacer_txt:
    .asciz " | "
eax_txt:
    .asciz "%eax = "
ebx_txt:
    .asciz "%ebx = "
ecx_txt:
    .asciz "%ecx = "
edx_txt:
    .asciz "%edx = "
esi_txt:
    .asciz "%esi = "
edi_txt:
    .asciz "%edi = "
ebp_txt:
    .asciz "%ebp = "
esp_txt:
    .asciz "%esp = "
eip_txt:
    .asciz "%eip = "
efl_txt:
    .asciz "%efl = "

.section .note.GNU-stack,"",@progbits
