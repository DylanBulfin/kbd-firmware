use std::{collections::VecDeque, iter::Peekable};

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
    pub const LCUBRK: Self = Self {
        right: false,
        ty: BracketType::Curly,
    };
    pub const RCUBRK: Self = Self {
        right: true,
        ty: BracketType::Curly,
    };
}

impl From<Bracket> for ScanToken {
    fn from(value: Bracket) -> Self {
        Self::Bracket(value)
    }
}

pub fn scan_input(into_iter: &mut VecDeque<u8>) -> VecDeque<ScanToken> {
    let mut res = VecDeque::new();

    loop {
        if let Some(c) = into_iter.pop_front() {
            res.push_back(match c {
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
                b'A'..=b'Z' | b'a'..=b'z' => ScanToken::Ident(scan_string(c, into_iter)),
                b'0'..=b'9' => ScanToken::Int(scan_int(c, into_iter)),
                _ => continue,
            });
        } else {
            break;
        }
    }

    res
}

fn scan_string(c: u8, iter: &mut VecDeque<u8>) -> String {
    let mut res = vec![c];

    loop {
        if let Some(c) = iter.front() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => {
                    res.push(iter.pop_front().unwrap())
                }
                _ => break,
            }
        } else {
            break;
        }
    }

    String::from_utf8(res).unwrap_or_default()
}

fn scan_int(c: u8, iter: &mut VecDeque<u8>) -> u32 {
    let conv = |cl: u8| cl.saturating_sub(b'0') as u32;

    let mut res = conv(c);

    loop {
        if let Some(c) = iter.front() {
            match c {
                b'0'..=b'9' => {
                    res *= 10;
                    res += conv(iter.pop_front().unwrap());
                }
                _ => break,
            }
        }
    }

    res
}
