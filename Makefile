ARCH := i386
TARGET := i686-unknown-linux-gnu
KERNEL_BIN := target/$(TARGET)/debug/kernel
ISO_DIR := isofiles
ISO_NAME := rust-kernel.iso

.PHONY: all clean run iso

all: $(KERNEL_BIN)

$(KERNEL_BIN): src/**/*.rs boot/boot.S boot/linker.ld
	cargo build --target $(TARGET)

run: $(KERNEL_BIN)
	qemu-system-i386 -kernel $(KERNEL_BIN)

run-iso: iso
	qemu-system-i386 -cdrom $(ISO_DIR)/$(ISO_NAME)

iso: $(KERNEL_BIN)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL_BIN) $(ISO_DIR)/boot/kernel.bin
	echo 'menuentry "KFS" {' > $(ISO_DIR)/boot/grub/grub.cfg
	echo '	multiboot /boot/kernel.bin' >> $(ISO_DIR)/boot/grub/grub.cfg
	echo '}' >> $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO_DIR)/$(ISO_NAME) $(ISO_DIR)

clean:
	cargo clean

fclean: clean
	rm -rf $(ISO_DIR)

re: fclean all

.PHONY: all clean run iso fclean re
.DEFAULT_GOAL := all