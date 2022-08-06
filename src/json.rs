use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum JObject {
    JNull,
    JNumber(i64),
    JString(String),
    JList(Vec<JObject>),

    // other stuff to make json a programming language
    JSymbol(String),
    JFunc {
        arguments: Vec<String>,
        definition: Box<JObject>,
    },
}

pub fn parse(line: &str) -> JObject {
    Parser::parse(&mut Parser {
        text: line.chars().enumerate().peekable(),
    })
}

struct Parser<'a> {
    text: Peekable<Enumerate<Chars<'a>>>,
}

impl Parser<'_> {
    fn parse(&mut self) -> JObject {
        return self
            .ws()
            .or_else(|| self.number())
            .or_else(|| self.null())
            .or_else(|| self.symbol())
            .or_else(|| self.list())
            .unwrap_or(JObject::JString("could not parse input".to_string()));
    }

    fn peek(&mut self) -> Option<char> {
        return self.text.peek().map(|(_, c)| *c);
    }

    fn ws(&mut self) -> Option<JObject> {
        loop {
            if self.peek().unwrap_or('e').is_whitespace() {
                self.text.next();
            } else {
                break;
            }
        }
        None
    }

    fn list(&mut self) -> Option<JObject> {
        if let Some(c) = self.peek() {
            if c != '[' {
                return None;
            } else {
                self.text.next();
            }
        }

        let mut result = Vec::new();

        loop {
            self.ws();
            if let Some(first) = self.number().or_else(|| self.symbol()) {
                result.push(first);
            } else {
                break;
            }

            self.ws();
            if let Some(c) = self.peek() {
                if c == ',' {
                    self.text.next();
                } else if c == ']' {
                    break;
                } else {
                    panic!("invalid list syntax 1: {:?}", c)
                }
            } else {
                panic!("invalid list syntax 2");
            }
        }

        return Some(JObject::JList(result));
    }

    fn number(&mut self) -> Option<JObject> {
        if self.peek().unwrap_or('e').is_numeric() {
            if let Some((_, c)) = self.text.next() {
                let n = c.to_string().parse().unwrap();
                return Some(JObject::JNumber(n));
            }
        }
        return None;
    }

    fn null(&mut self) -> Option<JObject> {
        if let Some(_) = self
            .char('n')
            .and_then(|_| self.char('u'))
            .and_then(|_| self.char('l'))
            .and_then(|_| self.char('l'))
        {
            if let Some(_) = self.text.next() {
                return Some(JObject::JNull);
            }
        }
        return None;
    }

    fn char(&mut self, target_char: char) -> Option<char> {
        if self.peek().map(|c| c.eq(&target_char)).unwrap_or(false) {
            if let Some((_, c)) = self.text.next() {
                return Some(c);
            }
        }
        return None;
    }

    fn symbol(&mut self) -> Option<JObject> {
        self.dquote().and_then(|_| self.rest_of_symbol())
    }

    fn dquote(&mut self) -> Option<()> {
        if self.peek().unwrap_or('0').eq(&'"') {
            if let Some(_) = self.text.next() {
                return Some(());
            }
        }
        return None;
    }

    fn rest_of_symbol(&mut self) -> Option<JObject> {
        let mut text = String::new();
        loop {
            if let Some((_, ch)) = self.text.next() {
                if ch == '"' {
                    break;
                }
                text.push(ch);
            } else {
                panic!("oh no u forgot quote");
            }
        }
        return Some(JObject::JSymbol(text));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list() {
        assert_eq!(parse("[]"), JObject::JList(vec![]));
        assert_eq!(parse("[1]"), JObject::JList(vec![JObject::JNumber(1)]));
        assert_eq!(
            parse("[1, 2]"),
            JObject::JList(vec![JObject::JNumber(1), JObject::JNumber(2)])
        );
    }
}

impl JObject {
    pub fn new_func(arguments: Vec<&str>, body: JObject) -> JObject {
        JObject::JFunc {
            arguments: arguments.iter().map(|&arg| arg.to_string()).collect(),
            definition: Box::new(body),
        }
    }
}
