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

var symbolTable map[string]interface{}

func main() {
	var reader *bufio.Reader
	symbolTable = make(map[string]interface{})

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
	case []interface{}:
		return funcall(source.([]interface{})[0].(string), source.([]interface{})[1:])
	case string:
		return source
	case float64:
		return source
	case nil:
		return nil
	}
	fmt.Printf("value: %+v", source)
	panic("eval fell off")
}

func funcall(name string, args []interface{}) interface{} {
	for i, arg := range args {
		args[i] = eval(arg)
	}

	switch name {
	case "program":
		for i, program := range args {
			if i == len(args)-1 {
				return eval(program)
			} else {
				eval(program)
			}
		}

	case "print":
		var sb strings.Builder
		for _, e := range args {
			sb.WriteString(fmt.Sprint(eval(e)))
		}
		s := sb.String()
		fmt.Print(s)
		return s

	case "+":
		return args[0].(float64) + args[1].(float64)

	case "set":
		symbolTable[args[0].(string)] = args[1]
		return args[1]

	case "get":
		return symbolTable[args[0].(string)]

	case "dump":
		return fmt.Sprintf("%+v\n", symbolTable)
	}
	panic("funcall fell through")
}
