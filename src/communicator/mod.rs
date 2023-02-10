use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Eof {
    pub cycle: String,
    pub eol: serde_json::Value,
    pub latest: String,
    pub latest_release_date: Option<String>,
    pub release_date: String,
    pub lts: serde_json::Value,
}

pub async fn lib_eof(product: &str) -> Result<Vec<Eof>, Error> {
    let request_url = format!("https://endoflife.date/api/{}.json", product);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?.text().await?;
    // let json_body: serde_json::Value = serde_json::from_str(&response).unwrap();
    let json_body: Vec<Eof> = serde_json::from_str(&response).unwrap();

    println!("{:?} :: {:?}", response, &json_body);
    Ok(json_body)
}

pub async fn lib_list() -> Result<Vec<String>, Error> {
    let request_url = format!("https://endoflife.date/api/all.json");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?.text().await?;
    // let json_body: serde_json::Value = serde_json::from_str(&response).unwrap();
    let json_body: Vec<String> = serde_json::from_str(&response).unwrap();

    println!("{:?} :: {:?}", response, &json_body);
    Ok(json_body)
}
