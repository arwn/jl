It's like json but with functions. Truly one of the lisps of all time.

```
; ["def", "id", ["f", ["x"], "x"]]
JFunc { arguments: ["x"], definition: JSymbol("x") }
; ["def", "pi", 3]
JNumber(3)
; ["id", "pi"]
JNumber(3)
; ["quasiquote", [1, ["splice-unquote", "pi"], 2]]
JList([JNumber(1), JNumber(3), JNumber(2)])
; ["def", "ignore", ["macro", ["x"], []]]
JMacro { arguments: ["x"], definition: JList([]) }
; ["ignore", ["crash"]]
JList([])
; ["crash"]
Segmentation fault: 11
```