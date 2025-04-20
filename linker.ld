ENTRY(start)

SECTIONS {
    . = 1M;
    
    .boot :
    {
        KEEP(*(.multiboot_header))
    }
    
    .text :
    {
        *(.text .text.*)
    }
    
    .bss :
    {
        *(.bss .bss.*)
    }
}
