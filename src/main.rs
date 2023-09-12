#[macro_use]
extern crate rocket;

use anyhow::Context;
use rocket::serde::json::Json;

mod payload;

async fn send_message(phone_number_id: String, to: String, text: String) {
    let token = std::env::var("WHATSAPP_TOKEN")
        .context("Unable to retrieve `WHATSAPP_TOKEN` from env")
        .unwrap();

    let client = reqwest::Client::new();

    let url =
        format!("https://graph.facebook.com/v12.0/{phone_number_id}/messages?access_token={token}");

    let response = client
        .post(&url)
        .json(&serde_json::json!({
        "messaging_product": "whatsapp",
        "to": to,
            "text": { "body": text },
        }))
        .header("Content-Type", "application/json")
        .send()
        .await;

    match response {
        Ok(res) => {
            info!("Response was sent successfully: \n{:#?}", res);
        }
        Err(err) => {
            error!("Error sending message: {:?}", err);
        }
    }
}

#[post("/webhook", data = "<request>")]
async fn post_webhook(request: Json<Option<payload::whatsapp::Data>>) -> rocket::http::Status {
    info!("\n{:#?}", request);

    if request.is_none() {
        return rocket::http::Status::NotFound;
    }

    if let Some(req) = &request.0 {
        if let Some(messages) = &req.entry[0].changes[0].value.messages {
            if let Some(text) = &messages[0].text {
                let phone_number_id = req.entry[0].changes[0]
                    .value
                    .metadata
                    .phone_number_id
                    .clone();
                let from = messages[0].from.clone();
                let text = text.body.clone();

                send_message(phone_number_id, from, text).await;
            }
        }
    }

    rocket::http::Status::Ok
}

#[post("/albato-webhook", data = "<request>")]
async fn post_albato_webhook(request: Json<Option<payload::albato::Data>>) -> rocket::http::Status {
    // let request_json: Value = serde_json::from_str(&request).unwrap();
    info!("\n{:#?}", request);
    // if request.is_none() {
    //     return rocket::http::Status::NotFound;
    // }

    rocket::http::Status::Ok
}

#[get("/webhook?<hub>")]
fn get_webhook(hub: payload::whatsapp::Hub) -> Option<(rocket::http::Status, String)> {
    let token = std::env::var("VERIFY_TOKEN")
        .context("Unable to retrieve `VERIFY_TOKEN` from env")
        .unwrap();

    if let payload::whatsapp::Hub {
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
async fn rocket() -> _ {
    pretty_env_logger::init();
    // send_message(
    //     String::from("113387005118555"),
    //     String::from("14167091633"),
    //     String::from("Hello!"),
    // )
    // .await;

    rocket::build().mount("/", routes![get_webhook, post_webhook, post_albato_webhook])
}
