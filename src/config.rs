use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(untagged)] 
enum Defines {
    U64HashMap(HashMap<String, Vec<u64>>),
    StringHashMap(HashMap<String, String>),
}

#[derive(Deserialize, Debug)]
struct SoundPack {
    id: String,
    name: String,
    defines: Defines,
}

impl SoundPack {
    
}