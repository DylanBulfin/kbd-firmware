use std::collections::HashMap;

use no_std::{Behavior, Layer, Options};
use scanner::{Bracket, ScanToken};

pub mod no_std;
pub mod scanner;

fn parse_options<I>(iter: &mut I) -> Options
where
    I: Iterator<Item = ScanToken>,
{
    assert_eq!(iter.next(), Some(Bracket::LCBRK.into()));
    assert_eq!(
        iter.next(),
        Some(ScanToken::Ident("tapping_term_ms".to_owned()))
    );
    assert_eq!(iter.next(), Some(ScanToken::Colon));
    assert_eq!(iter.next(), Some(ScanToken::Int(150)));
    assert_eq!(iter.next(), Some(ScanToken::Comma));
    assert_eq!(iter.next(), Some(Bracket::RCBRK.into()));
    assert_eq!(iter.next(), Some(ScanToken::Semicolon));

    Options {
        tapping_term_ms: Some(150),
    }
}

fn parse_layers<I>(iter: &mut I) -> [Option<Layer>; 10]
where
    I: Iterator<Item = ScanToken>,
{
    let mut res = [(); 10].map(|_| None);
    let mut map: HashMap<String, Layer> = HashMap::new();

    assert_eq!(iter.next(), Some(Bracket::LCBRK.into()));

    loop {
        if let Some(ScanToken::Ident(name)) = iter.next() {
            assert_eq!(iter.next(), Some(Bracket::LSBRK.into()));

            // TODO parse keys
            loop {
                assert_eq!(iter.next(), Some(Bracket::LPAREN.into()));

                assert_eq!(iter.next(), Some(Bracket::RPAREN.into()));
            }

            assert_eq!(iter.next(), Some(Bracket::RSBRK.into()));
            assert_eq!(iter.next(), Some(ScanToken::Comma));

            let layer = Layer {
                id: map.len() as u32,
                keys: [[Behavior::None; 6]; 4],
            };

            map.insert(name, layer);
        } else {
            panic!("Expected layer name")
        }
    }

    assert_eq!(iter.next(), Some(Bracket::RCBRK.into()));
    assert_eq!(iter.next(), Some(ScanToken::Semicolon));

    if map.len() > 10 {
        panic!("Only supports up to 10 layers")
    }

    for layer in map.into_values() {
        res[layer.id as usize] = Some(layer);
    }

    res
}

// If the behavior has a layer arg, that will need to be converted to int after layers are parsed.
// The second part of the return tuple holds this
fn parse_behavior<I>(iter: &mut I) -> (Behavior, Option<String>)
where
    I: Iterator<Item = ScanToken>,
{
    if let Some(ScanToken::Ident(behavior)) = iter.next() {
        match behavior.as_str() {
            "kp" => {
                if let Some(ScanToken::Ident(key)) = iter.next() {
                    (Behavior::Key(key.into()), None)
                } else {
                    panic!("Expected key name")
                }
            }
            "ml" => {
                if let Some(ScanToken::Ident(layer)) = iter.next() {
                    (Behavior::MomentaryLayer(0), Some(layer))
                } else {
                    panic!("Expected layer name")
                }
            }
            "ht" => {
                if let (Some(ScanToken::Ident(hold)), Some(ScanToken::Ident(tap))) =
                    (iter.next(), iter.next())
                {
                    (Behavior::HoldTap(hold.into(), tap.into()), None)
                } else {
                    panic!("Invalid args for ht behavior")
                }
            }
            "n" | "_" => (Behavior::None, None),
            other => panic!("Unexpected behavior ident: {}", other),
        }
    } else {
        panic!("Unexpected value, expected behavior specifier");
    }
}
