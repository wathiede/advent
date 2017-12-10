package main

import "testing"

func TestSum1(t *testing.T) {
	for _, ts := range []struct {
		in   string
		want int
	}{
		{in: "1122", want: 3},
		{in: "1111", want: 4},
		{in: "1234", want: 0},
	} {
		if got := sum1(ts.in); got != ts.want {
			t.Errorf("sum(%q) = %d, want %d", ts.in, got, ts.want)
		}
	}
}

func TestSum2(t *testing.T) {
	for _, ts := range []struct {
		in   string
		want int
	}{
		{in: "1212", want: 6},
		{in: "1221", want: 0},
		{in: "123425", want: 4},
		{in: "123123", want: 12},
		{in: "12131415", want: 4},
	} {
		if got := sum2(ts.in); got != ts.want {
			t.Errorf("sum(%q) = %d, want %d", ts.in, got, ts.want)
		}
	}
}
