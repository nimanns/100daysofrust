use std::{thread, time};
use std::process::Command;
use rdev::{simulate, Button, EventType, Key, SimulateError};

fn main() {
    Command::new("notepad.exe").spawn().expect("Failed to open Notepad");

    thread::sleep(time::Duration::from_secs(2));

    let text = "Y0u'v3 b33n h4ck3d";
    for ch in text.chars() {
        let key = match ch {
            'a' => Key::KeyA,
            'b' => Key::KeyB,
            'c' => Key::KeyC,
            'd' => Key::KeyD,
            'e' => Key::KeyE,
            'f' => Key::KeyF,
            'g' => Key::KeyG,
            'h' => Key::KeyH,
            'i' => Key::KeyI,
            'j' => Key::KeyJ,
            'k' => Key::KeyK,
            'l' => Key::KeyL,
            'm' => Key::KeyM,
            'n' => Key::KeyN,
            'o' => Key::KeyO,
            'p' => Key::KeyP,
            'q' => Key::KeyQ,
            'r' => Key::KeyR,
            's' => Key::KeyS,
            't' => Key::KeyT,
            'u' => Key::KeyU,
            'v' => Key::KeyV,
            'w' => Key::KeyW,
            'x' => Key::KeyX,
            'y' => Key::KeyY,
            'z' => Key::KeyZ,
            'A' => Key::KeyA,
            'B' => Key::KeyB,
            'C' => Key::KeyC,
            'D' => Key::KeyD,
            'E' => Key::KeyE,
            'F' => Key::KeyF,
            'G' => Key::KeyG,
            'H' => Key::KeyH,
            'I' => Key::KeyI,
            'J' => Key::KeyJ,
            'K' => Key::KeyK,
            'L' => Key::KeyL,
            'M' => Key::KeyM,
            'N' => Key::KeyN,
            'O' => Key::KeyO,
            'P' => Key::KeyP,
            'Q' => Key::KeyQ,
            'R' => Key::KeyR,
            'S' => Key::KeyS,
            'T' => Key::KeyT,
            'U' => Key::KeyU,
            'V' => Key::KeyV,
            'W' => Key::KeyW,
            'X' => Key::KeyX,
            'Y' => Key::KeyY,
            'Z' => Key::KeyZ,
            '0' => Key::Num0,
            '1' => Key::Num1,
            '2' => Key::Num2,
            '3' => Key::Num3,
            '4' => Key::Num4,
            '5' => Key::Num5,
            '6' => Key::Num6,
            '7' => Key::Num7,
            '8' => Key::Num8,
            '9' => Key::Num9,
            ' ' => Key::Space,
            ',' => Key::Comma,
            '.' => Key::Dot,
            _ => continue,
        };

        if ch.is_uppercase() {
            simulate(&EventType::KeyPress(Key::ShiftLeft)).unwrap();
        }

        simulate(&EventType::KeyPress(key)).unwrap();
        simulate(&EventType::KeyRelease(key)).unwrap();

        if ch.is_uppercase() {
            simulate(&EventType::KeyRelease(Key::ShiftLeft)).unwrap();
        }

        thread::sleep(time::Duration::from_millis(50));
    }
}
