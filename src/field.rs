#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub name: String,
    pub generator_name: String,
    pub generator_config: Option<String>,
}

impl Field {
    pub fn new(name: String, generator_name: String, generator_config: Option<String>) -> Self {
        Self { name, generator_name, generator_config }
    }
}


