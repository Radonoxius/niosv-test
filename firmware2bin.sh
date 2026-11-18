#! /bin/bash

#llvm-objcopy -O binary target/riscv32i-unknown-none-elf/release/niosv-firmware niosv.bin
llvm-objcopy -O binary target/riscv32i-unknown-none-elf/debug/niosv-firmware niosv.bin
