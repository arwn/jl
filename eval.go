package main

import "fmt"

func eval(o JLObject) JLObject {
	if o.Type() != JLArrayType {
		return o
	}
	program := clone(o).(JLArray)
	if len(program.Data) == 0 {
		return program
	}
	for i := range program.Data {
		program.Data[i] = eval(program.Data[i])
	}
	car, ok := eval(program.Data[0]).(JLString)
	if !ok {
		panic(fmt.Sprintf("%v is not a string", program.Data[0]))
	}
	cdr := program.Data[1:]
	switch car.Data {
	case "print":
		for _, v := range program.Data[1:] {
			fmt.Println(v.String())
		}
		return JLNull{}
	case "+":
		return add(cdr)
	default:
		fmt.Println("Can't find function")
	}
	return JLNull{}
}

func clone(o JLObject) JLObject {
	switch o.(type) {
	case JlNumber:
		return JlNumber{o.(JlNumber).Data}
	case JLString:
		return JLString{o.(JLString).Data}
	case JLArray:
		a := o.(JLArray)
		newArray := JLArray{make([]JLObject, len(a.Data))}
		for i := range a.Data {
			newArray.Data[i] = clone(a.Data[i])
		}
		return JLArray{o.(JLArray).Data}
	case JLNull:
		return JLNull{o.(JLNull).Data}
	default:
		panic("type not handled")
	}
}

func add(args []JLObject) JLObject {
	sum := 0.0
	for i := range args {
		typeCheck(args[i], JLNumberType, "can't add a non number")
		sum += args[i].(JlNumber).Data
	}
	return JlNumber{sum}
}
