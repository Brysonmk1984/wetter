#![allow(dead_code)]
mod error;
mod service;
mod settings;

use std::{env, str::FromStr};

use dotenv::dotenv;
use error::AcceptableError;
use service::{current_weather::CurrentWeatherRequest, models::RequestType};

use crate::service::{forecast_weather::ForecastWeatherRequest, request::WeatherRequest};

#[tokio::main]
async fn main() -> Result<(), AcceptableError> {
    dotenv().ok();

    let args = env::args()
        .enumerate()
        .filter_map(|(i, val): (usize, String)| {
            if i != 0 {
                return Some(val);
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    println!("{:?}", args);

    handle_inputs(args).await?;

    Ok(())
}

async fn handle_inputs(mut args: Vec<String>) -> Result<(), AcceptableError> {
    if args.len() == 0 {
        args.push(String::from("Current"));
    };

    let request_type = args[0].as_str();
    let request_type_as_enum = RequestType::from_str(request_type);

    match request_type_as_enum {
        Ok(RequestType::Current) => {
            let request = CurrentWeatherRequest::new();
            let current_weather = request.get().await?;

            println!("{}", current_weather);
        }
        Ok(RequestType::Forecast) => {
            let request = ForecastWeatherRequest::new();
            let forecast_weather = request.get().await?;

            println!("{}", forecast_weather);
        }
        Err(error) => {
            println!("ERROR: {}", error)
        }
    }

    Ok(())
}
