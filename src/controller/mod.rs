use crate::communicator;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
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
    _user: Option<String>,
    channel: String,
}

#[derive(Serialize, Debug, Clone)]
struct SlackTextSection {
    #[serde(rename = "type")]
    type_me: String,
    text: SlackTextBody,
}

#[derive(Serialize, Debug, Clone)]
struct SlackTextMultiSection {
    #[serde(rename = "type")]
    type_me: String,
    fields: Vec<SlackTextBody>,
}

#[derive(Serialize, Debug, Clone)]
struct SlackTextBody {
    #[serde(rename = "type")]
    type_me: String,
    text: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
enum SlackTexts {
    SlackTextMultiSection(SlackTextMultiSection),
    SlackTextSection(SlackTextSection),
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("echo")]
pub async fn echo(req_body: String) -> impl Responder {
    info!("Requsted: {:?}", &req_body);
    let body_json: BotRequest = serde_json::from_str(&req_body).unwrap();

    info!("BodyJson: {:?}", &body_json);

    if let Some(challenge) = &body_json.challenge {
        return HttpResponse::Ok().body(challenge.to_string());
    }

    HttpResponse::Ok().body("Error")
}
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There")
}

#[post("eof")]
async fn eof(req_body: web::Json<BotRequest>) -> impl Responder {
    info!("Requsted: {:?}", &req_body);
    if let Some(challenge) = &req_body.challenge {
        return HttpResponse::Ok().body(challenge.to_string());
    }

    let event = &req_body.event.as_ref().unwrap();

    let recieve_text = &event.text.clone();
    // info!("{:?}", gist);
    let re = Regex::new(r"<.*>").unwrap();
    let mut formated_text = re.replace(recieve_text, "").to_string();
    formated_text = formated_text.trim().to_string();
    // info!("{:?}", &recieve_text);

    if formated_text == "all" {
        let lib_all = match communicator::lib_list().await {
            Ok(v) => v,
            Err(err) => {
                info!("{}", err.to_string());
                vec![]
            }
        };
        let slack_text: Vec<SlackTextSection> = lib_all[0..20]
            .iter()
            .map(|lib| SlackTextSection {
                type_me: "section".to_string(),
                text: SlackTextBody {
                    type_me: "plain_text".to_string(),
                    text: lib.to_string(),
                },
            })
            .collect();
        let footer: Vec<SlackTexts> = vec![SlackTexts::SlackTextSection(SlackTextSection {
            type_me: "section".to_string(),
            text: SlackTextBody {
                type_me: "mrkdwn".to_string(),
                text: format!("*<https://endoflife.date|Show more info>*", formated_text),
            },
        })];
        let slack_body = json!({
            "channel": Some(&event.channel).unwrap(),
            "text": format!("ALL Support Product in EOL"),
            "blocks": slack_text,
        });
        info!("SLACK_BODY :: {}", slack_body);
        let mytoken = env::var("SLACK_TOKEN").unwrap_or_default();
        let request_url = "https://slack.com/api/chat.postMessage";
        let _response = Client::new()
            .post(request_url)
            .bearer_auth(mytoken)
            .json(&slack_body)
            .send()
            .await
            .unwrap();

        return HttpResponse::Ok().body("");
    }

    let users = match communicator::lib_eof(&formated_text).await {
        Ok(r) => r,
        Err(err) => {
            info!("{}", err.to_string());
            vec![]
        }
    };

    let slack_text: Vec<SlackTexts> = users
        .iter()
        .map(|info| {
            let body: Vec<SlackTextBody> = vec![
                SlackTextBody {
                    type_me: "mrkdwn".to_string(),
                    text: format!("*Version*\n {}", info.cycle),
                },
                SlackTextBody {
                    type_me: "mrkdwn".to_string(),
                    text: format!("*EOL* \n {}", info.eol),
                },
                SlackTextBody {
                    type_me: "mrkdwn".to_string(),
                    text: format!("*lastVersion* \n {}", info.latest),
                },
            ];
            SlackTexts::SlackTextMultiSection(SlackTextMultiSection {
                type_me: "section".to_string(),
                fields: body,
            })
        })
        .collect();
    // info!("{:?}", slack_text);

    // let test = test(&info.app).await.unwrap_or_default();
    println!("{:?}", users);

    let title: Vec<SlackTexts> = vec![SlackTexts::SlackTextSection(SlackTextSection {
        type_me: "section".to_string(),
        text: SlackTextBody {
            type_me: "plain_text".to_string(),
            text: format!("{} EOF INFO", &formated_text),
        },
    })];

    let footer: Vec<SlackTexts> = vec![SlackTexts::SlackTextSection(SlackTextSection {
        type_me: "section".to_string(),
        text: SlackTextBody {
            type_me: "mrkdwn".to_string(),
            text: format!(
                "*<https://endoflife.date/{}|Show more info>*",
                formated_text
            ),
        },
    })];

    let all_text: Vec<SlackTexts> = [title, slack_text, footer].concat();
    let gist_body = json!({
        "channel": Some(&event.channel).unwrap(),
        "text": format!("{} EOF INFO", formated_text),
        "blocks": all_text,
    });
    // info!("Slack Body: {}", gist_body);
    // info!("Slack Body: {:?}", gist_body);

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
