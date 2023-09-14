use rocket::serde::{json::Json, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    pub company_id: Option<String>,
    pub company_name: Option<String>,
    pub form_name: Option<String>,
    pub full_name: Option<String>,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub id: Option<String>,
    pub lead_id: Option<String>,
    pub lead_name: Option<String>,
    pub phone_number: Option<String>,
    pub platform: Option<String>,
    pub timestamp: Option<String>,
}

#[post("/albato-webhook", data = "<request>")]
pub async fn post_albato_webhook(request: Json<Option<Data>>) -> rocket::http::Status {
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
