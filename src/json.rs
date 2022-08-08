#[derive(Clone, Debug, PartialEq)]
pub enum JObject {
    JNull,
    JBool(bool),
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
    let x = line.to_string().chars().collect();
    Parser::parse(&mut Parser { text: x, i: 0 }).unwrap_or(JObject::JNull)
}

struct Parser {
    text: Vec<char>,
    i: usize,
}

impl Parser {
    fn parse(&mut self) -> Option<JObject> {
        self.ws();
        return self
            .number()
            .or_else(|| self.null())
            .or_else(|| self.bool())
            .or_else(|| self.string())
            .or_else(|| self.symbol())
            .or_else(|| self.list());
    }

    fn peek(&mut self) -> Option<char> {
        self.text.get(self.i).map(|c| *c)
    }

    fn peek2(&mut self) -> Option<(char, char)> {
        let first = self.text.get(self.i);
        let second = self.text.get(self.i + 1);
        return first.and_then(|c1| second.map(|c2| (*c1, *c2)));
    }

    fn ws(&mut self) -> Option<JObject> {
        loop {
            if self.peek().unwrap_or('e').is_whitespace() {
                self.i += 1;
            } else {
                break;
            }
        }
        return None;
    }

    fn list(&mut self) -> Option<JObject> {
        let mut builder = Vec::new();

        if let Some('[') = self.peek() {
            self.i += 1;
        } else {
            return None;
        }

        loop {
            self.ws();
            if let Some(element) = self.parse() {
                builder.push(element);
                self.ws();
                if let Some(',') = self.peek() {
                    self.i += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.ws();
        if let Some(']') = self.peek() {
            self.i += 1;
        } else {
            panic!("Invalid list in {:?} at index {:?}", self.text, self.i);
        }

        return Some(JObject::JList(builder));
    }

    fn number(&mut self) -> Option<JObject> {
        return self
            .text
            .get(self.i)
            .and_then(|c| c.to_string().parse().ok())
            .map(|n| {
                self.i += 1;
                JObject::JNumber(n)
            });
    }

    fn null(&mut self) -> Option<JObject> {
        let s: String = self.text.iter().skip(self.i).take(4).collect();

        if s == "null" {
            self.i += 4;
            return Some(JObject::JNull);
        } else {
            return None;
        }
    }

    fn bool(&mut self) -> Option<JObject> {
        let t: String = self.text.iter().skip(self.i).take(4).collect();
        let f: String = self.text.iter().skip(self.i).take(5).collect();
        if t == "true" {
            self.i += 4;
            return Some(JObject::JBool(true));
        } else if f == "false" {
            self.i += 5;
            return Some(JObject::JBool(false));
        } else {
            return None;
        }
    }

    fn symbol(&mut self) -> Option<JObject> {
        if let Some((a, b)) = self.peek2() {
            if a == '"' && b != '\'' {
                self.i += 1;
                return Some(JObject::JSymbol(self.rest_of_string()));
            }
        }

        return None;
    }

    fn string(&mut self) -> Option<JObject> {
        if let Some((a, b)) = self.peek2() {
            if a == '"' && b == '\'' {
                self.i += 2;
                return Some(JObject::JString(self.rest_of_string()));
            }
        }

        return None;
    }

    fn rest_of_string(&mut self) -> String {
        let mut text = String::new();
        loop {
            if let Some(ch) = self.peek() {
                if ch == '"' {
                    self.i += 1;
                    break;
                }
                text.push(ch);
            } else {
                panic!("oh no u forgot quote");
            }
            self.i += 1
        }
        return text;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(parse("true"), JObject::JBool(true));
        assert_eq!(parse("false"), JObject::JBool(false));
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(parse("[]"), JObject::JList(vec![]));
        assert_eq!(parse("[1]"), JObject::JList(vec![JObject::JNumber(1)]));
        assert_eq!(
            parse("[1, 2]"),
            JObject::JList(vec![JObject::JNumber(1), JObject::JNumber(2)])
        );

        assert_eq!(parse("[[]]"), JObject::JList(vec![JObject::JList(vec![])]));
        assert_eq!(
            parse("[[[]]]"),
            JObject::JList(vec![JObject::JList(vec![JObject::JList(vec![])])])
        );
        assert_eq!(
            parse("[[1]]"),
            JObject::JList(vec![JObject::JList(vec![JObject::JNumber(1)])])
        );
        assert_eq!(
            parse("[[1, 1]]"),
            JObject::JList(vec![JObject::JList(vec![
                JObject::JNumber(1),
                JObject::JNumber(1)
            ])])
        );
        assert_eq!(
            parse("[[1], 1]"),
            JObject::JList(vec![
                JObject::JList(vec![JObject::JNumber(1)]),
                JObject::JNumber(1)
            ])
        );
        assert_eq!(
            parse(r#"[["f", ["x"], 1], 1]")"#),
            JObject::JList(vec![
                JObject::JList(vec![
                    JObject::JSymbol("f".to_string()),
                    JObject::JList(vec![JObject::JSymbol("x".to_string())]),
                    JObject::JNumber(1)
                ]),
                JObject::JNumber(1)
            ])
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
