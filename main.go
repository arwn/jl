package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"strings"
)

const (
	interactive = iota
	script
)

var symbolTable map[string]interface{}
var runMode int

func main() {
	var reader *bufio.Reader
	symbolTable = make(map[string]interface{})
	symbolTable["lambda"] = "lambda"

	if len(os.Args) == 1 {
		runMode = interactive
		reader = bufio.NewReader(os.Stdin)
		for {
			fmt.Print("> ")
			input := readLine(reader)
			x := eval(input)
			fmt.Print("$it = ")
			jprint(x)
		}
	} else if len(os.Args) == 2 {
		runMode = script
		file, err := os.Open(os.Args[1])
		if err != nil {
			die(err.Error(), 1)
		}
		reader = bufio.NewReader(file)
		source := readAll(reader)
		eval(source)
	} else {
		fmt.Printf("too many args: %s\n", os.Args[1])
		die(fmt.Sprintf("usage: %s [filename]", os.Args[0]), 1)
	}

}

func die(msg string, opcode int) {
	fmt.Printf("\n%s\n", msg)
	os.Exit(opcode)
}

func readLine(r *bufio.Reader) interface{} {
	line, err := r.ReadString('\n')
	if err != nil {
		if err == io.EOF {
			die("goodbye :)", 0)
		}
		panic(err)
	}
	line = strings.TrimSuffix(line, "\n")
	var js interface{}
	err = json.Unmarshal([]byte(line), &js)
	if err != nil {
		fmt.Println(err)
	}
	return js
}

func readAll(r *bufio.Reader) interface{} {
	crap, err := ioutil.ReadAll(r)
	if err != nil {
		panic(err)
	}
	var js interface{}
	err = json.Unmarshal(crap, &js)
	if err != nil {
		die(err.Error(), 1)
	}
	return js
}

func jprint(x ...interface{}) (int, error) {
	for i := range x {
		js, err := json.Marshal(x[i])
		if err != nil {
			panic(err)
		}
		x[i] = string(js)
	}

	return fmt.Println(x...)
}

func eval(source interface{}) interface{} {
	switch source.(type) {
	case string:
		return symbolTable[source.(string)]
	case []interface{}:
		return apply(source.([]interface{}))
	default:
		return source
	}
}

func apply(list []interface{}) interface{} {
	if len(list) < 1 {
		return nil
	}

	switch list[0].(type) {
	case []interface{}: // a list to be evaluated
		for i := range list {
			list[i] = eval(list[i])
		}
		list0 := list[0].([]interface{})
		switch list0[0].(type) {
		case string:
			switch list0[0].(string) {
			case "lambda":
				return applyLambda(list)
			}
		}

	case string:
		str := symbolTable[list[0].(string)]
		if str != nil && str != "lambda" {
			list[0] = str
			return apply(list)
		}
		return applyBif(list)
	}
	panic("apply fell off")
}

// takes a lambda, and applies it to (cdr list)
func applyLambda(list []interface{}) interface{} {
	lambda := list[0].([]interface{})
	cdr := list[1:]

	lambdaArgs := lambda[1].([]interface{})
	lambdaBody := lambda[2]

	for i, arg := range lambdaArgs {
		cdr[i] = eval(cdr[i])
		lambdaBody = listWalkSub(lambdaBody, arg, cdr[i])
	}

	return eval(lambdaBody)
}

// recursively subsitutes every instance of `arg` with `newarg` in `list`
func listWalkSub(list interface{}, arg interface{}, newarg interface{}) interface{} {
	if list == nil {
		return nil
	}
	switch list.(type) {
	case string:
		if list == arg {
			list = newarg
		}
	case []interface{}:
		for i := range list.([]interface{}) {
			list.([]interface{})[i] = listWalkSub(list.([]interface{})[i], arg, newarg)
		}
	}
	return list
}

func applyBif(list []interface{}) interface{} {
	switch list[0].(string) {
	case "print":
		for i := range list {
			list[i] = eval(list[i])
		}
		n, err := jprint(list[1:]...)
		return append([]interface{}{}, n, err)
	case "lambda":
		return list
	case "quote":
		return list[1]
	case "apply":
		return apply(eval(list[1]).([]interface{}))
	case "assert":
		ok := eval(list[1]) != nil
		if !ok {
			panic(fmt.Sprintf("assert= failed for %v", list[1]))
		}
		return true
	case "assert=":
		a, b := eval(list[1]), eval(list[2])
		if !similar(a, b) {
			panic(fmt.Sprintf("assert= failed for [%v, %v]", list[1], list[2]))
		}
		return true
	case "program":
		var done interface{}
		for _, e := range list[1:] {
			done = eval(e)
		}
		return done
	case "define":
		symbolTable[list[1].(string)] = eval(list[2])
		return list[2]
	}
	fmt.Printf("could not find function `%v`\n", list[0])
	return nil
}

// similar does a deep compare of any two objects and determines if
// they share the same structure and the same values
func similar(a interface{}, b interface{}) bool {
	switch a.(type) {
	case []interface{}:
		for i := range a.([]interface{}) {
			if !similar(a.([]interface{})[i], b.([]interface{})[i]) {
				return false
			}
		}
	case map[string]interface{}:
		for i := range a.(map[string]interface{}) {
			if !similar(a.(map[string]interface{})[i], b.(map[string]interface{})[i]) {
				return false
			}
		}
	case interface{}:
		return a == b
	}
	return true
}
