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
            if unsafe { RNG_CAPITALIZATION } {
                if rand::random::<bool>() {
                    key + 0x20
                } else {
                    key
                }
            } else {
                if !(caps_pressed() || shift_pressed()) {
                    key + 0x20
                } else {
                    key
                }
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

/*
match code {
        VK_LBUTTON => "Left".into(),
        VK_RBUTTON => "Right".into(),
        VK_MBUTTON => "Middle".into(),
        VK_BACK => "Back".into(),
        VK_TAB => "Tab".into(),
        VK_CLEAR => "Clear".into(),
        VK_RETURN => "Enter".into(),
        VK_LSHIFT => "Left Shift".into(),
        VK_RSHIFT => "Right Shift".into(),
        VK_LCONTROL => "Left Control".into(),
        VK_RCONTROL => "Right Control".into(),
        VK_LMENU => "Left Alt".into(),
        VK_RMENU => "Right Alt".into(),
        VK_CAPITAL => "Caps Lock".into(),
        VK_ESCAPE => "Escape".into(),
        VK_SPACE => "Space".into(),
        VK_PRIOR => "Page Up".into(),
        VK_NEXT => "Page Down".into(),
        VK_END => "End".into(),
        VK_HOME => "Home".into(),
        VK_LEFT => "Left Arrow".into(),
        VK_UP => "Up Arrow".into(),
        VK_RIGHT => "Right Arrow".into(),
        VK_DOWN => "Down Arrow".into(),
        VK_SELECT => "Select".into(),
        VK_PRINT => "Print".into(),
        VK_EXECUTE => "Execute".into(),
        VK_SNAPSHOT => "Snapshot".into(),
        VK_INSERT => "Insert".into(),
        VK_DELETE => "Delete".into(),
        VK_HELP => "Help".into(),
        0x30..=0x39 => (code - 0x30).to_string().into(),
        0x41..=0x5A => (code as u8 as char).to_string().into(),
        VK_LWIN => "Left Windows".into(),
        VK_RWIN => "Right Windows".into(),
        VK_APPS => "Applications".into(),
        VK_SLEEP => "Sleep".into(),
        VK_NUMPAD0..=VK_NUMPAD9 => format!("Number Pad {}", code - VK_NUMPAD0).into(),
        VK_ADD => "Add".into(),
        VK_SUBTRACT => "Subtract".into(),
        VK_MULTIPLY => "Multiply".into(),
        VK_DIVIDE => "Divide".into(),
        VK_SEPARATOR => "Separator".into(),
        VK_DECIMAL => "Decimal".into(),
        VK_F1..=VK_F24 => format!("F{}", code - VK_F1 + 1).into(),
        VK_NUMLOCK => "Number Lock".into(),
        VK_SCROLL => "Scroll".into(),
        _ => format!("Key Code: {}", code).into(),
    } */
