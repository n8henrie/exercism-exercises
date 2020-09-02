// Package robotname generates random robot names
package robotname

import (
	"errors"
	"fmt"
	"math/rand"
	"time"
)

// Robot has a name
type Robot struct {
	name string
}

const limit = 26 * 26 * 10 * 10 * 10

var seenNames = make(map[string]bool)

var random = rand.New(rand.NewSource(time.Now().UnixNano()))

func newName() string {
	r1 := random.Intn(26) + 'A'
	r2 := random.Intn(26) + 'A'
	num := random.Intn(1000)
	return fmt.Sprintf("%c%c%03d", r1, r2, num)
}

// Name returns a unique name
func (r *Robot) Name() (string, error) {
	if len(seenNames) == limit {
		return "", errors.New("Out of unique names")
	}
	if r.name != "" {
		return r.name, nil
	}
	r.name = newName()
	for seenNames[r.name] {
		r.name = newName()
	}
	seenNames[r.name] = true
	return r.name, nil
}

// Reset resets the name
func (r *Robot) Reset() {
	r.name = ""
}
