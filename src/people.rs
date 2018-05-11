extern crate serde;

#[derive(Serialize)]
pub struct People {
  pub name: String,
  pub about: String,
  pub profile: String,
}
