# Kecleon OS

<img src="resources/kecleon.png" alt="Kecleon" style="width: 250px; height: 250px"/>

A simple OS I made with Rust, C and assembly (nasm) to learn more about kernels targetting the raspberry pi 2. Run
```bash
make help
```
for a list of options.
This is a work in progress =)
Part of the code was copied from https://github.com/jsandler18/raspi-kernel. Check the tutorial https://jsandler18.github.io also.

## Setup

You need to have `Rust + Cargo` working in your machine which wil not be covered in this tutorial. You also will need `qemu`, ensure you have the command `qemu-system-arm` with at least version 7 in your machine.

After that, run
```bash
make setup
```

## Building the Kernel

If you want to just build the kernel without launching in `qemu` you can run
```bash
make build
```
this command will generate the kernel in a file `kernel.bin` in the root folder.

## Running in QEMU
Simply run
```bash
make boot
```

## Cleaning the project
To delete all the object files and other files generated during compilation,linking and image generation run
```bash
make clean
```

## Source Code
All the kernel related code is inside the `kernel` folder. Inside `programs` you can find simple programs writtten for the OS. The grub specification is located in `boot/grub.cfg`.


Made with Love ❤️❤️❤️ By Daniel Santana.
