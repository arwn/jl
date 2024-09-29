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
    match o {
        JObject::List(list) => match list.split_first() {
            Some((
                JObject::Func {
                    parameters,
                    definition,
                },
                tl,
            )) => {
                let arguments = tl.iter().map(|to_eval| eval(e, to_eval)).collect();
                apply_f(e, *definition.clone(), parameters.clone(), arguments)
            }
            Some((
                JObject::Macro {
                    arguments,
                    definition,
                },
                tl,
            )) => apply_f(e, *definition.clone(), arguments.clone(), tl.to_vec()),
            Some((JObject::String(s), tl)) => {
                if e.symbols.contains_key(s) {
                    let new_head = eval(e, &JObject::String(s.to_string()));
                    let new_list = [vec![new_head], tl.to_vec()].concat();
                    return eval(e, &JObject::List(new_list));
                }
                let res = call_builtin(e, s, tl);
                if let Some(o) = res {
                    o
                } else {
                    JObject::Null
                }
            }
            Some((JObject::List(l), tl)) => {
                // let list = vec![vec![eval(e, h)], tl.to_vec()].concat();
                let hd = eval(e, &JObject::List(l.to_vec()));
                let new_list = [vec![hd], tl.to_vec()].concat();
                eval(e, &JObject::List(new_list))
            }
            Some(x) => {
                println!("1st element of list is not function-like: {:?}", x);
                JObject::Null
            }
            None => JObject::Null,
        },

        JObject::Map(m) => {
            let mut new_map = HashMap::new();
            for (k, v) in m {
                let new_v = eval(e, v);
                new_map.insert(k.to_string(), Box::new(new_v));
            }
            JObject::Map(new_map)
        }

        JObject::String(s) => e
            .symbols
            .get(s)
            .unwrap_or(&JObject::String(s.to_string()))
            .clone(),

        e => e.clone(),
    }
}

fn apply_f(
    e: &mut Environment,
    definition: JObject,
    parameters: Vec<String>,
    arguments: Vec<JObject>,
) -> JObject {
    let fu = std::iter::zip(parameters, arguments);
    // TODO: Don't insert this into the global symbol table, add scope.
    fu.for_each(|(param, arg)| {
        e.symbols.insert(param, arg.clone());
    });
    eval(e, &definition)
}

fn quasiwalk(env: &mut Environment, o: &JObject) -> JObject {
    if let JObject::List(l) = o {
        if l.len() > 1 && l[0] == JObject::String("unquote".to_string()) {
            return eval(env, &l[1].clone());
        }
        // else we check if anything should be spliced
        let mut done = Vec::new();
        for x in &mut l.iter() {
            if let JObject::List(l) = x {
                if let Some(JObject::String(s)) = l.first() {
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

fn call_builtin(env: &mut Environment, fname: &str, args: &[JObject]) -> Option<JObject> {
    match fname {
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
            if let JObject::String(s) = args[0].clone() {
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
                        if let JObject::String(s) = x {
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
                        if let JObject::String(s) = x {
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
        JObject::new_func(vec!["x"], JObject::String("x".to_string())),
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
