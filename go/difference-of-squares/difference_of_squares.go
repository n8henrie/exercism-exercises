package diffsquares

// SumOfSquares returns the sum of the square of the first n natural numbers
func SumOfSquares(n int) (sum int) {
	return n * (n + 1) * (2*n + 1) / 6
}

// SquareOfSum returns the square of the sum of first n natural numbers
func SquareOfSum(n int) int {
	s := n * (n + 1) / 2
	return s * s
}

// Difference returns the difference between the square of sum of n and sum of square of n
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
