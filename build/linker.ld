ENTRY(_start)

SECTIONS {
    . = 1M;

    .boot :
    {
        *(.multiboot_header)
    }

    . = 0x00000800;
    .gdt :
    {
        *(.gdt)
    }

    . = ALIGN(4K);
    .text :
    {
        *(.text)
    }

    .rodata :
    {
        *(.rodata)
    }

    .data :
    {
        *(.data)
    }

    .bss :
    {
        *(.bss)
    }

    .stack :
    {
        *(.stack)
    }
}
