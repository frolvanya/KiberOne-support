use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Entries {
    pub entry: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Entry {
    pub changes: Vec<Change>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Change {
    pub value: Value,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Value {
    pub metadata: Metadata,
    pub messages: Option<Vec<Message>>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Metadata {
    pub phone_number_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub from: String,
    pub text: Option<TextBody>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TextBody {
    pub body: String,
}

#[derive(Debug, PartialEq, FromForm)]
pub struct Hub {
    pub mode: Option<String>,
    pub challenge: Option<String>,
    pub verify_token: Option<String>,
}
