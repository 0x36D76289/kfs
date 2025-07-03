# KFS - Kernel From Scratch

Un kernel minimal écrit en Rust selon les spécifications KFS_1.

## Description

Ce projet implémente un kernel minimal compatible avec GRUB qui peut :
- Démarrer via GRUB avec le standard Multiboot
- Afficher du texte à l'écran via VGA text mode
- Fournir des fonctions de base (strlen, strcmp, etc.)
- Gérer les exceptions CPU de base
- Afficher "42" comme requis par le sujet

## Spécifications respectées

✅ **Base**: Kernel bootable avec GRUB  
✅ **ASM Boot**: Code assembleur avec header Multiboot  
✅ **Kernel Library**: Fonctions de base et types  
✅ **Screen Interface**: Affichage VGA text mode  
✅ **Hello World**: Affichage de "42"  
✅ **Size Limit**: Moins de 10MB  
✅ **Architecture**: i386/x86_64  
✅ **Compilation**: Flags corrects et linker personnalisé  

## Prérequis

```bash
# Installation des outils Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Installation des composants nécessaires
rustup install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview

# Installation de bootimage
cargo install bootimage

# Outils système (Ubuntu/Debian)
sudo apt install qemu-system-x86 grub-pc-bin xorriso nasm

# Outils système (Arch Linux)
sudo pacman -S qemu grub xorriso nasm

# Outils système (macOS)
brew install qemu nasm
# Note: GRUB peut nécessiter une installation manuelle sur macOS
```

## Construction et exécution

### Construction du kernel

```bash
# Construction simple
make build

# Construction avec image bootable
make bootimage

# Construction avec ISO GRUB
make iso
```

### Exécution

```bash
# Exécution avec bootimage (recommandé)
make run

# Exécution avec ISO GRUB
make run-iso

# Vérification de la taille
make check-size
```

### Tests

```bash
# Exécution des tests
make test
```

### Debug

```bash
# Démarrage avec support GDB
make debug

# Dans un autre terminal :
gdb -ex 'target remote localhost:1234'
```

## Structure du projet

```
kfs/
├── src/
│   ├── main.rs              # Point d'entrée principal
│   ├── lib.rs               # Module principal
│   ├── boot.s               # Code assembleur de boot
│   ├── vga_buffer.rs        # Interface VGA
│   ├── interrupts.rs        # Gestion des interruptions
│   └── kfs_lib.rs          # Fonctions utilitaires
├── .cargo/
│   └── config.toml          # Configuration Cargo
├── x86_64-kfs.json         # Target personnalisé
├── linker.ld               # Script de linkage
├── Cargo.toml              # Configuration du projet
└── Makefile                # Build system
```

## Fonctionnalités implémentées

### Affichage VGA
- Interface sécurisée pour VGA text buffer
- Support des couleurs (16 couleurs)
- Scroll automatique
- Macros `print!` et `println!`

### Fonctions de base
- `strlen()` - Calcul de longueur de chaîne
- `strcmp()` - Comparaison de chaînes
- `strcpy()` - Copie de chaîne
- `memset()` - Remplissage mémoire
- `memcpy()` - Copie mémoire
- `memcmp()` - Comparaison mémoire

### Gestion des exceptions
- Table IDT configurée
- Handler pour breakpoint exception
- Support pour x86-interrupt calling convention

### Boot et Multiboot
- Header Multiboot valide
- Compatible GRUB
- Configuration de stack
- Transition assembleur → Rust

## Utilisation

Après construction et lancement, le kernel :

1. Initialise les systèmes de base
2. Configure l'affichage VGA
3. Affiche "42" (requis par le sujet)
4. Teste les fonctions de base
5. Entre en boucle infinie (kernel actif)

## Compatibilité

- **Architecture**: x86_64 (i386 compatible)
- **Bootloader**: GRUB (Multiboot)
- **Émulation**: QEMU, VirtualBox, VMware
- **Matériel**: PC x86 réel

## Développement

### Ajout de nouvelles fonctionnalités

1. Modifier les sources dans `src/`
2. Rebuilder avec `make build`
3. Tester avec `make run`

### Flags de compilation respectés

Selon les spécifications KFS_1 :
- `-fno-builtin` (équivalent Rust : `#![no_std]`)
- `-fno-exception` (équivalent Rust : `panic = "abort"`)
- `-fno-stack-protector` (géré par le target)
- `-nostdlib` (équivalent Rust : `#![no_std]`)
- `-nodefaultlibs` (géré par cargo)

## Notes importantes

- **Taille limite**: Le kernel final doit faire moins de 10MB
- **No STD**: Aucune dépendance à la bibliothèque standard
- **Bare Metal**: Exécution directe sur le matériel
- **Safety**: Code Rust sûr sauf pour l'accès au buffer VGA

## Dépannage

### Erreur de build
```bash
# Réinstaller les composants
make install-tools
make clean
make build
```

### QEMU ne démarre pas
```bash
# Vérifier l'installation QEMU
qemu-system-x86_64 --version

# Utiliser l'image bootimage
make run
```

### Erreur GRUB
```bash
# Utiliser bootimage au lieu de ISO
make run

# Vérifier les outils GRUB
grub-mkrescue --version
```

## Licence

Ce projet est développé dans le cadre des projets 42 School.
