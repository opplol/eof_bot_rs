use crate::communicator;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

use strsim::normalized_damerau_levenshtein;
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

#[post("eol")]
async fn eol(req_body: web::Json<BotRequest>) -> impl Responder {
    info!("Requsted: {:?}", &req_body);
    if let Some(challenge) = &req_body.challenge {
        return HttpResponse::Ok().body(challenge.to_string());
    }

    let event = &req_body.event.as_ref().unwrap();
    let recieve_text = &event.text.clone();
    // info!("{:?}", &recieve_text);
    let reg = Regex::new(r"<.*>").unwrap();
    let mut formated_text = reg.replace(recieve_text, "").to_string();
    formated_text = formated_text.trim().to_string();

    let all_footer: Vec<SlackTexts> = vec![SlackTexts::SlackTextSection(SlackTextSection {
        type_me: "section".to_string(),
        text: SlackTextBody {
            type_me: "mrkdwn".to_string(),
            text: "*<https://endoflife.date|Show more info>*".to_string(),
        },
    })];
    if formated_text == "all" {
        let lib_all = match communicator::lib_list().await {
            Ok(v) => v,
            Err(err) => {
                info!("{}", err.to_string());
                vec![]
            }
        };
        let slack_text: Vec<SlackTexts> = lib_all[0..10]
            .iter()
            .map(|lib| {
                SlackTexts::SlackTextSection(SlackTextSection {
                    type_me: "section".to_string(),
                    text: SlackTextBody {
                        type_me: "plain_text".to_string(),
                        text: lib.to_string(),
                    },
                })
            })
            .collect();
        let all_text: Vec<SlackTexts> = [slack_text, all_footer].concat();
        let slack_body = json!({
            "channel": Some(&event.channel).unwrap(),
            "text": format!("ALL Support Product in EOL"),
            "blocks": all_text,
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

    let eol_res = match communicator::lib_eol(&formated_text).await {
        Ok(r) => r,
        Err(err) => {
            info!("{}", err.to_string());

            let lib_all = match communicator::lib_list().await {
                Ok(v) => v,
                Err(err) => {
                    info!("{}", err.to_string());
                    vec![]
                }
            };
            // lib_all.iter().for_each(|lib| {
            //     println!(
            //         "{} ** {} : {:?}",
            //         formated_text,
            //         lib,
            //         normalized_damerau_levenshtein(&formated_text, lib)
            //     )
            // });
            let suggest_libs: Vec<String> = lib_all
                .into_iter()
                .filter(|lib| normalized_damerau_levenshtein(&formated_text, lib) >= 0.4)
                .collect();
            println!("Suggest_lib{:?}", suggest_libs);
            let slack_text: Vec<SlackTexts> =
                vec![SlackTexts::SlackTextSection(SlackTextSection {
                    type_me: "section".to_string(),
                    text: SlackTextBody {
                        type_me: "plain_text".to_string(),
                        text: suggest_libs.join("、"),
                    },
                })];
            let title: Vec<SlackTexts> = vec![SlackTexts::SlackTextSection(SlackTextSection {
                type_me: "section".to_string(),
                text: SlackTextBody {
                    type_me: "plain_text".to_string(),
                    text: "Did You Mean?".to_string(),
                },
            })];

            let all_text: Vec<SlackTexts> = [title, slack_text, all_footer].concat();
            let slack_body = json!({
                "channel": Some(&event.channel).unwrap(),
                "text": format!("Did you mean theses?"),
                "blocks": all_text,
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
    };

    let slack_text: Vec<SlackTexts> = eol_res
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
    println!("{:?}", eol_res);

    let title: Vec<SlackTexts> = vec![SlackTexts::SlackTextSection(SlackTextSection {
        type_me: "section".to_string(),
        text: SlackTextBody {
            type_me: "plain_text".to_string(),
            text: format!("{} EOL INFO", &formated_text),
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
    let slack_body = json!({
        "channel": Some(&event.channel).unwrap(),
        "blocks": all_text,
    });
    // info!("Slack Body: {}", gist_body);
    // info!("Slack Body: {:?}", gist_body);

    let mytoken = env::var("SLACK_TOKEN").unwrap_or_default();
    let request_url = "https://slack.com/api/chat.postMessage";
    let _response = Client::new()
        .post(request_url)
        .bearer_auth(mytoken)
        .json(&slack_body)
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("{:?}", Some(&req_body)))
}
