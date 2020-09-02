// Package matrix takes input strings and makes them into a matrix
package matrix

import (
	"fmt"
	"strconv"
	"strings"
)

// Matrix is a matrix of ints
type Matrix [][]int

// New creates a new Matrix
func New(input string) (Matrix, error) {

	matrix := [][]int{}
	rows := strings.Split(input, "\n")

	for _, row := range rows {
		row = strings.TrimSpace(row)
		col := []int{}
		for _, s := range strings.Split(row, " ") {
			num, err := strconv.Atoi(s)
			if err != nil {
				return nil, err
			}
			col = append(col, num)
		}
		matrix = append(matrix, col)
	}
	firstRowLen := len(matrix[0])
	for idx, row := range matrix {
		rowLen := len(row)
		if rowLen != firstRowLen {
			return nil, fmt.Errorf("Expected length %d in row %d but found length %d", firstRowLen, idx, rowLen)
		}
	}

	return matrix, nil
}

// Rows returns the Matrix content as rows
func (m Matrix) Rows() [][]int {
	rows := make([][]int, len(m))
	numCols := len(m[0])
	for idx, vals := range m {
		row := make([]int, numCols)
		copy(row, vals)
		rows[idx] = row
	}
	return rows
}

// Cols returns the Matrix content as columns
func (m Matrix) Cols() [][]int {
	numCols := len(m)
	numRows := len(m[0])

	rows := make([][]int, numRows)
	for ri := range rows {
		col := make([]int, numCols)
		for ci := range col {
			col[ci] = m[ci][ri]
		}
		rows[ri] = col
	}
	return rows
}

// Set set the value at row, col, val
func (m Matrix) Set(row, col, val int) bool {
	numRows := len(m)
	numCols := len(m[0])
	if row < 0 || row > numRows-1 || col < 0 || col > numCols-1 {
		return false
	}
	m[row][col] = val
	return true
}
