// Package twofer completes the Twofer exercism project
package twofer

import "fmt"

// ShareWith returns a string that involves the name
func ShareWith(name string) string {
	if name == "" {
		name = "you"
	}
	return fmt.Sprintf("One for %v, one for me.", name)
}
