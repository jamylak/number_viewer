# Number Viewer

A tiny Rust CLI that prints a number in multiple representations (bases, ASCII art, bit view, and a signed meter). Handy for quick sanity checks when switching between decimal/hex/binary or when explaining two's complement visually.

## Install & run

- Install once, run anywhere:
  - `cargo install --path .` (or `cargo install --locked --path .` for reproducible builds)
  - Then call `number_viewer 42` (defaults to `1337` when no argument is given)
- Prefer to run without installing? Use `cargo run -- <number>`
- `-h` / `--help` shows a brief usage summary
- Prefixes `0b` / `0o` / `0x` select binary, octal, and hex; underscores are fine (`1_000_000`)

## Examples

```bash
cargo install --path .
number_viewer 42      # installed binary (preferred)
cargo run -- 42       # one-off via cargo
number_viewer 0x2a    # hex
number_viewer 0b101010 # binary
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
