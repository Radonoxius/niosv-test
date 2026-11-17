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
Core 0 starts running from ROM, so initialize its boot ROM with the firmware:

```bash
cargo build-firmware-debug
./firmware2hex.sh
```

In Quartus Platform Designer, set the boot core's `nios_rom` initialization file to the generated `firmware.hex`. Regenerate the system, then compile the Quartus project and create the RBF as described above.

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
