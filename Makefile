KERNEL_NAME = kfs
ARCH = x86_64

TARGET_TRIPLE = $(ARCH)-$(KERNEL_NAME)
TARGET_JSON = build/$(TARGET_TRIPLE).json

BUILD_DIR = target/$(TARGET_TRIPLE)/debug
BOOT_DIR = boot
ISO_DIR = $(BUILD_DIR)/iso

KERNEL_ELF = $(BUILD_DIR)/$(KERNEL_NAME)
KERNEL_BIN = $(BUILD_DIR)/$(KERNEL_NAME).bin
GRUB_CFG = $(BOOT_DIR)/grub.cfg
ISO_FILE = $(BUILD_DIR)/$(KERNEL_NAME).iso

CARGO = cargo
NASM = nasm
OBJCOPY = llvm-objcopy
GRUB_MKRESCUE = grub-mkrescue
QEMU = qemu-system-x86_64

.PHONY: all build run clean test iso help

all: build

build:
	$(CARGO) build --target $(TARGET_JSON)

kernel-bin: build
	$(OBJCOPY) --strip-all -O binary $(KERNEL_ELF) $(KERNEL_BIN)

iso: kernel-bin
	@mkdir -p $(ISO_DIR)/boot/grub
	@cp $(KERNEL_BIN) $(ISO_DIR)/boot/$(KERNEL_NAME).bin
	@echo 'set timeout=2' > $(ISO_DIR)/boot/grub/grub.cfg
	@echo 'set default=0' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo 'menuentry "KFS" {' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '    multiboot /boot/$(KERNEL_NAME).bin' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '    boot' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '}' >> $(ISO_DIR)/boot/grub/grub.cfg
	$(GRUB_MKRESCUE) -o $(ISO_FILE) $(ISO_DIR)

run: iso
	$(QEMU) -cdrom $(ISO_FILE) -m 32M

run-iso: iso
	$(QEMU) -cdrom $(ISO_FILE) -m 32M

run-nogui: iso
	$(QEMU) -nographic -cdrom $(ISO_FILE) -m 32M

test:
	$(CARGO) test --target $(TARGET_JSON)

clean:
	$(CARGO) clean
	rm -rf $(ISO_DIR)

fclean: clean
	rm -f $(ISO_FILE)

install-tools:
	rustup install nightly
	rustup default nightly
	rustup component add rust-src llvm-tools-preview

check-size: kernel-bin
	@echo "Checking kernel size..."
	@SIZE=$$(du -b $(KERNEL_BIN) | cut -f1); \
	SIZE_MB=$$((SIZE / 1024 / 1024)); \
	echo "Kernel size: $$SIZE bytes ($$SIZE_MB MB)"; \
	if [ $$SIZE -gt 10485760 ]; then \
		echo "ERROR: Kernel exceeds 10MB limit!"; \
		exit 1; \
	else \
		echo "Size check: OK (under 10MB limit)"; \
	fi

debug: kernel-bin
	$(QEMU) -s -S -drive format=raw,file=$(KERNEL_BIN) &

re: clean build

help:
	@echo "Available targets:"
	@echo "  build       - Build the kernel"
	@echo "  kernel-bin  - Build kernel and create binary"
	@echo "  iso         - Create bootable ISO"
	@echo "  run         - Run kernel in QEMU"
	@echo "  run-iso     - Run ISO in QEMU"
	@echo "  run-nogui   - Run without GUI"
	@echo "  test        - Run tests"
	@echo "  clean       - Clean build artifacts"
	@echo "  fclean      - Full clean including ISO"
	@echo "  debug       - Start QEMU with GDB server"
	@echo "  check-size  - Check kernel size"
	@echo "  install-tools - Install required tools"
	@echo "  re          - Clean and rebuild"
	@echo "  help        - Show this help"

.PHONY: all build run clean test iso help install-tools check-size debug re kernel-bin