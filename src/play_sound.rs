use std::{
    collections::HashMap,
    fs::{read_dir, DirEntry, File},
    io::BufReader,
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
};
use rdev::{listen, Event, EventType};
use rodio::{buffer::SamplesBuffer, Decoder, OutputStreamHandle, Source};
use serde::Serialize;
use crate::{config::{Defines, SoundPack}, constants::{FILE_PATH, KEY_MAP}, sounds::SoundFiles, utils::{is_audio_file, save_sound_buffers_to_json}};

#[derive(Clone, Debug, Serialize)]
pub struct SoundData {
    samples: Vec<f32>,
    channels: u16,
    sample_rate: u32,
}

pub fn listen_and_play(debug: bool, sound: &SoundFiles, stream_handle: OutputStreamHandle, config: SoundPack) {
    let sound_buffers: Arc<RwLock<HashMap<String, SoundData>>> = Arc::new(RwLock::new(HashMap::new()));
    let mut handles: Vec<_> = vec![];

    let dir_path: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(sound));

    for entry in read_dir(dir_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();
        if path.is_file() && is_audio_file(&path) {
            let sound_buffers = Arc::clone(&sound_buffers);
            let path = path.to_path_buf();

            let handle = thread::spawn(move || {
                let file = BufReader::new(File::open(&path).unwrap());
                let decoder = Decoder::new(file).unwrap();
                let channels = decoder.channels();
                let sample_rate = decoder.sample_rate();
                let samples: Vec<f32> = decoder.convert_samples().collect();

                let mut sound_buffers = sound_buffers.write().unwrap();
                sound_buffers.insert(
                    path.to_string_lossy().to_string(),
                    SoundData {
                        samples,
                        channels,
                        sample_rate,
                    },
                );
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if debug {
        let output_path = FILE_PATH.join("sound_buffers.json");
        let output_path_str = output_path.to_str().expect("Invalid UTF-8 in output path");

        save_sound_buffers_to_json(
&*sound_buffers.read().unwrap(),
        output_path_str,                
        debug,
        );
        println!("Map saved at: {:?}", output_path_str);
    }

    match config.key_define_type.as_str() {
        "multi" => {
            if debug {
                println!("Key define type: multi");
            }
            listen(move |event: Event| {
                if let EventType::KeyPress(key) = event.event_type {
                    let code: Option<&u64> = KEY_MAP.get(&key);
                    if let Some(code) = code {
                        if let Defines::StringHashMap(map) = &config.defines {
                            if let Some(file_name) = map.get(&code.to_string()) {
                                if let Some(sound_data) = sound_buffers.read().unwrap().get(file_name) {
                                    let sound_source: SamplesBuffer<f32> = SamplesBuffer::new(
                                        sound_data.channels,
                                        sound_data.sample_rate,
                                        sound_data.samples.clone(),
                                    );
    
                                    if let Err(e) = stream_handle
                                        .play_raw(sound_source.convert_samples())
                                        .map_err(|e| eprintln!("Playback error: {}", e))
                                    {
                                        if debug {
                                            eprintln!(
                                                "Failed to play sound for file {}: {:?}",
                                                file_name, e
                                            );
                                        }
                                    }
                                } else if debug {
                                    eprintln!("Sound file {} not found in buffers", file_name);
                                }
                            } else if debug {
                                eprintln!("No file name mapped for key code: {}", code);
                            }
                        } else if debug {
                            eprintln!("Config defines is not a StringHashMap, unexpected type!");
                        }
                    } else if debug {
                        eprintln!("No mapping found for key: {:?}", key);
                    }
                }
            })
            .expect("Failed to start global key listener");
        }
        _ => {
            if debug {
                println!("Key define type: default");
            }
            unimplemented!(".ogg files are still unimplemented");
        }
    }    

    std::thread::park();
}
