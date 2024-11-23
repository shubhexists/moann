use dirs::home_dir;
use lazy_static::lazy_static;
use rdev::Key;
use std::{collections::HashMap, path::PathBuf};

lazy_static! {
    pub static ref KEY_MAP: HashMap<Key, u64> = {
        let mut map = HashMap::new();
        let standard = [
            (1, Key::Escape),
            (59, Key::F1),
            (60, Key::F2),
            (61, Key::F3),
            (62, Key::F4),
            (63, Key::F5),
            (64, Key::F6),
            (65, Key::F7),
            (66, Key::F8),
            (67, Key::F9),
            (68, Key::F10),
            (87, Key::F11),
            (88, Key::F12),
            (91, Key::MetaLeft),
            (92, Key::MetaRight),
            (41, Key::BackQuote),
            (2, Key::Num1),
            (3, Key::Num2),
            (4, Key::Num3),
            (5, Key::Num4),
            (6, Key::Num5),
            (7, Key::Num6),
            (8, Key::Num7),
            (9, Key::Num8),
            (10, Key::Num9),
            (11, Key::Num0),
            (12, Key::Minus),
            (13, Key::Equal),
            (14, Key::Backspace),
            (15, Key::Tab),
            (58, Key::CapsLock),
            (30, Key::KeyA),
            (48, Key::KeyB),
            (46, Key::KeyC),
            (32, Key::KeyD),
            (18, Key::KeyE),
            (33, Key::KeyF),
            (34, Key::KeyG),
            (35, Key::KeyH),
            (23, Key::KeyI),
            (36, Key::KeyJ),
            (37, Key::KeyK),
            (38, Key::KeyL),
            (50, Key::KeyM),
            (49, Key::KeyN),
            (24, Key::KeyO),
            (25, Key::KeyP),
            (16, Key::KeyQ),
            (19, Key::KeyR),
            (31, Key::KeyS),
            (20, Key::KeyT),
            (22, Key::KeyU),
            (47, Key::KeyV),
            (17, Key::KeyW),
            (45, Key::KeyX),
            (21, Key::KeyY),
            (44, Key::KeyZ),
            (26, Key::LeftBracket),
            (27, Key::RightBracket),
            (43, Key::BackSlash),
            (39, Key::SemiColon),
            (40, Key::Quote),
            (28, Key::Return),
            (51, Key::Comma),
            (52, Key::Dot),
            (53, Key::Slash),
            (57, Key::Space),
            (3639, Key::PrintScreen),
            (70, Key::ScrollLock),
            (3653, Key::Pause),
            (3666, Key::Insert),
            (3667, Key::Delete),
            (3655, Key::Home),
            (3663, Key::End),
            (3657, Key::PageUp),
            (3665, Key::PageDown),
            (57416, Key::UpArrow),
            (57419, Key::LeftArrow),
            (57421, Key::RightArrow),
            (57424, Key::DownArrow),
            (42, Key::ShiftLeft),
            (54, Key::ShiftRight),
            (29, Key::ControlLeft),
            (3613, Key::ControlRight),
            (56, Key::Alt),
            (3640, Key::AltGr),
            (3675, Key::MetaLeft),
            (3676, Key::MetaRight),
            (3677, Key::Function),
            (69, Key::NumLock),
            (3637, Key::KpDivide),
            (55, Key::KpMultiply),
            (74, Key::KpMinus),
            (3597, Key::KeyP),
            (78, Key::KpPlus),
            (3612, Key::KpReturn),
            (83, Key::KpDelete),
            (79, Key::Kp1),
            (80, Key::Kp2),
            (81, Key::Kp3),
            (75, Key::Kp4),
            (76, Key::Kp5),
            (77, Key::Kp6),
            (71, Key::Kp7),
            (72, Key::Kp8),
            (73, Key::Kp9),
            (82, Key::Kp0),
        ];

        for (code, key) in standard.iter() {
            map.insert(key.clone(), *code);
        }

        map
    };
}

lazy_static! {
    pub static ref FILE_PATH: PathBuf = {
        let home_dir = home_dir().expect("Failed to retrieve the user's home directory");
        let pulse_path = home_dir.join(".pulse");

        pulse_path
    };
}
