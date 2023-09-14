#[macro_use]
extern crate rocket;

mod payload;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();

    rocket::build().mount(
        "/",
        routes![
            payload::whatsapp::get_webhook,
            payload::whatsapp::post_webhook,
            payload::facebook::get_webhook,
            payload::facebook::post_webhook,
        ],
    )
}

// FACEBOOK -> WEBHOOK -> WHATSAPP -> CHATGPT -> CHATGPT(double check) -> TELEGRAM
