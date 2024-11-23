use std::{error::Error, fs, io};
use crate::{config::SoundPack, constants::FILE_PATH, errors::PulseErrors, play_sound::listen_and_play, sounds::SoundFiles, utils::{download_file, unzip_sounds}};
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;
// use crate::utils::listen_and_play;
use rodio::OutputStreamHandle;

pub fn start(debug: bool, stream_handle: OutputStreamHandle) -> Result<(), Box<dyn Error>> {
    // listen_and_play(
    //     debug,
    //     "/home/jerry/Desktop/projects/sound/mech-keyboard-02-102918.mp3",
    //     stream_handle,
    // );

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
            // Check if directory exists
            let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ApexPro));
            download_file(
                "https://utfs.io/f/a6gjLUEvAeiKWDOt4d6pqQg5G0DIVMYr8sEyXJfZvWjaxmUz",
                &file,
            )
            .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

            let zip_path: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ApexPro));
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::ApexPro));
            unzip_sounds(&zip_path, &dir_path).map_err(|err: std::io::Error| PulseErrors::UnzipError { err } )?;
            fs::remove_file(zip_path).map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::ApexPro)?;
            listen_and_play(debug, &SoundFiles::ApexPro, stream_handle, config);
        }

        // EG Orea
        1 => {
            let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EgOrea));
            download_file(
                "https://utfs.io/f/a6gjLUEvAeiKzT3eW2mI0VLjinckoMapKPTDs9qeQYlW63F8",
                &file,
            )
            .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

            let zip_path: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EgOrea));
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::EgOrea));
            unzip_sounds(&zip_path, &dir_path).map_err(|err: std::io::Error| PulseErrors::UnzipError { err } )?;
            fs::remove_file(zip_path).map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::EgOrea)?;
            listen_and_play(debug, &SoundFiles::ApexPro, stream_handle, config);
        }

        // Fall Out
        2 => {
            let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::FallOut));
            download_file(
                "https://utfs.io/f/a6gjLUEvAeiKQ4geCET8GC92hSoEHjldmrxe1INVWTgYtnqB",
                &file,
            )
            .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

            let zip_path: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::FallOut));
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::FallOut));
            unzip_sounds(&zip_path, &dir_path).map_err(|err: std::io::Error| PulseErrors::UnzipError { err } )?;
            fs::remove_file(zip_path).map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;
            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::FallOut)?;
            listen_and_play(debug, &SoundFiles::ApexPro, stream_handle, config);
        }
        a => Err(PulseErrors::UnwantedSelectionIndex { index: *a })?,
    }
    Ok(())
}
