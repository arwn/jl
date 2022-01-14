package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"os"
)

func main() {
	if len(os.Args) == 1 {
		startRepl()
	} else {
		for _, v := range os.Args[1:] {
			program := readFile(v)
			eval(program)
		}
	}

}

func startRepl() {
	for true {
		read := newReader(os.Stdin)
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
		json.Unmarshal(input, &program)
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
	return newJLObject(program)
}

func jprint(o JLObject) {
	fmt.Println(string(o.String()))
}
