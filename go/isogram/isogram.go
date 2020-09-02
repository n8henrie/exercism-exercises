package isogram

import (
	"strings"
	"unicode"
)

// go test --bench in 0.007s on repeat runs for 1st submission
// go test --bench in 0.007s on repeat runs for 2nd submission
// go test --bench in 0.008s on repeat runs for 3rd submission

// IsIsogram determines if a word is an isogram
func IsIsogram(s string) bool {

	s = strings.ToLower(s)
	m := make(map[rune]bool)
	for _, letter := range s {
		if !unicode.IsLetter(letter) {
			continue
		}
		if m[letter] {
			return false
		}
		m[letter] = true
	}
	return true
}
