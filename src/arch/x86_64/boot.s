.set ALIGN,    1<<0             # align loaded modules on page boundaries
.set MEMINFO,  1<<1             # provide memory map
.set FLAGS,    ALIGN | MEMINFO  # this is the Multiboot 'flag' field
.set MAGIC,    0x1BADB002       # 'magic number' lets bootloader find the header
.set CHECKSUM, -(MAGIC + FLAGS) # checksum of above, to prove we are multiboot

# Multiboot header section - must be at address 0x00000800 as per subject
.section .multiboot_header
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

# Stack pour le kernel - 16KB comme spécifié dans la GDT
.section .bss
.align 16
stack_bottom:
.skip 16384 # 16 KiB kernel stack
stack_top:

# Point d'entrée du kernel
.section .text
.global _start
.type _start, @function
_start:
    # Setup de la stack kernel
    mov $stack_top, %esp
    
    # Push des informations Multiboot pour le kernel Rust
    push %ebx  # Multiboot info structure
    push %eax  # Multiboot magic number
    
    # Appel de la fonction kernel_main du kernel en Rust
    call kernel_main
    
    # Si le kernel retourne, on boucle indéfiniment
    cli
1:  hlt
    jmp 1b

.size _start, . - _start
