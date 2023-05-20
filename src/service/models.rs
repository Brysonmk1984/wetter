use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString)]
pub enum RequestType {
    #[strum(ascii_case_insensitive, serialize = "current")]
    Current,
    #[strum(ascii_case_insensitive, serialize = "forecast")]
    Forecast,
}

#[derive(Debug, Deserialize)]
pub struct WeatherDescription {
    icon: String,
    code: i16,
    description: String,
}
