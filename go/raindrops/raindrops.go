package raindrops

import "strconv"
import "bytes"

const testVersion = 3

func Convert(input int) string {
	var result bytes.Buffer

	if input%3 == 0 {
		result.WriteString("Pling")
	}

	if input%5 == 0 {
		result.WriteString("Plang")
	}

	if input%7 == 0 {
		result.WriteString("Plong")
	}

	if result.Len() == 0 {
		return strconv.Itoa(input)
	} else {
		return result.String()
	}
}

// Don't forget the test program has a benchmark too.
// How fast does your Convert convert?
