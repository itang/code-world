package main

import "fmt"

func Fib(i int) int {
	if i == 0 || i == 1 {
		return i
	}
	return Fib(i-1) + Fib(i-2)
}

func main() {
	assert(Fib(0) == 0)
	assert(Fib(1) == 1)
	assert(Fib(2) == 1)
	assert(Fib(3) == 2)
	assert(Fib(4) == 3)
	assert(Fib(5) == 5)
	assert(Fib(6) == Fib(5)+Fib(4))

	const len = 20
	var i = 0
	var ret = make([]int, len)
	for i < len {
		ret = append(ret, Fib(i))
		fmt.Printf("Fib(%d)=%d ", i, Fib(i))
		i += 1
		if i%5 == 0 {
			fmt.Println()
		}
	}
	for _, v := range ret {
		fmt.Printf("%d ", v)
	}
}

func assert(t bool) {
	if !t {
		panic("Assert Failed!")
	}
}
