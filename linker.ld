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
}
