ENTRY(start)

SECTIONS {
    . = 1M;

    .boot : ALIGN(4K)
    {
        KEEP(*(.multiboot_header))
    }

    .text BLOCK(4K) : ALIGN(4K)
    {
        *(.text)
    }

    .rodata BLOCK(4K) : ALIGN(4K)
    {
        *(.rodata)
    }

    .data BLOCK(4K) : ALIGN(4K)
    {
        *(.data)
    }

    .bss BLOCK(4K) : ALIGN(4K)
    {
        *(.bss)
    }

    . = 0x00000800;
    .gdt_space : {
        . = . + 0x100;  /* 256 bytes GDT */
    }
}
