use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, io, path::PathBuf};
use crate::{constants::FILE_PATH, errors::PulseErrors, sounds::SoundFiles};

#[derive(Deserialize, Debug)]
#[serde(untagged)] 
pub enum Defines {
    U64HashMap(HashMap<String, Vec<u64>>),
    StringHashMap(HashMap<String, String>),
}

#[derive(Deserialize, Debug)]
pub struct SoundPack {
    pub id: String,
    pub name: String,
    pub key_define_type: String,
    pub sound: String,
    pub defines: Defines,
}

impl SoundPack {
    pub fn parse_config_file(sound_type: &SoundFiles) -> Result<Self, Box<dyn Error>> {
        let sound_dir: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(sound_type)); 
        let config_file: PathBuf = sound_dir.join("config.json");
        let json_string: String = fs::read_to_string(config_file).map_err(|err: io::Error| PulseErrors::JSONReadError{err})?;
        Ok(serde_json::from_str(&json_string).map_err(|err: serde_json::Error|PulseErrors::JSONParseError { err } )?)
    }
}