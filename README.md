# Number Viewer

A tiny Rust CLI that prints a number in multiple representations (bases, ASCII art, bit view, and a signed meter). Handy for quick sanity checks when switching between decimal/hex/binary or when explaining two's complement visually.

## Install & run

- With Rust installed: `cargo run -- <number>`
- Add the prefixes `0b` / `0o` / `0x` for binary, octal, and hex. Underscores are allowed for readability (`1_000_000`).
- `-h` / `--help` shows a brief usage summary.
- If no argument is provided, it defaults to `1337`.

## Examples

```bash
cargo run -- 42       # decimal
cargo run -- 0x2a     # hex
cargo run -- 0b101010 # binary
```

Expected output (abridged for `42`):

```
✨ Number viewer ✨
============ 

🎯 Input : 42
🧾 Value : 42

🔢 Bases
-----
Decimal : 42
Hex     : 0x2a
Octal   : 0o52
Binary  : 0b101010

🖼️ ASCII digits
------------
 ### 
#   #
#####
#   #
 ### 
```

The full output also includes:
- An e-based scientific flavor (`mantissa * e^exponent` and ln(|n|))
- A 32-bit two's-complement bit string with colored 1/0 markers
- A signed meter showing where the value sits across the i64 range
