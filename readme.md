## JL
It's like json but with functions.

No dependencies. At least 10 unit tests. Fast as hell (probably).

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
