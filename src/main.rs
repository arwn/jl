use core::panic;
use std::collections::HashMap;
use std::io::{self, Write};
mod json;
use json::JObject;

#[derive(Debug, Clone)]
struct Environment {
    symbols: HashMap<String, JObject>,
}

fn eval<'e, 'o>(e: &'e mut Environment, o: &'o JObject) -> JObject {
    return match macroexpand(e, o.clone()) {
        JObject::JList(l) => {
            if l.len() == 0 {
                o.clone()
            } else {
                let rest = if l.len() == 1 {
                    Vec::new()
                } else {
                    l.split_first().unwrap().1.to_vec()
                };

                if let Some(r) = call_builtin(e, &l[0], &rest) {
                    return r;
                } else {
                    let first = eval(e, &l[0]);
                    let first1 = &eval_ast(e, &first);
                    let rest1 = &rest.iter().map(|x| eval_ast(e, x)).collect();
                    return apply(e, first1, rest1);
                }
            }
        }
        _ => eval_ast(e, o),
    };
}

fn eval_ast<'e, 'o>(e: &'e mut Environment, o: &'o JObject) -> JObject {
    return match o {
        JObject::JList(l) => JObject::JList(l.iter().map(|o| eval(e, o)).collect()),
        JObject::JSymbol(s) => e.symbols.get(s).unwrap_or(&JObject::JNull).clone(),
        _ => o.clone(),
    };
}

fn apply(env: &mut Environment, func: &JObject, args: &Vec<JObject>) -> JObject {
    let new_env = &mut env.clone();
    println!("applying {:?}", func);
    match func {
        JObject::JFunc {
            arguments: func_args,
            definition: func_def,
        }
        | JObject::JMacro {
            arguments: func_args,
            definition: func_def,
        } => {
            if func_args.len() != args.len() {
                panic!(
                    "function has an arity of {:?} but {:?} were given",
                    func_args.len(),
                    args.len()
                )
            }
            for (arg, val) in func_args.iter().zip(args) {
                new_env.symbols.insert(arg.to_string(), val.clone());
            }
            return eval(new_env, &func_def);
        }
        _ => panic!("cant apply {:?} to {:?}\nenv: {:?}", func, args, new_env),
    };
}

fn quasiwalk(env: &mut Environment, o: &JObject) -> JObject {
    if let JObject::JList(l) = o {
        if l.len() > 1 && l[0] == JObject::JSymbol("unquote".to_string()) {
            return eval(env, &l[1].clone());
        }
        // else we check if anything should be spliced
        let mut done = Vec::new();
        for x in &mut l.iter() {
            if let JObject::JList(l) = x {
                if let Some(JObject::JSymbol(s)) = l.get(0) {
                    if s == "splice-unquote" {
                        done.push(eval(env, &l[1]));
                    } else {
                        done.push(x.clone());
                    }
                } else {
                    done.push(x.clone());
                }
            } else {
                done.push(x.clone());
            }
        }
        return JObject::JList(done);
    }
    return o.clone();
}

fn macroexpand(env: &mut Environment, o: JObject) -> JObject {
    let mut done = o.clone();
    while let JObject::JList(l) = done.clone() {
        if let Some(JObject::JMacro {
            arguments: args,
            definition,
        }) = l.get(0)
        {
            done = apply(
                env,
                &definition,
                &args
                    .iter()
                    .map(|x| JObject::JString(x.to_string()))
                    .collect(),
            );
        } else {
            break;
        }
    }
    return done;
}

fn call_builtin(env: &mut Environment, head: &JObject, args: &Vec<JObject>) -> Option<JObject> {
    return if let JObject::JSymbol(fname) = head {
        match fname.as_str() {
            "quote" => {
                assert!(args.len() == 1);
                Some(args[0].clone())
            }
            "quasiquote" => {
                assert!(args.len() == 1);
                let walked = quasiwalk(env, &args[0]);
                Some(walked)
            }
            "def" => {
                assert!(args.len() == 2);
                if let JObject::JSymbol(s) = args[0].clone() {
                    let body = eval(env, &args[1]);
                    env.symbols.insert(s, body.clone());

                    Some(body)
                } else {
                    panic!("you can't assign a non-string to a value");
                }
            }
            "f" => {
                assert!(args.len() == 2);
                if let JObject::JList(fbody_args) = args[0].clone() {
                    let argsyms = fbody_args
                        .iter()
                        .map(|x| {
                            if let JObject::JSymbol(s) = x {
                                s.as_str()
                            } else {
                                panic!("can't use {:?} as function arg", x)
                            }
                        })
                        .collect();

                    Some(JObject::new_func(argsyms, args[1].clone()))
                } else {
                    None
                }
            }
            "macro" => {
                assert!(args.len() == 2);
                if let JObject::JList(fbody_args) = args[0].clone() {
                    let argsyms = fbody_args
                        .iter()
                        .map(|x| {
                            if let JObject::JSymbol(s) = x {
                                s.as_str()
                            } else {
                                panic!("can't use {:?} as macro arg", x)
                            }
                        })
                        .collect();

                    Some(JObject::new_macro(argsyms, args[1].clone()))
                } else {
                    None
                }
            }
            "if" => {
                assert!(args.len() == 3);
                let a = &args[..];
                if let &[b, t, f] = &a {
                    if truthy(b) {
                        Some(t.clone())
                    } else {
                        Some(f.clone())
                    }
                } else {
                    None
                }
            }
            "crash" => {
                unsafe { std::ptr::null_mut::<i8>().write(1) };
                None
            }
            _ => None,
        }
    } else {
        None
    };
}

fn truthy(o: &JObject) -> bool {
    match o {
        JObject::JNull => false,
        JObject::JBool(false) => false,
        JObject::JList(l) => l.len() > 0,
        _ => true,
    }
}

fn init() -> Environment {
    let mut env = Environment {
        symbols: HashMap::new(),
    };
    env.symbols.insert("pi".to_string(), JObject::JNumber(3));
    env.symbols.insert(
        "a-string".to_string(),
        JObject::JString("this is a string".to_string()),
    );

    return env;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let env = &mut init();
        env.symbols.insert("x".to_string(), JObject::JNumber(3));

        let expr = json::parse("\"x\"");
        let result = eval(env, &expr);

        assert_eq!(result, JObject::JNumber(3));
    }

    #[test]
    fn test_eval_ast() {
        let e = &mut init();

        e.symbols.insert(
            "x".to_string(),
            JObject::new_func(
                vec![],
                JObject::JString("a funmtciun wer called".to_string()),
            ),
        );
        let result = eval_ast(e, &JObject::JSymbol("x".to_string()));
        let result_eval = eval(e, &JObject::JSymbol("x".to_string()));

        assert_eq!(result, e.symbols.get("x").unwrap().clone());
        assert_eq!(result, result_eval);
    }

    #[test]
    fn test_builtin_def() {
        let env = &mut init();
        eval(env, &json::parse(r#"["def", "e", 3]")"#));
        assert_eq!(*env.symbols.get("e").unwrap(), JObject::JNumber(3));
    }

    #[test]
    fn test_func() {
        let env = &mut init();

        let func = JObject::new_func(vec!["x"], JObject::JSymbol("x".to_string()));
        let args = vec![JObject::JNumber(32)];

        let result = apply(env, &func, &args);

        assert_eq!(result, JObject::JNumber(32));
    }

    #[test]
    fn test_func_literal() {
        let env = &mut init();

        let prog = eval(env, &json::parse(r#"["f", [], "'hello"]"#));
        assert_eq!(
            prog,
            JObject::new_func(vec![], JObject::JString("hello".to_string())),
        );
    }

    #[test]
    fn test_func_args() {
        let env = &mut init();

        env.symbols.insert(
            "f".to_string(),
            JObject::new_func(vec!["x"], JObject::JString("x".to_string())),
        );

        let o = json::parse("[\"f\", 1]");
        assert_eq!(
            o,
            JObject::JList(vec![JObject::JSymbol("f".to_string()), JObject::JNumber(1)])
        )
    }

    // takes in something like ((\ [x] x) 12)
    #[test]
    fn test_func_as_list() {
        let env = &mut init();

        let func = JObject::new_func(vec!["x"], JObject::JSymbol("x".to_string()));
        let list = JObject::JList(vec![func, JObject::JNumber(42)]);

        let result = eval(env, &list);

        assert_eq!(result, JObject::JNumber(42))
    }

    #[test]
    fn test_func_call() {
        let env = &mut init();
        env.symbols.insert(
            "x".to_string(),
            JObject::new_func(
                vec![],
                JObject::JString("a funmtciun wer called".to_string()),
            ),
        );

        let o = json::parse("[\"x\"]");
        assert_eq!(o, JObject::JList(vec![JObject::JSymbol("x".to_string())]));

        let res = eval(env, &o);
        assert_eq!(res, JObject::JString("a funmtciun wer called".to_string()));
    }
}

fn readline() -> String {
    print!("; ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line;
}

fn read() -> JObject {
    let line = readline();
    let program = json::parse(&line);
    return program;
}

fn main() {
    let env = &mut init();

    env.symbols.insert(
        "f0".to_string(),
        JObject::new_func(vec![], JObject::JNumber(12)),
    );

    env.symbols.insert(
        "f1".to_string(),
        JObject::new_func(vec!["x"], JObject::JSymbol("x".to_string())),
    );

    env.symbols.insert("pi".to_string(), JObject::JNumber(3));
    env.symbols.insert(
        "pie".to_string(),
        JObject::JString("3.14159265359".to_string()),
    );

    loop {
        let program = read();
        let res = eval(env, &program);
        println!("{:?}", res)
    }
}
