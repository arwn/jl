package main

import (
	"encoding/json"
	"fmt"
	"strings"
)

type JLObjectType uint
type JLFunc func(self JLObject, args ...JLObject) JLObject

const (
	JLNumberType JLObjectType = iota
	JLStringType
	JLArrayType
	JLNullType
)

func typeCheck(o JLObject, t JLObjectType, failMessage string) {
	if o.Type() != t {
		panic(failMessage)
	}
}

type JLObject interface {
	Type() JLObjectType
	String() string
}

func newJLObject(a interface{}) JLObject {
	switch a.(type) {
	case int, float64:
		return JlNumber{a.(float64)}
	case string:
		return JLString{a.(string)}
	case nil:
		return JLNull{}
	case []interface{}:
		old := a.([]interface{})
		newArray := JLArray{make([]JLObject, len(old))}
		for i := range old {
			newArray.Data[i] = newJLObject(old[i])
		}
		return newArray
	}
	panic(fmt.Sprintf("Can't turn %v of type %T into a json object", a, a))
}

type JlNumber struct {
	Data float64
}

func (self JlNumber) Type() JLObjectType {
	return JLNumberType
}

func (self JlNumber) String() string {
	a, _ := json.Marshal(self.Data)
	return string(a)
}

type JLString struct {
	Data string
}

func (self JLString) Type() JLObjectType {
	return JLStringType
}

func (self JLString) String() string {
	a, _ := json.Marshal(self.Data)
	return string(a)
}

type JLArray struct {
	Data []JLObject
}

func (self JLArray) Type() JLObjectType {
	return JLArrayType
}

func (self JLArray) String() string {
	marshaled := make([]string, len(self.Data))
	for i := range self.Data {
		marshaled[i] = self.Data[i].String()
	}
	return "[" + strings.Join(marshaled, ", ") + "]"
}

type JLNull struct {
	Data []JLObject
}

func (self JLNull) Type() JLObjectType {
	return JLNullType
}

func (self JLNull) String() string {
	a, _ := json.Marshal(self.Data)
	return string(a)
}
