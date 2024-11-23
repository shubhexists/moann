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
    pub key_define_type: String,
    pub sound: String,
    pub defines: Option<Defines>,
}

impl SoundPack {
    pub fn parse_config_file(sound_type: &SoundFiles, debug: bool) -> Result<Self, Box<dyn Error>> {
        let sound_dir: PathBuf = FILE_PATH.join(SoundFiles::get_extract_dir(sound_type)); 
        if debug {
            println!("Reading config in: {:?}", sound_dir);
        }
    
        let config_file: PathBuf = sound_dir.join("config.json");
        let json_string: String = fs::read_to_string(&config_file)
            .map_err(|err: io::Error| PulseErrors::JSONReadError { err })?;
        
        if debug {
            println!("Config file content: {}", json_string);
        }
    
        let data: SoundPack = match serde_json::from_str(&json_string) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("JSON parsing error: {:?}", err);
                return Err(Box::new(PulseErrors::JSONParseError { err }));
            }
        };
        Ok(data)
    }    
}