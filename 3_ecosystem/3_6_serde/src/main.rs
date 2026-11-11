use std::{fs::File, io::{self, Write, BufReader}};
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(rename = "type")]
    response_type: String,
    stream: StreamInfo,
    gifts: Vec<Gift>,
    debug: DebugInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gift {
    id: usize,
    price: usize,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DebugInfo {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamInfo {
    user_id: Uuid,
    is_private: bool,
    settings: usize,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    id: usize,
    price: usize,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    client_price: usize,
    #[serde(with="humantime_serde")]
    duration: Duration,
    description: String,
}

fn main() -> io::Result<()> {
    let file = File::open("request.json")?;
    let reader = BufReader::new(file);
    let parsed: Request = serde_json::from_reader(reader)?;

    println!("{:#?}", parsed);

    let toml_string = toml::to_string_pretty(&parsed)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut toml_file = File::create("request.toml")?;
    toml_file.write_all(toml_string.as_bytes())?;

    let yaml_string = serde_yaml::to_string(&parsed)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut yaml_file = File::create("request.yaml")?;
    yaml_file.write_all(yaml_string.as_bytes())?;

    Ok(())
}
