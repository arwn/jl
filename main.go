package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"os"
	"reflect"
	"strings"
)

var symbolTable map[string]interface{}

func main() {

	log.SetFlags(log.LstdFlags | log.Lshortfile)

	var reader *bufio.Reader

	symbolTable = make(map[string]interface{})
	symbolTable["foo"] = "this-is-a-variable"

	if len(os.Args) == 1 {
		reader = bufio.NewReader(os.Stdin)
		for {
			fmt.Print("> ")
			input := readLine(reader)
			x := eval(input)
			fmt.Printf("$it = %v\n", x)
		}
	} else if len(os.Args) == 2 {
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

func eval(source interface{}) interface{} {
	switch source.(type) {
	case string:
		return symbolTable[source.(string)]
	case float64:
		return source
	case func(interface{}) interface{}:
		return source
	case []interface{}:
		return apply(source.([]interface{}))
	}
	fmt.Printf("value: %v", source)
	fmt.Printf(": %s\n", reflect.TypeOf((source)))
	panic("eval fell off")
}

func apply(list []interface{}) interface{} {
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
	lambda := list[0].([]interface{})
	args := list[1:]

	lambdaArgs := lambda[1].([]interface{})
	lambdaBody := lambda[2]

	for i, arg := range lambdaArgs {
		lambdaArgs[i] = listWalkSub(lambdaBody, arg, args[i])
	}

	switch lambdaBody.(type) {
	case []interface{}:
		if isLambda(lambdaBody) {
			return apply(append(append([]interface{}{}, lambdaBody), args...))
		} else {
			return eval(lambdaBody)
		}
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
		n, err := fmt.Println(list[1:])
		return append([]interface{}{}, n, err)
	case "lambda":
		return list
	}
	panic("applyBif fell off")
}
