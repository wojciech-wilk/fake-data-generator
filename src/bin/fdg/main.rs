use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::process::exit;
use clap::Parser;
use fake_data_generator::{generators, Generator};
use fake_data_generator::field::Field;
// use log::debug;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Show list of available generators
    #[clap(short, long)]
    generators: bool,

    /// Format of the generated output.
    /// Possible values:
    ///     CSV - comma-separated value
    #[clap(short, long, value_parser)]
    format: String,

    /// Definitions of fields to generate, FIELD_NAME=GENERATOR:GENERATOR_OPTIONS
    #[clap(required_unless_present = "generators")]
    fields: Vec<String>,
}

fn main() {
    // env_logger::init();
    let args = Args::parse();

    let generators = generators();

    if args.generators {
        eprintln!("List of available generators:");

        for generator in generators {
            eprintln!("\t{} - {}", generator.name(), generator.description());
            eprintln!("\t\ttype: {:?}", generator.value_type());
        }
        exit(0);
    }

    // println!("{:?}", args.fields);
    let fields: Vec<Result<Field, String>> = args.fields.iter().map(|field| parse_field(field)).collect();
    // println!("{:?}", &fields);

    let mut has_error = false;
    for field in &fields {
        if let Err(error) = field {
            has_error = true;
            eprintln!("{}", error);
        }
    }

    if has_error {
        exit(1);
    }

    let fields_without_errors = fields.iter().map(|field| field.unwrap()).collect();
    validate(fields_without_errors);
}

fn validate(fields: Vec<Field>) {
    // let mut fields_by_name = HashSet::new();
    // for field in &fields {
    //     fields_by_name.insert(field.name.as_str());
    // }
}

fn parse_field(field_str: &str) -> Result<Field, String> {
    // println!("{:?}", &field_str);
    // let field: Vec<&str> = field_str.split("=").collect();
    let maybe_field_separator = field_str.find("=");
    if let Some(field_separator) = maybe_field_separator {
        let field_name = &field_str[..field_separator];
        let generator_position = field_separator + 1;
        let generator_name_and_maybe_generator = &field_str[generator_position..];
        let maybe_config_separator_position = generator_name_and_maybe_generator.find(":");

        let generator_name = match maybe_config_separator_position {
            None => generator_name_and_maybe_generator,
            Some(position) => &generator_name_and_maybe_generator[..position],
        };

        let generator_config = maybe_config_separator_position.map(|position| &generator_name_and_maybe_generator[position + 1..]);

        // debug!("Name: {:?}, Generator: {:?}, Generator config string: {:?}", &field_name, &generator_name, &generator_config);
        Ok(Field::new(field_name.to_string(), generator_name.to_string(), generator_config.map(|x| x.to_string())))
    } else {
        Err(format!("Error in field {:?}, no generator provided", &field_str))
    }
}
