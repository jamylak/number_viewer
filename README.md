# Number Viewer

A tiny Rust CLI that prints a number in multiple representations (bases, ASCII art, bit view, and a signed meter). Handy for quick sanity checks when switching between decimal/hex/binary or when explaining two's complement visually.

## Install & run

- Install once, run anywhere:
  - `cargo install --path .` (or `cargo install --locked --path .` for reproducible builds)
  - Then call `number_viewer 42` (defaults to `1337` when no argument is given)
- Prefer to run without installing? Use `cargo run -- <number>`
- `-h` / `--help` shows a brief usage summary
- Prefixes `0b` / `0o` / `0x` select binary, octal, and hex; underscores are fine (`1_000_000`)
- Floats are accepted too (`3.14`, `-2e5`) and show IEEE 754 internals

## Examples

```bash
cargo install --path .
number_viewer 42      # installed binary (preferred)
cargo run -- 42       # one-off via cargo
number_viewer 0x2a    # hex
number_viewer 0b101010 # binary
number_viewer 3.14159 # float
```

Expected output (abridged for `0x80999`):

```
❯ number_viewer 0x80999
✨ Number viewer ✨
============

🎯 Input : 0x80999
🧾 Value : 526745

🔢 Bases
-----
Decimal : 526745
Hex     : 0x80999
Octal   : 0o2004631
Binary  : 0b10000000100110011001

🧮 Base e flavor
-------------
Scientific (e): 5.267450e5
526745 = 1.190617 * e^13.000000
ln(|n|) ≈ 13.174472

🖼️ ASCII digits
------------
#####  ###   ###  ##### #   # #####
#     #   # #         # #   # #    
####     #  ####     #  ##### #### 
    #   #   #   #   #       #     #
####  #####  ###    #       # #### 

🧠 Bits (32-bit two's complement view)
----------------------------------
.... .... .... #... .... #..# #..# #..#
Legend: # = 1, . = 0

📏 Signed meter (relative to i64 range)
------------------------------------
|------------------------^------------------------|
```

Float example `3.14159`
```
❯ number_viewer 3.14159
✨ Number viewer ✨
============

🎯 Input : 3.14159
🧾 Value : 3.14159

🧪 Float value
-----------
Decimal    : 3.14159
Scientific : 3.141590e0
Hex bits   : 0x400921f9f01b866e

🧬 Float internals (IEEE 754 f64)
------------------------------
.|#.. .... ....|#..# ..#. ...# #### #..# #### .... ...# #.## #... .##. .##. ###. 
Legend: sign|exponent|fraction
Sign      : +
Category  : Normal
Exponent  : 1024 (biased)
Fraction  : 0x921f9f01b866e
Exponent  : 1 (unbiased)
Mantissa  : 1.570795000000
Value form: (-1)⁰ × (1 + 0.570795000000 ) × 2¹
```

The full output also includes:
- An e-based scientific flavor (`mantissa * e^exponent` and ln(|n|))
- A 32-bit two's-complement bit string with colored 1/0 markers
- A signed meter showing where the value sits across the i64 range
