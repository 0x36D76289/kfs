section .multiboot_header
align 8
header_start:
    ; Magic value
    dd 0xe85250d6                ; Multiboot2 magic number
    dd 0                         ; Architecture (0 = i386)
    dd header_end - header_start ; Header length
    ; Checksum - must make magic + arch + length + checksum = 0
    dd -(0xe85250d6 + 0 + (header_end - header_start))
    
    ; End tag
    dw 0    ; Type
    dw 0    ; Flags
    dd 8    ; Size
header_end:

; Define the stack
section .bss
align 16
stack_bottom:
    resb 16384 ; 16 KiB
stack_top:

; Kernel entry point
section .text
global start
extern kmain

start:
    ; Set up stack
    mov esp, stack_top
    
    ; Clear interrupts during initialization
    cli
    
    ; Prepare CPU state for Rust
    ; - Clear direction flag (string operations increment pointers)
    cld
    
    ; Call the Rust main function
    call kmain
    
    ; If kmain ever returns (it shouldn't), just hang
.hang:
    hlt         ; Halt the CPU
    jmp .hang   ; If an NMI occurs, jump back to halt
