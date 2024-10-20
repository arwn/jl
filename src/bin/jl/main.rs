use std::env::args;
use std::io::{self, Write};

use jllib::{
    eval::{self, Environment},
    json::{self, JObject},
    stdlib,
};

fn main() -> Result<(), io::Error> {
    let env = &mut Environment::init();

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

    stdlib::load_mod(env);

    if args().len() == 1 {
        mainloop(env)
    } else if args().len() == 2 {
        let args: Vec<String> = args().collect();
        return eval::run_file(env, &args[1]);
    } else {
        println!("invald # of args: {}", args().len())
    }
    Ok(())
}

fn readline() -> String {
    print!("; ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn mainloop(env: &mut Environment) {
    loop {
        let program = json::parse(&readline());
        let res: JObject = eval::eval(env, &program);
        println!("{}", res)
    }
}
