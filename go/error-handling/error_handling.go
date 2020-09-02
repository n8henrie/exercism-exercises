// Package erratum demonstrates error handling in go
package erratum

import (
	"errors"
	"fmt"
)

// Use opens a resource, handles errors, and closes it
func Use(o ResourceOpener, input string) (err error) {
	var transientError TransientError

	resource, err := o()
	for err != nil {
		if !errors.As(err, &transientError) {
			return err
		}
		resource, err = o()
	}

	defer resource.Close()
	defer func() {
		if r := recover(); r != nil {
			if frob, ok := r.(FrobError); ok {
				resource.Defrob(frob.defrobTag)
			}
			if rerr, ok := r.(error); ok {
				err = rerr
			} else {
				err = fmt.Errorf("%v", r)
			}
		}
	}()

	resource.Frob(input)
	return nil
}
