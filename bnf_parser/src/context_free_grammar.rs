use std::collections::HashSet;

type ProductionRule = (String, Vec<String>);

#[derive(Debug, PartialEq)]
pub struct ContextFreeGrammar {
    variables: HashSet<String>,
    terminals: HashSet<String>,
    start_symbol: String,
    production_rules: HashSet<ProductionRule>,
}

pub fn build_grammar(bnf_grammar: &str) -> ContextFreeGrammar {
    let mut variables = HashSet::new();
    let mut terminals = HashSet::new();
    let mut start_symbol = String::new();
    let mut production_rules = HashSet::new();

    let mut first_iteration = true;
    for line in bnf_grammar.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            // skip empty and comment lines
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, "::=").map(|s| s.trim()).collect();

        let lhs_symbol = parts[0].to_string();
        variables.insert(lhs_symbol.clone());

        let expressions: Vec<String> = parts[1].split('|').map(|s| s.trim().to_string()).collect();
        for expression in expressions {
            let rhs_symbols: Vec<String> = expression
                .split(' ')
                .map(|s| s.trim().to_string())
                .collect();

            // add all new symbols to terminals or variables as appropriate
            for symbol in &rhs_symbols {
                if !variables.contains(symbol) && !terminals.contains(symbol) {
                    if symbol.starts_with('<') && symbol.ends_with('>') {
                        variables.insert(symbol.clone());
                    } else {
                        terminals.insert(symbol.clone());
                    }
                }
            }                

            let rule: ProductionRule = (
                lhs_symbol.clone(),
                rhs_symbols,
            );
            production_rules.insert(rule);
        }

        if first_iteration {
            // first line's symbol should be the start symbol
            start_symbol = lhs_symbol;
            first_iteration = false;
        }
    }

    ContextFreeGrammar {
        variables,
        terminals,
        start_symbol,
        production_rules,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_grammar() {
        let bnf_contents = "<S> ::= <NP> <VP> \n
                            <NP> ::= The dog | The cat \n
                            <VP> ::= played with the ball | ate its food";
                        

        let expected_rules: HashSet<(String, Vec<String>)> = vec![
            ("<S>".to_string(), vec!["<NP>".to_string(), "<VP>".to_string()]),
            ("<NP>".to_string(), vec!["The".to_string(), "dog".to_string()]),
            ("<NP>".to_string(), vec!["The".to_string(), "cat".to_string()]),
            ("<VP>".to_string(), vec!["played".to_string(), "with".to_string(), "the".to_string(), "ball".to_string()]),
            ("<VP>".to_string(), vec!["ate".to_string(), "its".to_string(), "food".to_string()]),
        ].into_iter()
        .collect();

        let expected_variables: HashSet<String> = vec![
            "<S>".to_string(),
            "<NP>".to_string(),
            "<VP>".to_string(),
        ].into_iter()
        .collect();

        let expected_terminals: HashSet<String> = vec![
            "The".to_string(),
            "dog".to_string(),
            "cat".to_string(),
            "played".to_string(),
            "with".to_string(),
            "the".to_string(),
            "ball".to_string(),
            "ate".to_string(),
            "its".to_string(),
            "food".to_string(),
        ].into_iter()
        .collect();

        let expected_grammar = ContextFreeGrammar {
            variables: expected_variables,
            terminals: expected_terminals,
            start_symbol: "<S>".to_string(),
            production_rules: expected_rules,
        };

        let actual_grammar = build_grammar(bnf_contents);

        assert_eq!(actual_grammar, expected_grammar);
    }
}