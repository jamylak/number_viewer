use std::env;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[37m";
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
        None => (default.to_string(), Number::Int(default)),
    };

    println!("{BOLD}{MAGENTA}✨ Number viewer ✨{RESET}");
    println!("{MAGENTA}============{RESET}\n");
    println!("🎯 Input : {}", colorize(raw.as_str(), YELLOW));
    match number {
        Number::Int(n) => println!("🧾 Value : {}", colorize(n, GREEN)),
        Number::Float(f) => println!("🧾 Value : {}", colorize(format!("{f}"), GREEN)),
    }
    println!();

    match number {
        Number::Int(n) => {
            print_bases(n);
            println!();
            print_base_e(n);
            println!();
            print_ascii_banner(n);
            println!();
            print_bits(n);
            println!();
            print_meter(n);
        }
        Number::Float(f) => {
            print_float_overview(f);
            println!();
            print_float_bits(f);
        }
    }
}

enum Number {
    Int(i64),
    Float(f64),
}

fn parse_number(raw: &str) -> Result<Number, String> {
    let cleaned: String = raw.chars().filter(|c| *c != '_').collect();
    let s = cleaned.as_str();
    let (base, digits) = if let Some(rest) = s.strip_prefix("0b") {
        (2, rest)
    } else if let Some(rest) = s.strip_prefix("0o") {
        (8, rest)
    } else if let Some(rest) = s.strip_prefix("0x") {
        (16, rest)
    } else {
        if looks_float(s) {
            return s
                .parse::<f64>()
                .map(Number::Float)
                .map_err(|e| format!("invalid float: {e}"));
        }

        if let Ok(n) = s.parse::<i64>() {
            return Ok(Number::Int(n));
        }

        return s
            .parse::<f64>()
            .map(Number::Float)
            .map_err(|e| format!("invalid decimal: {e}"));
    };

    i64::from_str_radix(digits, base)
        .map(Number::Int)
        .map_err(|e| format!("invalid base {base}: {e}"))
}

fn print_help() {
    println!("Usage: number_viewer [NUMBER]");
    println!("Show a number in multiple bases plus some ASCII visuals.");
    println!(
        "Accepted forms: decimal (42), float (3.14, -2e5), binary (0b1010), octal (0o52), hex (0x2a)."
    );
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

fn print_float_overview(f: f64) {
    section("Float value", "🧪", "-----------");
    println!("Decimal    : {}", colorize(format!("{f}"), GREEN));
    println!("Scientific : {}", colorize(format!("{f:.6e}"), BLUE));
    println!("Hex bits   : {}", colorize(format!("0x{:016x}", f.to_bits()), MAGENTA));
}

fn print_float_bits(f: f64) {
    section("Float internals (IEEE 754 f64)", "🧬", "------------------------------");
    let bits = f.to_bits();
    let sign_bit = (bits >> 63) & 1;
    let exponent = (bits >> 52) & 0x7ff;
    let fraction = bits & ((1_u64 << 52) - 1);

    let mut out = String::new();
    for bit in (0..64).rev() {
        let is_one = (bits >> bit) & 1 == 1;
        out.push_str(if is_one { BIT_ONE } else { BIT_ZERO });
        if bit == 63 || bit == 52 {
            out.push_str(EDGE_MARK);
        } else if bit % 4 == 0 {
            out.push(' ');
        }
    }
    println!("{out}");
    println!("Legend: sign|exponent|fraction");

    let sign = if sign_bit == 0 { "+" } else { "-" };
    let category = match f.classify() {
        std::num::FpCategory::Nan => "NaN",
        std::num::FpCategory::Infinite => "Infinity",
        std::num::FpCategory::Zero => "Zero",
        std::num::FpCategory::Subnormal => "Subnormal",
        std::num::FpCategory::Normal => "Normal",
    };

    println!("Sign      : {}", colorize(sign, YELLOW));
    println!("Category  : {}", colorize(category, CYAN));
    println!(
        "Exponent  : {} (biased)",
        colorize(format!("{exponent}"), GREEN)
    );
    println!(
        "Fraction  : {}",
        colorize(format!("0x{fraction:013x}"), BLUE)
    );

    let sign_power = if sign_bit == 0 { 0 } else { 1 };
    match category {
        "Normal" => {
            let exponent_unbiased = (exponent as i32) - 1023;
            let frac_value = (fraction as f64) / (1_u64 << 52) as f64;
            let mantissa = 1.0 + frac_value;
            println!(
                "Exponent  : {} (unbiased)",
                colorize(format!("{exponent_unbiased}"), MAGENTA)
            );
            println!(
                "Mantissa  : {}",
                colorize(format!("{mantissa:.12}"), GREEN)
            );
            println!(
                "Value form: {dim}(-1){reset}{sign} {dim}× (1 +{reset} {mant} {dim}) × 2{reset}{exp}",
                dim = DIM,
                reset = RESET,
                sign = colorize(superscript_int(sign_power), YELLOW),
                mant = colorize(format!("{frac_value:.12}"), GREEN),
                exp = colorize(superscript_int(exponent_unbiased), MAGENTA)
            );
        }
        "Subnormal" => {
            let exponent_unbiased = -1022;
            let mantissa = (fraction as f64) / (1_u64 << 52) as f64;
            println!(
                "Exponent  : {} (unbiased)",
                colorize(format!("{exponent_unbiased}"), MAGENTA)
            );
            println!(
                "Mantissa  : {}",
                colorize(format!("{mantissa:.12}"), GREEN)
            );
            println!(
                "Value form: {dim}(-1){reset}{sign} {dim}× (0 +{reset} {mant} {dim}) × 2{reset}{exp}",
                dim = DIM,
                reset = RESET,
                sign = colorize(superscript_int(sign_power), YELLOW),
                mant = colorize(format!("{mantissa:.12}"), GREEN),
                exp = colorize(superscript_int(exponent_unbiased), MAGENTA)
            );
        }
        "Zero" => {
            println!(
                "Value form: {dim}(-1){reset}{sign} {dim}× 0{reset}",
                dim = DIM,
                reset = RESET,
                sign = colorize(superscript_int(sign_power), YELLOW)
            );
        }
        "Infinity" => {
            println!(
                "Value form: {dim}(-1){reset}{sign} {dim}× Infinity{reset}",
                dim = DIM,
                reset = RESET,
                sign = colorize(superscript_int(sign_power), YELLOW)
            );
        }
        _ => {
            println!("Value form: {}", colorize("NaN", MAGENTA));
        }
    }
}

fn superscript_int(n: i32) -> String {
    let mut out = String::new();
    for ch in n.to_string().chars() {
        out.push(match ch {
            '-' => '⁻',
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => ch,
        });
    }
    out
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

fn looks_float(s: &str) -> bool {
    if s.contains('.') || s.contains('e') || s.contains('E') {
        return true;
    }
    let trimmed = s.trim_start_matches(['+', '-']);
    matches!(
        trimmed.to_ascii_lowercase().as_str(),
        "nan" | "inf" | "infinity"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_handles_base_prefixes() {
        match parse_number("0b1010").expect("binary parse") {
            Number::Int(n) => assert_eq!(n, 10),
            _ => panic!("expected int"),
        }

        match parse_number("0o52").expect("octal parse") {
            Number::Int(n) => assert_eq!(n, 42),
            _ => panic!("expected int"),
        }

        match parse_number("0x2a").expect("hex parse") {
            Number::Int(n) => assert_eq!(n, 42),
            _ => panic!("expected int"),
        }
    }

    #[test]
    fn parse_number_allows_underscores() {
        match parse_number("1_234_567").expect("underscore parse") {
            Number::Int(n) => assert_eq!(n, 1_234_567),
            _ => panic!("expected int"),
        }
    }

    #[test]
    fn parse_number_detects_float_forms() {
        match parse_number("3.14").expect("float parse") {
            Number::Float(f) => assert!((f - 3.14).abs() < f64::EPSILON),
            _ => panic!("expected float"),
        }

        match parse_number("-2e5").expect("scientific parse") {
            Number::Float(f) => assert_eq!(f, -200_000.0),
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn looks_float_recognizes_keywords_and_symbols() {
        assert!(looks_float("nan"));
        assert!(looks_float("INF"));
        assert!(looks_float("-infinity"));
        assert!(looks_float("1.0"));
        assert!(looks_float("1e3"));
        assert!(!looks_float("42"));
    }

    #[test]
    fn ascii_digits_renders_minus_and_digits() {
        let banner = ascii_digits(-10);
        let expected = [
            "        #    ### ",
            " ---   ##   #   #",
            "        #   #   #",
            "        #   #   #",
            "       ###   ### ",
        ]
        .join("\n");
        assert_eq!(banner, expected);
    }

    #[test]
    fn superscript_int_formats_negative_numbers() {
        assert_eq!(superscript_int(-1023), "⁻¹⁰²³");
    }

    #[test]
    fn parse_number_handles_positive_infinity() {
        // Value above f64::MAX should parse as infinity
        match parse_number("1e309").expect("positive infinity parse") {
            Number::Float(f) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_negative_infinity() {
        // Value below -f64::MAX should parse as negative infinity
        match parse_number("-1e309").expect("negative infinity parse") {
            Number::Float(f) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_nan() {
        match parse_number("nan").expect("nan parse") {
            Number::Float(f) => assert!(f.is_nan()),
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_inf_keyword() {
        match parse_number("inf").expect("inf parse") {
            Number::Float(f) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_negative_infinity_keyword() {
        match parse_number("-infinity").expect("-infinity parse") {
            Number::Float(f) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_negative());
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_smallest_positive_subnormal() {
        // Smallest positive subnormal value
        match parse_number("5e-324").expect("smallest subnormal parse") {
            Number::Float(f) => {
                assert!(f > 0.0);
                assert_eq!(f.classify(), std::num::FpCategory::Subnormal);
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_largest_normal_float() {
        // Largest finite f64 value
        let max_str = "1.7976931348623157e308";
        match parse_number(max_str).expect("max float parse") {
            Number::Float(f) => {
                assert!(f.is_finite());
                assert!(f > 0.0);
                assert_eq!(f.classify(), std::num::FpCategory::Normal);
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_positive_zero() {
        match parse_number("0.0").expect("positive zero parse") {
            Number::Float(f) => {
                assert_eq!(f, 0.0);
                assert!(f.is_sign_positive());
                assert_eq!(f.classify(), std::num::FpCategory::Zero);
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_negative_zero() {
        match parse_number("-0.0").expect("negative zero parse") {
            Number::Float(f) => {
                assert_eq!(f, 0.0); // -0.0 == 0.0
                assert!(f.is_sign_negative());
                assert_eq!(f.classify(), std::num::FpCategory::Zero);
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_huge_decimal_overflow() {
        // Huge number in decimal format (not scientific notation) should overflow to infinity
        let huge = "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.0";
        match parse_number(huge).expect("huge decimal parse") {
            Number::Float(f) => {
                assert!(f.is_infinite());
                assert!(f.is_sign_positive());
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_large_decimal_with_many_fractional_digits() {
        // Number with many fractional digits should parse and round appropriately
        let many_digits = "90.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001";
        match parse_number(many_digits).expect("many fractional digits parse") {
            Number::Float(f) => {
                // Due to f64 precision, this rounds to 90.0
                assert!(f.is_finite());
                assert_eq!(f, 90.0);
            }
            _ => panic!("expected float"),
        }
    }

    #[test]
    fn parse_number_handles_large_decimal_within_range() {
        // Large number in decimal format that fits within f64 range
        let large = "999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999.0";
        match parse_number(large).expect("large decimal parse") {
            Number::Float(f) => {
                assert!(f.is_finite());
                assert!(f > 0.0);
                assert_eq!(f.classify(), std::num::FpCategory::Normal);
            }
            _ => panic!("expected float"),
        }
    }
}
