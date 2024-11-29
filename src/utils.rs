use crate::errors::PulseErrors;
use crate::play_sound::SoundData;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::PathBuf,
};
use zip::read::ZipFile;
use zip::ZipArchive;

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

pub fn is_audio_file(path: &std::path::Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some("mp3") | Some("wav") | Some("ogg") => true,
        _ => false,
    }
}

pub fn save_sound_buffers_to_json(
    sound_buffers: &HashMap<String, SoundData>,
    output_path: &str,
    debug: bool,
) {
    match serde_json::to_string(&sound_buffers) {
        Ok(json_string) => {
            let mut file = File::create(output_path).expect("Failed to create output JSON file");
            file.write_all(json_string.as_bytes())
                .expect("Failed to write JSON to file");
            if debug {
                println!("Sound buffers written to JSON file at: {}", output_path);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize sound buffers: {}", e);
        }
    }
}
