use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScanToken {
    Ident(String),
    Int(u32),
    Comma,
    Bracket(Bracket),
    Colon,
    Semicolon,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BracketType {
    Paren,
    Square,
    Curly,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bracket {
    pub right: bool,
    pub ty: BracketType,
}

impl Bracket {
    pub const LPAREN: Self = Self {
        right: false,
        ty: BracketType::Paren,
    };
    pub const RPAREN: Self = Self {
        right: true,
        ty: BracketType::Paren,
    };
    pub const LSBRK: Self = Self {
        right: false,
        ty: BracketType::Square,
    };
    pub const RSBRK: Self = Self {
        right: true,
        ty: BracketType::Square,
    };
    pub const LCBRK: Self = Self {
        right: false,
        ty: BracketType::Curly,
    };
    pub const RCBRK: Self = Self {
        right: true,
        ty: BracketType::Curly,
    };
}

impl From<Bracket> for ScanToken {
    fn from(value: Bracket) -> Self {
        Self::Bracket(value)
    }
}

pub fn scan_input<I>(into_iter: I) -> Vec<ScanToken>
where
    I: IntoIterator<Item = u8>,
{
    let mut iter = into_iter.into_iter().peekable();
    let mut res = Vec::new();

    loop {
        if let Some(c) = iter.next() {
            res.push(match c {
                b'(' => Bracket {
                    right: false,
                    ty: BracketType::Paren,
                }
                .into(),
                b')' => Bracket {
                    right: true,
                    ty: BracketType::Paren,
                }
                .into(),
                b'[' => Bracket {
                    right: false,
                    ty: BracketType::Square,
                }
                .into(),
                b']' => Bracket {
                    right: true,
                    ty: BracketType::Square,
                }
                .into(),
                b'{' => Bracket {
                    right: false,
                    ty: BracketType::Curly,
                }
                .into(),
                b'}' => Bracket {
                    right: true,
                    ty: BracketType::Curly,
                }
                .into(),
                b',' => ScanToken::Comma,
                b';' => ScanToken::Semicolon,
                b':' => ScanToken::Colon,
                b'A'..=b'Z' | b'a'..=b'z' => ScanToken::Ident(scan_string(c, &mut iter)),
                b'0'..=b'9' => ScanToken::Int(scan_int(c, &mut iter)),
                _ => continue,
            });
        } else {
            break;
        }
    }

    res
}

fn scan_string<I>(c: u8, iter: &mut Peekable<I>) -> String
where
    I: Iterator<Item = u8>,
{
    let mut res = vec![c];

    loop {
        if let Some(c) = iter.peek() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => res.push(*c),
                _ => break,
            }
        }
    }

    String::from_utf8(res).unwrap_or_default()
}

fn scan_int<I>(c: u8, iter: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = u8>,
{
    let conv = |cl: u8| cl.saturating_sub(b'0') as u32;

    let mut res = conv(c);

    loop {
        if let Some(c) = iter.peek() {
            match c {
                b'0'..=b'9' => {
                    res *= 10;
                    res += conv(*c);
                }
                _ => break,
            }
        }
    }

    res
}
