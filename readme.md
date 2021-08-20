# What?
```json
> ["print", 123, 321]
[123 321]
$it = [10 <nil>]
> [["lambda", ["y"], ["lambda", ["x"], ["print", "x", "y"]]], 321, 123]
[321 321]
$it = [10 <nil>]
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
