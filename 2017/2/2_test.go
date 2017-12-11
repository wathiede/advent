package main

import "testing"

func TestChecksum(t *testing.T) {
	var spreadsheet = `5 1 9 5
7 5 3
2 4 6 8`
	want := 18
	if got := checksum(spreadsheet); got != want {
		t.Errorf("checksum() = %d; want %d", got, want)
	}
}
