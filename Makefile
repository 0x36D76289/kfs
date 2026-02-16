SRC_DIR		= src
BUILD_DIR	= build
ISO_DIR		= $(BUILD_DIR)/isofiles

KERNEL		= $(BUILD_DIR)/kfs.bin
ISO			= kfs.iso

NASM		= nasm
LD			= x86_64-elf-ld
CARGO		= cargo

TARGET		= i686-kfs

ASM_SRC		= $(SRC_DIR)/boot.asm
ASM_OBJ		= $(BUILD_DIR)/boot.o
RUST_LIB	= target/$(TARGET)/release/libkfs.a

NASMFLAGS	= -f elf32
LDFLAGS		= -m elf_i386 -T linker.ld -nostdlib

all: $(ISO)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(ASM_OBJ): $(ASM_SRC) | $(BUILD_DIR)
	$(NASM) $(NASMFLAGS) $< -o $@

$(RUST_LIB): $(SRC_DIR)/lib.rs $(SRC_DIR)/vga.rs Cargo.toml
	$(CARGO) build --release --target $(TARGET).json

$(KERNEL): $(ASM_OBJ) $(RUST_LIB)
	$(LD) $(LDFLAGS) -o $@ $(ASM_OBJ) $(RUST_LIB)

$(ISO): $(KERNEL)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/kfs.bin
	cp grub.cfg $(ISO_DIR)/boot/grub/grub.cfg
	i686-elf-grub-mkrescue -o $@ $(ISO_DIR) 2>/dev/null || \
		grub-mkrescue -o $@ $(ISO_DIR) 2>/dev/null || \
		grub2-mkrescue -o $@ $(ISO_DIR) 2>/dev/null

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO)

debug: $(ISO)
	qemu-system-i386 -cdrom $(ISO) -d int,cpu_reset -no-reboot

run-kvm: $(ISO)
	qemu-system-i386 -cdrom $(ISO) -enable-kvm

clean:
	rm -rf $(BUILD_DIR)
	rm -f $(ISO)
	$(CARGO) clean

re: clean all

.PHONY: all run debug run-kvm clean re
