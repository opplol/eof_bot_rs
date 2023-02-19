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
    let response = match reqwest::get(&request_url).await {
        Ok(res) => match res.error_for_status() {
            Ok(res) => res,
            Err(err) => {
                return Err(err);
            }
        },
        Err(e) => {
            return Err(e);
        }
    };

    let body = response.text().await.unwrap();
    println!("RESPONSE :: {:?}", body);

    // let json_body: serde_json::Value = serde_json::from_str(&response).unwrap();
    let json_body: Vec<Eof> = serde_json::from_str(&body).unwrap();

    println!("{:?} :: {:?}", body, &json_body);
    Ok(json_body)
}

pub async fn lib_list() -> Result<Vec<String>, Error> {
    let request_url = "https://endoflife.date/api/all.json".to_string();
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?.text().await?;
    // let json_body: serde_json::Value = serde_json::from_str(&response).unwrap();
    let json_body: Vec<String> = serde_json::from_str(&response).unwrap();

    println!("{:?} :: {:?}", response, &json_body);
    Ok(json_body)
}
