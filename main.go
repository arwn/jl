package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"os"
)

var symbols = JLMap{Data: make(map[string]JLObject)}
var scriptMode = false
var scriptFileName = "error: file name not set"

func main() {
	if len(os.Args) == 1 {
		startRepl()
	} else {
		scriptMode = true
		for _, v := range os.Args[1:] {
			program := readFile(v)
			eval(program)
		}
	}

}

func startRepl() {
	read := newReader(os.Stdin)
	for true {
		program := read()
		output := eval(program)
		jprint(output)
	}
}

func newReader(r io.Reader) func() JLObject {
	reader := bufio.NewReader(r)
	return func() JLObject {
		input, err := reader.ReadBytes('\n')
		if err != nil {
			panic(err)
		}
		var program interface{}
		err = json.Unmarshal(input, &program)
		if err != nil {
			return newJLObject(err.Error())
		}
		return newJLObject(program)
	}
}

func readFile(filename string) JLObject {
	input, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	var program interface{}
	err = json.Unmarshal(input, &program)
	if err != nil {
		panic(err)
	}
	scriptFileName = filename
	return newJLObject(program)
}

func jprint(o JLObject) {
	fmt.Println(o.String())
}
