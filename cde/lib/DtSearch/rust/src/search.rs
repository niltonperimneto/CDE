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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- helpers -----------------------------------------------------------

    /// Parse `input` and panic with the error if parsing fails.
    fn parse(input: &str) -> Query {
        QueryParser::new(input)
            .parse()
            .unwrap_or_else(|e| panic!("parse({:?}) failed: {}", input, e))
    }

    /// Assert the query parses to a `Query::Term` with the given word.
    fn assert_term(q: &Query, expected: &str) {
        match q {
            Query::Term(t) => assert_eq!(t, expected, "term mismatch"),
            other => panic!("expected Term({:?}), got {:?}", expected, other),
        }
    }

    // ---- tokenisation ------------------------------------------------------

    #[test]
    fn single_term() {
        let q = parse("hello");
        assert_term(&q, "hello");
    }

    #[test]
    fn empty_query_is_error() {
        assert!(
            QueryParser::new("").parse().is_err(),
            "empty string should fail to parse"
        );
    }

    #[test]
    fn whitespace_only_is_error() {
        assert!(QueryParser::new("   \t\n").parse().is_err());
    }

    // ---- boolean operators -------------------------------------------------

    #[test]
    fn explicit_and() {
        let q = parse("foo & bar");
        assert!(
            matches!(q, Query::And(_, _)),
            "expected And, got {:?}",
            q
        );
        if let Query::And(l, r) = &q {
            assert_term(l, "foo");
            assert_term(r, "bar");
        }
    }

    #[test]
    fn explicit_or() {
        let q = parse("foo | bar");
        assert!(matches!(q, Query::Or(_, _)), "expected Or");
        if let Query::Or(l, r) = &q {
            assert_term(l, "foo");
            assert_term(r, "bar");
        }
    }

    #[test]
    fn not_operator() {
        let q = parse("~hidden");
        assert!(matches!(q, Query::Not(_)), "expected Not");
        if let Query::Not(inner) = &q {
            assert_term(inner, "hidden");
        }
    }

    #[test]
    fn implicit_and_between_terms() {
        // "apple banana" should be treated as "apple AND banana"
        let q = parse("apple banana");
        assert!(
            matches!(q, Query::And(_, _)),
            "adjacent terms should produce implicit And"
        );
    }

    // ---- grouping ----------------------------------------------------------

    #[test]
    fn parenthesised_or() {
        // "a & (b | c)" — the Or is nested inside And's right operand
        let q = parse("a & (b | c)");
        assert!(matches!(q, Query::And(_, _)));
        if let Query::And(left, right) = &q {
            assert_term(left, "a");
            assert!(
                matches!(right.as_ref(), Query::Or(_, _)),
                "right of And should be Or"
            );
        }
    }

    #[test]
    fn unclosed_paren_is_error() {
        assert!(
            QueryParser::new("(foo & bar").parse().is_err(),
            "unclosed parenthesis should fail"
        );
    }

    #[test]
    fn nested_not() {
        let q = parse("~~word");
        // ~~word → Not(Not(Term("word")))
        assert!(matches!(q, Query::Not(_)));
        if let Query::Not(inner) = &q {
            assert!(matches!(inner.as_ref(), Query::Not(_)));
        }
    }

    // ---- complex expressions -----------------------------------------------

    #[test]
    fn three_term_and_chain() {
        // "a & b & c" left-associates: (a & b) & c
        let q = parse("a & b & c");
        assert!(matches!(q, Query::And(_, _)));
        // Verify the outer right arm is the term "c"
        if let Query::And(_, right) = &q {
            assert_term(right, "c");
        }
    }

    #[test]
    fn not_combined_with_and() {
        let q = parse("visible & ~hidden");
        assert!(matches!(q, Query::And(_, _)));
        if let Query::And(l, r) = &q {
            assert_term(l, "visible");
            assert!(matches!(r.as_ref(), Query::Not(_)));
        }
    }

    #[test]
    fn complex_expression() {
        // Ensure a non-trivial expression parses without panicking.
        let _ = parse("(dt & help) | (dtwm & ~crash)");
    }

    // ---- query display / Debug trait ---------------------------------------

    #[test]
    fn query_is_debug() {
        // Confirm the Debug derive works (useful for test failure messages).
        let q = parse("a | b");
        let _ = format!("{:?}", q);
    }

    // ---- clone semantics ---------------------------------------------------

    #[test]
    fn query_clone() {
        let q = parse("foo & bar");
        let cloned = q.clone();
        // Both should have the same structure — just verify they are And.
        assert!(matches!(cloned, Query::And(_, _)));
    }
}
