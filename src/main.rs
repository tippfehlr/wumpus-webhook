use actix_web::{post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
struct WumpusWebhook {
    webhookTest: bool,
    userId: String,
    botId: String,
    query: serde_json::Value,
}

#[derive(Deserialize)]
struct Query {
    webhook: String,
}

#[post("/")]
async fn handle_webhook(
    query: web::Query<Query>,
    data: web::Json<WumpusWebhook>,
) -> impl Responder {
    println!("request received");

    let client = reqwest::Client::new();
    let body = r#"
                {
                    "content": "",
                    "tts": false,
                    "embeds": [
                        {
                        "id": 10674342,
                        "title": "🔥 New vote! 🔥",
                        "description": "<@{{userId}}> just voted for <@{{botId}}>!\nYou can vote again <t:{{timeStamp}}:R>!",
                        "color": 2326507,
                        "fields": []
                        }
                    ],
                    "components": [],
                    "actions": {},
                    "username": "Wumpus.store",
                    "avatar_url": "https://cdn.discordapp.com/avatars/1207368481977147443/980c30c5c2e7896201f655a13af1037f.webp?size=80"
                }"#
                .replace("{{userId}}", data.userId.as_str())
                .replace("{{botId}}", data.botId.as_str())
                .replace("{{timeStamp}}", (Utc::now() + Duration::hours(12)).timestamp().to_string().as_str());

    match client
        .post(&query.webhook)
        .body(body)
        .header("content-type", "application/json")
        .send()
        .await
    {
        Ok(res) => {
            if res.status().is_success() {
                println!("Successful request to Discord");
                HttpResponse::Ok()
            } else {
                eprintln!("Not successful: {:#?}", res);
                HttpResponse::InternalServerError()
            }
        }
        Err(_) => {
            eprintln!("Failed to send request to Discord");
            HttpResponse::InternalServerError()
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(handle_webhook);
    };

    Ok(config.into())
}
