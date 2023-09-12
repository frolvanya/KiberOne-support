use rocket::serde::Deserialize;

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
