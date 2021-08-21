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
$it = [4,null]
> [["macro", ["b", "x", "y"], ["b", ["quote", "x"], ["quote", "y"]]], ["lambda", ["t", "f"], "t"], ["print", 1], ["print", 2]]
1
$it = [2,null]
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
