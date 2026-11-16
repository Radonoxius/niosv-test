#! /bin/bash

./scripts/ocd.sh start

env -u LD_LIBRARY_PATH RUST_GDB=gdb-multiarch rust-gdb -x ./scripts/debug.gdb \
    target/riscv32i-unknown-none-elf/debug/niosv-firmware

./scripts/ocd.sh stop
