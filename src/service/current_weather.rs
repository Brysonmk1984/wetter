use crate::{
    error::CombinedError,
    service::models::{RequestType, WeatherDescription},
};

use super::request::{verify_response, WeatherRequest};
use async_trait::async_trait;
use serde::Deserialize;

pub struct CurrentWeatherRequest {}

impl CurrentWeatherRequest {
    pub fn new() -> Self {
        CurrentWeatherRequest {}
    }
}

#[async_trait]
impl WeatherRequest for CurrentWeatherRequest {
    async fn get(&self, _: Option<u8>) -> Result<String, CombinedError> {
        let url = self.build_url(RequestType::Current, None);

        let body = reqwest::get(url).await?;
        let status = &body.status().as_u16();

        verify_response(status)?;

        let CurrentWeatherResponse { data, .. } = body.json::<CurrentWeatherResponse>().await?;
        let CurrentWeather {
            temp,
            app_temp,
            precip,
            wind_spd,
            wind_cdir,
            gust,
            city_name,
            ..
        } = &data[0];

        let result = format!(
            "LOCATION: {}\n\nTEMP: {:.0}\nFEELS LIKE: {:.0}\nPRECIP: {}\nWIND: {} {}\nGUST: {}\n",
            city_name,
            temp,
            app_temp,
            precip.unwrap_or(0.0),
            wind_spd,
            wind_cdir,
            gust,
        );

        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
struct CurrentWeatherResponse {
    count: u8,
    data: Vec<CurrentWeather>,
}
#[derive(Debug, Deserialize)]
struct CurrentWeather {
    wind_cdir: String,
    rh: i8,
    pod: char,
    lon: f64,
    pres: f64,
    timezone: String,
    ob_time: String,
    country_code: String,
    clouds: u8,
    vis: f64,
    wind_spd: f64,
    gust: f64,
    wind_cdir_full: String,
    app_temp: f64,
    state_code: String,
    ts: i64,
    h_angle: f64,
    dewpt: f64,
    weather: WeatherDescription,
    uv: f64,
    aqi: u8,
    station: String,
    sources: Vec<String>,
    wind_dir: u16,
    elev_angle: f64,
    datetime: String,
    precip: Option<f64>,
    ghi: f64,
    dni: f64,
    dhi: f64,
    solar_rad: f64,
    city_name: String,
    sunrise: String,
    sunset: String,
    temp: f64,
    lat: f64,
    slp: f64,
}
