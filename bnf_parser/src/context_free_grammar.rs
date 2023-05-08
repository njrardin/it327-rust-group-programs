use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Symbol {
    Terminal(String),
    Nonterminal(String),
}
impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            s => write!(f, "{}", s),
        }
    }
}

type ProductionRule = (Symbol, Vec<Symbol>);

#[derive(Debug, PartialEq)]
pub struct ContextFreeGrammar {
    start_symbol: Symbol,
    production_rules: HashSet<ProductionRule>,
}

//TODO - refactor to use Result for better error handling
//TODO - refactor error handling in general throughout app to print to standard err instead of standard out


pub fn build_grammar(bnf_grammar: &str) -> ContextFreeGrammar {
    let mut production_rules = HashSet::new();
    let mut start_symbol = Symbol::Nonterminal(String::new());

    let mut first_iteration = true;
    for line in bnf_grammar.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            // skip empty and comment lines
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, "::=").map(|s| s.trim()).collect();
        let lhs_symbol = Symbol::Nonterminal(parts[0].to_string());
        let expressions: Vec<String> = parts[1].split('|').map(|s| s.trim().to_string()).collect();

        for expression in expressions {
            let rhs_symbols: Vec<Symbol> = expression
                .split(' ')
                .map(|s| {
                    if s.starts_with('<') && s.ends_with('>') {
                        Symbol::Nonterminal(s.to_string())
                    } else {
                        Symbol::Terminal(s.to_string())
                    }
                })
                .collect();

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
        start_symbol,
        production_rules,
    }
}

#[derive(PartialEq)]
pub struct ParseTree {
    root: Symbol,
    children: Vec<ParseTree>,
}

impl ParseTree {
    fn new(root: Symbol) -> ParseTree {
        ParseTree {
            root,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: ParseTree) {
        self.children.push(child);
    }

    fn span(&self) -> usize {
        self.children.iter().map(|c| c.span()).sum()
    }
}
// implements fmt::Display for ParseTree to print the tree in a human-readable format which shows the tree structure
impl std::fmt::Display for ParseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn fmt_helper(tree: &ParseTree, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
            let indent = "-".repeat(depth * 2);
            writeln!(f, "{}|-{}", indent, tree.root)?;
            for child in &tree.children {
                fmt_helper(child, f, depth + 1)?;
            }
            Ok(())
        }

        fmt_helper(self, f, 0)
    }
}
//uses same output as display for debug
//TODO - refactor to use display instead of fmt
impl std::fmt::Debug for ParseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt(f)
    }
}


fn tokenize_sentence(sentence: &str) -> Vec<&str> {
    sentence.split(' ').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_grammar() {
        let bnf_contents = "<expr> ::= <char> | <char> <op> <expr> \n
                            <op> ::= + | - | * | / \n
                            <char> ::= a | b | c";

        let expected_rules: HashSet<(Symbol, Vec<Symbol>)> = [
            (Symbol::Nonterminal("<expr>".to_string()), vec![Symbol::Nonterminal("<char>".to_string())]),
            (Symbol::Nonterminal("<expr>".to_string()), vec![Symbol::Nonterminal("<char>".to_string()), 
            Symbol::Nonterminal("<op>".to_string()), Symbol::Nonterminal("<expr>".to_string())]),
            (Symbol::Nonterminal("<op>".to_string()), vec![Symbol::Terminal("+".to_string())]),
            (Symbol::Nonterminal("<op>".to_string()), vec![Symbol::Terminal("-".to_string())]),
            (Symbol::Nonterminal("<op>".to_string()), vec![Symbol::Terminal("*".to_string())]),
            (Symbol::Nonterminal("<op>".to_string()), vec![Symbol::Terminal("/".to_string())]),
            (Symbol::Nonterminal("<char>".to_string()), vec![Symbol::Terminal("a".to_string())]),
            (Symbol::Nonterminal("<char>".to_string()), vec![Symbol::Terminal("b".to_string())]),
            (Symbol::Nonterminal("<char>".to_string()), vec![Symbol::Terminal("c".to_string())]),
        ]
        .iter()
        .map(|(lhs, rhs)| (lhs.clone(), rhs.clone()))
        .collect();

        let expected_grammar = ContextFreeGrammar {
            start_symbol: Symbol::Nonterminal("<expr>".to_string()),
            production_rules: expected_rules,
        };

        let actual_grammar = build_grammar(bnf_contents);

        assert_eq!(actual_grammar, expected_grammar);
    }

    #[test]
    fn test_tokenize_sentence() {
        let sentence = "a + b * c";
        let expected_tokens = vec!["a", "+", "b", "*", "c"];

        let actual_tokens = tokenize_sentence(sentence);

        assert_eq!(actual_tokens, expected_tokens);
    }
}