use core::panic;
use std::collections::HashMap;
use std::io::{self, Write};
mod json;
use json::JObject;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
struct Environment {
    symbols: HashMap<String, JObject>,
}

fn eval(e: &mut Environment, o: &JObject) -> JObject {
    return match macroexpand(e, o.clone()) {
        JObject::List(l) => {
            if l.is_empty() {
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

fn eval_ast(e: &mut Environment, o: &JObject) -> JObject {
    return match o {
        JObject::List(l) => JObject::List(l.iter().map(|o| eval(e, o)).collect()),
        JObject::Symbol(s) => e.symbols.get(s).unwrap_or(&JObject::Null).clone(),
        _ => o.clone(),
    };
}

fn apply(env: &mut Environment, func: &JObject, args: &Vec<JObject>) -> JObject {
    let new_env = &mut env.clone();
    println!("applying {:?}", func);
    match func {
        JObject::Func {
            arguments: func_args,
            definition: func_def,
        }
        | JObject::Macro {
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
            eval(new_env, func_def)
        }
        _ => panic!("cant apply {:?} to {:?}\nenv: {:?}", func, args, new_env),
    }
}

fn quasiwalk(env: &mut Environment, o: &JObject) -> JObject {
    if let JObject::List(l) = o {
        if l.len() > 1 && l[0] == JObject::Symbol("unquote".to_string()) {
            return eval(env, &l[1].clone());
        }
        // else we check if anything should be spliced
        let mut done = Vec::new();
        for x in &mut l.iter() {
            if let JObject::List(l) = x {
                if let Some(JObject::Symbol(s)) = l.first() {
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
        return JObject::List(done);
    }
    o.clone()
}

fn macroexpand(env: &mut Environment, o: JObject) -> JObject {
    let mut done = o.clone();
    while let JObject::List(l) = done.clone() {
        if let Some(JObject::Macro {
            arguments: args,
            definition,
        }) = l.first()
        {
            done = apply(
                env,
                definition,
                &args
                    .iter()
                    .map(|x| JObject::String(x.to_string()))
                    .collect(),
            );
        } else {
            break;
        }
    }
    done
}

fn call_builtin(env: &mut Environment, head: &JObject, args: &[JObject]) -> Option<JObject> {
    return if let JObject::Symbol(fname) = head {
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
                if let JObject::Symbol(s) = args[0].clone() {
                    let body = eval(env, &args[1]);
                    env.symbols.insert(s, body.clone());

                    Some(body)
                } else {
                    panic!("you can't assign a non-string to a value");
                }
            }
            "f" => {
                assert!(args.len() == 2);
                if let JObject::List(fbody_args) = args[0].clone() {
                    let argsyms = fbody_args
                        .iter()
                        .map(|x| {
                            if let JObject::Symbol(s) = x {
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
                if let JObject::List(fbody_args) = args[0].clone() {
                    let argsyms = fbody_args
                        .iter()
                        .map(|x| {
                            if let JObject::Symbol(s) = x {
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
                let a = args;
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
        JObject::Null => false,
        JObject::Bool(false) => false,
        JObject::List(l) => !l.is_empty(),
        _ => true,
    }
}

fn init() -> Environment {
    let mut env = Environment {
        symbols: HashMap::new(),
    };
    env.symbols.insert("pi".to_string(), JObject::Number(3));
    env.symbols.insert(
        "a-string".to_string(),
        JObject::String("this is a string".to_string()),
    );

    env
}

fn readline() -> String {
    print!("; ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn read() -> JObject {
    let line = readline();
    json::parse(&line)
}

const HELP_STR: &str = "Helo!";

fn main() {
    let env = &mut init();

    env.symbols.insert(
        "f0".to_string(),
        JObject::new_func(vec![], JObject::Number(12)),
    );

    env.symbols.insert(
        "f1".to_string(),
        JObject::new_func(vec!["x"], JObject::Symbol("x".to_string())),
    );

    env.symbols.insert("pi".to_string(), JObject::Number(3));
    env.symbols.insert(
        "pie".to_string(),
        JObject::String("3.14159265359".to_string()),
    );

    env.symbols
        .insert("help".to_string(), JObject::String(HELP_STR.to_string()));

    loop {
        let program = read();
        let res = eval(env, &program);
        println!("{:?}", res)
    }
}
