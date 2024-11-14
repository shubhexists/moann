use console::Term;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let stdout = Term::buffered_stdout();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    'game_loop: loop {
        let file = BufReader::new(
            File::open("/home/jerry/Desktop/projects/sound/mech-keyboard-02-102918.mp3").unwrap(),
        );
        let source = Decoder::new(file).unwrap();
        if let Ok(character) = stdout.read_char() {
            match character {
                'w' => {
                    stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                'a' => {
                    stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                's' => {
                    stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                'd' => {
                    stream_handle
                        .play_raw(source.convert_samples())
                        .map_err(|_| panic!(" I dont know the error"));
                }
                _ => break 'game_loop,
            }
        }
    }
}
