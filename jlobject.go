package main

import (
	"encoding/json"
	"fmt"
	"strings"
)

type JLObjectType uint

const (
	JLNumberType JLObjectType = iota
	JLStringType
	JLArrayType
	JLNullType
	JLBoolType
	JLMapType
	JLAnyType // WART ALERT!!!
)

func typeCheck(os []JLObject, t ...JLObjectType) {
	for i, o := range os {
		if i >= len(t) {
			i = len(t) - 1 // Just use the last element to type check the rest if the args. (maybe a bad idea)
		}
		if o.Type() != t[i] && t[i] != JLAnyType {
			panic(fmt.Sprintf("Mismatched types %s and %s\n", o.Type(), t[i]))
		}
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
	case bool:
		return JLBool{a.(bool)}
	case []interface{}:
		old := a.([]interface{})
		newArray := JLArray{make([]JLObject, len(old))}
		for i := range old {
			newArray.Data[i] = newJLObject(old[i])
		}
		return newArray
	case map[string]interface{}:
		oldMap := a.(map[string]interface{})
		newMap := make(map[string]JLObject)
		for k, v := range oldMap {
			newMap[k] = newJLObject(v)
		}
		return JLMap{newMap}
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

type JLBool struct {
	Data bool
}

func (self JLBool) Type() JLObjectType {
	return JLBoolType
}

func (self JLBool) String() string {
	a, _ := json.Marshal(self.Data)
	return string(a)
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

type JLMap struct {
	Data map[string]JLObject
}

func (self JLMap) Type() JLObjectType {
	return JLMapType
}

func (self JLMap) String() string {
	b := strings.Builder{}
	b.WriteString("{")
	for k, v := range self.Data {
		if b.Len() > 1 {
			b.WriteRune(',')
		}
		b.WriteString(k + ":" + v.String())
	}
	b.WriteString("}")
	return string(b.String())
}

func (self JLMap) Put(where string, what JLObject) {
	self.Data[where] = what
}

func (self JLMap) Get(where string) JLObject {
	return self.Data[where]
}
