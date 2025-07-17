.set ALIGN,    1<<0          # Align flag for multiboot
.set MEMINFO,  1<<1          # Memory info flag for multiboot
.set FLAGS,    ALIGN | MEMINFO # Combined flags for multiboot
.set MAGIC,    0x1BADB002    # Multiboot magic number
.set CHECKSUM, -(MAGIC + FLAGS) # Checksum for multiboot header

.section .multiboot
.align 4
.long MAGIC                  # Multiboot magic number
.long FLAGS                  # Multiboot flags
.long CHECKSUM               # Multiboot checksum

.section .bss
.align 16
stack_bottom:                # Bottom of the stack
.skip 16384                  # Reserve 16 KB for the stack
stack_top:                   # Top of the stack

.section .text
.global _start
.type _start, @function
_start:
    mov $stack_top, %esp     # Initialize stack pointer
    
    call kernel_main         # Call the kernel main function
    
    cli                      # Disable interrupts
1:  hlt                      # Halt the CPU
    jmp 1b                   # Infinite loop to prevent execution past this point

.size _start, . - _start

.global load_gdt
.type load_gdt, @function
load_gdt:
    mov 4(%esp), %eax        # Load the address of the GDT descriptor
    lgdt (%eax)              # Load the Global Descriptor Table
    
    mov $0x10, %ax           # Load the data segment selector
    mov %ax, %ds             # Set DS register
    mov %ax, %es             # Set ES register
    mov %ax, %fs             # Set FS register
    mov %ax, %gs             # Set GS register
    mov %ax, %ss             # Set SS register
    
    ljmp $0x08, $flush_cs    # Long jump to flush the code segment
flush_cs:
    ret                      # Return from the function
