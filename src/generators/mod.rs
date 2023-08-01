pub mod null;
pub use null::NullGenerator;

pub trait Generator {
    fn name(&self) -> &str;
    fn value_type(&self) -> Type;
    fn description(&self) -> &str;

    fn generate(config: Option<&str>) -> &str;
}

#[derive(Debug)]
pub enum Type {
    NULL
}

pub fn generators() -> Vec<impl Generator> {
    vec![
        NullGenerator::new(),
    ]
}
