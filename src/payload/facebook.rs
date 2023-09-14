use anyhow::Context;
use rocket::serde::{json::Json, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    entry: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Entry {
    changes: Vec<Change>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Change {
    field: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Value {
    ad_id: String,
    form_id: String,
    leadgen_id: String,
    created_time: usize,
    page_id: String,
    adgroup_id: String,
}

#[derive(Debug, PartialEq, FromForm)]
pub struct Hub {
    pub mode: Option<String>,
    pub challenge: Option<String>,
    pub verify_token: Option<String>,
}

#[get("/facebook-webhook?<hub>")]
pub fn get_webhook(hub: Hub) -> Option<(rocket::http::Status, String)> {
    let token = std::env::var("FACEBOOK_TOKEN")
        .context("Unable to retrieve `FACEBOOK_TOKEN` from env")
        .unwrap();

    if let Hub {
        mode: Some(parsed_mode),
        challenge: Some(parsed_challenge),
        verify_token: Some(parsed_token),
    } = hub
    {
        if parsed_mode == "subscribe" && parsed_token == token {
            info!("FACEBOOK WEBHOOK WAS VERIFIED");
            return Some((rocket::http::Status::Ok, parsed_challenge));
        }
    }

    None
}

#[post("/facebook-webhook", data = "<request>")]
pub async fn post_webhook(request: Json<Option<Data>>) -> rocket::http::Status {
    info!("\n{:#?}", request);

    if request.is_none() {
        return rocket::http::Status::NotFound;
    }

    if let Some(req) = &request.0 {
        crate::payload::whatsapp::send_message(
            String::from("117532128115831"),
            String::from("14167091633"),
            format!("{:?}", req),
        )
        .await;
    }

    rocket::http::Status::Ok
}
