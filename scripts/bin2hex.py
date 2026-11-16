#!/usr/bin/env python3

"""
bin2hex.py - flat binary -> word-addressed Intel HEX for a memory of configurable width.

The input is a little-endian byte stream (RISC-V). The record address is the
word index (byte_addr / (width/8)), and each word is written MSB first (Quartus convention).
"""

# Usage
# python3 bin2hex.py firmware.bin firmware.hex              # Default: 32-bit memory port width
# python3 bin2hex.py -w 64 firmware.bin firmware.hex        # 64-bit memory port width

import argparse


def rec(addr, rtype, payload):
    body = bytes([len(payload), (addr >> 8) & 0xFF, addr & 0xFF, rtype]) + payload
    return ":" + (body + bytes([(-sum(body)) & 0xFF])).hex().upper()


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("input", help="flat binary (llvm-objcopy -O binary)")
    ap.add_argument("output", help="Intel HEX file to write")
    ap.add_argument("-w", "--width", type=int, default=32,
                    help="memory port width in bits, multiple of 8 (default: 32)")
    ap.add_argument("-r", "--per-record", type=int, default=None,
                    help="words per HEX record (default: fill 16 bytes)")
    args = ap.parse_args()

    if args.width < 8 or args.width % 8:
        ap.error("--width must be a positive multiple of 8")
    wb = args.width // 8                          # bytes per memory word
    per = args.per_record or max(1, 16 // wb)     # words per record
    if per < 1 or per * wb > 255:
        ap.error("record would hold more than 255 bytes")

    with open(args.input, "rb") as f:
        data = f.read()
    data += b"\0" * (-len(data) % wb)             # pad to a whole word
    nwords = len(data) // wb
    if nwords > 1 << 32:
        ap.error("image too large for 32-bit HEX addressing")

    lines = []
    upper = 0                                     # current extended linear address
    w = 0
    while w < nwords:
        if (w >> 16) != upper:                    # crossed a 64K-word boundary
            upper = w >> 16
            lines.append(rec(0, 0x04, upper.to_bytes(2, "big")))
        # never let a record straddle a 64K boundary
        n = min(per, nwords - w, 0x10000 - (w & 0xFFFF))
        # little-endian input -> MSB-first output: reverse the bytes of each word
        payload = b"".join(
            data[(w + k) * wb:(w + k + 1) * wb][::-1] for k in range(n)
        )
        lines.append(rec(w & 0xFFFF, 0x00, payload))
        w += n
    lines.append(":00000001FF")

    with open(args.output, "w", newline="\r\n") as f:
        f.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
