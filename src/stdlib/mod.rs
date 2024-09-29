use crate::json::ToJObject;
use crate::list;
use crate::{json::JObject, Environment};

pub mod io;

pub fn load_mod(env: &mut Environment) {
    env.import_builtin("println", |_env, o| {
        for arg in o {
            println!("{}", arg);
        }
        JObject::Null
    });

    env.import_builtin("type", |env, o| {
        let evaled: Vec<JObject> = o.iter().map(|x| crate::eval(env, x)).collect();
        if evaled.len() == 1 {
            evaled[0].typename().to_jobject()
        } else {
            let x: Vec<JObject> = evaled.iter().map(|x| x.typename().to_jobject()).collect();
            JObject::List(x)
        }
    });

    env.import_builtin("quote", |_env, o| {
        if o.len() != 1 {
            return list!["error", "badarity"];
        }
        assert!(o.len() == 1);
        o[0].clone()
    });

    env.import_builtin("quasiquote", |env, o| {
        if o.len() != 1 {
            return list!["error", "badarity"];
        }
        quasiwalk(env, &o[0])
    });

    env.import_builtin("def", |env, o| {
        if o.len() != 2 {
            return list!["error", "badarity"];
        }
        if let JObject::String(s) = o[0].clone() {
            let body = crate::eval(env, &o[1]);
            env.symbols.insert(s, body.clone());
            body
        } else {
            panic!("you can't assign a non-string to a value");
        }
    });

    env.import_builtin("f", |_env, o| {
        if o.len() != 2 {
            return list!["error", "badarity"];
        }
        if let JObject::List(fbody_args) = o[0].clone() {
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

            JObject::new_func(argsyms, o[1].clone())
        } else {
            JObject::Null
        }
    });

    env.import_builtin("macro", |_env, o| {
        if o.len() != 2 {
            return list!["error", "badarity"];
        }
        if let JObject::List(fbody_args) = o[0].clone() {
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

            JObject::new_macro(argsyms, o[1].clone())
        } else {
            JObject::Null
        }
    });

    env.import_builtin("if", |_env, o| {
        if o.len() != 3 {
            return list!["error", "badarity"];
        }
        let a = o;
        if let &[b, t, f] = &a {
            if truthy(b) {
                t.clone()
            } else {
                f.clone()
            }
        } else {
            JObject::Null
        }
    });

    env.import_builtin("crash", |_env, _o| {
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
