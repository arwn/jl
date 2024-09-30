use crate::{
    eval,
    json::{new_list, JObject},
    Environment,
};

pub fn load_mod(env: &mut Environment) {
    env.insert_builtin("head", |env, args| {
        if args.len() != 1 {
            println!("bar arity");
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 1)]);
        }

        match crate::eval(env, &args[0]) {
            JObject::List(vec) => vec.first().unwrap_or(&JObject::Null).clone(),
            _ => JObject::List(Vec::new()),
        }
    });

    env.insert_builtin("tail", |env, args| {
        if args.len() != 1 {
            println!("bar arity");
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 1)]);
        }

        match crate::eval(env, &args[0]) {
            JObject::List(vec) => JObject::List(vec[1..].to_vec()),
            _ => JObject::List(Vec::new()),
        }
    });

    env.insert_builtin("len", |env, args| {
        if args.len() != 1 {
            println!("bar arity");
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 1)]);
        }

        match crate::eval(env, &args[0]) {
            JObject::List(vec) => JObject::Number(vec.len() as i64),
            _ => JObject::Number(1),
        }
    });

    // (map fn array)
    env.insert_builtin("map", |env, args| {
        if args.len() != 2 {
            println!("bar arity");
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 2)]);
        }

        let func = crate::eval(env, &args[0]);
        let array = crate::eval(env, &args[1]);

        match (func, array) {
            (func, JObject::List(array)) => {
                let mut done = Vec::new();
                for element in array {
                    // let funcall = JObject::List(vec![func.clone(), element.clone()]);
                    // let res = eval(env, &funcall);
                    let res = eval(env, &JObject::List(vec![func.clone(), element.clone()]));
                    done.push(res)
                }
                JObject::List(done)
            }
            _ => JObject::Null,
        }
    });
}
