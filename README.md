# KFS - Kernel From Scratch
KFS is a simple kernel written in Rust that implements basic functionality needed for an operating system.

## Features
- Custom bootloader using Multiboot2
- VGA text mode interface with multiple virtual screens
- PS/2 keyboard driver with US QWERTY layout support
- Simple shell with basic commands
- Memory-safe implementation using Rust

## Commands
The shell supports the following commands:

- `help` - Display available commands
- `clear` - Clear the screen
- `info` - Display system information
- `stacktrace` - Show kernel stack trace
- `reboot` - Reboot the system
- `halt` - Halt the system

## Usage
You can switch between multiple virtual screens using function keys F1-F4.

## Building
Requirements
- Rust (nightly)
- NASM assembler
- GRUB2 (for grub-mkrescue)
- xorriso (for ISO creation)
- QEMU (for testing)

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