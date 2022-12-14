package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func checksum1(spreadsheet string) int {
	sum := 0
	for _, row := range strings.Split(spreadsheet, "\n") {
		min := math.MaxInt64
		max := math.MinInt64
		for _, col := range strings.Fields(row) {
			v, err := strconv.Atoi(col)
			if err != nil {
				panic(fmt.Sprintf("Failed to parse %q", col))
			}
			if v < min {
				min = v
			}
			if v > max {
				max = v
			}
		}
		sum += max - min
	}
	return sum
}

func checksum2(spreadsheet string) int {
	sum := 0
	for _, row := range strings.Split(spreadsheet, "\n") {
		var cols []int
		for _, col := range strings.Fields(row) {
			v, err := strconv.Atoi(col)
			if err != nil {
				panic(fmt.Sprintf("Failed to parse %q", col))
			}
			cols = append(cols, v)
		}
		for i, a1 := range cols[:len(cols)-1] {
			for _, a2 := range cols[i+1:] {
				var n, d int
				if a2 > a1 {
					n = a2
					d = a1
				} else {
					n = a1
					d = a2
				}
				v := n / d
				r := n - v*d
				if r == 0 {
					sum += v
				}
			}
		}
	}
	return sum
}

func main() {
	spreadsheet := `414	382	1515	319	83	1327	116	391	101	749	1388	1046	1427	105	1341	1590
	960	930	192	147	932	621	1139	198	865	820	597	165	232	417	19	183
	3379	987	190	3844	1245	1503	3151	3349	2844	4033	175	3625	3565	179	3938	184
	116	51	32	155	102	92	65	42	48	91	74	69	52	89	20	143
	221	781	819	121	821	839	95	117	626	127	559	803	779	543	44	369
	199	2556	93	1101	122	124	2714	625	2432	1839	2700	2636	118	2306	1616	2799
	56	804	52	881	1409	47	246	1368	1371	583	49	1352	976	400	1276	1240
	1189	73	148	1089	93	76	3205	3440	3627	92	853	95	3314	3551	2929	3626
	702	169	492	167	712	488	357	414	187	278	87	150	19	818	178	686
	140	2220	1961	1014	2204	2173	1513	2225	443	123	148	580	833	1473	137	245
	662	213	1234	199	1353	1358	1408	235	917	1395	1347	194	565	179	768	650
	119	137	1908	1324	1085	661	1557	1661	1828	1865	432	110	658	821	1740	145
	1594	222	4140	963	209	2782	180	2591	4390	244	4328	3748	4535	192	157	3817
	334	275	395	128	347	118	353	281	430	156	312	386	160	194	63	141
	146	1116	153	815	2212	2070	599	3018	2640	47	125	2292	165	2348	2694	184
	1704	2194	1753	146	2063	1668	1280	615	163	190	2269	1856	150	158	2250	2459`
	fmt.Println(checksum1(spreadsheet))
	fmt.Println(checksum2(spreadsheet))
}
