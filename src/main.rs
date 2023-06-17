#[macro_use]
extern crate rocket;

use anyhow::Context;
use rocket::serde::json::Json;

mod payload;

#[post("/webhook", data = "<request>")]
async fn post_webhook(request: Json<Option<payload::Entries>>) -> rocket::http::Status {
    let token = std::env::var("WHATSAPP_TOKEN")
        .context("Unable to retrieve `WHATSAPP_TOKEN` from env")
        .unwrap();
    info!("\n{:#?}", request);

    if request.is_none() {
        return rocket::http::Status::NotFound;
    }

    if let Some(req) = &request.0 {
        if let Some(messages) = &req.entry[0].changes[0].value.messages {
            if let Some(text) = &messages[0].text {
                let client = reqwest::Client::new();

                let phone_number_id = req.entry[0].changes[0]
                    .value
                    .metadata
                    .phone_number_id
                    .clone();
                let from = messages[0].from.clone();
                let text = text.body.clone();

                let url = format!(
                "https://graph.facebook.com/v12.0/{phone_number_id}/messages?access_token={token}"
            );

                let response = client
                    .post(&url)
                    .json(&serde_json::json!({
                        "messaging_product": "whatsapp",
                        "to": from,
                        "text": { "body": format!("Ack: {}", text) },
                    }))
                    .header("Content-Type", "application/json")
                    .send()
                    .await;

                match response {
                    Ok(res) => info!("Response was sent successfully: \n{:#?}", res),
                    Err(err) => error!("Error sending message: {}", err),
                }
            }
        }
    }

    rocket::http::Status::Ok
}

#[get("/webhook?<hub>")]
fn get_webhook(hub: payload::Hub) -> Option<(rocket::http::Status, String)> {
    let token = std::env::var("VERIFY_TOKEN")
        .context("Unable to retrieve `VERIFY_TOKEN` from env")
        .unwrap();

    if let payload::Hub {
        mode: Some(parsed_mode),
        challenge: Some(parsed_challenge),
        verify_token: Some(parsed_token),
    } = hub
    {
        if parsed_mode == "subscribe" && parsed_token == token {
            info!("WEBHOOK WAS VERIFIED");
            return Some((rocket::http::Status::Ok, parsed_challenge));
        }
    }

    None
}

#[launch]
fn rocket() -> _ {
    pretty_env_logger::init();

    rocket::build().mount("/", routes![get_webhook, post_webhook])
}
