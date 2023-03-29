# Kecleon OS

<img src="resources/kecleon.png" alt="Kecleon" style="width: 250px; height: 250px"/>

A simple OS I made with Rust, C and assembly (nasm) to learn more about kernels. Run
```bash
make help
```
for a list of options.
This is a work in progress =)

## Setup

You need to have `Rust + Cargo` working in your machine which wil not be covered in this tutorial. You also will need `qemu`, ensure you have the command `qemu-system-i386` in your machine. You also will need `grub-mkrescue` and consenquently `xorriso`. On Ubuntu you can install those with the commands:

```bash
sudo apt install qemu
sudo apt install xorriso
```
After that, run
```bash
make setup
```

## Building the Kernel

If you want to just build the kernel without launching in `qemu` you can run
```bash
make build-i386
```
this command will generate the kernel in a file `kernel.bin` in the root folder.

## Running in QEMU
Simply run
```bash
make boot-i386
```

## Cleaning the project
To delete all the object files and other files generated during compilation,linking and image generation run
```bash
make clean
```

## Source Code
All the kernel related code is inside the `kernel` folder. Inside `programs` you can find simple programs writtten for the OS. The grub specification is located in `boot/grub.cfg`.


Made with Love ❤️❤️❤️ By Daniel Santana.