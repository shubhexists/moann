use crate::errors::PulseErrors;
use indicatif::ProgressBar;
use rdev::{listen, Event, EventType};
use rodio::{Decoder, OutputStreamHandle, Source};
use zip::read::ZipFile;
use zip::ZipArchive;
use std::io::{Read, Write};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
};

pub fn create_pulse_directory() -> Result<(), Box<dyn Error>> {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to get home directory");
    let pulse_dir: PathBuf = home_dir.join(".pulse");
    let _ = fs::create_dir(&pulse_dir)
        .map_err(|err: io::Error| PulseErrors::CouldNotCreatePulseDirectory { err });

    let config_file: PathBuf = pulse_dir.join("config.yaml");
    File::create(config_file)
        .map_err(|err: io::Error| PulseErrors::CouldNotCreatePulseDirectory { err })?;
    Ok(())
}

pub fn listen_and_play(debug: bool, path: &str, stream_handle: OutputStreamHandle) {
    let file: BufReader<File> = BufReader::new(File::open(path).unwrap());
    let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
    let channels: u16 = source.channels();
    let sample_rate: u32 = source.sample_rate();
    let samples: Vec<f32> = source.convert_samples().collect();

    listen(move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            match key {
                _ => {
                    let samples: Vec<f32> = samples.clone();
                    let sound_source: rodio::buffer::SamplesBuffer<f32> =
                        rodio::buffer::SamplesBuffer::new(channels, sample_rate, samples);

                    let _ = stream_handle
                        .play_raw(sound_source.convert_samples())
                        .map_err(|e: rodio::PlayError| eprintln!("Playback error: {}", e));
                }
            }
        }
    })
    .expect("Failed to start global key listener");

    std::thread::park();
}

pub fn download_file(url: &str, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let response: ureq::Response = ureq::get(url).call()?;
    let total_size: u64 = response
        .header("content-length")
        .ok_or("Response doesn't include the content length")?
        .parse::<u64>()?;
    let mut file: File = File::create(path)?;
    let pb: ProgressBar = ProgressBar::new(total_size);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let mut downloaded: u64 = 0;
    let mut buffer: [u8; 8192] = [0; 8192];

    let mut reader: Box<dyn Read + Send + Sync> = response.into_reader();

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");

    Ok(())
}

pub fn unzip_sounds(zip_path: &PathBuf, output_dir: &PathBuf) -> io::Result<()> {
    let file: File = fs::File::open(&zip_path)?;
    let mut archive: ZipArchive<File> = ZipArchive::new(file)?;

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    for i in 0..archive.len() {
        let mut file: ZipFile<'_> = archive.by_index(i)?;
        let out_path: PathBuf = output_dir.join(file.sanitized_name());

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}