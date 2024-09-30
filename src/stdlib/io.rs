use crate::{json::JObject, Environment};

pub fn load_mod(env: &mut Environment) {
    env.insert_builtin("println", |env, args| {
        for arg in args {
            let evaled = crate::eval(env, arg);
            println!("{}", evaled);
        }
        JObject::Null
    });
}
