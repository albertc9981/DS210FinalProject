use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Actor {
    pub name: String,
}

impl Actor {
    pub fn new(name: String) -> Self {
        Actor { name }
    }
}
