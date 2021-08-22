# What?
```json
>  ["print", 1, 2, 3]
1 2 3
$it = [6,null]
> [["lambda", ["x"], ["print", 1, "x", 3]], 2]
1 2 3
$it = [6,null]
> [["lambda", ["x"], ["lambda", ["y"], ["print", "x", "y"]], 1], 2]
$it = ["lambda",["y"],["print",2,"y"]]
> [[["lambda", ["x"], ["lambda", ["y"], ["print", "x", "y"]], 1], 2]]
2 2
> ["define", "my-macro", [[["macro", [], ["macro", ["x"], ["macro", ["y"], "x"]]]], ["print", {"hello":"world"}]]]
$it = [["macro",["x"],["macro",["y"],["print",{"hello":"world"}]]],["print",{"hello":"world"}]]
> ["my-macro", ["define", "foo", 12]]
{"hello":"world"}
$it = [18,null]
> ["define", "my-macro", [[["macro", [], ["macro", ["x"], ["macro", ["y"], "y"]]]], ["print", {"hello":"world"}]]]
$it = [["macro",["x"],["macro",["y"],"y"]],["print",{"hello":"world"}]]
> ["my-macro", ["define", "foo", 12]]
$it = 12
> ["assert=", "foo", 12]
$it = true
> 
```

# Why?
Because it's hilarious!

# How?
```bash
git clone git@github.com:arwn/jl.git
cd jl
go build
./jl
```
