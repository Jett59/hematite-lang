use std::{error::Error, fs::File, io::BufReader};

use utf8_chars::BufReadCharsExt;

use clap::Parser;

mod ast;
mod lexer;
mod parser;

#[derive(Debug, clap::Parser)]
struct CommandLineOptions {
    /// The amount of optimization to perform on the code;
    /// 0 = no optimizations,
    /// 1 = some optimizations,
    /// 2 = most optimizations,
    /// 3 = aggressive optimizations
    #[clap(short = 'O', default_value = "2")]
    optimization_level: i32,
    #[clap(short, long = "output", default_value = "<stdin>")]
    output_file: String,

    input_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = CommandLineOptions::parse();
    let input_file = File::open(options.input_file).unwrap();
    let mut buffered_file_reader = BufReader::new(input_file);
    let character_iterator = buffered_file_reader.chars();
    let mut character_iterator =
        character_iterator.map(|possibly_char| possibly_char.expect("Failed to read from file"));
    let token_iterator = lexer::tokenize(&mut character_iterator);
    let program = parser::parse(&mut token_iterator.peekable())?;
    println!("{:#?}", program);
    Ok(())
}
