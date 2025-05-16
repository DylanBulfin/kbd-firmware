use core::cell::{Cell, RefCell};

use usbd_human_interface_device::page::Keyboard;

/// This file handles the layout of a keyboard's keys in rows and columns

const ROWS: usize = 4;
const COLS: usize = 6;

#[derive(Clone, Copy)]
pub enum Behavior {
    None,
    Trans,
    KeyPress(Keyboard),
    MomentaryLayer(u32),
}

// pub struct Matrix {
//     keys: [[Key; COLS]; ROWS],
// }

#[derive(Clone, Copy)]
pub struct Layer {
    behaviors: [[Behavior; COLS]; ROWS],
}

const MAX_LAYERS: usize = 10;

pub struct State {
    layers: [Option<Layer>; MAX_LAYERS],
    end_ptr: usize,
}

impl State {
    pub fn push_layer(&mut self, layer: Layer) -> bool {
        self.end_ptr < 10 && {
            self.layers[self.end_ptr] = Some(layer);
            self.end_ptr += 1;
            true
        }
    }

    pub fn pop_layer(&mut self) -> Option<Layer> {
        let res = self.layers[self.end_ptr];
        self.layers[self.end_ptr] = None;
        self.end_ptr.checked_sub(1).unwrap_or(0);
        res
    }
}

