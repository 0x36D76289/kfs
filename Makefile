KERNEL_NAME = kfs
ARCH = x86_64

TARGET_TRIPLE = $(ARCH)-$(KERNEL_NAME)
TARGET_JSON = $(TARGET_TRIPLE).json

BUILD_DIR = target/$(TARGET_TRIPLE)/debug
BOOT_DIR = boot
ISO_DIR = $(BUILD_DIR)/iso

KERNEL_BIN = $(BUILD_DIR)/$(KERNEL_NAME)
GRUB_CFG = $(BOOT_DIR)/grub.cfg
ISO_FILE = $(BUILD_DIR)/$(KERNEL_NAME).iso

CARGO = cargo
NASM = nasm
GRUB_MKRESCUE = grub-mkrescue
QEMU = qemu-system-x86_64

.PHONY: all build run clean test iso help

all: build

build:
	@rustup component add rust-src llvm-tools-preview 2>/dev/null || true
	@cargo install bootimage 2>/dev/null || true
	$(CARGO) build --target $(TARGET_JSON)

bootimage: build
	$(CARGO) bootimage --target $(TARGET_JSON)

iso: bootimage
	@mkdir -p $(ISO_DIR)/boot/grub
	@cp $(BUILD_DIR)/bootimage-$(KERNEL_NAME).bin $(ISO_DIR)/boot/$(KERNEL_NAME).bin
	@echo 'set timeout=0' > $(ISO_DIR)/boot/grub/grub.cfg
	@echo 'set default=0' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo 'menuentry "KFS" {' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '    multiboot /boot/$(KERNEL_NAME).bin' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '    boot' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '}' >> $(ISO_DIR)/boot/grub/grub.cfg
	$(GRUB_MKRESCUE) -o $(ISO_FILE) $(ISO_DIR)

run: bootimage
	$(CARGO) run --target $(TARGET_JSON)

run-iso: iso
	$(QEMU) -cdrom $(ISO_FILE) -m 32M

run-nogui: iso
	$(QEMU) -nographic -cdrom $(ISO_FILE) -m 32M

test:
	$(CARGO) test --target $(TARGET_JSON)

clean:
	$(CARGO) clean
	rm -rf $(ISO_DIR)
	rm -f $(ISO_FILE)

install-tools:
	rustup install nightly
	rustup default nightly
	rustup component add rust-src llvm-tools-preview
	cargo install bootimage

check-size: bootimage
	@echo "Checking kernel size..."
	@SIZE=$$(du -b $(BUILD_DIR)/bootimage-$(KERNEL_NAME).bin | cut -f1); \
	SIZE_MB=$$((SIZE / 1024 / 1024)); \
	echo "Kernel size: $$SIZE bytes ($$SIZE_MB MB)"; \
	if [ $$SIZE -gt 10485760 ]; then \
		echo "ERROR: Kernel exceeds 10MB limit!"; \
		exit 1; \
	else \
		echo "Size check: OK (under 10MB limit)"; \
	fi

debug: bootimage
	$(QEMU) -s -S -drive format=raw,file=$(BUILD_DIR)/bootimage-$(KERNEL_NAME).bin &

re: clean build

.PHONY: all build run clean test iso help install-tools check-size debug re