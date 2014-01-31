package main

import (
	"fmt"
	"math/rand"
	"sync"
)

type Delayable interface {
	Force() interface{}
	IsForced() bool
}

func Delay(fn func() interface{}) Delayable {
	return &delay{fn: fn}
}

type delay struct {
	value interface{}
	fn    func() interface{}
	mux   sync.RWMutex
}

func (self *delay) Force() interface{} {
	self.mux.Lock()
	defer self.mux.Unlock()

	if self.value == nil {
		self.value = self.fn()
	}

	return self.value
}

func (self *delay) IsForced() bool {
	self.mux.RLock()
	defer self.mux.RUnlock()
	return self.value != nil
}

func main() {
	d := Delay(func() interface{} {
		println("Force....")
		return rand.Int()
	})

	fmt.Printf("%v\n", d)
	fmt.Printf("%v\n", d.IsForced())
	fmt.Printf("%v\n", d.Force())
	fmt.Printf("%v\n", d.IsForced())
	fmt.Printf("%v\n", d.Force())
	fmt.Printf("%v\n", d.IsForced())

	var ret int = d.Force().(int)
	fmt.Printf("%d\n", ret)
}
