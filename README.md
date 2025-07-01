# KFS - Global Descriptor Table & Stack

A kernel implementing a Global Descriptor Table (GDT) and stack management system according to the KFS project specifications.

## Shell Commands

- `help` - Display available commands
- `clear` - Clear the screen  
- `info` - Display system information (GDT, stack pointers)
- `stacktrace` - Show current kernel stack trace
- `stack` - Show detailed stack information with memory dump
- `gdt` - Display complete GDT information
- `gdttest` - Test GDT functionality and segment registers
- `trigger_panic` - Test panic handler with stack trace
- `reboot` - Reboot the system
- `shutdown` - Shutdown the system  
- `halt` - Halt the system
- `42` - Display the answer to everything

## Build Requirements

- Rust with `i386-unknown-none` target
- NASM (Netwide Assembler)
- GNU Linker (ld)
- GRUB utilities (grub-mkrescue)
- QEMU (for testing)

## Build Instructions

```bash
# Build the kernel
make

# Run in QEMU
qemu-system-i386 -cdrom kfs.iso -boot d -m 512

# Or use the test script  
./test_kernel.sh
```

Building with Make
```sh
# Build the kernel and create an ISO
make

# Run the kernel in QEMU
make run

# Debug the kernel
make debug
```

Building with Docker
```sh
# Build and run using Docker
make docker
```

## Project Structure
- `boot` - Bootloader assembly code and GRUB configuration
- `src` - Rust kernel source code
    - `io.rs` - Port I/O functions
    - `keyboard.rs` - Keyboard driver
    - `kmain.rs` - Kernel entry point
    - `screen.rs` - VGA text mode driver
    - `shell.rs` - Command line interface

## Development
KFS is designed to be educational and a starting point for OS development in Rust.