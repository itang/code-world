/**
 * fibonacci go version
 */

package main

import "fmt"

func Fib(i int) int {
	if i == 0 || i == 1 {
		return i
	}
	return Fib(i-1) + Fib(i-2)
}

func main() {
	for i, v := range fibs(20) {
		fmt.Printf("Fib(%d)=%d\n", i, v)
	}

	assert(Fib(19) == 4181)
}

/////////////////////////////////////////////////
func fibs(max int) (ret []int) {
	ret = make([]int, 0, max)
	for i := 0; i < max; i++ {
		ret = append(ret, Fib(i))
	}
	return
}

func assert(b bool) {
	if !b {
		panic("Assert fail!")
	}
}
