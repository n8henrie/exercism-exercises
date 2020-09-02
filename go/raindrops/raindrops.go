// Package raindrops converts numbers to sound
package raindrops

import "strconv"

// Convert turns numbers into sounds
func Convert(num int) string {

	response := ""
	soundMap := map[int]string{
		3: "Pling",
		5: "Plang",
		7: "Plong",
	}

	for k := 3; k <= 7; k += 2 {
		if num%k == 0 {
			response += soundMap[k]
		}
	}
	if response == "" {
		return strconv.Itoa(num)
	}
	return response
}
