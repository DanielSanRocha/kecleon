.PHONY: help, build, boot, bootimage

help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

lint: # lint rust code
	cargo fmt

setup: # Creates the iso file to mount with EXT2
	dd if=/dev/zero of=disk.iso bs=1M count=16
	mkfs.ext2 disk.iso
	mkdir -p tmp
	sudo mount -o loop disk.iso tmp

clean: # Cleans the directory
	rm -rf kernel/*.o
	rm -f kernel-101
	rm -rf target/
	sudo umount tmp
	rm -rf tmp

build-i386: ## Builds the kernel and all the programs to the i386 architecture
	nasm -f elf32 kernel/main.asm -o kernel/main_asm.o
	nasm -f elf32 kernel/gdt.asm -o kernel/gdt_asm.o
	cargo build --target i686-unknown-linux-gnu
	i686-linux-gnu-ld -T kernel/link-i386.ld -o kernel-101 -Ltarget/i686-unknown-linux-gnu/debug kernel/main_asm.o kernel/gdt_asm.o -lmissigno

boot-i386: build-i386 ## Boots the kernel in a i386 machine
	qemu-system-i386 -kernel kernel-101 disk.iso

build-armv7: ## Builds the kernel and all the programs targetting armv7 architecture
	nasm -f elf32 kernel/main.asm -o kernel/main_asm.o
	nasm -f elf32 kernel/gdt.asm -o kernel/gdt_asm.o
	cargo build --target armv7a-none-eabi
	arm-none-eabi-ld -T kernel/link-armv7.ld -o kernel-101 -Ltarget/armv7a-none-eabi/debug kernel/main_asm.o kernel/gdt_asm.o -lmissigno

boot-armv7: build-armv7 ## Boot the kernel in a armv7 machine
	qemu-arm -kernel kernel-101 disk.iso