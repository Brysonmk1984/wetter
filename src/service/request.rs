use std::env;

use async_trait::async_trait;

use crate::{
    error::AcceptableError,
    settings::{read_settings, UserSettings},
};

use super::models::RequestType;

#[derive(Debug)]
pub struct APICredentials {
    key: String,
    domain: String,
}

impl APICredentials {
    pub fn new() -> Self {
        APICredentials {
            key: env::var("API_KEY").expect("API_KEY is not set"),
            domain: env::var("API_URL").expect("API_URL is not set"),
        }
    }

    pub fn get(&self) -> (&String, &String) {
        (&self.domain, &self.key)
    }
}

#[async_trait]
pub trait WeatherRequest {
    async fn get(&self, days: Option<u8>) -> Result<String, AcceptableError>;

    fn read_settings() {
        let settings = read_settings();

        println!("{:?}", settings);
    }

    fn build_url(&self, request_variant: RequestType, days: Option<u8>) -> String {
        let api_credentials = APICredentials::new();
        let (domain, key) = api_credentials.get();

        let user_settings = read_settings();
        let UserSettings {
            city,
            state,
            country_code,
        } = user_settings;

        let request_variant_url_part = match request_variant {
            RequestType::Current => format!("{}?", request_variant.to_string()),
            RequestType::Forecast => format!(
                "{}/daily?days={}&",
                request_variant.to_string(),
                days.unwrap_or(5)
            ),
        };

        let url = format!(
            "{}{}key={}&city={}&state={}&country={}&units=I",
            domain, request_variant_url_part, key, city, state, country_code
        );

        url
    }
}

pub fn verify_response(status: &u16) -> Result<(), AcceptableError> {
    if (400..499).contains(status) {
        Err(AcceptableError::ClientRequestError(*status))
    } else if *status == 500 {
        Err(AcceptableError::ServerResponseError(*status))
    } else {
        Ok(())
    }
}
