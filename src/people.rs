extern crate serde;

#[derive(Serialize)]
pub struct People {
  pub name: String,
  pub about: String,
  pub profile: String,
}

impl People {
    pub fn new(name: &'static str, about: &'static str, profile: &'static str) -> People {
      People {
        name: String::from(name),
        about: String::from(about),
        profile: String::from(profile),
      }
    }
}
