.PHONY: help, build, boot, bootimage

PREFIX = arm-none-eabi

CC = $(PREFIX)-gcc
CC_PARAMS = -Wall -Werror -nostdlib -nostartfiles -fpic -mcpu=cortex-a7 -ffreestanding

AS = $(PREFIX)-as
AS_PARAMS = -mcpu=cortex-a7

LD = $(PREFIX)-ld
OBJCOPY = $(PREFIX)-objcopy

CARGO = cargo
CARGO_TARGET = armv7a-none-eabi

help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

lint: # lint rust code
	cargo fmt

setup: # Mount disk.img on the tmp folder
	mkdir -p tmp
	dd if=/dev/zero of=disk.img bs=1M count=32
	mkfs.ext2 -b 1024 disk.img
	sudo mount -o loop,offset=0 disk.img tmp

clean: # Cleans the directory
	sudo umount tmp || true
	rm -rf kernel/*.o
	rm -rf disk.img
	rm -rf target/
	sudo rm -rf tmp
	rm -f *.vdi
	rm -f *.bin
	rm -rf *.lock
	rm -rf *.iso
	rm -rf *.elf
	rm -f out.bochs

install: # Generate the iso image used by qemu
	sudo mkdir -p tmp/boot
	sudo cp kernel.bin tmp/boot

build: ## Builds the kernel targetting the armv7 architecture
	$(AS) $(AS_PARAMS) kernel/main.s -o kernel/main_s.o
	$(AS) $(AS_PARAMS) kernel/memory.s -o kernel/memory_s.o
	$(AS) $(AS_PARAMS) kernel/interrupts.s -o kernel/interrupts_s.o
	$(CC) $(CC_PARAMS) -c kernel/mailbox.c -o kernel/mailbox_c.o
	$(CC) $(CC_PARAMS) -c kernel/stdlib.c -o kernel/stdlib_c.o
	$(CC) $(CC_PARAMS) -c kernel/framebuffer.c -o kernel/framebuffer_c.o
	$(CC) $(CC_PARAMS) -c kernel/font.c -o kernel/font_c.o
	$(CC) $(CC_PARAMS) -c kernel/delays.c -o kernel/delays_c.o
	$(CC) $(CC_PARAMS) -c kernel/emmc.c -o kernel/emmc_c.o
	$(CARGO) build --target $(CARGO_TARGET)
	$(LD) -nostdlib -T kernel/link.ld -o kernel.elf kernel/interrupts_s.o kernel/main_s.o kernel/memory_s.o kernel/framebuffer_c.o kernel/font_c.o kernel/mailbox_c.o kernel/stdlib_c.o kernel/emmc_c.o kernel/delays_c.o -Ltarget/$(CARGO_TARGET)/debug -lkecleon
	$(OBJCOPY) -O binary kernel.elf kernel.bin

boot: build install ## Boots the kernel in a arm machine
	qemu-system-arm -cpu arm1176 -M raspi2b -kernel kernel.bin -sd disk.img -no-reboot -monitor telnet:127.0.0.1:1234,server,nowait -serial stdio

debug: build install ## Starts qemu in debug mode (gdb)
	qemu-system-arm -s -S -d trace:bcm2835_* -cpu arm1176 -M raspi2b -kernel kernel.bin -sd disk.img -no-reboot -serial stdio
