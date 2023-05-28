use serde::Deserialize;
use strum_macros::{Display, EnumString};

pub enum ArgType {
    OptionalDays(Option<u8>),
    RequestType(RequestType),
}

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
