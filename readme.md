# What?
```json
> ["print", 1, 2, 3]
1 2 3
$it = [6 <nil>]
> [["lambda", ["x"], ["print", 1, "x", 3]], 2]
1 2 3
$it = [6 <nil>]
> [["lambda", ["x"], ["lambda", ["y"], ["print", "x", "y"]], 1], 2]
$it = [lambda [y] [print 2 y]]
> [["lambda", ["x"], [["lambda", ["y"], ["print", "x", "y"]], 1]], 2]
2 1
$it = [4 <nil>]
> [["macro", ["b", "x", "y"], ["b", ["quote", "x"], ["quote", "y"]]], ["lambda", ["t", "f"], "t"], ["print", 1], ["print", 2]]
1
$it = [2,null]
> 
```

# Why?
Json was inspired by lisp so it's only natural

# How?
```bash
git clone git@github.com:arwn/gross.git
cd gross
go build
./gross
```
