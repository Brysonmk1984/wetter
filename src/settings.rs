use std::{fs::File, io, io::BufRead};

#[derive(Debug)]
pub struct UserSettings {
    pub city: String,
    pub state: String,
    pub country_code: String,
}

pub fn read_settings() -> UserSettings {
    let settings_result = File::open("./user-settings.txt".to_string()).unwrap();

    //println!("{:?}", settings_result);

    let buffer = io::BufReader::new(settings_result).lines();

    let settings_vec: Vec<String> = buffer
        .into_iter()
        .map(|line| line.unwrap().split("=").last().unwrap().to_string())
        .collect();

    let settings = UserSettings {
        city: settings_vec[0].clone(),
        state: settings_vec[1].clone(),
        country_code: settings_vec[2].clone(),
    };

    settings
}
