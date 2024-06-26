use std::{thread, time};
use std::process::Command;
use rdev::{simulate, EventType, Key, SimulateError};

fn main() {
    Command::new("notepad.exe").spawn().expect("Failed to open Notepad");
    thread::sleep(time::Duration::from_secs(2));
    let text = "Y0u'v3 b33n h4ck3d";
    for ch in text.chars() {
        type_char(ch).unwrap();
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn type_char(ch: char) -> Result<(), SimulateError> {
    let key = match ch {
        'a' | 'A' => Key::KeyA,
        'b' | 'B' => Key::KeyB,
        'c' | 'C' => Key::KeyC,
        'd' | 'D' => Key::KeyD,
        'e' | 'E' => Key::KeyE,
        'f' | 'F' => Key::KeyF,
        'g' | 'G' => Key::KeyG,
        'h' | 'H' => Key::KeyH,
        'i' | 'I' => Key::KeyI,
        'j' | 'J' => Key::KeyJ,
        'k' | 'K' => Key::KeyK,
        'l' | 'L' => Key::KeyL,
        'm' | 'M' => Key::KeyM,
        'n' | 'N' => Key::KeyN,
        'o' | 'O' => Key::KeyO,
        'p' | 'P' => Key::KeyP,
        'q' | 'Q' => Key::KeyQ,
        'r' | 'R' => Key::KeyR,
        's' | 'S' => Key::KeyS,
        't' | 'T' => Key::KeyT,
        'u' | 'U' => Key::KeyU,
        'v' | 'V' => Key::KeyV,
        'w' | 'W' => Key::KeyW,
        'x' | 'X' => Key::KeyX,
        'y' | 'Y' => Key::KeyY,
        'z' | 'Z' => Key::KeyZ,
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
        _ => return Ok(()),
    };

    if ch.is_uppercase() {
        simulate(&EventType::KeyPress(Key::ShiftLeft))?;
    }

    simulate(&EventType::KeyPress(key))?;
    simulate(&EventType::KeyRelease(key))?;

    if ch.is_uppercase() {
        simulate(&EventType::KeyRelease(Key::ShiftLeft))?;
    }

    Ok(())
}

