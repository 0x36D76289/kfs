# Guide de Développement KFS

Ce guide fournit toutes les informations nécessaires pour contribuer au développement du noyau KFS.

## Structure du Projet

La structure modulaire du projet KFS facilite la maintenance et l'évolution :

```
kfs/
├── src/                   # Code source principal
│   ├── arch/              # Code spécifique à l'architecture
│   │   ├── mod.rs         # Déclarations des modules d'architecture
│   │   └── x86_64/        # Implémentation x86_64
│   │       ├── mod.rs     # Déclarations des modules x86_64
│   │       ├── boot.s     # Code de démarrage assembleur
│   │       ├── gdt.rs     # Table de Descripteurs Globaux
│   │       └── idt.rs     # Gestion des interruptions
│   │
│   ├── drivers/           # Pilotes matériels
│   │   ├── mod.rs         # Déclarations des modules pilotes
│   │   ├── vga_buffer.rs  # Pilote VGA mode texte
│   │   └── keyboard.rs    # Pilote clavier PS/2
│   │
│   ├── kernel/            # Fonctionnalités cœur du noyau
│   │   ├── mod.rs         # Déclarations des modules noyau
│   │   ├── memory.rs      # Gestion mémoire (placeholder)
│   │   └── process.rs     # Gestion des processus (placeholder)
│   │
│   ├── utils/             # Fonctions utilitaires
│   │   ├── mod.rs         # Déclarations des modules utils
│   │   └── kfs_lib.rs     # Bibliothèque de fonctions C-style
│   │
│   ├── ui/                # Interface utilisateur
│   │   ├── mod.rs         # Déclarations des modules UI
│   │   └── shell.rs       # Implémentation du shell
│   │
│   ├── lib.rs             # Fichier de bibliothèque principal
│   └── main.rs            # Point d'entrée du noyau
│
├── build/                 # Configuration et scripts de compilation
│   ├── linker.ld          # Script du linkeur
│   └── x86_64-kfs.json    # Spécification de cible
│
├── docs/                  # Documentation
│   ├── API.md             # Documentation API
│   └── DEVELOPMENT.md     # Ce guide
│
├── .cargo/                # Configuration Cargo
│   └── config.toml        # Configuration du projet
│
├── Cargo.toml             # Configuration Rust
├── Makefile               # Système de compilation
└── README.md              # Vue d'ensemble du projet
```

## Ajout de Nouvelles Fonctionnalités

### Ajouter un Nouveau Pilote

1. **Créer le fichier du pilote**
   ```bash
   touch src/drivers/nouveau_pilote.rs
   ```

2. **Implémenter le pilote**
   ```rust
   // src/drivers/nouveau_pilote.rs
   
   /// Initialise le nouveau pilote
   pub fn init() {
       // Code d'initialisation
   }
   
   /// Fonction principale du pilote
   pub fn operation() -> Result<(), &'static str> {
       // Logique du pilote
       Ok(())
   }
   ```

3. **Exporter dans le module**
   ```rust
   // src/drivers/mod.rs
   pub mod nouveau_pilote;
   
   pub fn init_all_drivers() {
       nouveau_pilote::init();
   }
   ```

4. **Initialiser dans le noyau**
   ```rust
   // src/main.rs
   fn kernel_main() {
       drivers::nouveau_pilote::init();
   }
   ```

### Ajouter une Nouvelle Architecture

1. **Créer le répertoire d'architecture**
   ```bash
   mkdir -p src/arch/nouvelle_arch
   ```

2. **Implémenter les modules requis**
   ```rust
   // src/arch/nouvelle_arch/mod.rs
   pub mod boot;
   pub mod gdt;
   pub mod idt;
   
   pub fn init() {
       gdt::init_gdt();
       idt::init_idt();
   }
   ```

3. **Ajouter la compilation conditionnelle**
   ```rust
   // src/arch/mod.rs
   #[cfg(target_arch = "x86_64")]
   pub mod x86_64;
   
   #[cfg(target_arch = "nouvelle_arch")]
   pub mod nouvelle_arch;
   
   #[cfg(target_arch = "x86_64")]
   pub use x86_64::*;
   
   #[cfg(target_arch = "nouvelle_arch")]
   pub use nouvelle_arch::*;
   ```

### Ajouter une Commande Shell

1. **Étendre l'énumération des commandes**
   ```rust
   // src/ui/shell.rs
   pub enum ShellCommand {
       // ... commandes existantes
       NouvelleCommande,
   }
   ```

2. **Ajouter l'analyse de commande**
   ```rust
   fn parse_command(input: &str) -> ShellCommand {
       match input.trim().to_lowercase().as_str() {
           // ... cas existants
           "nouvelle" | "nv" => ShellCommand::NouvelleCommande,
           _ => ShellCommand::Unknown(input.to_string()),
       }
   }
   ```

3. **Implémenter l'exécution**
   ```rust
   fn execute_command(cmd: ShellCommand) -> bool {
       match cmd {
           // ... cas existants
           ShellCommand::NouvelleCommande => {
               execute_nouvelle_commande();
               true
           }
       }
   }
   
   fn execute_nouvelle_commande() {
       println!("Exécution de la nouvelle commande");
   }
   ```

4. **Mettre à jour l'aide**
   ```rust
   fn print_help() {
       println!("Commandes disponibles:");
       // ... aide existante
       println!("  nouvelle, nv    - Description de la nouvelle commande");
   }
   ```

## Conventions de Code

### Style Rust

- **Nommage** : Utiliser snake_case pour les fonctions et variables
- **Structs** : Utiliser PascalCase pour les types
- **Constantes** : Utiliser SCREAMING_SNAKE_CASE
- **Modules** : Utiliser snake_case pour les noms de modules

### Documentation

- **Fonctions publiques** : Documenter avec `///`
- **Modules** : Ajouter une documentation de module
- **Exemples** : Inclure des exemples d'utilisation

```rust
/// Initialise le pilote VGA buffer
/// 
/// Cette fonction configure le buffer VGA en mode texte 80x25
/// avec support couleur 16 bits.
/// 
/// # Exemples
/// 
/// ```
/// use drivers::vga_buffer;
/// 
/// vga_buffer::init();
/// println!("Hello, World!");
/// ```
pub fn init() {
    // Implémentation
}
```

### Gestion d'Erreur

- **Utiliser Result** pour les opérations qui peuvent échouer
- **Panic** uniquement pour les erreurs irrécupérables
- **Documenter** les conditions d'erreur

```rust
/// Lit un caractère du clavier
/// 
/// # Erreurs
/// 
/// Retourne `Err` si aucun caractère n'est disponible
pub fn get_char() -> Result<char, &'static str> {
    // Implémentation
}
```

## Système de Compilation

### Configuration Cargo

```toml
[package]
name = "kfs"
version = "0.1.0"
edition = "2021"

[dependencies]
# Pas de dépendances externes pour bare-metal

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

### Configuration Target

```json
{
  "llvm-target": "x86_64-kfs",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "arch": "x86_64",
  "target-endian": "little",
  "target-pointer-width": "64",
  "target-c-int-width": "32",
  "os": "none",
  "executables": true,
  "linker-flavor": "ld.lld",
  "linker": "rust-lld",
  "panic-strategy": "abort",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float"
}
```

### Makefile

```makefile
# Variables
KERNEL = target/x86_64-kfs/debug/kfs
ISO = target/kfs.iso

# Compilation
build:
	cargo build --target x86_64-kfs.json

# Exécution
run: build
	qemu-system-x86_64 -cdrom $(ISO)

# Création ISO
iso: build
	mkdir -p target/isofiles/boot/grub
	cp $(KERNEL) target/isofiles/boot/kernel.bin
	cp build/grub.cfg target/isofiles/boot/grub/
	grub-mkrescue -o $(ISO) target/isofiles

# Nettoyage
clean:
	cargo clean
	rm -rf target/isofiles
```

## Tests

### Framework de Test

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test_case]
    fn test_fonction() {
        // Logique de test
        assert_eq!(resultat_attendu, fonction_testee());
    }
}
```

### Tests d'Intégration

```rust
// tests/integration_test.rs
#![no_std]
#![no_main]

use kfs::drivers::vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_vga_buffer();
    loop {}
}

fn test_vga_buffer() {
    vga_buffer::clear_screen();
    println!("Test VGA buffer: OK");
}
```

## Débogage

### Outils de Débogage

1. **QEMU Monitor**
   ```bash
   qemu-system-x86_64 -monitor stdio -cdrom target/kfs.iso
   ```

2. **GDB avec QEMU**
   ```bash
   qemu-system-x86_64 -s -S -cdrom target/kfs.iso &
   gdb target/x86_64-kfs/debug/kfs
   (gdb) target remote :1234
   ```

3. **Commandes Shell de Debug**
   - `stack` : Affiche l'état de la pile
   - `callstack` : Trace d'appels
   - `gdt` : Information GDT

### Logging

```rust
// Utiliser les macros de debug
debug_print!(Color::Yellow, "Debug info: {}", value);
error_print!("Error: {}", error_message);
```

## Bonnes Pratiques

### Sécurité

- **Valider toutes les entrées** utilisateur
- **Vérifier les pointeurs** avant déréférencement
- **Gérer les cas d'erreur** explicitement
- **Éviter les débordements** de buffer

### Performance

- **Minimiser les allocations** dynamiques
- **Utiliser const fn** quand possible
- **Optimiser les chemins critiques**
- **Éviter les copies inutiles**

### Maintenance

- **Garder les fonctions petites** et focalisées
- **Séparer les responsabilités** par modules
- **Documenter les interfaces** publiques
- **Écrire des tests** pour les nouvelles fonctionnalités

## Workflow de Développement

1. **Fork et clone** le repository
2. **Créer une branche** pour la fonctionnalité
3. **Implémenter** la fonctionnalité avec tests
4. **Tester** localement avec `make test`
5. **Documenter** les changements
6. **Soumettre** une pull request

## Ressources

- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [OSDev Wiki](https://wiki.osdev.org/)
- [Intel x86-64 Manual](https://www.intel.com/content/www/us/en/architecture-and-technology/64-ia-32-architectures-software-developer-vol-3a-part-1-manual.html)
- [Rust Reference](https://doc.rust-lang.org/reference/)

## FAQ

### Q: Comment ajouter un nouveau type d'exception ?
A: Modifier `src/arch/x86_64/idt.rs` et ajouter le gestionnaire dans la table IDT.

### Q: Peut-on utiliser des bibliothèques externes ?
A: Non, le noyau utilise `#![no_std]` et ne peut pas utiliser la bibliothèque standard.

### Q: Comment déboguer un kernel panic ?
A: Utiliser la commande `callstack` du shell ou examiner la sortie QEMU.

### Q: Comment optimiser les performances ?
A: Profiler avec QEMU, optimiser les chemins critiques, et utiliser les optimisations Rust.