// Package scrabble score calculates a scrabble score
package scrabble

// Score calculates a scrabble score
func Score(word string) (score int) {
	for _, char := range word {
		switch char {
		case 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T', 'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't':
			score++
		case 'D', 'G', 'd', 'g':
			score += 2
		case 'B', 'C', 'M', 'P', 'b', 'c', 'm', 'p':
			score += 3
		case 'F', 'H', 'V', 'W', 'Y', 'f', 'h', 'v', 'w', 'y':
			score += 4
		case 'K', 'k':
			score += 5
		case 'J', 'X', 'j', 'x':
			score += 8
		case 'Q', 'Z', 'q', 'z':
			score += 10
		}
	}
	return
}
