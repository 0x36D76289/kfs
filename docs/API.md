# KFS API Documentation

This documentation describes the public API of the KFS (Kernel From Scratch) kernel, organized by modules.

## Overview

KFS is a learning kernel written in Rust for the x86_64 architecture. It provides:

- **x86_64 Support**: GDT, IDT, interrupt management
- **Drivers**: VGA text mode, PS/2 keyboard
- **User Interface**: Interactive shell with debug commands
- **Utilities**: C-style functions and printf formatting
- **Testing**: Integrated testing framework

## Dependencies

The project uses the following crates:

```toml
[dependencies]
x86_64 = "0.14"          # x86_64 architecture support
volatile = "0.2"         # Volatile memory access
spin = "0.5"             # Spinlocks for synchronization
lazy_static = "1.0"      # Thread-safe static variables
bit_field = "0.10"       # Bit manipulation
bitflags = "1.3"         # Bit flags
```

## Architecture Module (`arch`)

### x86_64 Module

The x86_64 module provides support for the x86_64 architecture with centralized initialization.

#### Main Module Functions - `mod.rs`

```rust
pub use gdt::*;
pub use idt::*;

// Full initialization of the x86_64 architecture
pub fn init() -> ()
```

**Usage:**
```rust
use arch::x86_64;

// Initialize all x86_64 components
x86_64::init(); // Equivalent to gdt::init_gdt() + idt::init_idt()
```

#### GDT (Global Descriptor Table) - `gdt.rs`

Manages the global descriptor table for x86_64 segmentation.

```rust
// Public functions
pub fn init_gdt() -> ()
pub fn print_gdt_info() -> ()
pub fn print_kernel_stack() -> ()
pub fn print_call_stack() -> ()
pub fn get_selectors() -> &'static Selectors

// Structures
pub struct Selectors {
    pub kernel_code_selector: SegmentSelector,
    pub kernel_data_selector: SegmentSelector,
    pub user_code_selector: SegmentSelector,
    pub user_data_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}
```

**Usage:**
```rust
use arch::x86_64::gdt;

// Initialize the GDT
gdt::init_gdt();

// Display GDT information
gdt::print_gdt_info();

// Get selectors
let selectors = gdt::get_selectors();
println!("Code: {:?}", selectors.kernel_code_selector);
```

#### IDT (Interrupt Descriptor Table) - `idt.rs`

Manages x86_64 interrupts and exceptions.

```rust
// Public functions
pub fn init_idt() -> ()

// Interrupt handlers
pub extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: InterruptStackFrame) -> ()
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) -> ()
pub extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) -> ()
pub extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ()
pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) -> ()
pub extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ()
pub extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ()
pub extern "x86-interrupt" fn double_fault_handler_wrapper(stack_frame: InterruptStackFrame, error_code: u64) -> ()

// Internal functions
fn init_pics() -> ()
fn send_eoi(interrupt_id: u8) -> ()
```

## Drivers Module (`drivers`)

### VGA Buffer Driver - `vga_buffer.rs`

Driver for VGA text display with color support.

#### Enumerations

```rust
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
```

#### Functions

```rust
// Screen management
pub fn clear_screen() -> ()
pub fn set_color(foreground: Color, background: Color) -> ()

// Cursor management
pub fn set_cursor_position(row: usize, col: usize) -> ()
pub fn get_cursor_position() -> (usize, usize)
pub fn move_cursor_up() -> ()
pub fn move_cursor_down() -> ()
pub fn move_cursor_left() -> ()
pub fn move_cursor_right() -> ()

// Screen information
pub const BUFFER_HEIGHT: usize = 25
pub const BUFFER_WIDTH: usize = 80

// Internal structures
pub struct Writer {
    pub fn write_byte(&mut self, byte: u8) -> ()
    pub fn write_string(&mut self, s: &str) -> ()
    pub fn clear_screen(&mut self) -> ()
    pub fn set_color(&mut self, foreground: Color, background: Color) -> ()
    pub fn set_cursor_position(&mut self, row: usize, col: usize) -> ()
    pub fn get_cursor_position(&self) -> (usize, usize)
    pub fn move_cursor_up(&mut self) -> ()
    pub fn move_cursor_down(&mut self) -> ()
    pub fn move_cursor_left(&mut self) -> ()
    pub fn move_cursor_right(&mut self) -> ()
}
```

#### Macros

```rust
// Display macros
print!("Hello, {}!", "World");
println!("Line with newline");
```

**Example Usage:**
```rust
use drivers::vga_buffer::{Color, set_color, clear_screen};

// Set red text on black background
set_color(Color::Red, Color::Black);
println!("Text in red");

// Clear the screen
clear_screen();
```

### Keyboard Driver - `keyboard.rs`

PS/2 keyboard driver with scancode handling.

```rust
// Public functions
pub fn init() -> ()
pub fn handle_keyboard_interrupt() -> ()
pub fn get_char() -> Option<char>
pub fn wait_for_char() -> char
pub fn read_line(buffer: &mut [u8]) -> usize

// Structures
pub struct Keyboard {
    pub fn new() -> Self
    pub fn read_scancode(&mut self) -> Option<u8>
    pub fn handle_interrupt(&mut self) -> ()
    pub fn get_input_line(&self) -> &str
    pub fn clear_input_line(&mut self) -> ()
}

// Constants
pub const KEYBOARD_DATA_PORT: u16 = 0x60
pub const KEYBOARD_STATUS_PORT: u16 = 0x64
```

**Usage:**
```rust
use drivers::keyboard;

// Initialize the keyboard driver
keyboard::init();

// Read an input line
let mut buffer = [0u8; 256];
let len = keyboard::read_line(&mut buffer);
```

## Utils Module (`utils`)

### KFS Library - `kfs_lib.rs`

Library of C-style utility functions.

#### String Functions

```rust
pub fn strlen(s: *const u8) -> usize
pub fn strcmp(s1: *const u8, s2: *const u8) -> i32
pub fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8
pub fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8
```

#### Memory Functions

```rust
pub fn memset(ptr: *mut u8, value: u8, size: usize) -> *mut u8
pub fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8
pub fn memcmp(ptr1: *const u8, ptr2: *const u8, size: usize) -> i32
```

#### Formatting Functions

```rust
pub fn itoa(value: i32, buffer: &mut [u8], base: u32) -> &str
pub fn utoa(value: u32, buffer: &mut [u8], base: u32) -> &str
pub fn kprintf(format: &str, args: &[PrintfArg]) -> ()
pub fn debug_printf(color: Color, format: &str, args: &[PrintfArg]) -> ()
pub fn error_printf(format: &str, args: &[PrintfArg]) -> ()
pub fn info_printf(format: &str, args: &[PrintfArg]) -> ()
pub fn warn_printf(format: &str, args: &[PrintfArg]) -> ()
pub fn success_printf(format: &str, args: &[PrintfArg]) -> ()

// Enumeration for printf arguments
pub enum PrintfArg<'a> {
    Int(i32),
    Str(&'a str),
    Char(char),
    Ptr(*const u8),
}
```

#### Debug Macros

```rust
// Utility macros
kprintf!("Format: %s %d", "value", 42);
debug_print!("Debug info: %s", "test");
error_print!("Error: %s", "error message");
```

**Example Usage:**
```rust
use utils::kfs_lib::*;

// Integer to string conversion
let mut buffer = [0u8; 32];
let result = itoa(42, &mut buffer, 10);
println!("Number: {}", result);

// String copy
let src = b"Hello World\0";
let mut dest = [0u8; 64];
strcpy(dest.as_mut_ptr(), src.as_ptr());

// Using macros
kprintf!("Test: %s %d", "number", 42);
debug_print!("Debug message");
error_print!("Error occurred");
```

## UI Module (`ui`)

### Shell - `shell.rs`

Interactive shell interface with debugging commands.

#### Enumerations

```rust
#[derive(Debug)]
enum ShellCommand {
    Help,
    Clear,
    Reboot,
    Halt,
    Stack,
    CallStack,
    GdtInfo,
    Test,
    Exit,
    Screen,
    Unknown,
}
```

#### Functions

```rust
pub fn start_shell() -> ()
pub fn handle_command(cmd: &str) -> ()
pub fn show_prompt() -> ()
pub fn switch_screen(screen_num: u8) -> ()

// Structures
pub struct Shell {
    pub fn new() -> Self
    pub fn run(&mut self) -> ()
}
```

#### Available Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `help` | `h` | Displays help |
| `clear` | `cls` | Clears the screen |
| `stack` | `st` | Displays stack state |
| `callstack` | `cs` | Displays call stack trace |
| `gdt` | `gdtinfo` | Displays GDT information |
| `test` | | Runs tests |
| `screen` | | Displays screen information |
| `reboot` | `restart` | Reboots the system |
| `halt` | `shutdown` | Shuts down the system |
| `exit` | `quit` | Exits the shell |

#### Keyboard Shortcuts

| Shortcut | Description |
|----------|-------------|
| `Ctrl+L` | Clears the screen |
| `Ctrl+C` | Cancels the current line |
| `Ctrl+D` | Exits the shell |
| `Alt+1-9` | Switches between virtual screens |

## Kernel Module (`kernel`)

### Memory Management - `memory.rs`

**Note:** Placeholder module for future features.

```rust
// Planned functions
pub fn init_memory_manager() -> ()
pub fn allocate_page() -> Option<*mut u8>
pub fn deallocate_page(ptr: *mut u8) -> ()
pub fn get_memory_info() -> (usize, usize) // (total, available)
```

### Process Management - `process.rs`

**Note:** Placeholder module for future features.

```rust
// Planned functions
pub fn init_scheduler() -> ()
pub fn create_process(entry_point: fn()) -> ProcessId
pub fn schedule() -> ()
pub fn kill_process(pid: ProcessId) -> ()
```

## Error Handling

The kernel uses Rust's panic system with advanced x86_64 exception handling.

#### Panic Handler

```rust
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Displays error information with color
    // Shows stack trace and call stack
    // Safely halts the system
    loop {
        x86_64::instructions::hlt();
    }
}
```

#### Exception Handlers

Exception handlers provide detailed debugging information:

```rust
// Example: Page Fault Handler
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    
    // Displays debug information
    crate::arch::x86_64::gdt::print_kernel_stack();
    crate::arch::x86_64::gdt::print_call_stack();
    
    panic!("Page fault");
}
```

#### Managed Exception Types

- **Page Fault**: Invalid memory access
- **General Protection Fault**: General protection violation
- **Invalid Opcode**: Invalid instruction
- **Segment Not Present**: Segment not present
- **Stack Segment Fault**: Stack segment fault
- **Double Fault**: Double fault (uses a separate TSS stack)
- **Breakpoint**: Debug exception (int3)

## Testing

Integrated testing framework with support for unit and integration tests.

#### Test Attributes

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test_case]
    fn test_gdt_initialization() {
        let selectors = get_selectors();
        assert_ne!(selectors.kernel_code_selector.0, 0);
        assert_ne!(selectors.kernel_data_selector.0, 0);
        assert_ne!(selectors.tss_selector.0, 0);
    }
    
    #[test_case]
    fn test_keyboard_creation() {
        let keyboard = Keyboard::new();
        assert_eq!(keyboard.shift_pressed, false);
        assert_eq!(keyboard.ctrl_pressed, false);
        assert_eq!(keyboard.caps_lock, false);
    }
    
    #[test_case]
    fn test_breakpoint_exception() {
        // Triggers a breakpoint exception
        x86_64::instructions::interrupts::int3();
        // If we reach here, the exception was handled correctly
    }
}
```

#### Integration Tests via Shell

```rust
// Via the "test" command in the shell
shell> test
Running basic kernel tests...
  strlen test: length = 4
  kprintf test: number 42
  Testing breakpoint exception...
  Exception handling working correctly!
All tests passed!
```

## Constants and Configuration

### VGA Buffer

```rust
pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER: *mut Buffer = 0xb8000 as *mut Buffer;
```

### Keyboard

```rust
pub const KEYBOARD_DATA_PORT: u16 = 0x60;
pub const KEYBOARD_STATUS_PORT: u16 = 0x64;
const BUFFER_SIZE: usize = 64;
```

### GDT/IDT

```rust
pub const STACK_SIZE: usize = 4096 * 4;
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
const KEYBOARD_INTERRUPT_ID: u8 = 33;
```

### Hardware Ports

```rust
// PIC (Programmable Interrupt Controller)
const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

// VGA Cursor Control
const VGA_CURSOR_CMD: u16 = 0x3D4;
const VGA_CURSOR_DATA: u16 = 0x3D5;
```

## Conventions

- **Documentation**: All public functions are documented
- **Safety**: Errors are handled with `Result<T, E>` when possible
- **Hardware**: Raw pointers are used only for hardware interfaces
- **Debugging**: Macros are preferred for debugging operations
- **Formatting**: Supports printf-style with `kprintf!()` and Rust-style with `println!()`
- **Colors**: Uses VGA colors to differentiate message types
- **Testing**: `#[test_case]` attribute for unit tests
- **Interrupts**: Handlers with `extern "x86-interrupt"` signature
- **Lazy Static**: Uses `lazy_static!` for thread-safe global structures

## Usage Examples

### Full Initialization

```rust
use arch::x86_64;
use drivers::{vga_buffer, keyboard};
use ui::shell;

fn kernel_main() {
    // Initialize x86_64 architecture (GDT + IDT)
    x86_64::init();
    
    // Initialize drivers
    vga_buffer::clear_screen();
    keyboard::init();
    
    // Start the shell
    shell::start_shell();
}
```

### Using Interrupt Handlers

```rust
// Handlers are automatically installed by init_idt()
// Example of testing a breakpoint exception
x86_64::instructions::interrupts::int3(); // Triggers breakpoint_handler
```

### C-style Formatting with printf

```rust
use utils::kfs_lib::*;

// Using kprintf with C-style formatting
kprintf!("Hello %s, number: %d", "World", 42);

// Using colored debug macros
debug_print!("Debug message");
error_print!("Error occurred");
info_printf("Info: %s", &[PrintfArg::Str("test")]);
```