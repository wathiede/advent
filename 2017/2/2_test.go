package main

import "testing"

func TestChecksum1(t *testing.T) {
	var spreadsheet = `5 1 9 5
7 5 3
2 4 6 8`
	want := 18
	if got := checksum1(spreadsheet); got != want {
		t.Errorf("checksum1() = %d; want %d", got, want)
	}
}

func TestChecksum2(t *testing.T) {
	var spreadsheet = `5 9 2 8
	9 4 7 3
	3 8 6 5`
	want := 9
	if got := checksum2(spreadsheet); got != want {
		t.Errorf("checksum2() = %d; want %d", got, want)
	}
}
