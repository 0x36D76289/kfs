# KFS API Documentation

Cette documentation décrit l'API publique du noyau KFS organisée par modules.

## Architecture Module (`arch`)

### x86_64 Module

#### GDT (Global Descriptor Table) - `gdt.rs`

Gestion de la table de descripteurs globaux pour la segmentation x86_64.

```rust
// Fonctions publiques
pub fn init_gdt() -> ()
pub fn print_gdt_info() -> ()
pub fn print_kernel_stack() -> ()
pub fn print_call_stack() -> ()
pub fn get_selectors() -> (u16, u16, u16, u16) // (code, data, tss, user)
```

**Utilisation :**
```rust
use arch::x86_64::gdt;

// Initialiser la GDT
gdt::init_gdt();

// Afficher les informations GDT
gdt::print_gdt_info();
```

#### IDT (Interrupt Descriptor Table) - `idt.rs`

Gestion des interruptions et exceptions x86_64.

```rust
// Fonctions publiques
pub fn init_idt() -> ()
pub fn keyboard_interrupt_handler() -> ()

// Gestionnaires d'exceptions
pub fn divide_error_handler() -> ()
pub fn debug_handler() -> ()
pub fn nmi_handler() -> ()
pub fn breakpoint_handler() -> ()
pub fn overflow_handler() -> ()
pub fn bound_range_exceeded_handler() -> ()
pub fn invalid_opcode_handler() -> ()
pub fn device_not_available_handler() -> ()
pub fn double_fault_handler() -> ()
pub fn invalid_tss_handler() -> ()
pub fn segment_not_present_handler() -> ()
pub fn stack_segment_fault_handler() -> ()
pub fn general_protection_fault_handler() -> ()
pub fn page_fault_handler() -> ()
pub fn x87_floating_point_handler() -> ()
pub fn alignment_check_handler() -> ()
pub fn machine_check_handler() -> ()
pub fn simd_floating_point_handler() -> ()
pub fn virtualization_handler() -> ()
pub fn security_exception_handler() -> ()
```

## Drivers Module (`drivers`)

### VGA Buffer Driver - `vga_buffer.rs`

Pilote pour l'affichage texte VGA avec support couleur.

#### Énumérations

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

#### Fonctions

```rust
// Gestion de l'écran
pub fn clear_screen() -> ()
pub fn set_color(foreground: Color, background: Color) -> ()
pub fn reset_color() -> ()

// Gestion du curseur
pub fn set_cursor_position(row: usize, col: usize) -> ()
pub fn get_cursor_position() -> (usize, usize)
pub fn move_cursor_up() -> ()
pub fn move_cursor_down() -> ()
pub fn move_cursor_left() -> ()
pub fn move_cursor_right() -> ()

// Informations d'écran
pub fn get_screen_info() -> (usize, usize, usize, usize) // (rows, cols, current_row, current_col)
```

#### Macros

```rust
// Macros d'affichage
print!("Hello, {}!", "World");
println!("Line with newline");
```

**Exemple d'utilisation :**
```rust
use drivers::vga_buffer::{Color, set_color, clear_screen};

// Définir couleur rouge sur fond noir
set_color(Color::Red, Color::Black);
println!("Texte en rouge");

// Effacer l'écran
clear_screen();
```

### Keyboard Driver - `keyboard.rs`

Pilote clavier PS/2 avec gestion des scancodes.

```rust
// Fonctions publiques
pub fn init() -> ()
pub fn handle_keyboard_interrupt() -> ()
pub fn get_char() -> Option<char>
pub fn wait_for_char() -> char
pub fn read_line(buffer: &mut [u8]) -> usize
pub fn has_pending_input() -> bool
```

**Utilisation :**
```rust
use drivers::keyboard;

// Initialiser le pilote clavier
keyboard::init();

// Lire une ligne d'entrée
let mut buffer = [0u8; 256];
let len = keyboard::read_line(&mut buffer);
```

## Utils Module (`utils`)

### KFS Library - `kfs_lib.rs`

Bibliothèque de fonctions utilitaires style C.

#### Fonctions String

```rust
pub fn strlen(s: &str) -> usize
pub fn strcmp(s1: &str, s2: &str) -> i32
pub fn strcpy(dest: &mut [u8], src: &str) -> usize
pub fn strncpy(dest: &mut [u8], src: &str, n: usize) -> usize
pub fn strcat(dest: &mut [u8], src: &str) -> usize
pub fn strchr(s: &str, c: char) -> Option<usize>
pub fn strstr(haystack: &str, needle: &str) -> Option<usize>
```

#### Fonctions Memory

```rust
pub fn memset(ptr: *mut u8, value: u8, size: usize) -> *mut u8
pub fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8
pub fn memcmp(ptr1: *const u8, ptr2: *const u8, size: usize) -> i32
pub fn memmove(dest: *mut u8, src: *const u8, size: usize) -> *mut u8
```

#### Fonctions de Formatage

```rust
pub fn itoa(value: i32, buffer: &mut [u8], base: u32) -> usize
pub fn utoa(value: u32, buffer: &mut [u8], base: u32) -> usize
pub fn ltoa(value: i64, buffer: &mut [u8], base: u32) -> usize
pub fn ftoa(value: f64, buffer: &mut [u8], precision: usize) -> usize
```

#### Macros de Debug

```rust
// Macros utilitaires
kprintf!("Format: {}", value);
debug_print!(Color::Yellow, "Debug: {}", info);
error_print!("Error: {}", error_msg);
```

**Exemple d'utilisation :**
```rust
use utils::kfs_lib::*;

// Conversion entier vers string
let mut buffer = [0u8; 32];
let len = itoa(42, &mut buffer, 10);

// Copie de string
let mut dest = [0u8; 64];
strcpy(&mut dest, "Hello World");
```

## UI Module (`ui`)

### Shell - `shell.rs`

Interface shell interactive avec commandes de débogage.

#### Énumérations

```rust
pub enum ShellCommand {
    Help,
    Clear,
    Stack,
    CallStack,
    Gdt,
    Test,
    Screen,
    Reboot,
    Halt,
    Exit,
    Unknown(String),
}
```

#### Fonctions

```rust
pub fn start_shell() -> ()
pub fn handle_command(input: &str) -> bool
pub fn parse_command(input: &str) -> ShellCommand
pub fn execute_command(cmd: ShellCommand) -> bool
pub fn print_help() -> ()
pub fn switch_screen(screen_num: usize) -> ()
```

#### Commandes Disponibles

| Commande | Alias | Description |
|----------|-------|-------------|
| `help` | `h` | Affiche l'aide |
| `clear` | `cls` | Efface l'écran |
| `stack` | `st` | Affiche l'état de la pile |
| `callstack` | `cs` | Affiche la trace d'appels |
| `gdt` | `gdtinfo` | Affiche les informations GDT |
| `test` | | Exécute les tests |
| `screen` | | Affiche les informations d'écran |
| `reboot` | | Redémarre le système |
| `halt` | | Arrête le système |
| `exit` | `quit` | Quitte le shell |

## Kernel Module (`kernel`)

### Memory Management - `memory.rs`

**Note :** Module placeholder pour futures fonctionnalités.

```rust
// Fonctions prévues
pub fn init_memory_manager() -> ()
pub fn allocate_page() -> Option<*mut u8>
pub fn deallocate_page(ptr: *mut u8) -> ()
pub fn get_memory_info() -> (usize, usize) // (total, available)
```

### Process Management - `process.rs`

**Note :** Module placeholder pour futures fonctionnalités.

```rust
// Fonctions prévues
pub fn init_scheduler() -> ()
pub fn create_process(entry_point: fn()) -> ProcessId
pub fn schedule() -> ()
pub fn kill_process(pid: ProcessId) -> ()
```

## Gestion d'Erreur

Le noyau utilise le système de panic de Rust :

```rust
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Affiche les informations d'erreur
    // Affiche la trace de pile
    // Arrête le système de manière sûre
}
```

## Testing

Framework de test intégré :

```rust
#[cfg(test)]
mod tests {
    #[test_case]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Conventions

- Toutes les fonctions publiques sont documentées
- Les erreurs sont gérées avec `Result<T, E>` quand possible
- Les pointeurs raw sont utilisés uniquement pour l'interface hardware
- Les macros sont préférées pour les opérations de débogage

## Exemples d'Utilisation

### Initialisation Complète

```rust
use arch::x86_64::{gdt, idt};
use drivers::{vga_buffer, keyboard};
use ui::shell;

fn kernel_main() {
    // Initialiser l'architecture
    gdt::init_gdt();
    idt::init_idt();
    
    // Initialiser les pilotes
    vga_buffer::clear_screen();
    keyboard::init();
    
    // Démarrer le shell
    shell::start_shell();
}
```

### Gestion des Interruptions

```rust
// Le pilote clavier gère automatiquement les interruptions
// L'utilisateur peut simplement lire l'entrée
let c = keyboard::wait_for_char();
println!("Caractère reçu: {}", c);
```