use std::{error::Error, fs, io};
use crate::{config::SoundPack, constants::FILE_PATH, errors::PulseErrors, play_sound::listen_and_play, sounds::SoundFiles, utils::{download_file, unzip_sounds}};
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;
use rodio::OutputStreamHandle;

pub fn start(debug: bool, stream_handle: OutputStreamHandle) -> Result<(), Box<dyn Error>> {
    let selection_array: Vec<String> = vec![
        SoundFiles::get_name(&SoundFiles::ApexPro),
        SoundFiles::get_name(&SoundFiles::EgOrea),
        SoundFiles::get_name(&SoundFiles::FallOut),
    ];

    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a version")
        .default(0)
        .items(&selection_array)
        .interact()
        .unwrap();

    match &selection {
        // Apex Pro
        0 => {
            let dir_path: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::ApexPro));
            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ApexPro));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKzT3eW2mI0VLjinckoMapKPTDs9qeQYlW63F8",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;
            
                let zip_path: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ApexPro));
                unzip_sounds(&zip_path, &dir_path).map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path).map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;
            
                if debug {
                    println!("Successfully downloaded and extracted files to {:?}", dir_path);
                }
            }
            
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::ApexPro, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::ApexPro, stream_handle, config);
        }

        // EG Orea
        1 => {
            let dir_path: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::EgOrea));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EgOrea));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKzT3eW2mI0VLjinckoMapKPTDs9qeQYlW63F8",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EgOrea));
                unzip_sounds(&zip_path, &dir_path).map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path).map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!("Successfully downloaded and extracted files to {:?}", dir_path);
                }
            }

           
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::EgOrea, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }
            listen_and_play(debug, &SoundFiles::EgOrea, stream_handle, config);
        }

        // Fall Out
        2 => {
            let dir_path: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::FallOut));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::FallOut));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKzT3eW2mI0VLjinckoMapKPTDs9qeQYlW63F8",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::FallOut));
                unzip_sounds(&zip_path, &dir_path).map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path).map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!("Successfully downloaded and extracted files to {:?}", dir_path);
                }
            }
           
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::FallOut, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::FallOut, stream_handle, config);
        }
        a => Err(PulseErrors::UnwantedSelectionIndex { index: *a })?,
    }
    Ok(())
}
