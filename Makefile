.PHONY: help, build, boot, bootimage

help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

lint: # lint rust code
	cargo fmt

setup: # Creates the iso file to mount with EXT2
	mkdir -p tmp/boot/grub

clean: # Cleans the directory
	rm -rf kernel/*.o
	rm -f kernel-101
	rm -rf target/
	rm -rf tmp
	rm -f disk.img

install: # Generate the iso image used by qemu
	cp boot/grub.cfg tmp/boot/grub/
	cp kernel.bin tmp/boot
	grub-mkrescue -o disk.img tmp

programs-i386: # Build and copy the programs to the loopback device
	mkdir -p tmp/bin
	gcc programs/shell/main.c -o programs/shell/shell
	cp programs/shell/shell tmp/bin/shell

build-i386: programs-i386 ## Builds the kernel and all the programs to the i386 architecture
	nasm -f elf32 kernel/main.asm -o kernel/main_asm.o
	nasm -f elf32 kernel/gdt.asm -o kernel/gdt_asm.o
	nasm -f elf32 kernel/idt.asm -o kernel/idt_asm.o
	i686-linux-gnu-gcc -g -O -c kernel/memory.c -o kernel/memory_c.o
	i686-linux-gnu-gcc -g -O -c kernel/idt.c -o kernel/idt_c.o
	i686-linux-gnu-gcc -g -O -c kernel/gdt.c -o kernel/gdt_c.o
	cargo build --target i686-unknown-linux-gnu
	i686-linux-gnu-ld -T kernel/link-i386.ld -o kernel.bin -Ltarget/i686-unknown-linux-gnu/debug kernel/main_asm.o kernel/gdt_asm.o kernel/idt_asm.o kernel/idt_c.o kernel/memory_c.o kernel/gdt_c.o -lkecleon

boot-i386: build-i386 install ## Boots the kernel in a i386 machine
	qemu-system-i386 -drive format=raw,file=disk.img -d int,cpu_reset -no-reboot

debug-i386: build-i386 install ## Starts qemu in debug mode (gdb)
	qemu-system-i386 -s -S -drive format=raw,file=disk.img -d int,cpu_reset -no-reboot

build-armv7: ## Builds the kernel and all the programs targetting armv7 architecture
	nasm -f elf32 kernel/main.asm -o kernel/main_asm.o
	cargo build --target armv7a-none-eabi
	arm-none-eabi-ld -T kernel/link-armv7.ld -o kernel.bin -Ltarget/armv7a-none-eabi/debug kernel/main_asm.o -lkecleon

boot-armv7: build-armv7 ## Boot the kernel in a armv7 machine
	qemu-arm -drive format=raw,file=disk.img
