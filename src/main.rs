use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use log::info;
use regex::Regex;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

// #[derive(Deserialize)]
// struct EofQuery {
//     app: String,
// }
#[derive(Deserialize, Debug, Default)]
struct BotRequest {
    _token: Option<String>,
    challenge: Option<String>,
    _team_id: Option<String>,
    _api_app_id: Option<String>,
    event: Option<SlackEvent>,
}
#[derive(Deserialize, Debug)]
struct SlackEvent {
    text: String,
    _user: String,
    channel: String,
}

#[derive(Serialize, Debug, Clone)]
struct SlackTextSection {
    #[serde(rename = "type")]
    type_me: String,
    text: SlackTextBody,
}

#[derive(Serialize, Debug, Clone)]
struct SlackTextBody {
    #[serde(rename = "type")]
    type_me: String,
    text: String,
}

// {
//     "type": "section",
//     "text": {
//         "type": "mrkdwn",
//         "text": format!("*Version : {}*\n EOL: {}\n lastVersion: {} \n", &users[0].cycle, &users[0].eol, &users[0].latest)
//     },
// },

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

    let slack_text: Vec<SlackTextSection> = users
        .iter()
        .map(|info| {
            let body = SlackTextBody {
                type_me: "mrkdwn".to_string(),
                text: format!(
                    "*Version : {}*\n EOL: {}\n lastVersion: {} \n",
                    info.cycle, info.eol, info.latest
                ),
            };
            SlackTextSection {
                type_me: "section".to_string(),
                text: body,
            }
        })
        .collect();
    info!("{:?}", slack_text);

    // let test = test(&info.app).await.unwrap_or_default();
    println!("{:?}", users);

    let title: Vec<SlackTextSection> = vec![SlackTextSection {
        type_me: "section".to_string(),
        text: SlackTextBody {
            type_me: "plain_text".to_string(),
            text: format!("{} EOF INFO", &con_app_name),
        },
    }];

    let footer: Vec<SlackTextSection> = vec![SlackTextSection {
        type_me: "section".to_string(),
        text: SlackTextBody {
            type_me: "mrkdwn".to_string(),
            text: format!(
                "*<https://endoflife.date/{}|Show more info>*",
                &con_app_name
            ),
        },
    }];

    let all_text = [title, slack_text, footer].concat();
    let gist_body = json!({
        "channel": Some(&event.channel).unwrap(),
        "text": format!("{} EOF INFO", &con_app_name),
        "blocks": all_text,
    // "blocks": [
    //     {
    //         "type": "section",
    //         "text": {
    //             "type": "plain_text",
    //             "emoji": true,
    //             "text": format!("{} EOF INFO", &con_app_name)
    //         }
    //     },
        // {
        //     "type": "section",
        //     "text": {
        //         "type": "mrkdwn",
        //         "text": format!("*<https://endoflife.date/{}|Show more info>*", &con_app_name)
        //     }
        // }
    // ]

    });

    let mytoken = env::var("SLACK_TOKEN").unwrap_or_default();
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
    latest_release_date: Option<String>,
    release_date: String,
    lts: serde_json::Value,
}

async fn test(product: &str) -> Result<Vec<User>, Error> {
    let request_url = format!("https://endoflife.date/api/{}.json", product);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?.text().await?;
    // let json_body: serde_json::Value = serde_json::from_str(&response).unwrap();
    let json_body: Vec<User> = serde_json::from_str(&response).unwrap();

    println!("{:?} :: {:?}", response, &json_body);
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
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
