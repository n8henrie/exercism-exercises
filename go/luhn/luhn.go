package luhn

// Strings of length 1 or less are not valid. Spaces are allowed in the input, but they should be stripped before checking. All other non-digit characters are disallowed.
// The first step of the Luhn algorithm is to double every second digit, starting from the right. We will be doubling
// If doubling the number results in a number greater than 9 then subtract 9 from the product. The results of our doubling:
// Then sum all of the digits:
// If the sum is evenly divisible by 10, then the number is valid. This number is valid!

import (
	"strconv"
	"strings"
)

// Valid checks whether input is a valid Luhn number
func Valid(input string) bool {
	input = strings.Replace(input, " ", "", -1)
	if len(input) < 2 {
		return false
	}
	sum := 0

	double := len(input)%2 == 0
	for _, char := range input {
		num, err := strconv.Atoi(string(char))
		if err != nil {
			return false
		}

		if double {
			num *= 2
			if num > 9 {
				num -= 9
			}
		}
		sum += num
		double = !double
	}
	return sum%10 == 0
}
