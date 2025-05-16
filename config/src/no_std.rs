#![no_std]
const ROWS: u32 = 4;
const COLS: u32 = 6;
/// Types to export to the firmware

pub struct Config {
    options: Options,
    layers: [Option<Layer>; 10],
}

pub struct Options {
    pub tapping_term_ms: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    ESC,
    LCTL,
    LSFT,
    LGUI,
    LALT,
    BKSP, // Backspace
    TAB,  // Tab
    SPC,  // Space
    N0,   // Numbers
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    RET,
    DEL,
    MNS,  // Minus
    EQL,  // Equal
    BSLH, // Backslash
    FSLH, // Forward slash
    LPRN, // Parentheses
    RPRN,
    LSBR, // Square Brackets
    RSBR,
    LCBR, // Curly Braces
    RCBR,
    QUOT, // Quotation mark
    UP,   // Directions
    LFT,
    RHT,
    DN,
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            "G" => Self::G,
            "H" => Self::H,
            "I" => Self::I,
            "J" => Self::J,
            "K" => Self::K,
            "L" => Self::L,
            "M" => Self::M,
            "N" => Self::N,
            "O" => Self::O,
            "P" => Self::P,
            "Q" => Self::Q,
            "R" => Self::R,
            "S" => Self::S,
            "T" => Self::T,
            "U" => Self::U,
            "V" => Self::V,
            "W" => Self::W,
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            "ESC" => Self::ESC,
            "LCTL" => Self::LCTL,
            "LSFT" => Self::LSFT,
            "LGUI" => Self::LGUI,
            "LALT" => Self::LALT,
            "BKSP" => Self::BKSP,
            "TAB" => Self::TAB,
            "SPC" => Self::SPC,
            "N0" => Self::N0,
            "N1" => Self::N1,
            "N2" => Self::N2,
            "N3" => Self::N3,
            "N4" => Self::N4,
            "N5" => Self::N5,
            "N6" => Self::N6,
            "N7" => Self::N7,
            "N8" => Self::N8,
            "N9" => Self::N9,
            "RET" => Self::RET,
            "DEL" => Self::DEL,
            "MNS" => Self::MNS,
            "EQL" => Self::EQL,
            "BSLH" => Self::BSLH,
            "FSLH" => Self::FSLH,
            "LPRN" => Self::LPRN,
            "RPRN" => Self::RPRN,
            "LSBR" => Self::LSBR,
            "RSBR" => Self::RSBR,
            "LCBR" => Self::LCBR,
            "RCBR" => Self::RCBR,
            "QUOT" => Self::QUOT,
            "UP" => Self::UP,
            "LFT" => Self::LFT,
            "RHT" => Self::RHT,
            "DN" => Self::DN,
            _ => panic!("Unexpected key: {}", value),
        }
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Behavior {
    Key(Key),
    MomentaryLayer(u32),
    HoldTap(Key, Key),
    None,
}

pub struct Layer {
    pub id: u32,
    pub keys: [[Behavior; COLS as usize]; ROWS as usize],
}
