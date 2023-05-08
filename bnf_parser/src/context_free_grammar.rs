use std::collections::HashSet;
use std::fmt;

type ProductionRule = (String, Vec<String>);

#[derive(Debug, PartialEq)]
pub struct ContextFreeGrammar {
    variables: HashSet<String>,
    terminals: HashSet<String>,
    start_symbol: String,
    production_rules: HashSet<ProductionRule>,
}

impl fmt::Display for ContextFreeGrammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        output.push_str("Variables =\n");
        for variable in &self.variables {
            output.push_str(&format!("\t{}\n", variable));
        }

        output.push_str("\nTerminals =\n");
        for terminal in &self.terminals {
            output.push_str(&format!("\t{}\n", terminal));
        }

        output.push_str(&format!("\nStart Symbol = {}\n", self.start_symbol));

        output.push_str("\nProduction Rules =\n");
        for rule in &self.production_rules {
            output.push_str(&format!("\t{}\n", format_rule(rule)));
        }

        write!(f, "{}", output)
    }
}

fn format_rule(rule: &ProductionRule) -> String {
    let lhs = &rule.0;
    let rhs = &rule.1;

    let mut output = String::new();
    output.push_str(&format!("{} ::= ", lhs));

    for symbol in rhs {
        output.push_str(&format!("{} ", symbol));
    }

    output
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

/// If the start symbol is on the right hand side of any production rules, removes it and add a new start symbol.
fn remove_start_on_rhs(grammar: &mut ContextFreeGrammar) -> () {
    let mut start_on_rhs = false;
    for rule in &grammar.production_rules {
        if rule.1.contains(&grammar.start_symbol) {
            start_on_rhs = true;
            break;
        }
    }
    if start_on_rhs {

        let new_start_symbol = format!("<{}'>", grammar.start_symbol.trim_matches(|c| c == '<' || c == '>'));
        grammar.variables.insert(new_start_symbol.clone());
        grammar.production_rules.insert((new_start_symbol.clone(), vec![grammar.start_symbol.clone()]));
        grammar.start_symbol = new_start_symbol;
    }
}

// /// Converts a grammar to Chompsky Normal Form. Grammar must already be simplified.
// fn convert_to_chompsky_normal_form(grammar: &mut ContextFreeGrammar) -> () {

//     //2. Decompose mixed terminals and variables on RHS (e.g. A ::= xY)
//     for rule in grammar.production_rules.clone() {
//         let mut new_rules = HashSet::new();
//         let lhs = rule.0.clone();
//         let rhs = rule.1.clone();
//         // check whether the rule has mixed terminals and variables
//         let mut contains_terminal = false;
//         for symbol in rhs.clone() {
//             if grammar.terminals.contains(&symbol) {
//                 contains_terminal = true;
//                 break;
//             }
//         }
//         let mut contains_variable = false;
//         for symbol in rhs.clone() {
//             if grammar.variables.contains(&symbol) {
//                 contains_variable = true;
//                 break;
//             }
//         }
//         let contains_mixed = contains_terminal && contains_variable;
//         // skip this rule if it doesn't have mixed terminals and variables
//         if !contains_mixed {
//             continue;
//         }
//         // if it does have mixed, create a new rule mapping the lhs to each terminal
//         for symbol in rhs.clone() {
//             if grammar.terminals.contains(&symbol) {
//                 new_rules.insert((lhs.clone(), vec![symbol]));
//             }
//         }
//         // replace the old rule with a new rule mapping the lhs to the new variables
//         let mut new_rhs = Vec::new();
//         for symbol in rhs.clone() {
//                 new_rhs.push(symbol);
//         }
//         // insert the new rules
//         new_rules.insert((rule.0.clone(), new_rhs));
//         // remove the old rule
//         grammar.production_rules.remove(&rule);
//     }

//     //3. Decompose >2 variables on RHS
//     for rule 
// }

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

    #[test]
    fn test_remove_start_on_rhs() {
        let bnf_grammar = "<S> ::= <X> <Y> \n
                            <X> ::= a <S> | b \n
                            <Y> ::= bb | c";
        let mut input_cfg = build_grammar(bnf_grammar);

        let expected_bnf = "<S'> ::= <S> \n
                            <S> ::= <X> <Y> \n
                            <X> ::= a <S> | b \n
                            <Y> ::= bb | c";

        let expected_cfg = build_grammar(expected_bnf);
        remove_start_on_rhs(&mut input_cfg);

        assert_eq!(input_cfg, expected_cfg);
    }
}