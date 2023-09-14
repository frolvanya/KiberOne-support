use rocket::serde::{json::Json, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    company_id: Option<String>,
    company_name: Option<String>,
    form_name: Option<String>,
    full_name: Option<String>,
    group_id: Option<String>,
    group_name: Option<String>,
    id: Option<String>,
    lead_id: Option<String>,
    lead_name: Option<String>,
    phone_number: Option<String>,
    platform: Option<String>,
    timestamp: Option<String>,
}

#[post("/albato-webhook", data = "<request>")]
pub async fn post_webhook(request: Json<Option<Data>>) -> rocket::http::Status {
    info!("\n{:#?}", request);

    if request.is_none() {
        return rocket::http::Status::NotFound;
    }

    if let Some(req) = &request.0 {
        let full_name = req
            .full_name
            .clone()
            .unwrap_or(String::from("FULL NAME IS MISSING"));
        let phone_number = req
            .phone_number
            .clone()
            .unwrap_or(String::from("PHONE NUMBER IS MISSING"));

        crate::payload::whatsapp::send_message(
            String::from("117532128115831"),
            String::from("14167091633"),
            format!("Full name: {}\nPhone number: {}", full_name, phone_number),
        )
        .await;
    }

    rocket::http::Status::Ok
}
