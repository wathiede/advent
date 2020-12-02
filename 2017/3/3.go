package main

import (
	"fmt"
	"log"
	"math"
)

// spiralIndex computes the x, y offsets for a counter-clockwise spiral.  It
// assumes a cartesian coordinate system centered at 0,0 with an index of 1.
func spiralIndex(addr int) (xoff, yoff int) {
	r := math.Sqrt(float64(addr))
	i, f := math.Modf(r)
	log.Printf("addr = %d r = %f i = %f f = %f", addr, r, i, f)
	if f == 0 {
		i := int(i)
		// Bottom right corner.
		return (i - 1) / 2, -(i - 1) / 2
	}
	return 0, 0
}

func stepCount(cell int) int {
	cnt := 0
	return cnt
}
func main() {
	fmt.Println(stepCount(265149))
}
