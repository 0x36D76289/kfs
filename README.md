# KFS - Kernel From Scratch

Un noyau de système d'exploitation minimal écrit en Rust, conçu pour l'apprentissage et l'expérimentation.

## Aperçu

KFS est un projet éducatif qui implémente un noyau bare-metal en Rust pour l'architecture x86_64. Il démontre les concepts fondamentaux du développement de systèmes d'exploitation, incluant la gestion des interruptions, les pilotes matériels et une interface utilisateur interactive.

## Fonctionnalités

- **Support x86_64** : Processus de démarrage complet avec configuration GDT et IDT
- **Mode texte VGA** : Sortie texte couleur avec contrôle du curseur
- **Clavier PS/2** : Pilote complet avec mappage scancode vers caractères
- **Gestion des interruptions** : Gestion appropriée des interruptions clavier et exceptions système
- **Shell interactif** : Interface en ligne de commande avec commandes de débogage
- **Sécurité mémoire** : Écrit en Rust avec no_std pour la programmation bare-metal

## Architecture

Le noyau est organisé en modules distincts :

### `arch/` - Code spécifique à l'architecture
- **x86_64/** : Implémentations spécifiques x86_64
  - `boot.s` : Code assembleur de démarrage avec en-tête multiboot
  - `gdt.rs` : Configuration de la Table de Descripteurs Globaux
  - `idt.rs` : Table de Descripteurs d'Interruption et gestionnaires d'exceptions

### `drivers/` - Pilotes matériels
- `vga_buffer.rs` : Pilote mode texte VGA
- `keyboard.rs` : Pilote clavier PS/2

### `kernel/` - Fonctionnalités cœur du noyau
- `memory.rs` : Gestion mémoire (placeholder pour futures fonctionnalités)
- `process.rs` : Gestion des processus (placeholder pour futures fonctionnalités)

### `utils/` - Fonctions utilitaires
- `kfs_lib.rs` : Fonctions de bibliothèque style C et printf

### `ui/` - Interface utilisateur
- `shell.rs` : Implémentation du shell interactif

## Compilation

### Prérequis
- Rust (version stable récente)
- QEMU pour les tests
- Outils GRUB pour la création d'ISO
- Outils LLVM (installés avec Rust)

### Commandes
```bash
# Installer les outils requis
make install-tools

# Compiler le noyau
make build

# Créer une ISO bootable
make iso

# Exécuter dans QEMU
make run

# Afficher toutes les commandes disponibles
make help
```

## Shell Commands

Le noyau inclut un shell interactif avec les commandes suivantes :

| Commande | Alias | Description |
|----------|-------|-------------|
| `help` | `h` | Afficher les commandes disponibles |
| `clear` | `cls` | Effacer l'écran |
| `stack` | `st` | Afficher les informations de pile du noyau |
| `callstack` | `cs` | Afficher la trace de pile d'appels |
| `gdt` | `gdtinfo` | Afficher les informations GDT |
| `test` | | Exécuter les tests de base du noyau |
| `screen` | | Afficher les informations d'écran |
| `reboot` | | Redémarrer le système |
| `halt` | | Arrêter le système |
| `exit` | `quit` | Quitter le shell |

## Développement

Ce noyau est conçu à des fins éducatives et démontre :

- Programmation Rust bare-metal
- Programmation système x86_64
- Gestion des interruptions
- Concepts de gestion mémoire
- Développement de pilotes

## Configuration Build

Le projet utilise :
- **Cargo** pour la gestion des dépendances Rust
- **Makefile** pour l'orchestration du build
- **Script linker personnalisé** (`build/linker.ld`)
- **Spécification de cible** (`build/x86_64-kfs.json`)

## Roadmap

### Implémenté ✅
- [x] Processus de démarrage x86_64
- [x] Configuration GDT/IDT
- [x] Pilote VGA
- [x] Pilote clavier PS/2
- [x] Shell interactif
- [x] Gestion des exceptions

### En cours de développement 🚧
- [ ] Correction de l'en-tête multiboot pour le lancement via ISO
- [ ] Gestion mémoire avancée
- [ ] Support multi-tasking

### Planifié 📋
- [ ] Système de fichiers
- [ ] Pile réseau
- [ ] Support multi-architecture

## Contribution

Ce projet est principalement éducatif. Pour contribuer :

1. Consultez `docs/DEVELOPMENT.md` pour les guidelines
2. Respectez les conventions de code Rust
3. Ajoutez des tests pour les nouvelles fonctionnalités
4. Documentez les nouvelles APIs

## Licence

Ce projet est à des fins éducatives.

## Ressources

- [Documentation API](docs/API.md)
- [Guide de développement](docs/DEVELOPMENT.md)
- [OSDev Wiki](https://wiki.osdev.org/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)