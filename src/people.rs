extern crate serde;

#[derive(Serialize)]
pub struct People {
    pub id: i32,
    pub name: String,
    pub about: String,
    pub profile: String,
}
