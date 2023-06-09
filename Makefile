.PHONY: help, build, boot, bootimage, programs

PREFIX = arm-none-eabi

CC = $(PREFIX)-gcc
CC_PARAMS = -Wall -Werror -nostdlib -nostartfiles -fpic -mcpu=cortex-a7 -ffreestanding

AS = $(PREFIX)-as
AS_PARAMS = -mcpu=cortex-a7

LD = $(PREFIX)-ld
OBJCOPY = $(PREFIX)-objcopy
AR = $(PREFIX)-ar

CARGO = cargo
CARGO_TARGET = armv7a-none-eabi

help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

lint: # lint rust code
	cargo fmt

setup: # Mount disk.img on the tmp folder
	mkdir -p tmp
	dd if=/dev/zero of=disk.img bs=1M count=128
	mkfs.ext2 -b 4096 disk.img
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
	rm -rf programs/lib/*.o
	rm -rf programs/lib/*.a
	rm -rf programs/shell/*.o
	rm -rf programs/shell/*.elf

programs: # Build the programs (shell, lib)
	cd programs/lib   && $(AS) $(AS_PARAMS) syscalls.s -o syscalls_s.o
	cd programs/lib   && $(CC) $(CC_PARAMS) -c screen.c -o screen_c.o
	cd programs/lib   && $(CC) $(CC_PARAMS) -c process.c -o process_c.o
	cd programs/lib   && $(CC) $(CC_PARAMS) -c keyboard.c -o keyboard_c.o
	cd programs/lib   && $(AR) rvs libstd.a syscalls_s.o screen_c.o process_c.o keyboard_c.o

	cd programs/shell && $(AS) $(AS_PARAMS) start.s -o start_s.o
	cd programs/shell && $(CC) $(CC_PARAMS) -I../lib -c main.c -o main_c.o
	cd programs/shell && $(LD) -nostdlib -T link.ld start_s.o main_c.o -o shell.elf -L../lib -lstd

	cd programs/echo  && $(AS) $(AS_PARAMS) start.s -o start_s.o
	cd programs/echo  && $(CC) $(CC_PARAMS) -I../lib -c main.c -o main_c.o
	cd programs/echo  && $(LD) -nostdlib -T link.ld start_s.o main_c.o -o echo.elf -L../lib -lstd

install: # Generate the iso image used by qemu
	sudo mkdir -p tmp/boot
	sudo cp kernel.bin tmp/boot
	sudo mkdir -p tmp/bin

	sudo $(OBJCOPY) -O binary programs/shell/shell.elf tmp/bin/shell
	sudo $(OBJCOPY) -O binary programs/echo/echo.elf tmp/bin/echo

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
	$(LD) -nostdlib -T kernel/link.ld -o kernel.elf kernel/interrupts_s.o kernel/main_s.o kernel/memory_s.o kernel/framebuffer_c.o kernel/font_c.o kernel/mailbox_c.o kernel/stdlib_c.o kernel/emmc_c.o kernel/delays_c.o -Ltarget/$(CARGO_TARGET)/debug -lkecleon -Llib
	$(OBJCOPY) -O binary kernel.elf kernel.bin

boot: build programs install ## Boots the kernel in a raspi2b machine
	sync
	qemu-system-arm -cpu cortex-a7 -M raspi2b -kernel kernel.bin -sd disk.img -no-reboot -monitor telnet:127.0.0.1:1234,server,nowait -serial stdio -usb -device usb-kbd

debug: build programs install ## Starts qemu in debug mode (gdb)
	qemu-system-arm -s -S -d trace:bcm2835_* -cpu cortex-a7 -M raspi2b -kernel kernel.bin -sd disk.img -no-reboot -monitor telnet:127.0.0.1:1235,server,nowait -serial stdio -usb -device usb-kbd
