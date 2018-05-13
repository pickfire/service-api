extern crate serde;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub about: String,
    pub profile: String,
}
