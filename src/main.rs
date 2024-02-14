use actix_web::{
    post,
    web::{Data, Json},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Clone)]
struct Env {
    discord_webhook: String,
    wumpus_token: String,
    webhook_body: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
struct WumpusWebhook {
    webhookTest: bool,
    userId: String,
    botId: String,
    query: serde_json::Value,
}

#[post("/")]
async fn handle_webhook(
    env: Data<Env>,
    data: Json<WumpusWebhook>,
    req: HttpRequest,
) -> impl Responder {
    println!("received request");
    if let Some(auth) = req.headers().get("Authorization") {
        if auth.to_str().unwrap() == env.wumpus_token {
            let client = reqwest::Client::new();
            let body = env
                .webhook_body
                .replace("{{userId}}", data.userId.as_str())
                .replace("{{botId}}", data.botId.as_str());

            match client
                .post(&env.discord_webhook)
                .body(body)
                .header("content-type", "application/json")
                .send()
                .await
            {
                Ok(res) => {
                    println!("{:?}", res);
                    if res.status().is_success() {
                        println!("Successful request to Discord");
                        HttpResponse::Ok()
                    } else {
                        eprintln!("Not successful: {:?}", res);
                        HttpResponse::InternalServerError()
                    }
                }
                Err(_) => {
                    eprintln!("Failed to send request to Discord");
                    HttpResponse::InternalServerError()
                }
            }
        } else {
            eprintln!("Invalid authorization header, aborting.");
            HttpResponse::Unauthorized()
        }
    } else {
        eprintln!("No authorization header, aborting.");
        HttpResponse::Unauthorized()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env {
        discord_webhook: std::env::var("DISCORD_WEBHOOK").expect("DISCORD_WEBHOOK is not set"),
        wumpus_token: std::env::var("WUMPUS_TOKEN").expect("WUMPUS_TOKEN is not set"),
        webhook_body: std::env::var("WEBHOOK_BODY").unwrap_or(
            r#"
{
  "content": "",
  "tts": false,
  "embeds": [
    {
      "id": 10674342,
      "title": "🔥 New vote! 🔥",
      "description": "<@{{userId}}> just voted for <@{{botId}}>!",
      "color": 2326507,
      "fields": []
    }
  ],
  "components": [],
  "actions": {},
  "username": "wumpus.store",
  "avatar_url": "https://wumpus.store/assets/icon.svg"
}"#
            .to_string(),
        ),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(env.clone()))
            .service(handle_webhook)
    })
    .bind(("0.0.0.0", 4056))?
    .run()
    .await
}
