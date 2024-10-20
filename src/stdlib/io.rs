use crate::eval::eval;
use crate::{eval::Environment, json::JObject};

pub fn load_mod(env: &mut Environment) {
    env.insert_builtin("println", |env, args| {
        for arg in args {
            let evaled = eval(env, arg);
            println!("{}", evaled);
        }
        JObject::Null
    });
}
