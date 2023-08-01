mod generators;
mod format;
pub mod field;

pub use generators::generators;
pub use generators::Generator;
pub use generators::Type as GeneratorType;

// pub struct Field {
//     pub name: String,
//     pub
// }

pub fn generate() -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
