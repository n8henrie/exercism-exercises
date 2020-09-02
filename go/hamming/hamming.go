// Package hamming contains functions related to measuring hamming distance between two strings.
package hamming

import (
	"errors"
)

// Distance measures the hamming distance between two strings.
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("Strings are not the same length")
	}

	distance := 0
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			distance++
		}
	}
	return distance, nil
}
