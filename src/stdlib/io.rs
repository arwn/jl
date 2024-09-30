use crate::{json::JObject, Environment};

pub fn load_mod(env: &mut Environment) {
    env.insert_builtin("println", |env, args| {
        for arg in args {
            println!("{}", crate::eval(env, arg));
        }
        JObject::Null
    });
}
