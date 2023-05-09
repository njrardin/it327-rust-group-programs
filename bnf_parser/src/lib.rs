use std::error::Error;
use std::fs;

mod context_free_grammar;
use context_free_grammar as cfg;

/// Configuration struct for the bnf_lexer binary.
pub struct Config {
    pub bnf_grammar_filepath: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: bnf_lexer <bnf-grammar-filepath>");
        }

        let bnf_grammar_filepath = args[1].clone();

        Ok(Config { bnf_grammar_filepath })
    }
}

/// Runs the bnf_lexer binary.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("bnf_grammar_filepath: {}", config.bnf_grammar_filepath);

    let bnf_grammar = read_bnf_file(&config.bnf_grammar_filepath)?;

    let grammar = cfg::build_grammar(&bnf_grammar);

    // prints grammar to the screen
    println!("{}", grammar);

    Ok(())
}

/// Reads a BNF grammar file and returns the contents as a String.
pub fn read_bnf_file(filename: &str) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(filename).map_err(|e| e.into())
}