use error::CombinedError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, process};

#[derive(Serialize, Deserialize, Debug)]
struct RAAMOrigin {
    name: String,
    url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct RAAMLocation {
    name: String,
    url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct RickAndMortyResult {
    id: u16,
    name: String,
    status: String,
    species: String,
    r#type: String,
    gender: String,
    url: String,
    created: String,
    image: String,
    episode: Vec<String>,
    location: RAAMOrigin,
    origin: RAAMLocation,
}
#[derive(Serialize, Deserialize, Debug)]
struct DeserializedResponse {
    info: Info,
    results: Vec<RickAndMortyResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    count: u16,
    next: String,
    pages: u16,
    prev: Option<u16>,
}

async fn make_request() -> Result<Vec<RickAndMortyResult>, CombinedError> {
    let body: DeserializedResponse = reqwest::get("https://rickandmortyapi.com/api/character")
        .await?
        .json()
        .await?;

    // Parse the string of data into serde_json::Value.
    //let v: Info = serde_json::from_str(&data)?;

    Ok(body.results)
}

fn getRmData() {
    let response_result = make_request().await;

    match response_result {
        Ok(results) => println!("{:?}", results[1].name),
        Err(error) => {
            println!("Error - {:?}", error);
            process::exit(1)
        }
    }
}
