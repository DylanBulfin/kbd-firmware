use std::collections::{HashMap, VecDeque};

use no_std::{Behavior, COLS, Config, KEYS, Layer, Options, ROWS};
use scanner::{Bracket, ScanToken};

pub mod no_std;
pub mod scanner;

pub const NUM_LAYERS: usize = 10;

pub fn parse_config(iter: &mut VecDeque<ScanToken>) -> Config {
    assert_eq!(
        iter.pop_front(),
        Some(ScanToken::Ident("options".to_owned()))
    );
    let options = parse_options(iter);

    assert_eq!(
        iter.pop_front(),
        Some(ScanToken::Ident("layers".to_owned()))
    );
    let layers = parse_layers(iter);

    Config { options, layers }
}

fn parse_options(iter: &mut VecDeque<ScanToken>) -> Options {
    assert_eq!(iter.pop_front(), Some(ScanToken::Colon));
    assert_eq!(iter.pop_front(), Some(Bracket::LCUBRK.into()));
    assert_eq!(
        iter.pop_front(),
        Some(ScanToken::Ident("tapping_term_ms".to_owned()))
    );
    assert_eq!(iter.pop_front(), Some(ScanToken::Colon));
    if let Some(ScanToken::Int(tt)) = iter.pop_front() {
        assert_eq!(iter.pop_front(), Some(ScanToken::Comma));
        assert_eq!(iter.pop_front(), Some(Bracket::RCUBRK.into()));
        assert_eq!(iter.pop_front(), Some(ScanToken::Semicolon));

        Options {
            tapping_term_ms: Some(tt),
        }
    } else {
        panic!("Unable to parse tapping term as int")
    }
}

pub struct RichLayer {
    id: u32,
    behaviors: [RichBehavior; (ROWS * COLS) as usize],
}

fn parse_layers(iter: &mut VecDeque<ScanToken>) -> [Option<Layer>; NUM_LAYERS] {
    let mut res = [(); NUM_LAYERS].map(|_| None);
    let mut map: Vec<RichLayer> = vec![];
    let mut name_id_map: HashMap<String, u32> = HashMap::new();

    assert_eq!(iter.pop_front(), Some(ScanToken::Colon));
    assert_eq!(iter.pop_front(), Some(Bracket::LCUBRK.into()));

    loop {
        match iter.pop_front() {
            Some(ScanToken::Ident(name)) => {
                assert_eq!(iter.pop_front(), Some(ScanToken::Colon));
                assert_eq!(iter.pop_front(), Some(Bracket::LSBRK.into()));

                let mut layer = RichLayer {
                    id: map.len() as u32,
                    behaviors: [false; (ROWS * COLS) as usize].map(|_| RichBehavior {
                        base: Behavior::None,
                        layer_name: None,
                    }),
                };

                let mut i = 0;

                loop {
                    assert_eq!(iter.pop_front(), Some(Bracket::LPAREN.into()));

                    layer.behaviors[i] = parse_behavior(iter);

                    assert_eq!(iter.pop_front(), Some(Bracket::RPAREN.into()));

                    i += 1;

                    if i == KEYS {
                        // Processed all keys, stop parsing
                        break;
                    }
                }

                assert_eq!(iter.pop_front(), Some(Bracket::RSBRK.into()));
                assert_eq!(iter.pop_front(), Some(ScanToken::Comma));

                name_id_map.insert(name, map.len() as u32);
                map.push(layer);
            }
            Some(ScanToken::Bracket(Bracket::RCUBRK)) => {
                break;
            }
            _ => panic!("Expected layer name"),
        }
    }

    assert_eq!(iter.pop_front(), Some(ScanToken::Semicolon));

    if map.len() > NUM_LAYERS {
        panic!("Only supports up to 10 layers")
    }

    // Set up correct layer ids. Needs to be done after base processing since that's when we find
    // out what layers they are and what id they'll have.
    // TODO Probably lan start pre-generating layers as soon as they're referenced?
    for layer in map.iter_mut() {
        for behavior in layer.behaviors.iter_mut() {
            if let Some(ref name) = behavior.layer_name {
                if let Some(id) = name_id_map.get(name) {
                    if let Behavior::MomentaryLayer(_) = behavior.base {
                        behavior.base = Behavior::MomentaryLayer(*id);
                    }
                }
            }
        }
    }

    for (i, layer) in map.into_iter().enumerate() {
        res[i] = Some(Layer {
            id: layer.id,
            keys: layer.behaviors.map(|rb| rb.base),
        });
    }

    res
}

#[derive(Debug, PartialEq, Eq)]
struct RichBehavior {
    base: Behavior,
    layer_name: Option<String>,
}

impl RichBehavior {
    fn new(base: Behavior, layer_name: Option<String>) -> Self {
        Self { base, layer_name }
    }
}

// If the behavior has a layer arg, that will need to be converted to int after layers are parsed.
// The second part of the return tuple holds this
fn parse_behavior(iter: &mut VecDeque<ScanToken>) -> RichBehavior {
    if let Some(ScanToken::Ident(behavior)) = iter.pop_front() {
        match behavior.as_str() {
            "kp" => {
                if let Some(ScanToken::Ident(key)) = iter.pop_front() {
                    RichBehavior::new(Behavior::Key(key.into()), None)
                } else {
                    panic!("Expected key name")
                }
            }
            "ml" => {
                if let Some(ScanToken::Ident(layer)) = iter.pop_front() {
                    RichBehavior::new(Behavior::MomentaryLayer(0), Some(layer))
                } else {
                    panic!("Expected layer name")
                }
            }
            "ht" => {
                if let (Some(ScanToken::Ident(hold)), Some(ScanToken::Ident(tap))) =
                    (iter.pop_front(), iter.pop_front())
                {
                    RichBehavior::new(Behavior::HoldTap(hold.into(), tap.into()), None)
                } else {
                    panic!("Invalid args for ht behavior")
                }
            }
            "t" => RichBehavior::new(Behavior::Transparent, None),
            "n" => RichBehavior::new(Behavior::None, None),
            other => panic!("Unexpected behavior ident: {}", other),
        }
    } else {
        panic!("Unexpected value, expected behavior specifier");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        NUM_LAYERS, RichBehavior,
        no_std::{Behavior, COLS, Config, KEYS, Key, Layer, Options, ROWS},
        parse_behavior, parse_config, parse_layers, parse_options,
        scanner::scan_input,
    };

    #[test]
    fn test_parse_behavior() {
        let e1 = RichBehavior {
            base: Behavior::Transparent,
            layer_name: None,
        };
        let e2 = RichBehavior {
            base: Behavior::MomentaryLayer(0),
            layer_name: Some("TestLayer".to_owned()),
        };
        let e3 = RichBehavior {
            base: Behavior::Key(Key::B),
            layer_name: None,
        };

        let mut s1 = "t".bytes();
        let mut s2 = "ml TestLayer".bytes();
        let mut s3 = "kp B".bytes();

        let mut t1 = scan_input(&mut s1.collect());
        let mut t2 = scan_input(&mut s2.collect());
        let mut t3 = scan_input(&mut s3.collect());

        assert_eq!(e1, parse_behavior(&mut t1));
        assert_eq!(e2, parse_behavior(&mut t2));
        assert_eq!(e3, parse_behavior(&mut t3));
    }

    #[test]
    fn test_parse_options() {
        let e1 = Options {
            tapping_term_ms: Some(150),
        };

        let mut s1 = ": {
            tapping_term_ms: 150,
        };"
        .bytes();

        let mut t1 = scan_input(&mut s1.collect());

        assert_eq!(e1, parse_options(&mut t1));
    }

    #[test]
    fn test_parse_layers() {
        let e1 = [
            Some(Layer {
                id: 0,
                keys: [Behavior::Key(Key::BKSP); (ROWS * COLS) as usize],
            }),
            Some(Layer {
                id: 1,
                keys: [Behavior::MomentaryLayer(2); (ROWS * COLS) as usize],
            }),
            Some(Layer {
                id: 2,
                keys: [Behavior::Transparent; (ROWS * COLS) as usize],
            }),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];

        let mut s1 = ": { BASE: [".to_owned();
        s1.push_str(["(kp BKSP)"; (ROWS * COLS) as usize].join(" ").as_str());
        s1.push_str("], RAISE: [");

        s1.push_str(["(ml LOWER)"; (ROWS * COLS) as usize].join(" ").as_str());
        s1.push_str("], LOWER: [");

        s1.push_str(["(t)"; (ROWS * COLS) as usize].join(" ").as_str());
        s1.push_str("],};");

        let mut t1 = scan_input(&mut s1.bytes().collect());

        assert_eq!(e1, parse_layers(&mut t1));
    }

    #[test]

    fn test_parse_config() {
        let mut behaviors: [Behavior; KEYS] = [Behavior::None; KEYS];
        behaviors[0] = Behavior::Transparent;
        behaviors[6] = Behavior::HoldTap(Key::A, Key::LCTL);
        behaviors[12] = Behavior::Key(Key::B);

        let e1 = Config {
            options: Options {
                tapping_term_ms: Some(100),
            },
            layers: [
                Some(Layer {
                    id: 0,
                    keys: behaviors,
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        };

        let s1 = "options: {tapping_term_ms: 100,}; layers: {BASE: [
                    (t)         (n)(n)(n)(n)(n)
                    (ht A LCTL) (n)(n)(n)(n)(n)
                    (kp B)      (n)(n)(n)(n)(n)
                    (n)         (n)(n)(n)(n)(n)],
                };";
        let mut t1 = scan_input(&mut s1.bytes().collect());

        let c1 = parse_config(&mut t1);

        assert_eq!(c1, e1);
    }
}
