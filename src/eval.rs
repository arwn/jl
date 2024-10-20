use std::collections::HashMap;
use std::fs;

use crate::json;
use crate::json::JObject;

type JlFn = fn(&mut Environment, &[JObject]) -> JObject;

#[derive(Debug, Clone)]
pub struct Environment {
    pub symbols: HashMap<String, JObject>,
    pub builtins: HashMap<String, JlFn>,
}

impl Environment {
    pub fn init() -> Environment {
        let env = Environment {
            symbols: HashMap::new(),
            builtins: HashMap::new(),
        };
        env
    }

    pub fn insert_builtin(
        &mut self,
        fname: &str,
        fbody: fn(&mut Environment, &[JObject]) -> JObject,
    ) {
        self.builtins.insert(fname.to_string(), fbody);
    }
}

pub fn eval(e: &mut Environment, o: &JObject) -> JObject {
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
                    parameters: arguments,
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

pub fn run_file(env: &mut Environment, path: &str) -> Result<(), std::io::Error> {
    let program = fs::read_to_string(path)?;
    eval(env, &json::parse(&program));
    Ok(())
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

fn call_builtin(env: &mut Environment, fname: &str, args: &[JObject]) -> Option<JObject> {
    if env.builtins.contains_key(fname) {
        return Some(env.builtins.get(fname)?(env, args));
    }
    None
}
