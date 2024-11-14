use console::Term;
use rdev::{listen, Event, EventType};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let _ = Term::buffered_stdout();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    listen(move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            let file = BufReader::new(
                File::open("/home/jerry/Desktop/projects/sound/mech-keyboard-02-102918.mp3")
                    .unwrap(),
            );
            let source = Decoder::new(file).unwrap();
            match key {
                rdev::Key::KeyA => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyB => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyC => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyD => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyE => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyF => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyG => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyH => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyI => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyJ => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyK => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyL => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyM => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyN => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyO => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyP => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyQ => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyR => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyS => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyT => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyU => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyV => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyW => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyX => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyY => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                rdev::Key::KeyZ => {
                    let _ = stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                _ => {}
            }
        }
    })
    .expect("Failed to start global key listener");
}
