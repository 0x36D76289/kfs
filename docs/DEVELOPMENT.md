# KFS Development Guide

## Project Structure

The KFS project is organized as follows:

```
kfs/
├── src/
│   ├── arch/               # Architecture-specific code
│   │   └── x86_64/        # x86_64 implementations
│   │       ├── boot.s     # Assembly boot code
│   │       ├── gdt.rs     # Global Descriptor Table
│   │       ├── idt.rs # Interrupt handling
│   │       └── mod.rs     # Module declarations
│   ├── drivers/           # Hardware drivers
│   │   ├── vga_buffer.rs  # VGA text mode driver
│   │   ├── keyboard.rs    # PS/2 keyboard driver
│   │   └── mod.rs         # Module declarations
│   ├── kernel/            # Core kernel functionality
│   │   ├── memory.rs      # Memory management (placeholder)
│   │   ├── process.rs     # Process management (placeholder)
│   │   └── mod.rs         # Module declarations
│   ├── utils/             # Utility functions
│   │   ├── kfs_lib.rs     # C-style library functions
│   │   └── mod.rs         # Module declarations
│   ├── ui/                # User interface
│   │   ├── shell.rs       # Interactive shell
│   │   └── mod.rs         # Module declarations
│   ├── lib.rs             # Main library file
│   └── main.rs            # Kernel entry point
├── build/                 # Build scripts and configuration
├── docs/                  # Documentation
├── target/                # Compiled output
├── Cargo.toml            # Rust project configuration
├── Makefile              # Build system
└── README.md             # Project overview
```

## Adding New Features

### Adding a New Driver

1. Create a new file in `src/drivers/`
2. Implement the driver functionality
3. Export it in `src/drivers/mod.rs`
4. Initialize it in the `drivers::init()` function

### Adding Architecture Support

1. Create a new directory under `src/arch/`
2. Implement the required functions (init, gdt, idt, etc.)
3. Update `src/arch/mod.rs` to include the new architecture
4. Add appropriate conditional compilation

### Adding Shell Commands

1. Add the command to the `ShellCommand` enum in `src/ui/shell.rs`
2. Add the command parsing logic in `parse_command()`
3. Add the command execution logic in `execute_command()`
4. Implement the command function
5. Update the help text

## Code Style

- Follow Rust conventions
- Use descriptive variable names
- Add documentation comments for public functions
- Keep functions small and focused
- Use proper error handling

## Testing

The kernel includes a basic test framework:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test_case]
    fn test_example() {
        // Test code here
    }
}
```

Run tests with the test runner built into the kernel.

## Debugging

The kernel includes several debugging utilities:

- Stack trace printing
- GDT information display
- Memory dump capabilities
- Interrupt monitoring

Use the shell commands `stack`, `callstack`, and `gdt` for debugging.
