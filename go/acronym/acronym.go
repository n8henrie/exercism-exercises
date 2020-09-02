// Package acronym has tools for converting a phrase into an acronym
package acronym

import (
	"strings"
	"unicode"
)

// Abbreviate takes a phrase and turns it into an acronym
func Abbreviate(s string) string {
	var response strings.Builder

	for _, word := range strings.FieldsFunc(s, func(c rune) bool {
		return !unicode.IsLetter(c)
	}) {
		response.WriteByte(word[0])
	}
	return strings.ToUpper(response.String())
}
