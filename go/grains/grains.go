package grains

import "errors"

// Square takes the number of a chessboard square and returns the number of
// grains of rice on it
func Square(s int) (uint64, error) {
	if s < 1 || s > 64 {
		return 0, errors.New("no square zero")
	}
	return 1 << (s - 1), nil
}

// Total returns the total number of grains of rice on the chessboard
func Total() (sum uint64) {
	return (1 << 64) - 1
}
