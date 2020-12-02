package main

import "testing"

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func TestSpiralIndex(t *testing.T) {
	for _, ts := range []struct {
		idx  int
		x, y int
	}{
		{idx: 1, x: 0, y: 0},
		{idx: 2, x: 1, y: 0},
		{idx: 3, x: 1, y: 1},
		{idx: 4, x: 0, y: 1},
		{idx: 5, x: -1, y: 1},
		{idx: 6, x: -1, y: 0},
		{idx: 7, x: -1, y: -1},
		{idx: 8, x: 0, y: -1},
		{idx: 9, x: 1, y: -1},
		{idx: 10, x: 2, y: -1},
		{idx: 11, x: 2, y: 0},
		{idx: 12, x: 2, y: 1},
		{idx: 13, x: 2, y: 2},
		{idx: 14, x: 1, y: 2},
		{idx: 15, x: 0, y: 2},
		{idx: 16, x: -1, y: 2},
		{idx: 17, x: -2, y: 2},
		{idx: 18, x: -2, y: 1},
		{idx: 19, x: -2, y: 0},
		{idx: 20, x: -2, y: -1},
		{idx: 21, x: -2, y: -2},
		{idx: 22, x: -1, y: -2},
		{idx: 23, x: 0, y: -2},
		{idx: 24, x: 1, y: -2},
		{idx: 25, x: 2, y: -2},
	} {
		if x, y := spiralIndex(ts.idx); x != ts.x || y != ts.y {
			t.Errorf("spiralIndex(%2d) = %2d,%2d; want %2d,%2d d %d", ts.idx, x, y, ts.x, ts.y, abs(ts.x)+abs(ts.y))
		}
	}
}

func TestStepCount(t *testing.T) {
	t.Skip("Skipping until index code works")
	for _, ts := range []struct {
		cell int
		want int
	}{
		{cell: 1, want: 0},
		{cell: 12, want: 3},
		{cell: 23, want: 2},
		{cell: 1024, want: 31},
	} {
		if got := stepCount(ts.cell); got != ts.want {
			t.Errorf("stepCount(%d) = %d; want %d", ts.cell, got, ts.want)
		}
	}
}
