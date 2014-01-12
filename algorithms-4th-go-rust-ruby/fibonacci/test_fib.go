package main

import "testing"

func TestFib(t *testing.T) {
	data := []struct {
		Input    int
		Expected int
	}{{0, 0}, {1, 1}, {2, 1}, {3, 2}}
	for _, v := range data {
		if Fib(v.Input) != v.Expected {
			t.Errorf("fib(%d) should equal %d", v.Input, v.Expected)
		}
	}
}
