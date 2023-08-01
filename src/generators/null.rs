use crate::generators::{Generator, Type};

pub struct NullGenerator {
    name: &'static str,
    description: &'static str,
}

impl NullGenerator {
    pub fn new() -> NullGenerator {
        NullGenerator {
            name: "null",
            description: "generates null values",
        }
    }
}

impl Generator for NullGenerator {
    fn name(&self) -> &str {
        self.name
    }

    fn value_type(&self) -> Type {
        Type::NULL
    }

    fn description(&self) -> &str {
        self.description
    }

    fn generate(_: Option<&str>) -> &str {
        "null"
    }
}
