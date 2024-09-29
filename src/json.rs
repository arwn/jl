use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JObject {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    List(Vec<JObject>),
    Map(HashMap<String, Box<JObject>>),

    // other stuff to make json a programming language
    // Symbol(String),
    Func {
        parameters: Vec<String>,
        definition: Box<JObject>,
    },
    Macro {
        arguments: Vec<String>,
        definition: Box<JObject>,
    },
}

pub fn parse(line: &str) -> JObject {
    let x = line.to_string().chars().collect();
    Parser::parse(&mut Parser { text: x, i: 0 }).unwrap_or(JObject::Null)
}

#[derive(Debug)]
struct Parser {
    text: Vec<char>,
    i: usize,
}

impl Parser {
    fn parse(&mut self) -> Option<JObject> {
        self.ws();
        self.number()
            .or_else(|| self.null())
            .or_else(|| self.bool())
            .or_else(|| self.string())
            // .or_else(|| self.symbol())
            .or_else(|| self.list())
            .or_else(|| self.map())
    }

    fn peek(&mut self) -> Option<char> {
        self.text.get(self.i).copied()
    }

    fn ws(&mut self) -> Option<JObject> {
        loop {
            if self.peek().unwrap_or('e').is_whitespace() {
                self.i += 1;
            } else {
                break;
            }
        }
        None
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
            println!("List not terminated {:?} at index {:?}", self.text, self.i);
        }

        Some(JObject::List(builder))
    }

    fn map(&mut self) -> Option<JObject> {
        let mut builder = HashMap::new();

        if let Some('{') = self.peek() {
            self.i += 1;
        } else {
            return None;
        }

        loop {
            self.ws();
            let key = if let Some(JObject::String(s)) = self.string() {
                Some(s)
            } else {
                println!("object key is not string");
                return None;
            }?;
            if self.peek() == Some(':') {
                self.i += 1;
            }
            let value = self.parse()?;
            self.ws();
            builder.insert(key, Box::new(value));
            if self.peek() != Some(',') {
                break;
            }
            self.i += 1;
        }

        self.ws();
        if let Some('}') = self.peek() {
            self.i += 1;
        } else {
            println!("Map not terminated {:?} at index {:?}", self.text, self.i);
        }

        Some(JObject::Map(builder))
    }

    fn number(&mut self) -> Option<JObject> {
        let str = self.text[self.i..]
            .iter()
            .take_while(|c| (&&'0'..=&&'9').contains(&c))
            .collect::<String>();

        if str.is_empty() {
            None
        } else {
            self.i += str.len();
            Some(JObject::Number(str.parse().unwrap()))
        }
    }

    fn null(&mut self) -> Option<JObject> {
        let s: String = self.text.iter().skip(self.i).take(4).collect();

        if s == "null" {
            self.i += 4;
            Some(JObject::Null)
        } else {
            None
        }
    }

    fn bool(&mut self) -> Option<JObject> {
        let t: String = self.text.iter().skip(self.i).take(4).collect();
        let f: String = self.text.iter().skip(self.i).take(5).collect();
        if t == "true" {
            self.i += 4;
            Some(JObject::Bool(true))
        } else if f == "false" {
            self.i += 5;
            Some(JObject::Bool(false))
        } else {
            None
        }
    }

    fn string(&mut self) -> Option<JObject> {
        if let Some('\"') = self.peek() {
            self.i += 1;
            return Some(JObject::String(self.rest_of_string()));
        }

        None
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
                println!("Missing quote, continuing");
                break;
            }
            self.i += 1
        }
        text
    }
}

impl JObject {
    pub fn new_func(arguments: Vec<&str>, body: JObject) -> JObject {
        JObject::Func {
            parameters: arguments.iter().map(|&arg| arg.to_string()).collect(),
            definition: Box::new(body),
        }
    }
    pub fn new_macro(arguments: Vec<&str>, body: JObject) -> JObject {
        JObject::Macro {
            arguments: arguments.iter().map(|&arg| arg.to_string()).collect(),
            definition: Box::new(body),
        }
    }
}
