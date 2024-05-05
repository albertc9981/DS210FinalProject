use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Movie {
    pub title: String,
}

impl Movie {
    pub fn new(title: String) -> Self {
        Movie { title }
    }
}
