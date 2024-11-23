use crate::{
    config::{Defines, SoundPack},
    constants::{FILE_PATH, KEY_MAP},
    errors::PulseErrors,
    sounds::SoundFiles,
    utils::{is_audio_file, save_sound_buffers_to_json},
};
use rdev::{listen, Event, EventType};
use rodio::{buffer::SamplesBuffer, Decoder, OutputStreamHandle, Sink, Source};
use serde::Serialize;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{read_dir, DirEntry, File},
    io::BufReader,
    path::PathBuf,
    sync::{Arc, RwLock, RwLockWriteGuard},
    thread,
    time::Duration,
};

#[derive(Clone, Debug, Serialize)]
pub struct SoundData {
    samples: Vec<f32>,
    channels: u16,
    sample_rate: u32,
}

pub fn listen_and_play(
    debug: bool,
    sound: &SoundFiles,
    stream_handle: OutputStreamHandle,
    config: SoundPack,
) {
    let sound_buffers: Arc<RwLock<HashMap<String, SoundData>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let mut handles: Vec<_> = vec![];

    let dir_path: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(sound));

    for entry in read_dir(dir_path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();
        if path.is_file() && is_audio_file(&path) {
            let sound_buffers: Arc<RwLock<HashMap<String, SoundData>>> = Arc::clone(&sound_buffers);
            let path: PathBuf = path.to_path_buf();

            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                let file: BufReader<File> = BufReader::new(File::open(&path).unwrap());
                let decoder: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
                let channels: u16 = decoder.channels();
                let sample_rate: u32 = decoder.sample_rate();
                let samples: Vec<f32> = decoder.convert_samples().collect();

                let file_name: String = path
                    .file_name()
                    .and_then(|os_str: &OsStr| os_str.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| path.to_string_lossy().to_string());

                let mut sound_buffers: RwLockWriteGuard<'_, HashMap<String, SoundData>> =
                    sound_buffers.write().unwrap();
                sound_buffers.insert(
                    file_name,
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
        let output_path: PathBuf = FILE_PATH.join("sound_buffers.json");
        let output_path_str: &str = output_path.to_str().expect("Invalid UTF-8 in output path");

        save_sound_buffers_to_json(&*sound_buffers.read().unwrap(), output_path_str, debug);
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
                        if let Some(Defines::StringHashMap(map)) = &config.defines {
                            if let Some(file_name) = map.get(&code.to_string()) {
                                if let Some(sound_data) =
                                    sound_buffers.read().unwrap().get(file_name)
                                {
                                    let sound_source: SamplesBuffer<f32> = SamplesBuffer::new(
                                        sound_data.channels,
                                        sound_data.sample_rate,
                                        sound_data.samples.clone(),
                                    );
                                    if let Err(e) = stream_handle
                                        .play_raw(sound_source.convert_samples())
                                        .map_err(|e: rodio::PlayError| {
                                            eprintln!("Playback error: {}", e);
                                            PulseErrors::UnableToPlayFile { err: e }
                                        })
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
                            eprintln!("Config defines is either None or not a StringHashMap!");
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
                println!("Key define type: single");
            }

            listen(move |event: Event| {
                if let EventType::KeyPress(key) = event.event_type {
                    let code: Option<&u64> = KEY_MAP.get(&key);
                    if let Some(code) = code {
                        if let Some(Defines::U64HashMap(map)) = &config.defines {
                            if let Some(sound_segment) = map.get(&code.to_string()) {
                                if let Some(sound_data) =
                                    sound_buffers.read().unwrap().get(&config.sound)
                                {
                                    let sound_source: SamplesBuffer<f32> = SamplesBuffer::new(
                                        sound_data.channels,
                                        sound_data.sample_rate,
                                        sound_data.samples.clone(),
                                    );
                                    let start_ms: f32 = sound_segment[0] as f32 / 1000.0;
                                    let duration: Duration =
                                        Duration::from_millis(sound_segment[1] as u64);
                                    let stream_handle_clone: OutputStreamHandle =
                                        stream_handle.clone();
                                    thread::spawn(move || {
                                        let sink = Sink::try_new(&stream_handle_clone).unwrap();
                                        sink.try_seek(Duration::from_secs_f32(start_ms)).unwrap();
                                        sink.append(sound_source.convert_samples::<f32>());
                                        std::thread::sleep(duration);
                                        sink.stop();
                                    });
                                }
                            }
                        }
                    }
                }
            })
            .expect("Failed to start global key listener");
        }
    }

    std::thread::park();
}
