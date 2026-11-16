#! /bin/bash

#llvm-objcopy -O binary target/riscv32i-unknown-none-elf/release/niosv-firmware firmware.bin
llvm-objcopy -O binary target/riscv32i-unknown-none-elf/debug/niosv-firmware firmware.bin

python3 scripts/bin2hex.py -w 512 firmware.bin firmware.hex
rm firmware.bin
