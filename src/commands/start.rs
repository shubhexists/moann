use crate::{
    config::SoundPack,
    constants::FILE_PATH,
    errors::PulseErrors,
    play_sound::listen_and_play,
    sounds::SoundFiles,
    utils::{download_file, unzip_sounds},
};
use dialoguer::{theme::ColorfulTheme, Select};
use rodio::OutputStreamHandle;
use std::path::PathBuf;
use std::{error::Error, fs, io};

pub fn start(debug: bool, stream_handle: OutputStreamHandle) -> Result<(), Box<dyn Error>> {
    let selection_array: Vec<String> = vec![
        SoundFiles::get_name(&SoundFiles::ApexPro),
        SoundFiles::get_name(&SoundFiles::EgOrea),
        SoundFiles::get_name(&SoundFiles::FallOut),
        SoundFiles::get_name(&SoundFiles::Ahegao),
        SoundFiles::get_name(&SoundFiles::AnimeMoan),
        SoundFiles::get_name(&SoundFiles::Bruh),
        SoundFiles::get_name(&SoundFiles::EightBit),
        SoundFiles::get_name(&SoundFiles::ACNL)
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
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::ApexPro));
            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ApexPro));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKWDOt4d6pqQg5G0DIVMYr8sEyXJfZvWjaxmUz",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ApexPro));
                unzip_sounds(&zip_path, &dir_path)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
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
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::EgOrea));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EgOrea));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKyfs4UbnDO3HC1SAaFYcT5QKPzN4dxUG9bpEq",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EgOrea));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
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
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::FallOut));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::FallOut));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKiaZz5PCUoHzBesmgDYlfWx4Fa5r37bXCZ6M2",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::FallOut));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::FallOut, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::FallOut, stream_handle, config);
        }

        // AHEGAO
        3 => {
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::Ahegao));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::Ahegao));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKrZRP8iFquAJs04y6BeIU1TMxaPKOLH3nvpcE",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::Ahegao));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::Ahegao, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::Ahegao, stream_handle, config);
        }

        // Anime Moan
        4 => {
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::AnimeMoan));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::AnimeMoan));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKBxjs13GSwAGHVgyWLb1KfhTQnpj2lukqic4x",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::AnimeMoan));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::AnimeMoan, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::AnimeMoan, stream_handle, config);
        }

        // Bruh
        5 => {
            let dir_path: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::Bruh));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::Bruh));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiKozyGBwWPe2hzLd9KCZjNHkMQ6gaWqxSfnr3Y",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::Bruh));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::Bruh, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::Bruh, stream_handle, config);
        }

        // Eight Bit
        6 => {
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::EightBit));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EightBit));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiK5rUlaFo21GE0KcWTnjNMYwlR6QbJP8vIqiVh",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::EightBit));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::EightBit, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::EightBit, stream_handle, config);
        }

        // ACNL
        7 => {
            let dir_path: PathBuf =
                FILE_PATH.join(SoundFiles::get_extract_dir(&SoundFiles::ACNL));

            if dir_path.exists() {
                if debug {
                    println!("Directory already exists: {:?}", dir_path);
                }
            } else {
                let file: PathBuf = FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ACNL));
                download_file(
                    "https://utfs.io/f/a6gjLUEvAeiK1gOzn09GODxCrvfnhHFEzIP4VBtMumRdYSgy",
                    &file,
                )
                .map_err(|err: Box<dyn Error>| PulseErrors::UnableToDownloadFile { err })?;

                let zip_path: PathBuf =
                    FILE_PATH.join(SoundFiles::get_zip_path(&SoundFiles::ACNL));
                let output_dir: PathBuf = PathBuf::from(FILE_PATH.to_str().unwrap());
                unzip_sounds(&zip_path, &output_dir)
                    .map_err(|err: std::io::Error| PulseErrors::UnzipError { err })?;
                fs::remove_file(zip_path)
                    .map_err(|err: io::Error| PulseErrors::RemoveFileError { err })?;

                if debug {
                    println!(
                        "Successfully downloaded and extracted files to {:?}",
                        dir_path
                    );
                }
            }

            let config: SoundPack = SoundPack::parse_config_file(&SoundFiles::ACNL, debug)?;
            if debug {
                println!("Config generated: {:?}", config);
            }

            listen_and_play(debug, &SoundFiles::ACNL, stream_handle, config);
        }

        a => Err(PulseErrors::UnwantedSelectionIndex { index: *a })?,
    }
    Ok(())
}
