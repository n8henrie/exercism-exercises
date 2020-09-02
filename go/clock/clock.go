package clock

import "fmt"

// Clock represents a click with minutes and hours
type Clock struct {
	hours, minutes int
}

// New creates a new clock from a given number of hours and minutes, which can be positive, negative, and greater than 24 and 60, respectively
func New(h, m int) Clock {
	totalM := 60*h + m
	hours, minutes := totalM/60%24, totalM%60
	if minutes < 0 {
		minutes = 60 + minutes
		hours--
	}
	if hours < 0 {
		hours = 24 + hours
	}
	return Clock{hours, minutes}
}

// Add adds minutes to a Clock
func (c Clock) Add(minutes int) Clock {
	totalM := c.hours*60 + c.minutes + minutes
	return New(0, totalM)
}

// Subtract subtracts minutes from a Clock
func (c Clock) Subtract(minutes int) Clock {
	totalM := c.hours*60 + c.minutes - minutes
	return New(0, totalM)
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.hours, c.minutes)
}
