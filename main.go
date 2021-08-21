package main

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"reflect"
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
		switch err.(type) {
		case *json.SyntaxError:
			handleUserError(err)
			return nil
		}
		fmt.Println(reflect.TypeOf(err))
		panic(err)
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
	case float64:
		return source
	case func(interface{}) interface{}:
		return source
	case map[string]interface{}:
		return source
	case []interface{}:
		return apply(source.([]interface{}))
	case nil:
		return nil
	}
	fmt.Printf("value: %v", source)
	fmt.Printf(": %s\n", reflect.TypeOf((source)))
	panic("eval fell off")
}

func apply(list []interface{}) interface{} {
	if len(list) < 1 {
		return nil
	}
	switch list[0].(type) {
	case []interface{}:
		return applyLambda(list)
	case string:
		str := symbolTable[list[0].(string)]
		if str != nil {
			list[0] = str
			return apply(list)
		}
		return applyBif(list)
	}
	panic("apply fell off")
}

func applyLambda(list []interface{}) interface{} {
	if !validLambdaForm(list[0]) {
		handleUserError(errors.New(fmt.Sprintf("bad lambda form in `%v`", list)))
		return nil
	}
	lambda := list[0].([]interface{})
	args := list[1:]

	lambdaArgs := lambda[1].([]interface{})
	lambdaBody := lambda[2]

	for i, arg := range lambdaArgs {
		lambdaBody = listWalkSub(lambdaBody, arg, args[i])
	}

	switch lambdaBody.(type) {
	case []interface{}:
		return eval(lambdaBody)
	}

	return lambdaBody
}

func isLambda(x interface{}) bool {
	if list, ok := x.([]interface{}); ok {
		if str, ok := list[0].(string); ok {
			if str == "lambda" {
				return true
			}
		}
	}
	return false
}

func listWalkSub(list interface{}, arg interface{}, newarg interface{}) interface{} {
	if list == nil {
		return nil
	}
	switch list.(type) {
	case []interface{}:
		for i := range list.([]interface{}) {
			list.([]interface{})[i] = listWalkSub(list.([]interface{})[i], arg, newarg)
		}
	case interface{}:
		if list == arg {
			list = newarg
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
		if !validLambdaForm(list) {
			handleUserError(errors.New(fmt.Sprintf("bad lambda form in `%v`", list)))
			return nil
		}
		return list
	case "quote":
		return list[1]
	case "eval":
		return eval(eval(list[1]))
	}
	handleUserError(errors.New(fmt.Sprintf("could not find function `%v`", list[0])))
	return nil
}

func validLambdaForm(x interface{}) bool {
	list, ok := x.([]interface{})
	if !ok || len(list) != 3 || list[0] != "lambda" ||
		reflect.TypeOf(list[1]).Kind() != reflect.Slice {
		return false
	}
	return true
}

func handleUserError(err error) {
	fmt.Println("err:", err)
	if runMode == script {
		die("error in source file", 1)
	}
}
