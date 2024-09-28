use super::*;
use crate::json;

#[test]
fn test_env() {
    let env = &mut init();
    env.symbols.insert("x".to_string(), JObject::Number(3));

    let expr = json::parse("\"x\"");
    let result = eval(env, &expr);

    assert_eq!(result, JObject::Number(3));
}

#[test]
fn test_eval_ast() {
    let e = &mut init();

    e.symbols.insert(
        "x".to_string(),
        JObject::new_func(
            vec![],
            JObject::String("a funmtciun wer called".to_string()),
        ),
    );
    let result = eval_ast(e, &JObject::Symbol("x".to_string()));
    let result_eval = eval(e, &JObject::Symbol("x".to_string()));

    assert_eq!(result, e.symbols.get("x").unwrap().clone());
    assert_eq!(result, result_eval);
}

#[test]
fn test_builtin_def() {
    let env = &mut init();
    eval(env, &json::parse(r#"["def", "e", 3]")"#));
    assert_eq!(*env.symbols.get("e").unwrap(), JObject::Number(3));
}

#[test]
fn test_func() {
    let env = &mut init();

    let func = JObject::new_func(vec!["x"], JObject::Symbol("x".to_string()));
    let args = vec![JObject::Number(32)];

    let result = apply(env, &func, &args);

    assert_eq!(result, JObject::Number(32));
}

#[test]
fn test_quasiquote() {
    let env = &mut init();
    let cmd = r#"["quasiquote", [1, ["splice-unquote", "pi"], 2]]"#;
    let o = json::parse(cmd);
    let new_o = eval(env, &o);
    assert!(new_o == json::parse(r#"[1,3,2]"#))
}

#[test]
fn test_func_literal() {
    let env = &mut init();

    let prog = eval(env, &json::parse(r#"["f", [], "'hello"]"#));
    assert_eq!(
        prog,
        JObject::new_func(vec![], JObject::String("hello".to_string())),
    );
}

#[test]
fn test_func_args() {
    let env = &mut init();

    env.symbols.insert(
        "f".to_string(),
        JObject::new_func(vec!["x"], JObject::String("x".to_string())),
    );

    let o = json::parse("[\"f\", 1]");
    assert_eq!(
        o,
        JObject::List(vec![JObject::Symbol("f".to_string()), JObject::Number(1)])
    )
}

// takes in something like ((\ [x] x) 12)
#[test]
fn test_func_as_list() {
    let env = &mut init();

    let func = JObject::new_func(vec!["x"], JObject::Symbol("x".to_string()));
    let list = JObject::List(vec![func, JObject::Number(42)]);

    let result = eval(env, &list);

    assert_eq!(result, JObject::Number(42))
}

#[test]
fn test_func_call() {
    let env = &mut init();
    env.symbols.insert(
        "x".to_string(),
        JObject::new_func(
            vec![],
            JObject::String("a funmtciun wer called".to_string()),
        ),
    );

    let o = json::parse("[\"x\"]");
    assert_eq!(o, JObject::List(vec![JObject::Symbol("x".to_string())]));

    let res = eval(env, &o);
    assert_eq!(res, JObject::String("a funmtciun wer called".to_string()));
}

// json stuff

#[test]
fn test_bool() {
    assert_eq!(json::parse("true"), JObject::Bool(true));
    assert_eq!(json::parse("false"), JObject::Bool(false));
}

#[test]
fn test_parse_list() {
    assert_eq!(json::parse("[]"), JObject::List(vec![]));
    assert_eq!(json::parse("[1]"), JObject::List(vec![JObject::Number(1)]));
    assert_eq!(
        json::parse("[1, 2]"),
        JObject::List(vec![JObject::Number(1), JObject::Number(2)])
    );

    assert_eq!(
        json::parse("[[]]"),
        JObject::List(vec![JObject::List(vec![])])
    );
    assert_eq!(
        json::parse("[[[]]]"),
        JObject::List(vec![JObject::List(vec![JObject::List(vec![])])])
    );
    assert_eq!(
        json::parse("[[1]]"),
        JObject::List(vec![JObject::List(vec![JObject::Number(1)])])
    );
    assert_eq!(
        json::parse("[[1, 1]]"),
        JObject::List(vec![JObject::List(vec![
            JObject::Number(1),
            JObject::Number(1)
        ])])
    );
    assert_eq!(
        json::parse("[[1], 1]"),
        JObject::List(vec![
            JObject::List(vec![JObject::Number(1)]),
            JObject::Number(1)
        ])
    );
    assert_eq!(
        json::parse(r#"[["f", ["x"], 1], 1]")"#),
        JObject::List(vec![
            JObject::List(vec![
                JObject::Symbol("f".to_string()),
                JObject::List(vec![JObject::Symbol("x".to_string())]),
                JObject::Number(1)
            ]),
            JObject::Number(1)
        ])
    );
}
