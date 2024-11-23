pub enum SoundFiles {
    FallOut,
    EgOrea,
    ApexPro,
}

impl SoundFiles {
    pub fn get_name(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::ApexPro => "Apex Pro".to_string(),
            SoundFiles::EgOrea => "EG Orea".to_string(),
            SoundFiles::FallOut => "Fall Out".to_string(),
        }
    }

    pub fn get_zip_path(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::ApexPro => "apex_pro.zip".to_string(),
            SoundFiles::EgOrea => "eg_orea.zip".to_string(),
            SoundFiles::FallOut => "fall_out.zip".to_string(),
        }
    }

    pub fn get_extract_dir(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::ApexPro => "APEX".to_string(),
            SoundFiles::EgOrea => "EGOREA".to_string(),
            SoundFiles::FallOut => "FALLOUT".to_string(),
        }
    }
}
