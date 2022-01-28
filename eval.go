package main

import (
	"fmt"
	"log"
)

func eval(o JLObject) JLObject {
	if o.Type() != JLArrayType {
		return o
	}
	program := clone(o).(JLArray)
	if len(program.Data) == 0 {
		return program
	}

	car, ok := eval(program.Data[0]).(JLString)
	if !ok {
		panic(fmt.Sprintf("%v is not a string", program.Data[0]))
	}
	cdr := program.Data[1:]
	switch car.Data {
	case "set":
		cdr = evalCdr(cdr)
		typeCheck(cdr, JLStringType, JLAnyType)
		symbols.Put(cdr[0].String(), cdr[1])
		return symbols.Get(cdr[0].String())
	case "get":
		evalCdr(cdr)
		typeCheck(cdr, JLStringType)
		return symbols.Get(cdr[0].String())
	case "dump":
		return symbols
	case "print":
		evalCdr(cdr)
		for _, v := range cdr {
			jprint(v)
		}
		return JLNull{}
	case "program":
		for i, v := range cdr {
			x := eval(v)
			if i == len(cdr)-1 {
				return x
			}
		}
	case "=":
		evalCdr(cdr)
		if eq(cdr...) {
			return JLBool{true}
		}
		return JLBool{false}
	case "+":
		evalCdr(cdr)
		return add(cdr)
	case "test":
		for _, v := range cdr {
			res := eval(v)
			if eq(res, JLBool{false}) && !eq(res, JLNull{}) {
				var msg = ""
				if scriptMode {
					msg = scriptFileName + ": "
				}
				log.Printf("test failed: %v%v\n", msg, v)
				return JLBool{false}
			}
			return JLBool{true}
		}
	default:
		fmt.Println(fmt.Sprintf("Can't find function: %v\n", car))
	}
	return JLNull{}
}

func evalCdr(xs []JLObject) []JLObject {
	done := make([]JLObject, len(xs))
	for i, x := range xs {
		done[i] = eval(x)
	}
	return done
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
	case JLBool:
		return JLBool{o.(JLBool).Data}
	case JLMap:
		return JLMap{o.(JLMap).Data}
	default:
		panic(fmt.Sprintf("type not handled: %T\n", o))
	}
}

func eq(objects ...JLObject) bool {
	arrayEq := func(l, r JLArray) bool {
		if len(l.Data) != len(r.Data) {
			return false
		}
		for i := range l.Data {
			if !eq(l.Data[i], r.Data[i]) {
				return false
			}
		}
		return true
	}
	if len(objects) <= 1 {
		return true
	}
	for _, v := range objects[1:] {
		if v.Type() != objects[0].Type() {
			return false
		}
		if v.Type() == JLArrayType {
			for i := range v.(JLArray).Data {
				if !arrayEq(objects[0].(JLArray), objects[i].(JLArray)) {
					return false
				}
			}
		}
		if v != objects[0] {
			return false
		}
	}
	return true
}

func add(args []JLObject) JLObject {
	sum := 0.0
	typeCheck(args, JLNumberType)
	for i := range args {
		sum += args[i].(JlNumber).Data
	}
	return JlNumber{sum}
}
