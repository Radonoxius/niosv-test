#!/usr/bin/env bash

# Usage: ./firmware2bin.sh [debug|release]  (defaults to debug)

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$SCRIPT_DIR"
cd "$ROOT_DIR"

PROFILE="${1:-debug}"
if [[ "$PROFILE" != debug && "$PROFILE" != release ]]; then
    echo "Usage: $0 [debug|release]" >&2
    exit 2
fi

TARGET_DIR="target/riscv32i-unknown-none-elf/$PROFILE"
MAIN_ELF="$TARGET_DIR/niosv-main"
SECONDARY_ELF="$TARGET_DIR/niosv-secondary"

# These offsets and slot sizes come from the FLASH regions in each memory.x:
# main is at 0x00000000, secondary at 0x00008000, each with a 32 KiB slot.
IMAGE_SIZE=65536
SLOT_SIZE=32768
SECONDARY_OFFSET=32768

for elf in "$MAIN_ELF" "$SECONDARY_ELF"; do
    if [[ ! -f "$elf" ]]; then
        echo "Missing firmware ELF: $elf (build the $PROFILE firmware first)" >&2
        exit 1
    fi
done

work_dir="$(mktemp -d "$ROOT_DIR/.firmware2bin.XXXXXX")"
trap 'python3 -c '\''import shutil,sys; shutil.rmtree(sys.argv[1], ignore_errors=True)'\'' "$work_dir"' EXIT

llvm-objcopy -O binary "$MAIN_ELF" "$work_dir/niosv-main.bin"
llvm-objcopy -O binary "$SECONDARY_ELF" "$work_dir/niosv-secondary.bin"

for bin in "$work_dir/niosv-main.bin" "$work_dir/niosv-secondary.bin"; do
    size="$(wc -c < "$bin")"
    if (( size > SLOT_SIZE )); then
        echo "Firmware image $bin is $size bytes; its flash slot is only $SLOT_SIZE bytes" >&2
        exit 1
    fi
done

# Erased flash reads as 0xff. Place each image at its FLASH origin and retain
# the full 64 KiB address space, including padding after each firmware.
head -c "$IMAGE_SIZE" /dev/zero | tr '\000' '\377' > "$work_dir/niosv.bin"
dd if="$work_dir/niosv-main.bin" of="$work_dir/niosv.bin" bs=4096 seek=0 conv=notrunc status=none
dd if="$work_dir/niosv-secondary.bin" of="$work_dir/niosv.bin" bs=4096 seek=$((SECONDARY_OFFSET / 4096)) conv=notrunc status=none

mv "$work_dir/niosv.bin" niosv.bin

echo "Wrote niosv.bin (64 KiB; main @ 0x0000, secondary @ 0x8000)"
