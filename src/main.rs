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
    return match o {
        JObject::JList(l) => match l.len() {
            0 => o.clone(),
            1 => {
                let empty: Vec<JObject> = Vec::new();
                let new_o = &eval_ast(e, &l[0]);
                apply(e, new_o, &empty)
            }
            _n => {
                let (first, rest) = l.split_first().unwrap();
                let first1 = &eval_ast(e, first);
                let rest1 = &rest.iter().map(|x| eval_ast(e, x)).collect();
                apply(e, first1, rest1)
            }
        },
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
    if let JObject::JFunc {
        arguments: func_args,
        definition: func_def,
    } = func
    {
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
    panic!("cant apply {:?} to {:?}\nenv: {:?}", func, args, new_env);
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
    fn test_func() {
        let env = &mut init();

        let func = JObject::new_func(vec!["x"], JObject::JSymbol("x".to_string()));
        let args = vec![JObject::JNumber(32)];

        let result = apply(env, &func, &args);

        assert_eq!(result, JObject::JNumber(32));
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

    loop {
        let program = read();
        let res = eval(env, &program);
        println!("{:?}", res)
    }
}
