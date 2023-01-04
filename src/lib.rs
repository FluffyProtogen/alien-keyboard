use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    sync::Mutex,
};

use lazy_static::lazy_static;
use winapi::um::winuser::{GetKeyState, VK_CAPITAL, VK_SHIFT};

pub mod hook;

lazy_static! {
    pub static ref KEYS: Mutex<HashMap<u32, Vec<char>>> = Mutex::new(HashMap::new());
}

pub static mut RNG_CAPITALIZATION: bool = true;

pub fn initialize_keys() {
    let mut keys = KEYS.lock().unwrap();

    let file = File::open("keys.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        let mut split = line.split(' ');

        let char = split.nth(0).unwrap();

        let mut new_keys = vec![char.chars().nth(0).unwrap()];

        new_keys.append(
            &mut split
                .map(|char| char.chars().nth(0).unwrap())
                .collect::<Vec<_>>(),
        );

        keys.insert(char.chars().nth(0).unwrap() as u32, new_keys);
    }
}

pub fn modify_shift_caps(key: u32) -> u32 {
    match key {
        0x41..=0x5A => {
            if caps_pressed() || shift_pressed() {
                key
            } else {
                if unsafe { RNG_CAPITALIZATION } {
                    if rand::random::<bool>() {
                        key + 0x20
                    } else {
                        key
                    }
                } else {
                    key + 0x20
                }
            }
        }
        0x30 if shift_pressed() => 0x29,
        0x31 if shift_pressed() => 0x21,
        0x32 if shift_pressed() => 0x40,
        0x33 if shift_pressed() => 0x23,
        0x34 if shift_pressed() => 0x24,
        0x35 if shift_pressed() => 0x25,
        0x36 if shift_pressed() => 0x5e,
        0x37 if shift_pressed() => 0x26,
        0x38 if shift_pressed() => 0x2a,
        0x39 if shift_pressed() => 0x28,
        0xba => {
            if shift_pressed() {
                0x3a
            } else {
                0x3b
            }
        }
        0xbd => {
            if shift_pressed() {
                0x5f
            } else {
                0x2d
            }
        }
        0xbb => {
            if shift_pressed() {
                0x2b
            } else {
                0x3d
            }
        }
        0xbc => {
            if shift_pressed() {
                0x3c
            } else {
                0x2c
            }
        }
        0xbe => {
            if shift_pressed() {
                0x3e
            } else {
                0x2e
            }
        }
        0xbf => {
            if shift_pressed() {
                0x3f
            } else {
                0x2f
            }
        }
        0xc0 => {
            if shift_pressed() {
                0x7e
            } else {
                0x60
            }
        }
        0xdb => {
            if shift_pressed() {
                0x7b
            } else {
                0x5b
            }
        }
        0xdc => {
            if shift_pressed() {
                0x7c
            } else {
                0x5c
            }
        }
        0xdd => {
            if shift_pressed() {
                0x7d
            } else {
                0x5d
            }
        }
        0xde => {
            if shift_pressed() {
                0x22
            } else {
                0x27
            }
        }
        _ => key,
    }
}

fn caps_pressed() -> bool {
    unsafe { (GetKeyState(VK_CAPITAL) & 0x0001) != 0 }
}

fn shift_pressed() -> bool {
    unsafe { winapi::um::winuser::GetAsyncKeyState(VK_SHIFT) < 0 }
}
