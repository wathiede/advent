package main

import "testing"

func TestOne(t *testing.T) {
	for _, ts := range []struct {
		in   string
		want int
	}{
		{in: "1122", want: 3},
		{in: "1111", want: 4},
		{in: "1234", want: 0},
		// {in:"91212129 produces
	} {
		if got := sum(ts.in); got != ts.want {
			t.Errorf("sum(%q) = %d, want %d", ts.in, got, ts.want)
		}
	}
}
