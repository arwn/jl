use crate::eval;
use crate::json::{new_list, ToJObject};
use crate::{json::JObject, Environment};

pub mod array;
pub mod io;
pub mod logic;
pub mod object;

pub fn import_builtin_library(env: &mut Environment, name: &str) -> JObject {
    // TODO: Is there a better way to import libraries?
    match name {
        "std::io" => io::load_mod(env),
        "std::array" => array::load_mod(env),
        "std::object" => object::load_mod(env),
        "std::logic" => logic::load_mod(env),
        _ => {
            println!("builtin library not found: {}", name);
            return new_list(&["error", "bad-import"]);
        }
    }
    "ok".to_jobject()
}

pub fn load_mod(env: &mut Environment) {
    env.insert_builtin("import", |env, args| {
        let mut last_import = "ok".to_jobject();
        for arg in args {
            if let JObject::String(s) = arg {
                last_import = import_builtin_library(env, s);
            } else {
                println!("{}: not module name", arg);
                return new_list(&["error", "bad-import"]);
            }
        }
        last_import
    });

    env.insert_builtin("type", |env, args| {
        let evaled: Vec<JObject> = args.iter().map(|x| crate::eval(env, x)).collect();
        if evaled.len() == 1 {
            evaled[0].typename().to_jobject()
        } else {
            let x: Vec<JObject> = evaled.iter().map(|x| x.typename().to_jobject()).collect();
            JObject::List(x)
        }
    });

    env.insert_builtin("->string", |env, args| {
        if args.len() != 1 {
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 1)]);
        }
        JObject::String(eval(env, &args[0]).to_string())
    });

    env.insert_builtin("quote", |_env, args| {
        if args.len() != 1 {
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 1)]);
        }
        args[0].clone()
    });

    env.insert_builtin("quasiquote", |env, args| {
        if args.len() != 1 {
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 1)]);
        }
        quasiwalk(env, &args[0])
    });

    env.insert_builtin("def", |env, args| {
        if args.len() != 2 {
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 2)]);
        }
        if let JObject::String(s) = args[0].clone() {
            let body = crate::eval(env, &args[1]);
            env.symbols.insert(s, body.clone());
            body
        } else {
            panic!("you can't assign a non-string to a value");
        }
    });

    env.insert_builtin("f", |_env, args| {
        if args.len() != 2 {
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 2)]);
        }
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

            JObject::new_func(argsyms, args[1].clone())
        } else {
            JObject::Null
        }
    });

    env.insert_builtin("macro", |_env, args| {
        if args.len() != 2 {
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 2)]);
        }
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

            JObject::new_macro(argsyms, args[1].clone())
        } else {
            JObject::Null
        }
    });

    env.insert_builtin("program", |env, args| {
        let mut last_expression = JObject::Null;
        for arg in args {
            last_expression = crate::eval(env, arg)
        }
        last_expression
    });

    env.insert_builtin("crash", |_env, _args| {
        unsafe { std::ptr::null_mut::<i8>().write(1) };
        JObject::Null
    });
}

fn quasiwalk(env: &mut Environment, o: &JObject) -> JObject {
    if let JObject::List(l) = o {
        if l.len() > 1 && l[0] == "unquote".to_jobject() {
            return crate::eval(env, &l[1].clone());
        }
        // else we check if anything should be spliced
        let mut done = Vec::new();
        for x in &mut l.iter() {
            if let JObject::List(l) = x {
                if let Some(JObject::String(s)) = l.first() {
                    if s == "splice-unquote" {
                        done.push(crate::eval(env, &l[1]));
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

fn truthy(o: &JObject) -> bool {
    match o {
        JObject::Null => false,
        JObject::Bool(false) => false,
        JObject::List(l) => !l.is_empty(),
        _ => true,
    }
}
