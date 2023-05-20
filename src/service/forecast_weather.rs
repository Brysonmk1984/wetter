use async_trait::async_trait;
use serde::Deserialize;

use crate::error::AcceptableError;

use super::{
    models::{RequestType, WeatherDescription},
    request::{verify_response, WeatherRequest},
};

pub struct ForecastWeatherRequest {}

impl ForecastWeatherRequest {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl WeatherRequest for ForecastWeatherRequest {
    async fn get(&self) -> Result<String, AcceptableError> {
        let url = self.build_url(RequestType::Forecast);
        // "https://api.weatherbit.io/v2.0/current?key=ea74dca6df3c42c590a077ff2568048c&city=laramie,Wyoming&units=I";

        let body = reqwest::get(url).await?;
        let status = &body.status().as_u16();

        verify_response(status)?;

        //let val = body.text().await?;
        //println!("{}", val);

        let ForecastWeatherResponse { data } = body.json::<ForecastWeatherResponse>().await?;
        // let ForecastWeather {
        //     temp,
        //     app_temp,
        //     precip,
        //     wind_spd,
        //     wind_cdir,
        //     gust,
        //     city_name,
        //     ..
        // } = &data[0];

        // let result = format!(
        //     "TEMP: {}\nFEELS LIKE: {}\nPRECIP: {}\nWIND: {}\nGUST: {} {}\nLOCATION: {}",
        //     temp,
        //     app_temp,
        //     precip.unwrap_or(0.0),
        //     wind_spd,
        //     gust,
        //     wind_cdir,
        //     city_name
        // );

        Ok(String::from("asd"))
    }
}

#[derive(Debug, Deserialize)]
struct ForecastWeatherResponse {
    data: Vec<ForecastWeather>,
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
