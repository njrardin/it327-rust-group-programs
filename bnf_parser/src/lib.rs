use std::error::Error;

mod bnf_reader;

pub struct Config {
    pub bnf_grammar_filepath: String,
    pub to_parse: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: bnf_parser <bnf-grammar-filepath> <string-to-parse>");
        }

        let bnf_grammar_filepath = args[1].clone();
        let to_parse = args[2].clone();

        Ok(Config { bnf_grammar_filepath, to_parse })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("bnf_grammar_filepath: {}", config.bnf_grammar_filepath);
    println!("to_parse: {}", config.to_parse);


    Ok(())
}