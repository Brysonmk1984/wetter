#![allow(dead_code)]
mod error;
mod service;
mod settings;

use std::env;

use dotenv::dotenv;
use error::CombinedError;
use service::{
    current_weather::CurrentWeatherRequest,
    models::{ArgType, RequestType},
};

use crate::service::{forecast_weather::ForecastWeatherRequest, request::WeatherRequest};

#[tokio::main]
async fn main() -> Result<(), CombinedError> {
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

    handle_inputs(args).await?;

    Ok(())
}

async fn handle_inputs(mut args: Vec<String>) -> Result<(), CombinedError> {
    if args.len() == 0 {
        args.push(String::from("current"));
    };

    let first_arg = reformat_arg(&args[0], 0, None)?;
    let mut second_arg: Option<ArgType> = None;
    if args.len() > 1 {
        second_arg = Some(reformat_arg(&args[1], 1, Some(&first_arg))?);
    }

    match first_arg {
        ArgType::RequestType(RequestType::Current) => {
            let request = CurrentWeatherRequest::new();
            let current_weather = request.get(None).await?;

            println!("{}", current_weather);
        }
        ArgType::RequestType(RequestType::Forecast) => {
            let request = ForecastWeatherRequest::new();

            let mut days: u8 = 5;
            if let Some(ArgType::OptionalDays(Some(any))) = second_arg {
                days = any;
            }

            let forecast_weather = request.get(Some(days)).await?;

            println!("{}", forecast_weather);
        }
        _ => {}
    }

    Ok(())
}

fn reformat_arg(
    arg: &String,
    position: u8,
    prev_arg: Option<&ArgType>,
) -> Result<ArgType, CombinedError> {
    match position {
        0 => {
            return match arg.as_str() {
                "current" | "--c" | "--C" | "-current" | "Current" => {
                    Ok(ArgType::RequestType(RequestType::Current))
                }
                "forecast" | "--f" | "--F" | "-forecast" | "Forecast" => {
                    Ok(ArgType::RequestType(RequestType::Forecast))
                }
                _ => Err(CombinedError::InvalidArgument(
                    "Did not recognize that command!".to_string(),
                )),
            };
        }
        1 => {
            let result = arg.parse::<u8>();

            match result {
                Ok(days) => {
                    if let Some(prev) = prev_arg {
                        if let ArgType::RequestType(RequestType::Forecast) = prev {
                            return Ok(ArgType::OptionalDays(Some(days)));
                        } else {
                            Err(CombinedError::InvalidArgument(String::from(
                                "Days argument only applies to forecasts!",
                            )))
                        }
                    } else {
                        return Ok(ArgType::OptionalDays(None));
                    }
                }
                Err(error) => Err(CombinedError::ParseError(error)),
            }
        }
        _ => Err(CombinedError::TooManyArguments("too many args".to_string())),
    }
}
