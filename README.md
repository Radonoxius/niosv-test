# Nios-V Test
A test design with Nios V/m soft-cores on Terasic DE10-Standard.

# Board Info
Terasic DE10-Standard board running Linux Kernel 6.18

# FPGA Usage
The FPGA is flashed at boot-time before booting into Linux automatically.

You will need to use Quartus Lite 25.1 to create an RBF file (use PPx16 and enable Compression in settings) from your SOF file.
Rename the generated file **strictly** as `fpga.rbf` and copy it to the boot partition on the SD card that the board will use.

If the RBF file doesnt exist or has a different name, the FPGA will **not** be flashed.

## Loading Nios V Firmware into ROM
The Nios V firmware runs from the `nios_rom` on-chip memory at address `0x00000000`. Build the firmware for the bare-metal RISC-V target and convert it to the 512-bit Intel HEX format expected by that ROM:

```bash
cargo build-firmware-debug
./firmware2hex.sh
```

The `build-firmware-debug` Cargo alias is defined in `.cargo/config.toml`; it builds the debug firmware binary at `target/riscv32i-unknown-none-elf/debug/niosv-firmware`, which `firmware2hex.sh` converts into `firmware.hex` in the project root. ROM initialization is enabled for `nios_rom`. In Quartus Platform Designer, set the `nios_rom` on-chip memory **Initialization file** to the generated `firmware.hex` (the file must be present). Use the file path in your checkout; do not leave a path from another machine in the system configuration. Regenerate the Platform Designer system, then compile the Quartus project to produce the SOF/RBF containing the initialized ROM. Copy the resulting `fpga.rbf` to the board's boot partition as described above.

Repeat the build, HEX conversion, Platform Designer regeneration, and Quartus compilation whenever the firmware changes. The generated HEX file is incorporated into the FPGA configuration through the ROM initialization setting; it is not loaded by the Linux program at runtime.

# Project Requirements
This project requires:
+ `rust via rustup` (armv7-unknown-linux-gnueabihf & riscv32i-unknown-none-elf targets)
+ `clang`
+ `lld`
+ `bash`
+ `python3`
+ `gdb (multiarch)`

To work with the FPGA side, youll need to install Quartus Lite 25.1 (keep the defaults!)

**NOTE:** Nios-V debug doesnt work if you havent installed Quartus!

# Installing Sysroot
The sysroot for the board has been provided in this repo to make things easier
for development. It was extracted from the Yocto Linux build (nodistro, full-cli image).

Just extract `cortexa9t2hf-neon-sysroot.tar.xz` in /opt. Youre good to
go after that!
