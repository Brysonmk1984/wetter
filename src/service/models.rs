use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum RequestType {
    #[strum(serialize = "current")]
    Current,
    #[strum(serialize = "forecast")]
    Forecast,
}

#[derive(Debug, Deserialize)]
pub struct WeatherDescription {
    icon: String,
    code: i16,
    pub description: String,
}
