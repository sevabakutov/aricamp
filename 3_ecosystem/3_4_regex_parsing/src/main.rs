use regex::Regex;
use once_cell::{sync::Lazy};
static REGEX_MINUX_SIGN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(".*[-].*").unwrap()
});

static REGEX_PLUS_SIGN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(".*[+].*").unwrap()
});

static REGEX_WIDTH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:[^\d.]*?)(\d+)(?:\.|$)").unwrap()
});

static REGEX_PRECISION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\.(?P<prec>\*|\d+\$|\d+)").unwrap()
});

fn main() {
}

fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {

    let sign = if REGEX_MINUX_SIGN.is_match(input) {
        Some(Sign::Minus)
    } else if REGEX_PLUS_SIGN.is_match(input) {
        Some(Sign::Plus)
    } else {
        None
    };

    let width = REGEX_WIDTH
        .captures_iter(input)
        .filter_map(|c| c.get(1))
        .map(|m| m.as_str())
        .filter_map(|s| s.parse::<usize>().ok())
        .next();

    let precision = REGEX_PRECISION
        .captures_iter(input)
        .filter_map(|c| c.name("prec"))
        .map(|m| m.as_str())
        .filter_map(|s| match s {
            "*" => Some(Precision::Asterisk),
            s if s.ends_with('$') => s[..s.len() - 1]
                .parse::<usize>()
                .ok()
                .map(Precision::Argument),
            s => s
                .parse::<usize>().
                ok()
                .map(Precision::Integer),
        })
        .next();

    (sign, width, precision)
}



#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),

        ] {
            let (sign, ..) = parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),

        ] {
            let (_, width, _) = parse(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ]
        {
            let (_, _, precision) = parse(input);
            assert_eq!(precision, expected);
        }
    }
}

