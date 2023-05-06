use std::collections::HashSet;
use std::fs;

type ProductionRule = (String, Vec<String>);

#[derive(Debug)]
pub struct ContextFreeGrammar {
    start_symbol: String,
    production_rules: HashSet<ProductionRule>,
}

impl PartialEq for ContextFreeGrammar {
    fn eq(&self, other: &Self) -> bool {
        self.start_symbol == other.start_symbol
            && self.production_rules == other.production_rules
    }
}

//TODO - refactor to use Result for better error handling
//TODO - refactor error handling in general throughout app to print to standard err instead of standard out
pub fn read_bnf_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Could not read file")
}

pub fn build_grammar(bnf_grammar: &str) -> ContextFreeGrammar {
    let mut production_rules = HashSet::new();
    let mut start_symbol = String::new();

    let mut first_iteration = true;
    for line in bnf_grammar.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            // skip empty and comment lines
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, "::=").map(|s| s.trim()).collect();
        let symbol = parts[0].to_string();
        let expressions: Vec<String> = parts[1].split('|').map(|s| s.trim().to_string()).collect();

        for expression in expressions {
            let rule: ProductionRule = (
                symbol.clone(),
                expression.split(' ').map(|s| s.to_string()).collect(),
            );
            production_rules.insert(rule);
        }

        if first_iteration {
            // first line's symbol should be the start symbol
            start_symbol = symbol;
            first_iteration = false;
        }
    }

    ContextFreeGrammar {
        start_symbol,
        production_rules,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_grammar() {
        let bnf_contents = "<expr> ::= <char> | <char> <op> <expr> \n
                            <op> ::= + | - | * | / \n
                            <char> ::= a | b | c";

        let expected_rules: HashSet<(String, Vec<String>)> = [
            ("<expr>", vec!["<char>"]),
            ("<expr>", vec!["<char>", "<op>", "<expr>"]),
            ("<op>", vec!["+"]),
            ("<op>", vec!["-"]),
            ("<op>", vec!["*"]),
            ("<op>", vec!["/"]),
            ("<char>", vec!["a"]),
            ("<char>", vec!["b"]),
            ("<char>", vec!["c"]),
        ]
        .iter()
        .map(|&(s, ref v)| (s.to_string(), v.iter().map(|&s| s.to_string()).collect()))
        .collect();

        let expected_grammar = ContextFreeGrammar {
            start_symbol: "<expr>".to_string(),
            production_rules: expected_rules,
        };

        let actual_grammar = build_grammar(bnf_contents);

        assert_eq!(actual_grammar, expected_grammar);
    }
}
