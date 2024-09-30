use crate::{
    json::{new_list, JObject, ToJObject},
    Environment,
};

pub fn load_mod(env: &mut Environment) {
    env.insert_builtin("contains-key", |env, args| {
        if args.len() != 2 {
            println!("bar arity");
            return new_list(&["error", "bad-arity", &format!("{} != {}", args.len(), 2)]);
        }

        match (crate::eval(env, &args[0]), crate::eval(env, &args[1])) {
            (JObject::Map(map), JObject::String(key)) => map.contains_key(&key).to_jobject(),
            _ => JObject::Bool(false),
        }
    });

    env.insert_builtin("insert", |env, args| {
        if let &[map, key, value] = &args {
            match (
                crate::eval(env, map),
                crate::eval(env, key),
                crate::eval(env, value),
            ) {
                (JObject::Map(map), JObject::String(key), value) => {
                    let mut new_map = map.clone();
                    new_map.insert(key.clone(), Box::new(value.clone()));
                    JObject::Map(new_map)
                }
                (JObject::Map(map), JObject::Number(key), value) => {
                    let mut new_map = map.clone();
                    new_map.insert(key.to_string(), Box::new(value.clone()));
                    JObject::Map(new_map)
                }
                _x => JObject::Null,
            }
        } else {
            JObject::Null
        }
    });
}
