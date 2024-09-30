## JL
It's like json but with functions.

No dependencies. At least 10 unit tests. Fast as hell (probably).
```json
["program",
    ["quote", "Given an integer array nums, return true if any value"],
    ["quote", "appears at least twice in the array, and return false"],
    ["quote", "if every element is distinct."],

    ["import", "std::io"],
    ["import", "std::array"],
    ["import", "std::object"],
    ["import", "std::logic"],

    ["def", "contains-duplicate", ["f", ["array", "cache"],
        ["aux", ["map", "->string", "array"], {}]]],

    ["def", "aux", ["f", ["array", "cache"],
        ["if", "array",
            ["or",
                ["contains-key", "cache", ["head", "array"]],
                ["aux",
                    ["tail", "array"],
                    ["insert",
                        "cache",
                        ["head", "array"],
                        true]]],
            false]]],


    ["println", ["contains-duplicate", ["quote", [1,2,3]]]],
    ["println", ["contains-duplicate", ["quote", [1,2,1]]]]]
```

```shell
; ["def", "id", ["f", ["x"], "x"]]
["f", [x], "x"]
; ["type", "id"]
"Func"
; ["def", "pi", 3]
3
; ["id", "pi"]
3
; ["quasiquote", [1, ["splice-unquote", "pi"], 2]]
[1,3,2]
; ["def", "ignore", ["macro", ["x"], []]]
["macro", [x], []]
; ["ignore", ["crash"]]
[]
; ["crash"]
Segmentation fault: 11
```
## FAQ
Here are some frequently asked questions:
- What utility does this have?
- Why didn't you write it in Javascript?
- How can I get started with JL in my company?
- Aren't there better things you should be doing with your time?

## Getting Started
Requirements:
- rust

```shell
% cargo build --release
% ./target/release/jl examples/types.json
```
