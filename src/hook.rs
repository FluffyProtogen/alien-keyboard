use rand::seq::SliceRandom;
use std::{
    hint::unreachable_unchecked, mem::*, sync::mpsc::sync_channel, thread, time::SystemTime,
};
use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::POINT},
    um::{processthreadsapi::GetCurrentThreadId, sysinfoapi::GetTickCount, winuser::*},
};

use crate::{modify_shift_caps, KEYS};

pub fn run() {
    let keyboard_thread = unsafe { generate_hook() };

    while !stop_key_pressed() {}

    unsafe { PostThreadMessageA(keyboard_thread, WM_QUIT, 0, 0) };
}

unsafe fn generate_hook() -> u32 {
    let (keyboard_sender, keyboard_receiver) = sync_channel(0);

    thread::spawn(move || {
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard), std::ptr::null_mut(), 0);

        let mut msg = zeroed();

        keyboard_sender.send(GetCurrentThreadId()).unwrap();

        while GetMessageA(&mut msg, zeroed(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        UnhookWindowsHookEx(hook);
    });

    keyboard_receiver.recv().unwrap()
}

fn send_key(char: char, pressed: bool) {
    let dw_flags = if pressed { 0 } else { KEYEVENTF_KEYUP } | KEYEVENTF_UNICODE;

    if char as u32 <= u16::MAX as u32 {
        let mut keybd_input: INPUT_u = unsafe { std::mem::zeroed() };
        unsafe {
            *keybd_input.ki_mut() = KEYBDINPUT {
                wVk: 0,
                dwExtraInfo: 0,
                wScan: char as u16,
                time: 0,
                dwFlags: dw_flags,
            };
        };
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: keybd_input,
        };

        unsafe {
            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
        };
    } else
    // split the character into 2 utf 16 parts and send both of the inputs at the same time owo
    {
        let mut char_utf16 = [0; 2];
        char.encode_utf16(&mut char_utf16);

        let mut keybd_inputs: [INPUT_u; 2] = unsafe { std::mem::zeroed() };

        unsafe {
            *keybd_inputs[0].ki_mut() = KEYBDINPUT {
                wVk: 0,
                dwExtraInfo: 0,
                wScan: char_utf16[0],
                time: 0,
                dwFlags: dw_flags,
            };
            *keybd_inputs[1].ki_mut() = KEYBDINPUT {
                wVk: 0,
                dwExtraInfo: 0,
                wScan: char_utf16[1],
                time: 0,
                dwFlags: dw_flags,
            };
        };

        let mut inputs = [
            INPUT {
                type_: INPUT_KEYBOARD,
                u: keybd_inputs[0],
            },
            INPUT {
                type_: INPUT_KEYBOARD,
                u: keybd_inputs[1],
            },
        ];

        unsafe {
            SendInput(2, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
        };
    }
}

unsafe extern "system" fn keyboard(n_code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let info = *transmute::<LPARAM, PKBDLLHOOKSTRUCT>(l_param);

    if (info.flags & LLKHF_INJECTED == 0 || info.vkCode == 0xbf)
        && !(ctrl_pressed() || alt_pressed())
        && !(info.vkCode == VK_LEFT as u32
            || info.vkCode == VK_RIGHT as u32
            || info.vkCode == VK_UP as u32
            || info.vkCode == VK_DOWN as u32
            || info.vkCode == VK_LWIN as u32
            || info.vkCode == VK_RWIN as u32)
    {
        if let Some(key_list) = KEYS.lock().unwrap().get(&modify_shift_caps(info.vkCode)) {
            let random_char = *(key_list.choose(&mut rand::thread_rng()).unwrap_unchecked());

            let pressed = match w_param as u32 {
                WM_KEYDOWN | WM_SYSKEYDOWN => true,
                WM_KEYUP | WM_SYSKEYUP => false,
                _ => unreachable_unchecked(),
            };

            send_key(random_char, pressed);

            return -1;
        }
    }
    CallNextHookEx(zeroed(), n_code, w_param, l_param)
}

fn ctrl_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_CONTROL) < 0 }
}
fn alt_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_MENU) < 0 }
}

pub fn stop_key_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_CONTROL) < 0 && GetAsyncKeyState(0x51) < 0 }
}
