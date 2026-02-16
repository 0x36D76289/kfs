MBOOT_MAGIC         equ 0x1BADB002
MBOOT_PAGE_ALIGN    equ 1 << 0
MBOOT_MEM_INFO      equ 1 << 1
MBOOT_FLAGS         equ MBOOT_PAGE_ALIGN | MBOOT_MEM_INFO
MBOOT_CHECKSUM      equ -(MBOOT_MAGIC + MBOOT_FLAGS)
STACK_SIZE          equ 0x4000

global _start
global stack_bottom
global stack_top
extern kernel_main

section .multiboot
align 4
    dd MBOOT_MAGIC
    dd MBOOT_FLAGS
    dd MBOOT_CHECKSUM

section .bss
align 16
stack_bottom:
    resb STACK_SIZE
stack_top:

section .text
bits 32

_start:
    cli
    mov esp, stack_top
    push 0
    popf
    push ebx
    push eax
    call kernel_main
.hang:
    cli
    hlt
    jmp .hang
