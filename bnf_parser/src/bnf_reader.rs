use std::collections::HashSet;
use std::fs;

type ProductionRule = (String, Vec<String>);

pub fn read_bnf_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Could not read file")
}

pub fn build_production_rules(bnf_grammar: &str) -> HashSet<ProductionRule> {
    let mut production_rules = HashSet::new();

    for line in bnf_grammar.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            // skip empty and comment lines
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, "::=").map(|s| s.trim()).collect();
        let symbol = parts[0].to_string();
        let expressions: Vec<String> = parts[1]
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();

        for expression in expressions {
            let rule: ProductionRule = (symbol.clone(), expression.split(' ').map(|s| s.to_string()).collect());
            production_rules.insert(rule);
        }
    }

    production_rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_grammar() {
        let bnf_contents = "<expr> ::= <char> | <char> <op> <expr> \n
                            <op> ::= + | - | * | / \n
                            <char> ::= a | b | c";
                            use std::collections::HashSet;

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
        ].iter()
        .map(|&(s, ref v)| (s.to_string(), v.iter().map(|&s| s.to_string()).collect()))
        .collect();                                       

        let actual_rules = build_production_rules(bnf_contents);

        assert_eq!(actual_rules, expected_rules);
    }
}


