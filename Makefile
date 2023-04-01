.PHONY: help, build, boot, bootimage

help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

lint: # lint rust code
	cargo fmt

setup: # Mount disk.img on the tmp folder
	mkdir -p tmp
	sudo mount -o loop,offset=1048576 disk.img tmp

clean: # Cleans the directory
	sudo umount tmp || true
	rm -rf kernel/*.o
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
	arm-none-eabi-as -march=armv7-a -mcpu=cortex-a7 kernel/main.s -o kernel/main_s.o
	cross build --target arm-unknown-linux-gnueabi
	arm-none-eabi-ld -T kernel/link.ld -o kernel.elf kernel/main_s.o -Ltarget/arm-unknown-linux-gnueabi/debug -lkecleon
	arm-none-eabi-objcopy -O binary kernel.elf kernel.bin

boot: build install ## Boots the kernel in a arm machine
	qemu-system-arm -m 128M -M versatilepb -nographic -kernel kernel.bin -drive if=sd,cache=unsafe,file=disk.img -no-reboot -monitor telnet:127.0.0.1:1234,server,nowait -serial stdio

debug: build install ## Starts qemu in debug mode (gdb)
	qemu-system-arm -s -S -m 128M -M versatilepb -nographic -kernel kernel.bin -no-reboot -serial stdio

