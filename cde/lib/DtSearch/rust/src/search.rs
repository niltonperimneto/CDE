use crate::format::or_hwordrec;
use crate::parser::DtSearchParser;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Query {
    Term(String),
    And(Box<Query>, Box<Query>),
    Or(Box<Query>, Box<Query>),
    Not(Box<Query>),
}

pub struct Searcher {
    parser: DtSearchParser,
}

impl Searcher {
    pub fn new() -> Self {
        Searcher {
            parser: DtSearchParser::new(),
        }
    }

    pub fn search(&self, query: &Query) -> HashSet<i32> {
        match query {
            Query::Term(term) => self.search_term(term),
            Query::And(left, right) => {
                let l = self.search(left);
                // Optimization: if l is empty, result is empty
                if l.is_empty() {
                    return l;
                }
                let r = self.search(right);
                l.intersection(&r).cloned().collect()
            }
            Query::Or(left, right) => {
                let l = self.search(left);
                let r = self.search(right);
                l.union(&r).cloned().collect()
            }
            Query::Not(operand) => {
                // Warning: Pure NOT queries are expensive as they return "Everything else"
                // Ideally this interacts with a parent AND.
                // For a standalone implementation, we need the Universe of all docs.
                // We'll implement this by getting Total Recs from DBREC if possible.
                // For now, let's assume we can get all valid IDs.
                // But efficient NOT usually requires participating in a set operation.

                // If this is top-level, we fetch all docs.
                // Let's implement a 'get_all_doc_ids' helper on Parser if needed.
                // For now, return empty set or implement slow full scan?
                // Use a placeholder logic: NOT only works well if combined?
                // Let's implement a naive "All - Set" logic.

                let operand_set = self.search(operand);
                // We need the universe.
                self.get_universe()
                    .difference(&operand_set)
                    .cloned()
                    .collect()
            }
        }
    }

    fn search_term(&self, term: &str) -> HashSet<i32> {
        if let Some(word) = self.parser.find_term(term) {
            self.parser.read_occurrences(&word).into_iter().collect()
        } else {
            HashSet::new()
        }
    }

    fn get_universe(&self) -> HashSet<i32> {
        // This is expensive. We should cache this or avoid purely negative queries.
        // We can read DBREC to get record count.
        if let Ok(dbrec) = self.parser.read_dbrec() {
            // IDs are 1-based up to or_reccount?
            // Logic in boolean_search.c implies recno goes up to tot_addr_count
            // but effective ones are in this range.
            // Let's assume 1..=reccount for valid docs for now.
            (1..=dbrec.or_reccount).collect()
        } else {
            HashSet::new()
        }
    }
}

// Simple Recursive Descent Parser for Boolean syntax
pub struct QueryParser {
    tokens: Vec<String>,
    pos: usize,
}

impl QueryParser {
    pub fn new(input: &str) -> Self {
        let mut tokens = Vec::new();
        // Naive tokenizer: extract specialized chars AND OR NOT ( )
        // and words.
        let mut current_word = String::new();
        for c in input.chars() {
            match c {
                '&' | '|' | '~' | '(' | ')' => {
                    if !current_word.is_empty() {
                        tokens.push(current_word.clone());
                        current_word.clear();
                    }
                    tokens.push(c.to_string());
                }
                ' ' | '\t' | '\n' => {
                    if !current_word.is_empty() {
                        tokens.push(current_word.clone());
                        current_word.clear();
                    }
                }
                _ => current_word.push(c),
            }
        }
        if !current_word.is_empty() {
            tokens.push(current_word);
        }

        QueryParser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Query, String> {
        self.parse_expression()
    }

    fn peek(&self) -> Option<&String> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<String> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    // Expression -> Term { ( & | | ) Term }
    fn parse_expression(&mut self) -> Result<Query, String> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.peek() {
            match token.as_str() {
                "&" => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Query::And(Box::new(left), Box::new(right));
                }
                "|" => {
                    self.consume();
                    let right = self.parse_term()?;
                    left = Query::Or(Box::new(left), Box::new(right));
                }
                ")" => break, // Stop at closing paren
                _ => {
                    // Implicit AND if just followed by another term?
                    // "apple banana" -> "apple AND banana"
                    if token != "(" && token != "~" && !self.is_identifier(token) {
                        break;
                    }
                    // It's a start of another term/factor
                    let right = self.parse_term()?;
                    left = Query::And(Box::new(left), Box::new(right));
                }
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Query, String> {
        if let Some(token) = self.peek() {
            if token == "~" {
                self.consume();
                let term = self.parse_term()?;
                Ok(Query::Not(Box::new(term)))
            } else if token == "(" {
                self.consume();
                let expr = self.parse_expression()?;
                if self.consume().as_deref() != Some(")") {
                    return Err("Missing closing parenthesis".to_string());
                }
                Ok(expr)
            } else {
                // Identifier
                let t = self.consume().unwrap();
                Ok(Query::Term(t))
            }
        } else {
            Err("Unexpected end of query".to_string())
        }
    }

    fn is_identifier(&self, s: &str) -> bool {
        !matches!(s, "&" | "|" | "~" | "(" | ")")
    }
}
