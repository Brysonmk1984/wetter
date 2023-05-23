use super::{
    models::{RequestType, WeatherDescription},
    request::{verify_response, WeatherRequest},
};
use crate::error::AcceptableError;
use async_trait::async_trait;
use serde::Deserialize;

pub struct ForecastWeatherRequest {}

impl ForecastWeatherRequest {
    pub fn new() -> Self {
        Self {}
    }

    fn right_pad(string: String, length: Option<i8>, filler: Option<char>) -> String {
        let mut i: i8 = -1;
        let filler_value = filler.unwrap_or(' ');
        let length_value = length.unwrap_or(23) - string.len() as i8;

        let mut padded_string = string.to_string();

        while i < length_value {
            padded_string = format!("{}{}", padded_string.to_string(), filler_value);
            i += 1;
        }
        padded_string
    }

    fn format_into_rows(data: &Vec<ForecastWeather>, city: String) -> String {
        let day = data
            .iter()
            .map(|day| ForecastWeatherRequest::right_pad(format!("{}", day.datetime), None, None))
            .collect::<Vec<String>>()
            .join("");
        let lows = data
            .iter()
            .map(|day| {
                ForecastWeatherRequest::right_pad(format!("LOW: {:.0}", day.low_temp), None, None)
            })
            .collect::<Vec<String>>()
            .join("");
        let highs = data
            .iter()
            .map(|day| {
                ForecastWeatherRequest::right_pad(format!("HIGH: {:.0}", day.high_temp), None, None)
            })
            .collect::<Vec<String>>()
            .join("");
        let forecast = data
            .iter()
            .map(|day| {
                ForecastWeatherRequest::right_pad(
                    format!("{}", day.weather.description),
                    None,
                    None,
                )
            })
            .collect::<Vec<String>>()
            .join("");

        format!(
            "LOCATION:{}\n\n{}\n\n{}\n{}\n{}\n",
            city, day, highs, lows, forecast
        )
    }
}

#[async_trait]
impl WeatherRequest for ForecastWeatherRequest {
    async fn get(&self, days: Option<u8>) -> Result<String, AcceptableError> {
        let url = self.build_url(RequestType::Forecast, days);
        let body = reqwest::get(url).await?;
        let status = &body.status().as_u16();

        verify_response(status)?;

        let ForecastWeatherResponse { data, city_name } =
            body.json::<ForecastWeatherResponse>().await?;

        let rows = ForecastWeatherRequest::format_into_rows(&data, city_name);

        Ok(rows)
    }
}

#[derive(Debug, Deserialize)]
struct ForecastWeatherResponse {
    data: Vec<ForecastWeather>,
    city_name: String,
}

#[derive(Debug, Deserialize)]
struct ForecastWeather {
    valid_date: String,
    ts: u64,
    datetime: String,
    wind_gust_spd: f64,
    wind_spd: f64,
    wind_dir: u16,
    wind_cdir: String,
    wind_cdir_full: String,
    temp: f64,
    max_temp: f32,
    min_temp: f32,
    high_temp: f32,
    low_temp: f64,
    app_max_temp: f32,
    app_min_temp: f32,
    pop: u64,
    precip: f64,
    snow: f64,
    snow_depth: f64,
    slp: f64,
    pres: f64,
    dewpt: f64,
    rh: f64,
    weather: WeatherDescription,
    clouds_low: u8,
    clouds_mid: u8,
    clouds_hi: u8,
    clouds: u8,
    vis: f64,
    max_dhi: Option<i16>,
    uv: f64,
    moon_phase: f64,
    moon_phase_lunation: f64,
    moonrise_ts: u64,
    moonset_ts: u64,
    sunrise_ts: u64,
    sunset_ts: u64,
}
