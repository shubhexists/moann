pub enum SoundFiles {
    FallOut,
    EgOrea,
    ApexPro,
    Bruh,
    AnimeMoan,
    Ahegao,
    EightBit,
    ACNL,
}

impl SoundFiles {
    pub fn get_name(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::ApexPro => "Apex Pro".to_string(),
            SoundFiles::EgOrea => "EG Orea".to_string(),
            SoundFiles::FallOut => "Fall Out".to_string(),
            SoundFiles::Bruh => "Bruh".to_string(),
            SoundFiles::AnimeMoan => "Anime Moan".to_string(),
            SoundFiles::Ahegao => "Ahegao".to_string(),
            SoundFiles::EightBit => "Eight Bit".to_string(),
            SoundFiles::ACNL => "ACNL".to_string(),
        }
    }

    pub fn get_zip_path(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::ApexPro => "apex_pro.zip".to_string(),
            SoundFiles::EgOrea => "eg_orea.zip".to_string(),
            SoundFiles::FallOut => "fall_out.zip".to_string(),
            SoundFiles::Bruh => "bruh.zip".to_string(),
            SoundFiles::AnimeMoan => "anime_moan.zip".to_string(),
            SoundFiles::Ahegao => "ahegao.zip".to_string(),
            SoundFiles::EightBit => "eight_bit.zip".to_string(),
            SoundFiles::ACNL => "acnl.zip".to_string(),
        }
    }

    pub fn get_extract_dir(sound: &SoundFiles) -> String {
        match &sound {
            SoundFiles::ApexPro => "APEX".to_string(),
            SoundFiles::EgOrea => "EGOREA".to_string(),
            SoundFiles::FallOut => "FALLOUT".to_string(),
            SoundFiles::Bruh => "BRUH".to_string(),
            SoundFiles::AnimeMoan => "ANIMEMOAN".to_string(),
            SoundFiles::Ahegao => "AHEGAO".to_string(),
            SoundFiles::EightBit => "EIGHTBIT".to_string(),
            SoundFiles::ACNL => "ACNL".to_string(),
        }
    }
}
