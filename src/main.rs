use std::env;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
// const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const BIT_ONE: &str = "\x1b[32m#\x1b[0m";
const BIT_ZERO: &str = "\x1b[2m.\x1b[0m";
const EDGE_MARK: &str = "\x1b[2m|\x1b[0m";
const ZERO_MARK: &str = "\x1b[34m|\x1b[0m";
const POINTER_MARK: &str = "\x1b[33m^\x1b[0m";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if matches!(args.get(0).map(String::as_str), Some("-h" | "--help")) {
        print_help();
        return;
    }

    let default = 1337_i64;
    let (raw, number) = match args.get(0) {
        Some(raw) => match parse_number(raw) {
            Ok(n) => (raw.clone(), n),
            Err(err) => {
                eprintln!("Could not parse '{raw}': {err}");
                eprintln!("Try plain decimals or prefixes 0b / 0o / 0x");
                std::process::exit(1);
            }
        },
        None => (default.to_string(), default),
    };

    println!("{BOLD}{MAGENTA}✨ Number viewer ✨{RESET}");
    println!("{MAGENTA}============{RESET}\n");
    println!("🎯 Input : {}", colorize(raw.as_str(), YELLOW));
    println!("🧾 Value : {}", colorize(number, GREEN));
    println!();

    print_bases(number);
    println!();
    print_base_e(number);
    println!();
    print_ascii_banner(number);
    println!();
    print_bits(number);
    println!();
    print_meter(number);
}

fn parse_number(raw: &str) -> Result<i64, String> {
    let cleaned: String = raw.chars().filter(|c| *c != '_').collect();
    let s = cleaned.as_str();
    let (base, digits) = if let Some(rest) = s.strip_prefix("0b") {
        (2, rest)
    } else if let Some(rest) = s.strip_prefix("0o") {
        (8, rest)
    } else if let Some(rest) = s.strip_prefix("0x") {
        (16, rest)
    } else {
        return s
            .parse::<i64>()
            .map_err(|e| format!("invalid decimal: {e}"));
    };

    i64::from_str_radix(digits, base).map_err(|e| format!("invalid base {base}: {e}"))
}

fn print_help() {
    println!("Usage: number_viewer [NUMBER]");
    println!("Show a number in multiple bases plus some ASCII visuals.");
    println!("Accepted forms: decimal (42), binary (0b1010), octal (0o52), hex (0x2a).");
    println!("If omitted, defaults to 1337.");
}

fn print_bases(n: i64) {
    section("Bases", "🔢", "-----");
    println!("Decimal : {}", colorize(n, GREEN));
    println!("Hex     : {}", colorize(format!("0x{:x}", n), BLUE));
    println!("Octal   : {}", colorize(format!("0o{o:o}", o = n), YELLOW));
    println!("Binary  : {}", colorize(format!("0b{:b}", n), CYAN));
}

fn print_base_e(n: i64) {
    let f = n as f64;
    section("Base e flavor", "🧮", "-------------");
    println!("Scientific (e): {}", colorize(format!("{f:.6e}"), BLUE));
    if n != 0 {
        let sign = if n < 0 { "-" } else { "" };
        let magnitude = (n.abs()) as f64;
        let ln_n = magnitude.ln();
        let exponent = ln_n.floor();
        let mantissa = (ln_n - exponent).exp();
        println!(
            "{n} = {sign}{} * e^{}",
            colorize(format!("{mantissa:.6}"), GREEN),
            colorize(format!("{exponent:.6}"), MAGENTA)
        );
        println!("ln(|n|) ≈ {}", colorize(format!("{ln_n:.6}"), BLUE));
    } else {
        println!("ln(0) is -infinity; sticking with zero here.");
    }
}

fn print_ascii_banner(n: i64) {
    section("ASCII digits", "🖼️", "------------");
    let banner = ascii_digits(n);
    println!("{BLUE}{banner}{RESET}");
}

fn ascii_digits(n: i64) -> String {
    const DIGITS: [[&str; 5]; 10] = [
        [" ### ", "#   #", "#   #", "#   #", " ### "],
        ["  #  ", " ##  ", "  #  ", "  #  ", " ### "],
        [" ### ", "#   #", "   # ", "  #  ", "#####"],
        [" ### ", "    #", " ### ", "    #", " ### "],
        ["#   #", "#   #", "#####", "    #", "    #"],
        ["#####", "#    ", "#### ", "    #", "#### "],
        [" ### ", "#    ", "#### ", "#   #", " ### "],
        ["#####", "    #", "   # ", "  #  ", "  #  "],
        [" ### ", "#   #", " ### ", "#   #", " ### "],
        [" ### ", "#   #", " ####", "    #", " ### "],
    ];
    const MINUS: [&str; 5] = ["     ", " --- ", "     ", "     ", "     "];

    let text = n.to_string();
    let mut lines = vec![String::new(); 5];
    for ch in text.chars() {
        let glyph = match ch {
            '-' => MINUS,
            d if d.is_ascii_digit() => {
                let idx = (d as u8 - b'0') as usize;
                DIGITS[idx]
            }
            _ => ["?????", "?????", "?????", "?????", "?????"],
        };
        for (line, part) in lines.iter_mut().zip(glyph.iter()) {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(part);
        }
    }
    lines.join("\n")
}

fn print_bits(n: i64) {
    section(
        "Bits (32-bit two's complement view)",
        "🧠",
        "----------------------------------",
    );
    let mut out = String::new();
    for bit in (0..32).rev() {
        if bit % 4 == 3 && bit != 31 {
            out.push(' ');
        }
        let mask = 1_i64 << bit;
        out.push_str(if n & mask != 0 { BIT_ONE } else { BIT_ZERO });
    }
    println!("{out}");
    println!("Legend: {BIT_ONE} = 1, {BIT_ZERO} = 0");
}

fn print_meter(n: i64) {
    section(
        "Signed meter (relative to i64 range)",
        "📏",
        "------------------------------------",
    );
    const WIDTH: usize = 48;
    let ratio = (n as f64) / (i64::MAX as f64);
    let pos = (((ratio + 1.0) / 2.0) * (WIDTH as f64)).clamp(0.0, WIDTH as f64);
    let pos_idx = pos.round() as usize;
    let mut bar = String::new();
    bar.push_str(EDGE_MARK);
    for i in 0..=WIDTH {
        if i == pos_idx {
            bar.push_str(POINTER_MARK); // pointer
        } else if i == WIDTH / 2 {
            bar.push_str(ZERO_MARK); // zero marker
        } else {
            bar.push('-');
        }
    }
    bar.push_str(EDGE_MARK);
    println!("{bar}");
}

fn section(title: &str, emoji: &str, underline: &str) {
    println!("{BOLD}{CYAN}{emoji} {title}{RESET}");
    println!("{CYAN}{underline}{RESET}");
}

fn colorize<T: std::fmt::Display>(value: T, color: &str) -> String {
    format!("{color}{value}{RESET}")
}
