use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use log::info;
use regex::Regex;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

// #[derive(Deserialize)]
// struct EofQuery {
//     app: String,
// }
#[derive(Deserialize, Debug, Default)]
struct BotRequest {
    token: Option<String>,
    challenge: Option<String>,
    team_id: Option<String>,
    api_app_id: Option<String>,
    event: Option<SlackEvent>,
}
#[derive(Deserialize, Debug)]
struct SlackEvent {
    text: String,
    user: String,
    channel: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
#[post("echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There")
}

#[post("eof")]
async fn eof(req_body: web::Json<BotRequest>) -> impl Responder {
    if let Some(challenge) = &req_body.challenge {
        return HttpResponse::Ok().body(challenge.to_string());
    }
    info!("Requsted: {:?}", &req_body);

    let event = &req_body.event.as_ref().unwrap();

    let app_name = &event.text.clone();
    // info!("{:?}", gist);
    let re = Regex::new(r"<.*>").unwrap();
    let mut con_app_name = re.replace(app_name, "").to_string();
    con_app_name = con_app_name.trim().to_string();
    info!("{:?}", &app_name);

    let users = match test(&con_app_name).await {
        Ok(r) => r,
        Err(err) => {
            info!("{}", err.to_string());
            vec![]
        }
    };

    // let test = test(&info.app).await.unwrap_or_default();
    println!("{:?}", users);

    let gist_body = json!({
        "channel": Some(&event.channel).unwrap(),
        "text": users
    });

    let mytoken = "MYTOKEN";
    let request_url = "https://slack.com/api/chat.postMessage";
    let _response = Client::new()
        .post(request_url)
        .bearer_auth(mytoken)
        .json(&gist_body)
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("{:?}", Some(&req_body)))
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    cycle: String,
    eol: serde_json::Value,
    latest: String,
    latest_release_date: String,
    release_date: String,
    lts: serde_json::Value,
}

async fn test(product: &str) -> Result<Vec<User>, Error> {
    let request_url = format!("https://endoflife.date/api/{}.json", product);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?.text().await?;
    // let json_body: serde_json::Value = serde_json::from_str(&response).unwrap();
    let json_body: Vec<User> = serde_json::from_str(&response).unwrap();
    for a in &json_body {
        println!("{:?}", a.eol)
    }

    println!("{:?} :: {:?}", response, &json_body);
    // &json_body.iter().for_each(|a| {
    //     println!(
    //         "Cycle: {}, eol: {:?}",
    //         a.cycle,
    //         a.eol.as_bool().unwrap_or_default()
    //     );
    //     println!(
    //         "Cycle: {}, eol: {:?}",
    //         a.cycle,
    //         a.eol.as_str().unwrap_or_default()
    //     );
    // });
    Ok(json_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .service(eof)
            .route("hey", web::get().to(manual_hello))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
