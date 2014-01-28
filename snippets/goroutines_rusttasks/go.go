package main

import (
	"fmt"
	"runtime"
)

func ParSum(to int) (sum int) {
	c := make(chan int)
	times(to, func(i int) {
		go func(c chan<- int) {
			c <- i
		}(c)
	})

	times(to, func(_ int) {
		sum += <-c
	})
	return
}

func main() {
	const N = 100000
	fmt.Printf("ParSum(%d): %d\n", N, ParSum(N))
}

////////////////////////////////////////////////

func init() {
	runtime.GOMAXPROCS(runtime.NumCPU())
}

func times(to int, f func(int)) {
	for i := 0; i < to; i += 1 {
		f(i)
	}
}
