use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{Duration, Utc};
use serde::Deserialize;

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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "Put

https://wumpus-webhook.tippfehlr.dev/?webhook=<your-discord-webhook>
 
into the shuttle `webhook` field.
The password doesn't matter, input anything for the form to be happy.


Source code: https://github.com/tippfehlr/wumpus-webhook",
    )
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
                        "description": "<@{{userId}}> just voted for <@{{botId}}>!\nYou can vote again <t:{{timeStamp}}:R>.\nVote [here](https://wumpus.store/bot/813130993640013874)!",
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(handle_webhook).service(index))
        .bind(("0.0.0.0", 4056))?
        .run()
        .await
}
