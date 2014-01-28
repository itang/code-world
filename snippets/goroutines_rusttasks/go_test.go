package main

import "testing"

func TestParSum(t *testing.T) {
	if ParSum(100000) != 4999950000 {
		t.Fail()
	}
}
