use std::{fs::File, io::BufReader};

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

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
    duration: String,
    at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamInfo {
    user_id: String,
    is_private: bool,
    settings: usize,
    shard_url: String,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    id: usize,
    price: usize,
    duration: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    client_price: usize,
    duration: String,
    description: String,
}

fn main() {
    let file = File::open("request.json").unwrap();
    let reader = BufReader::new(file);

    let parsed: Request = serde_json::from_reader(reader).unwrap();

    println!("{:#?}", parsed);
}
