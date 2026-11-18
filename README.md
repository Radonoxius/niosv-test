# Nios-V Test
A test design with Nios V/m soft-cores on Terasic DE10-Standard.

# Board Info
Terasic DE10-Standard board running Linux Kernel 6.18

# FPGA Usage
The FPGA is flashed at boot-time before booting into Linux automatically.

You will need to use Quartus Lite 25.1 to create an RBF file (use PPx16 and enable Compression in settings) from your SOF file.
Rename the generated file **strictly** as `fpga.rbf` and copy it to the boot partition on the SD card that the board will use.

If the RBF file doesnt exist or has a different name, the FPGA will **not** be flashed.

### Flashing Nios-V Firmware
Nios-V Core 0 starts running from ROM (its SRAM under the hood actually! :P):

First compile the `firmware-loader` program:
```bash
cargo build-firmware-loader
```
Place the executable in `/home` (or anywhere suitable) on the SD card.

After that, compile the firmware:
```bash
cargo build-firmware-debug
./firmware2bin.sh
```
The script turns the compiled firmware into `niosv.bin` for the ROM.
Once generated, place this binary blob in `/lib/firmware` on the SD card.

Run the `firmware-loader` program on the board to flash the firmware onto the Nios-V ROM and start the core.

If the binary doesnt exist, has a different name, or is put in the wrong location, the firmware will **not** be flashed.

# Project Requirements
This project requires:
+ `rust via rustup` (armv7-unknown-linux-gnueabihf & riscv32i-unknown-none-elf targets)
+ `clang`
+ `lld`
+ `llvm-objcopy`
+ `gdb (multiarch)`
+ `bash`
+ Included Sysroot

To work with the FPGA side, youll need to install Quartus Lite 25.1 (keep the defaults!).

**NOTE:** Nios-V CPU debug doesnt work if you havent installed Quartus!

# Installing Sysroot
The sysroot for the board has been provided in this repo to make things easier
for development. It was extracted from the Yocto Linux build (nodistro, full-cli image).

Just extract `cortexa9t2hf-neon-sysroot.tar.xz` in /opt. Youre good to
go after that!
